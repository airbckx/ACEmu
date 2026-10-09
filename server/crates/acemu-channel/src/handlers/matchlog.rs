//! In-match reports the host sends while a round is running: round ends,
//! bomb plants and defuses (Search & Destroy), points taken and the score
//! ticker (Domination / Conquest), chopper kills, plus the host-priority
//! bookkeeping and the UDP force-disconnect relay that live in the same
//! opcode block. Everything is logged and written to `match_events` against
//! the room; `results::record_match` attaches the rows to the match once the
//! score table arrives. Only `rfReqForceDisconnectUDPFromHost` and
//! `rfReqGetUserHostPriorityPoint` have an ack in the protocol; the rest are
//! fire-and-forget.
//!
//! Semantics (client behaviour, see docs/PROTOCOL.md "In-match reports"):
//! - `wSDGameInfo.SetObjectiveSetted` / `OnBombDiffused` send InstallBomb /
//!   DefuseBomb with the planter's uid, the site index (0 = Set_0/Flag_A,
//!   1 = Set_1/Flag_B), position, posture (0 stand, 1 crouch, 2 prone), the
//!   weapon held and the distance to the site.
//! - `wDOMGameInfo` / `wCOLGameInfo.FlagState_TakePoint` send TakeAPoint with
//!   every taker in parallel arrays and the flag index (Flag_A/B/C = 0/1/2);
//!   `FlagState_UpdateDomScore` sends the running AF / RSA score.
//! - `GameInfo.KDLOG_Finalize_HeliCopter` sends GetChopper for a helicopter
//!   kill: killer, assists, kill position/weapon/damage, who called the
//!   chopper and where it died.
//! - `w*GameInfo.ServerLogEndRound` sends LogEndEachRound: players at start
//!   and end, the end type, the round length in seconds, the winning team,
//!   whether the winner was attacking (1) or defending (2), both scores and
//!   the round number.
//! - `BTPageRoomLobby.SendPingAndSendHostPoint` reports the player's own
//!   host point (ping + hardware + NAT penalty; lower is better) and the
//!   room-list context menu asks for another player's with
//!   GetUserHostPriorityPoint; below 100 the client changes host at once,
//!   otherwise it asks first.

use super::Reply;
use crate::inventory;
use crate::room;
use crate::session::{self, Ctx};
use acemu_db::matches::NewMatchEvent;
use acemu_proto::channel;
use acemu_proto::frame::Packet;
use serde_json::{json, Value};

/// Bots are handed out as 1000+ (AF) and 2000+ (RSA) by `ReqGetBotID`.
const BOT_AF: std::ops::Range<i32> = 1000..2000;
const BOT_RSA: std::ops::Range<i32> = 2000..3000;

/// Name and team (0 AF, 1 RSA, -1 unknown) of a uid seen in a report.
fn who(uid: i32) -> (String, i32) {
    if let Some(m) = room::member(uid) {
        return (m.name, m.team as i32);
    }
    if let Some(n) = session::name_of(uid) {
        return (n, -1);
    }
    if BOT_AF.contains(&uid) {
        (format!("Bot #{uid}"), 0)
    } else if BOT_RSA.contains(&uid) {
        (format!("Bot #{uid}"), 1)
    } else {
        (format!("#{uid}"), -1)
    }
}

/// Writes one event for the current match of the reporter's room.
fn record(reporter: i32, kind: &str, actor_uid: i32, team_override: Option<i32>, extra: Value) {
    let (actor_name, team) = if actor_uid > 0 { who(actor_uid) } else { (String::new(), -1) };
    let e = NewMatchEvent {
        room_number: room::number_of(reporter).unwrap_or(0),
        at_secs: room::match_log(reporter).0,
        kind: kind.to_string(),
        actor_uid,
        actor_name,
        team: team_override.unwrap_or(team),
        extra,
    };
    if let Err(err) = inventory::db().record_match_event(&e) {
        tracing::warn!(kind, error = %err, "match event not recorded");
    }
}

