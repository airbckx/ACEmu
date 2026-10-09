//! End of a match: the score table and career stats, kill log, game-over
//! bracket, weapon wear, booty, and the tutorial bracket the client wraps a
//! first match in.

use super::Reply;
use crate::inventory;
use crate::player::level_of;
use crate::room;
use crate::session::{self, Ctx};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

/// `TutorialResultType` the client sends when the tutorial was abandoned
/// (observed: 0 and 1 after a full run, 3 when left early).
const TUTORIAL_LEFT_EARLY: i32 = 3;
/// Mode name the tutorial run is filed under in the match history.
const TUTORIAL_MODE: &str = "Tutorial";

/// (EXP, points) granted for finishing the tutorial once. The retail amounts
/// are unknown (the client shows its own numbers); `ACEMU_TUTORIAL_EXP` and
/// `ACEMU_TUTORIAL_POINTS` override the defaults.
fn tutorial_reward() -> (i32, i32) {
    let env = |k: &str, d: i32| std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
    (env("ACEMU_TUTORIAL_EXP", 300), env("ACEMU_TUTORIAL_POINTS", 500))
}

/// Level-up announcement for `name` going from `old` to `new`: grants the
/// promotion-table rewards of every level crossed and returns the two packets
/// the client expects - `rfReqNotifyLevelUp` (fills its level-up window) and
/// `rfAckAcquirePromotionItems` (marks the rewards delivered; the client then
/// refreshes its item and skill lists). `PartID` carries the rental hours.
fn level_up_packets(uid: i32, name: &str, old: i32, new: i32) -> Vec<Packet> {
    let mut granted = vec![];
    for lvl in (old + 1)..=new {
        granted.extend(inventory::grant_promotions(name, lvl));
    }
    let exp = inventory::read_named(name, |inv| inv.stats.exp);
    vec![
        channel::ReqNotifyLevelUp {
            level_up_user_id: uid,
            level: new,
            exp,
            item_id: granted.iter().map(|g| g.item_id).collect(),
            part_id: granted.iter().map(|g| g.hours).collect(),
        }
        .packet(),
        channel::AckAcquirePromotionItems {
            uid,
            result: 0,
            err_msg: String::new(),
            item_unique_id: granted.iter().map(|g| g.unique_id).collect(),
            item_i_ds: granted.iter().map(|g| g.item_id).collect(),
            part_i_ds: granted.iter().map(|g| g.hours).collect(),
        }
        .packet(),
    ]
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        // The host reports everyone's line at the end of a match; every real
        // player's account is updated, the table is echoed back with zero
        // bonuses, and level-ups are announced to the player concerned.
        channel::ReqUpdateScore::OPCODE => {
            let req = channel::ReqUpdateScore::decode(&pkt.payload)?;
            tracing::info!(
                room = req.game_number, users = ?req.user_i_ds, kills = ?req.kills, deaths = ?req.deaths,
                exps = ?req.ex_ps, points = ?req.points, scores = ?req.scores, team_scores = ?req.team_scores,
                playtime = req.playtime, end_type = req.game_end_type, "ReqUpdateScore"
            );
            let n = req.user_i_ds.len();
            let at = |v: &Vec<i32>, i: usize| v.get(i).copied().unwrap_or(0);
            let won_for = |team: u16| match (req.team_scores.first(), req.team_scores.get(1)) {
                (Some(a), Some(b)) if a != b => Some(if team == 0 { a > b } else { b > a }),
                _ => None,
            };
            let mut level_ups = vec![];
            record_match(uid, &req);
            for (i, &u) in req.user_i_ds.iter().enumerate() {
                let Some(m) = room::member(u) else { continue };
                let (old, new) = inventory::apply_match(
                    &m.name,
                    inventory::MatchLine {
                        kills: at(&req.kills, i),
                        assists: at(&req.assists, i),
                        deaths: at(&req.deaths, i),
                        exp: at(&req.ex_ps, i),
                        points: at(&req.points, i),
                        playtime: req.playtime,
                        won: won_for(m.team),
                    },
                );
                if new > old {
                    for p in level_up_packets(u, &m.name, old, new) {
                        level_ups.push((u, p));
                    }
                }
            }
            let mut out = vec![
                channel::AckUpdateScore {
                    user_i_ds: req.user_i_ds.clone(),
                    nick_names: req.user_names.clone(),
                    team_i_ds: req.user_i_ds.iter().map(|&u| room::member(u).map(|m| m.team as u8).unwrap_or(0)).collect(),
                    scores: req.scores.clone(),
                    kills: req.kills.clone(),
                    assists: req.assists.clone(),
                    deaths: req.deaths.clone(),
                    ex_ps: req.ex_ps.clone(),
                    exp_bonuses: vec![0; n],
                    points: req.points.clone(),
                    point_bonuses: vec![0; n],
                    team_scores: req.team_scores.clone(),
                    bonus_owner_id: vec![],
                    bonus_id: vec![],
                }
                .packet(),
                channel::AckUpdateScoreResult { result: 0, err_msg: String::new() }.packet(),
            ];
            // The host reports for the whole room; every guest's client sits on
            // the results screen until it gets the score table too.
            session::send_many(&room::others(uid), &out);
            for (u, p) in level_ups {
                if u == uid {
                    out.push(p);
                } else {
                    session::send_to(u, &[p]);
                }
            }
            out
        }

        channel::ReqLogGetScore::OPCODE => {
            let req = channel::ReqLogGetScore::decode(&pkt.payload)?;
            let k = &req.kill_death_log_param;
            tracing::info!(
                killer = k.char_kill_uid, victim = k.char_death_uid, weapon = k.kill_weapon,
                body_part = k.kill_body_part, assists = ?req.assist_ui_ds, "ReqLogGetScore (kill)"
            );
            // headshot: body part 1 is assumed to be the head
            if k.kill_body_part == 1 {
                if let Some(m) = room::member(k.char_kill_uid) {
                    inventory::with_named(&m.name, |inv| {
                        inv.stats.headshots += 1;
                        ((), true)
                    });
                }
            }
            room::record_kill(uid, k.char_kill_uid, k.char_death_uid, k.kill_weapon, k.kill_body_part as i32, req.assist_ui_ds.clone());
            vec![]
        }

        channel::ReqGameOver::OPCODE => {
            let req = channel::ReqGameOver::decode(&pkt.payload)?;
            tracing::info!(uid = req.uid, room = req.game_room_number, "ReqGameOver");
            let ack = channel::AckGameOver { uid: req.uid, game_room_number: req.game_room_number, result: 0, err_msg: String::new() }.packet();
            if room::is_owner(uid) {
                super::netgame::end_match(uid);
                // the guests leave the results screen on this too
                session::send_many(&room::others(uid), std::slice::from_ref(&ack));
            }
            vec![ack]
        }

        channel::ReqForceGameOver::OPCODE => vec![channel::AckForceGameOver {
            user_id: uid,
            game_room_number: room::number_of(uid).unwrap_or(0) as u16,
            result: 0,
            err_msg: String::new(),
        }
        .packet()],

        channel::ReqGameOverEnd::OPCODE => {
            vec![channel::AckGameOverEnd { result: 0, err_msg: String::new() }.packet()]
        }

        // End of match: the host reports how long each weapon was used; reply
        // with the worn durabilities (the client refuses a 0-durability weapon).
        channel::ReqUpdateWeaponDurability::OPCODE => {
            let req = channel::ReqUpdateWeaponDurability::decode(&pkt.payload)?;
            tracing::info!(play_time = req.total_play_time, weapons = req.unique_item_id.len(), "ReqUpdateWeaponDurability");
            let mut ids = Vec::new();
            let mut durs = Vec::new();
            let mut destroyed = Vec::new();
            let mut failed = Vec::new();
            // weapons that just hit zero, per owner uid, for the durability-zero push
            let mut zeroed: std::collections::BTreeMap<i32, (String, Vec<f64>)> = std::collections::BTreeMap::new();
            for (k, &id) in req.unique_item_id.iter().enumerate() {
                let secs = req.using_sec.get(k).copied().unwrap_or(0);
                // OwnerID is the room member's uid; the reporting host may not own the weapon
                let owner_uid = req.owner_id.get(k).copied().unwrap_or(ctx.uid);
                let owner = session::name_of(owner_uid).unwrap_or_else(|| ctx.name.clone());
                match inventory::wear(&owner, id, secs) {
                    Some((d, dead)) => {
                        ids.push(id);
                        durs.push(d);
                        destroyed.push(dead as u8);
                        if dead {
                            zeroed.entry(owner_uid).or_insert_with(|| (owner.clone(), vec![])).1.push(id);
                        }
                    }
                    None => failed.push(id),
                }
            }
            for (uid, (owner, worn)) in zeroed {
                if let Some(p) = super::upkeep::durability_zero_notice(uid, &owner, &worn) {
                    session::send_to(uid, &[p]);
                }
            }
            vec![channel::AckUpdateWeaponDurability {
                result: 0,
                err_msg: String::new(),
                unique_item_id: ids,
                durability: durs,
                is_destroy: destroyed,
                failed_weapon_unique_id: failed,
            }
            .packet()]
        }

        // End-of-match booty: every finisher wins a spoils box, delivered straight
        // to the inventory (the client shows "loot could not be acquired" when
        // this stays unanswered).
        // The host asks once for the whole room. Every member is drawn a
        // prize (once per match), all of them get the winners list
        // (`rfAckGetBooty` is per room: one entry per winner) and each its own
        // instance (`rfAckGetBootyChoose`); a guest's result screen waits for
        // these. A later request from a member repeats its result.
        channel::ReqGetBooty::OPCODE => {
            let req = channel::ReqGetBooty::decode(&pkt.payload)?;
            tracing::info!(uid, room = req.room_number, "ReqGetBooty");
            let members = if room::is_owner(uid) { room::members_of(uid) } else { room::member(uid).into_iter().collect() };
            for m in &members {
                if m.booty.is_none() {
                    if let Some(b) = inventory::grant_booty(&m.name) {
                        room::set_booty(m.uid, (b.item_id, b.unique_id, b.item_type, b.until));
                    }
                }
            }
            let winners: Vec<(i32, (i32, f64, i32, f64))> = room::members_of(uid).into_iter().filter_map(|m| m.booty.map(|b| (m.uid, b))).collect();
            if winners.is_empty() {
                return Ok(Some(vec![channel::AckGetBooty { result: 1, err_msg: "no booty".into(), ..Default::default() }.packet()]));
            }
            let list = channel::AckGetBooty {
                result: 0,
                err_msg: String::new(),
                prize_win_user_id: winners.iter().map(|w| w.0).collect(),
                booty_item_id: winners.iter().map(|w| w.1 .0).collect(),
                booty_parts_id: vec![0; winners.len()],
            }
            .packet();
            let choose = |b: (i32, f64, i32, f64)| {
                channel::AckGetBootyChoose {
                    result: 0,
                    err_msg: String::new(),
                    give_type: 0,
                    item_unique_number: b.1,
                    item_id: b.0,
                    part_id: 0,
                    item_type: b.2 as u8,
                    until_time: b.3,
                    stack_count: 1,
                    paint_id: 0,
                }
                .packet()
            };
            let mut mine = vec![];
            for (w_uid, b) in &winners {
                if *w_uid == uid {
                    mine = vec![list.clone(), choose(*b)];
                } else if room::is_owner(uid) {
                    session::send_to(*w_uid, &[list.clone(), choose(*b)]);
                }
            }
            mine
        }

        // The client wraps a first match in a "tutorial" bracket; acknowledge
        // both ends so it does not wait on us.
        channel::ReqNotifyStartTutorial::OPCODE => {
            vec![channel::AckNotifyStartTutorial { result: 0, user_id: uid }.packet()]
        }

        // The tutorial is simulated by the client (no score table reaches us), so
        // the points it shows are granted here once per player when it reports
        // completion, and the run is written to the match history as "Tutorial".
        channel::ReqNotifyEndTutorial::OPCODE => {
            let req = channel::ReqNotifyEndTutorial::decode(&pkt.payload)?;
            tracing::info!(result_type = req.tutorial_result_type, "ReqNotifyEndTutorial");
            let mut out = vec![];
            // observed result types: 0 and 1 after a full run, 3 when left early
            if req.tutorial_result_type != TUTORIAL_LEFT_EARLY && !ctx.name.is_empty() {
                let done = inventory::db().has_played_mode(&ctx.name, TUTORIAL_MODE).unwrap_or(true);
                if !done {
                    let (exp, points) = tutorial_reward();
                    let (old, new) = inventory::apply_match(
                        &ctx.name,
                        inventory::MatchLine { kills: 0, assists: 0, deaths: 0, exp, points, playtime: 0, won: None },
                    );
                    let rec = acemu_db::matches::NewMatch {
                        room_name: "Tutorial".into(),
                        map: "Tutorial".into(),
                        mode: TUTORIAL_MODE.into(),
                        host: ctx.name.clone(),
                        duration: 0,
                        end_type: req.tutorial_result_type,
                        team0_score: 0,
                        team1_score: 0,
                        winner: -1,
                        players: vec![acemu_db::matches::NewMatchPlayer {
                            uid,
                            name: ctx.name.clone(),
                            team: 0,
                            is_bot: false,
                            score: 0,
                            kills: 0,
                            assists: 0,
                            deaths: 0,
                            exp,
                            points,
                            headshots: 0,
                            won: None,
                        }],
                        kills: vec![],
                    };
                    if let Err(e) = inventory::db().record_match(&rec) {
                        tracing::warn!(error = %e, "tutorial not recorded");
                    }
                    tracing::info!(name = %ctx.name, exp, points, level = new, "tutorial reward granted");
                    // the level-1 promotion row is the starter kit the tutorial hands out
                    let kit = inventory::grant_promotions(&ctx.name, 1);
                    if !kit.is_empty() {
                        out.push(
                            channel::AckAcquirePromotionItems {
                                uid,
                                result: 0,
                                err_msg: String::new(),
                                item_unique_id: kit.iter().map(|g| g.unique_id).collect(),
                                item_i_ds: kit.iter().map(|g| g.item_id).collect(),
                                part_i_ds: kit.iter().map(|g| g.hours).collect(),
                            }
                            .packet(),
                        );
                    }
                    if new > old {
                        out.extend(level_up_packets(uid, &ctx.name, old, new));
                    }
                }
            }
            out.push(
                channel::AckNotifyEndTutorial {
                    user_id: uid,
                    char_name: ctx.name.clone(),
                    level: level_of(&ctx.name),
                    result: 0,
                    ..Default::default()
                }
                .packet(),
            );
            out
        }

        _ => return Ok(None),
    }))
}