/// One taker / actor line as JSON: where they stood and what they held.
fn actor_json(uid: i32, x: f32, y: f32, z: f32, posture: i32, weapon: i32, part: i32, range: i32) -> Value {
    let (name, team) = who(uid);
    json!({
        "uid": uid, "name": name, "team": team,
        "x": x, "y": y, "z": z,
        "posture": posture, "weapon": weapon, "weapon_part": part, "range": range,
    })
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        // Search & Destroy / Defense / Alien: one round is over.
        // RoundEndType (from GetRoundEndType): 2 a side was wiped out, 3 bomb
        // exploded (attackers win), 4 bomb defused / time ran out (attackers
        // lose), 11 one side left empty, -1 unknown; Alien mode: 2 aliens, 1 humans.
        // VTeamAD: 1 the winner was attacking, 2 defending.
        channel::ReqLogEndEachRound::OPCODE => {
            let req = channel::ReqLogEndEachRound::decode(&pkt.payload)?;
            tracing::info!(
                uid, round = req.round, start_players = req.start_cnt, end_players = req.end_cnt, end_type = req.round_end_type,
                secs = req.round_end_time, winner = req.v_team, attacking = req.v_team_ad, af = req.af_score, rsa = req.rsa_score,
                "ReqLogEndEachRound"
            );
            record(
                uid,
                "round_end",
                0,
                Some(req.v_team as i32),
                json!({
                    "round": req.round, "end_type": req.round_end_type, "round_secs": req.round_end_time,
                    "winner": req.v_team, "winner_ad": req.v_team_ad,
                    "start_players": req.start_cnt, "end_players": req.end_cnt,
                    "af": req.af_score, "rsa": req.rsa_score,
                }),
            );
            vec![]
        }

        channel::ReqGamePlayInstallBomb::OPCODE => {
            let req = channel::ReqGamePlayInstallBomb::decode(&pkt.payload)?;
            tracing::info!(
                uid, planter = req.installer_uid, site = req.install_pos, weapon = req.installer_weapon_id,
                posture = req.installer_pos, range = req.installer_range, "ReqGamePlay_InstallBomb"
            );
            let mut extra = actor_json(
                req.installer_uid,
                req.installer_x_coord,
                req.installer_y_coord,
                req.installer_z_coord,
                req.installer_pos,
                req.installer_weapon_id,
                req.installer_weapon_part_id,
                req.installer_range,
            );
            extra["site"] = json!(req.install_pos);
            record(uid, "bomb_planted", req.installer_uid, None, extra);
            vec![]
        }

        channel::ReqGamePlayDefuseBomb::OPCODE => {
            let req = channel::ReqGamePlayDefuseBomb::decode(&pkt.payload)?;
            tracing::info!(
                uid, defuser = req.defuser_uid, site = req.defuse_pos, weapon = req.defuser_weapon_id,
                posture = req.defuser_pos, range = req.defuser_range, "ReqGamePlay_DefuseBomb"
            );
            let mut extra = actor_json(
                req.defuser_uid,
                req.defuser_x_coord,
                req.defuser_y_coord,
                req.defuser_z_coord,
                req.defuser_pos,
                req.defuser_weapon_id,
                req.defuser_weapon_part_id,
                req.defuser_range,
            );
            extra["site"] = json!(req.defuse_pos);
            record(uid, "bomb_defused", req.defuser_uid, None, extra);
            vec![]
        }

        // Domination / Conquest: a flag changed hands. Everyone who stood on
        // it is listed in parallel arrays; the first taker is the actor.
        channel::ReqGamePlayTakeAPoint::OPCODE => {
            let req = channel::ReqGamePlayTakeAPoint::decode(&pkt.payload)?;
            tracing::info!(uid, flag = req.take_pos, takers = ?req.taker_uid, "ReqGamePlay_TakeAPoint");
            let at = |v: &Vec<i32>, i: usize| v.get(i).copied().unwrap_or(0);
            let atf = |v: &Vec<f32>, i: usize| v.get(i).copied().unwrap_or(0.0);
            let takers: Vec<Value> = req
                .taker_uid
                .iter()
                .enumerate()
                .map(|(i, &t)| {
                    actor_json(
                        t,
                        atf(&req.taker_x_coord, i),
                        atf(&req.taker_y_coord, i),
                        atf(&req.taker_z_coord, i),
                        at(&req.taker_pos, i),
                        at(&req.taker_weapon_id, i),
                        at(&req.taker_weapon_part_id, i),
                        at(&req.taker_range, i),
                    )
                })
                .collect();
            let first = req.taker_uid.first().copied().unwrap_or(0);
            record(uid, "point_taken", first, None, json!({ "flag": req.take_pos, "takers": takers }));
            vec![]
        }

        channel::ReqGamePlayUpdateDomScore::OPCODE => {
            let req = channel::ReqGamePlayUpdateDomScore::decode(&pkt.payload)?;
            tracing::info!(uid, af = req.af_score, rsa = req.rsa_score, "ReqGamePlay_UpdateDomScore");
            record(uid, "dom_score", 0, None, json!({ "af": req.af_score, "rsa": req.rsa_score }));
            vec![]
        }

        channel::ReqGamePlayGetChopper::OPCODE => {
            let req = channel::ReqGamePlayGetChopper::decode(&pkt.payload)?;
            tracing::info!(
                uid, killer = req.get_chopper_uid, assists = ?req.get_chopper_assist_uid, weapon = req.kill_weapon_id,
                damage = req.kill_damage, caller = req.chopper_caller_id, caller_name = %req.chopper_caller_name,
                "ReqGamePlay_GetChopper"
            );
            let mut extra = actor_json(
                req.get_chopper_uid,
                req.kill_x_coord,
                req.kill_y_coord,
                req.kill_z_coord,
                req.kill_pos,
                req.kill_weapon_id,
                req.kill_weapon_part_id,
                req.kill_range,
            );
            extra["damage"] = json!(req.kill_damage);
            extra["assists"] = json!(req
                .get_chopper_assist_uid
                .iter()
                .map(|&a| json!({ "uid": a, "name": who(a).0 }))
                .collect::<Vec<_>>());
            extra["caller"] = json!({
                "uid": req.chopper_caller_id, "name": req.chopper_caller_name,
                "x": req.chopper_x_coord, "y": req.chopper_y_coord, "z": req.chopper_z_coord,
            });
            record(uid, "chopper_kill", req.get_chopper_uid, None, extra);
            vec![]
        }

        // The host dropped a guest's UDP link (after rfAckDisconnectGame):
        // acknowledge, and tell the guest to tear down its side too - the
        // client's rfReqRecvForceDisconnectUDPFromHost handler calls
        // ForceDisconnectUDPConnectionClientSide when it is not the owner.
        channel::ReqForceDisconnectUDPFromHost::OPCODE => {
            let req = channel::ReqForceDisconnectUDPFromHost::decode(&pkt.payload)?;
            let (name, _) = who(req.user_id);
            tracing::info!(uid, target = req.user_id, target_name = %name, "ReqForceDisconnectUDPFromHost");
            record(uid, "udp_kick", req.user_id, None, json!({ "by": uid, "by_name": ctx.name }));
            if req.user_id != uid {
                session::send_to(req.user_id, &[channel::ReqRecvForceDisconnectUDPFromHost { result: 0 }.packet()]);
            }
            vec![channel::AckForceDisconnectUDPFromHost { result: 0 }.packet()]
        }

        // Normally a server -> guest push (above); if a client sends it, just log.
        channel::ReqRecvForceDisconnectUDPFromHost::OPCODE => {
            let req = channel::ReqRecvForceDisconnectUDPFromHost::decode(&pkt.payload)?;
            tracing::info!(uid, result = req.result, "ReqRecvForceDisconnectUDPFromHost (from client)");
            vec![]
        }

        // Host-priority point of the sender: ping + hardware points, +30 for
        // a symmetric NAT; lower is a better host. Sent every few seconds in
        // the room lobby and in-game.
        channel::ReqUpdateMyHostPriorityPoint::OPCODE => {
            let req = channel::ReqUpdateMyHostPriorityPoint::decode(&pkt.payload)?;
            tracing::info!(uid, name = %ctx.name, point = req.host_priority_point, "ReqUpdateMyHostPriorityPoint");
            if !ctx.name.is_empty() {
                if let Err(e) = inventory::db().set_host_priority(&ctx.name, uid, req.host_priority_point) {
                    tracing::warn!(error = %e, "host priority not stored");
                }
            }
            vec![]
        }

        // "Make host" from the room's player list: answer with the last point
        // that player reported (0 = never reported, which the client treats
        // as a good host and changes owner without asking).
        channel::ReqGetUserHostPriorityPoint::OPCODE => {
            let req = channel::ReqGetUserHostPriorityPoint::decode(&pkt.payload)?;
            let point = session::name_of(req.user_id)
                .and_then(|n| inventory::db().host_priority(&n).ok().flatten())
                .map(|h| h.point)
                .unwrap_or(0);
            tracing::info!(uid, target = req.user_id, point, "ReqGetUserHostPriorityPoint");
            vec![channel::AckGetUserHostPriorityPoint { result: 0, user_id: req.user_id, host_priority_point: point }.packet()]
        }

        // A guest lost the host while loading (or the host reports a guest
        // did): the client then leaves the pending start (error 5080) or the
        // running game (5072) on its own.
        channel::ReqNotifyDisconnectHostInLoading::OPCODE => {
            let req = channel::ReqNotifyDisconnectHostInLoading::decode(&pkt.payload)?;
            tracing::info!(uid, result = req.result, err = %req.err_msg, host = req.host_id, "ReqNotifyDisconnectHostInLoading");
            record(uid, "host_lost", req.host_id, None, json!({ "result": req.result, "err_msg": req.err_msg, "reporter": uid }));
            vec![]
        }

        // Sent when a pending start is abandoned (host change or host lost
        // while the team-balance check was running); nothing to keep.
        channel::ReqRemoveCheckTeamBalanceList::OPCODE => {
            tracing::info!(uid, "ReqRemoveCheckTeamBalanceList");
            vec![]
        }

        _ => return Ok(None),
    }))
}