/// Writes the finished match to the history: the host's score table (bots
/// included) and the kill feed collected during the match.
fn record_match(host_uid: i32, req: &channel::ReqUpdateScore) {
    use acemu_db::matches::{NewKill, NewMatch, NewMatchPlayer};
    let at = |v: &Vec<i32>, i: usize| v.get(i).copied().unwrap_or(0);
    let (t0, t1) = (req.team_scores.first().copied().unwrap_or(0), req.team_scores.get(1).copied().unwrap_or(0));
    let winner = if req.team_scores.len() < 2 || t0 == t1 { -1 } else if t0 > t1 { 0 } else { 1 };
    let (elapsed, kills) = room::match_log(host_uid);
    let info = room::info_of(host_uid);
    let room_number = room::number_of(host_uid).unwrap_or(0);
    let name_of = |uid: i32| -> String {
        req.user_i_ds
            .iter()
            .position(|&u| u == uid)
            .and_then(|i| req.user_names.get(i).cloned())
            .or_else(|| room::member(uid).map(|m| m.name))
            .unwrap_or_else(|| format!("#{uid}"))
    };
    // headshots per player from the kill feed (body part 1 = head)
    let headshots = |uid: i32| kills.iter().filter(|k| k.killer == uid && k.body_part == 1).count() as i32;
    let players = req
        .user_i_ds
        .iter()
        .enumerate()
        .map(|(i, &u)| {
            let member = room::member(u);
            let team = member.as_ref().map(|m| m.team as i32).unwrap_or_else(|| {
                // bots are handed out as 1000+ (AF) and 2000+ (RSA) by ReqGetBotID
                if (1000..2000).contains(&u) {
                    0
                } else {
                    1
                }
            });
            NewMatchPlayer {
                uid: u,
                name: req.user_names.get(i).cloned().unwrap_or_else(|| name_of(u)),
                team,
                is_bot: member.is_none(),
                score: at(&req.scores, i),
                kills: at(&req.kills, i),
                assists: at(&req.assists, i),
                deaths: at(&req.deaths, i),
                exp: at(&req.ex_ps, i),
                points: at(&req.points, i),
                headshots: headshots(u),
                won: if winner < 0 { None } else { Some(team == winner) },
            }
        })
        .collect();
    let m = NewMatch {
        room_name: info.as_ref().map(|i| i.room_name.clone()).unwrap_or_default(),
        map: info.as_ref().map(|i| i.map_name.clone()).unwrap_or_default(),
        mode: info.as_ref().map(|i| i.game_class.clone()).unwrap_or_default(),
        host: room::owner_of(host_uid).and_then(|o| session::name_of(o)).unwrap_or_default(),
        duration: if req.playtime > 0 { req.playtime } else { elapsed as i32 },
        end_type: req.game_end_type as i32,
        team0_score: t0,
        team1_score: t1,
        winner,
        players,
        kills: kills
            .iter()
            .map(|k| NewKill {
                at_secs: k.at_secs,
                killer_uid: k.killer,
                killer_name: name_of(k.killer),
                victim_uid: k.victim,
                victim_name: name_of(k.victim),
                weapon: k.weapon,
                body_part: k.body_part,
                headshot: k.body_part == 1,
            })
            .collect(),
    };
    match inventory::db().record_match(&m) {
        Ok(id) => {
            // the objective events matchlog.rs wrote during this match (round
            // ends, bomb plants, points taken, ...) now belong to it
            let events = inventory::db().attach_match_events(id, room_number, m.duration as i64 + 120).unwrap_or_else(|e| {
                tracing::warn!(error = %e, "match events not attached");
                0
            });
            tracing::info!(match_id = id, players = m.players.len(), kills = m.kills.len(), events, winner, "match recorded");
        }
        Err(e) => tracing::warn!(error = %e, "match not recorded"),
    }
}
