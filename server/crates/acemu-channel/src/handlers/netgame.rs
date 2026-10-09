//! Starting a match and the host handshake. The room owner hosts; the game
//! runs as a listen server on that client and the guests connect to the UDP
//! endpoint it registers. Covers loading-state relays, UDP endpoint
//! registration and fan-out, hole punching, the connect/success/failed
//! reports, and the in-match queries the HUD makes of the server.

use super::Reply;
use crate::room::{self, Member, Udp};
use crate::session::{self, Ctx};
use crate::{unix_now, local_ip};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        // Loading progress: echoed to every room member so the host sees its
        // guests come up.
        channel::ReqNotifyLoadingState::OPCODE => {
            let req = channel::ReqNotifyLoadingState::decode(&pkt.payload)?;
            let ack = channel::AckNotifyLoadingState { result: 0, user_id: uid, loading_state: req.loading_state }.packet();
            session::send_many(&room::others(uid), std::slice::from_ref(&ack));
            vec![ack]
        }

        channel::ReqNotifyHostBlocking::OPCODE => {
            let req = channel::ReqNotifyHostBlocking::decode(&pkt.payload)?;
            vec![channel::AckNotifyHostBlocking { result: 0, blocking: req.blocking }.packet()]
        }

        channel::ReqGetIsHostBlocking::OPCODE => {
            vec![channel::AckGetIsHostBlocking { result: 0, blocking: 0 }.packet()]
        }

        channel::ReqIsHostAvailable::OPCODE => {
            let req = channel::ReqIsHostAvailable::decode(&pkt.payload)?;
            tracing::info!(uid, ip = %req.my_ip, cpu = req.cpu_grade, ram = req.ram_grade, vga = req.vga_grade, "ReqIsHostAvailable");
            vec![channel::AckIsHostAvailable { result: 0 }.packet()]
        }

        channel::ReqStartGame::OPCODE => {
            let req = channel::ReqStartGame::decode(&pkt.payload)?;
            // Only guests that pressed Ready take part: the others' clients
            // answer rfAckStartGame with "Start GameRoom Without Me" and never
            // register, and a host waiting for them fails at step 2 (5009).
            let ready = room::ready_guests_of(uid);
            let members = room::start(uid).unwrap_or_else(|| vec![uid]);
            // ClientsCount = players other than the host. Host loading step 2 waits
            // for that many rfAckRegisterOtherUdpInfo before it moves on (10 s limit
            // "WaitClientOutSideAddrInHost", error 5009), so a lone host must get 0.
            let clients = ready.len() as u16;
            tracing::info!(uid, room = req.game_number, clients, members = members.len(), "ReqStartGame");
            let ack = channel::AckStartGame {
                game_number: req.game_number,
                clients_count: clients,
                start_seed: unix_now(),
                result: 0,
                err_msg: String::new(),
            }
            .packet();
            session::send_many(&members.into_iter().filter(|&u| u != uid).collect::<Vec<_>>(), std::slice::from_ref(&ack));
            if let Some(number) = room::number_of(uid) {
                super::lobby::push_room_list(number);
            }
            vec![ack]
        }

        // Each player registers the UDP endpoint it will play on. Everyone else in
        // the room learns it (rfAckRegisterOtherUdpInfo), and the newcomer gets the
        // endpoints already registered.
        channel::ReqRegisterMyUdpInfo::OPCODE => {
            let req = channel::ReqRegisterMyUdpInfo::decode(&pkt.payload)?;
            tracing::info!(
                uid,
                is_host = req.is_host,
                ip = %req.ip,
                port = req.port,
                inner_ip = %req.host_inner_ip,
                inner_port = req.host_inner_port,
                "ReqRegisterMyUdpInfo"
            );
            let udp = Udp {
                ip: if req.ip.trim().is_empty() { local_ip() } else { req.ip.clone() },
                port: req.port,
                inner_ip: req.host_inner_ip.clone(),
                inner_port: req.host_inner_port,
                is_host: req.is_host != 0,
                intervention: req.intervention != 0,
            };
            if !room::set_udp(uid, udp) {
                tracing::warn!(uid, "udp info from a player outside the room");
            }
            // A late joiner while the host is still loading the map: the host's
            // client refuses the registration ("Host Loading. Can't process
            // intervention user"), so hold it and replay it once the host is in.
            let mut held = false;
            if req.intervention != 0 {
                room::set_late_join(uid, true);
                held = !room::is_host_ready_of(uid);
                tracing::info!(uid, held, "late join: intervention registration");
                if held {
                    room::queue_late(uid);
                }
            }
            let mut out = vec![channel::AckRegisterMyUdpInfo { result: 0, err_msg: String::new() }.packet()];
            if let Some(me) = room::member(uid) {
                if let Some(mine) = other_udp_info(&me, session::nat_of(uid)) {
                    if !held {
                        session::send_many(&room::others(uid), std::slice::from_ref(&mine));
                    }
                }
                for m in room::members_of(uid).iter().filter(|m| m.uid != uid) {
                    if let Some(p) = other_udp_info(m, session::nat_of(m.uid)) {
                        out.push(p);
                    }
                }
            }
            out
        }

        channel::ReqGameHostInfo::OPCODE => {
            let req = channel::ReqGameHostInfo::decode(&pkt.payload)?;
            let owner = room::owner_of(uid).unwrap_or(uid);
            let host = room::host_udp_of(uid);
            vec![channel::AckGameHostInfo {
                game_number: req.game_number,
                user_id: session::name_of(owner).unwrap_or_else(|| ctx.name.clone()),
                ip: host.as_ref().map(|u| u.ip.clone()).unwrap_or_else(|| local_ip()),
                port: host.as_ref().map(|u| u.port as u16).unwrap_or(0),
                result: if host.is_some() { 0 } else { 1 },
                err_msg: String::new(),
            }
            .packet()]
        }

        channel::ReqClientUdpInfoList::OPCODE => {
            let _ = channel::ReqClientUdpInfoList::decode(&pkt.payload)?;
            let reg: Vec<(i32, Udp)> = room::members_of(uid).into_iter().filter_map(|m| m.udp.map(|u| (m.uid, u))).collect();
            vec![channel::AckClientUdpInfoList {
                result: 0,
                err_msg: String::new(),
                uid: reg.iter().map(|r| r.0).collect(),
                ip: reg.iter().map(|r| r.1.ip.clone()).collect(),
                port: reg.iter().map(|r| r.1.port).collect(),
            }
            .packet()]
        }

        // The host asks for hole punching once it holds every guest's UDP info.
        // Every member must get the ack: BTPageRoomLobby.rfAckStartHolePunching
        // runs StartHolePunchingHost on the owner and StartHolePunchingClient on
        // the guests, who otherwise wait for it until they time out (5088).
        channel::ReqStartHolePunching::OPCODE => {
            let req = channel::ReqStartHolePunching::decode(&pkt.payload)?;
            tracing::info!(uid, game_type = req.game_type, start_user = req.req_start_user_id, "ReqStartHolePunching");
            room::punch_reset(uid);
            let ack = channel::AckStartHolePunching {
                result: 0,
                err_msg: String::new(),
                game_type: req.game_type,
                req_start_user_id: req.req_start_user_id,
            }
            .packet();
            // a held late joiner's punching request is replayed with its registration
            if !room::is_queued_late(uid) {
                session::send_many(&room::others(uid), std::slice::from_ref(&ack));
            }
            vec![ack]
        }

        // Each side reports the UDP dummies it received from the other; no ack.
        // A late joiner ("intervention") that hears the host's dummies has a
        // working path: nobody will send `rfReqSuccessStartGame` again for it,
        // so the server itself tells it where the host is (loading step 4).
        channel::ReqNotifyRecvDummyForHolePunching::OPCODE => {
            let req = channel::ReqNotifyRecvDummyForHolePunching::decode(&pkt.payload)?;
            tracing::info!(uid, from = req.client_user_id, phrase = req.phrase, "ReqNotifyRecvDummyForHolePunching");
            room::punch_report(uid, req.phrase);
            let mut out = vec![];
            if room::is_late_join(uid) && room::is_playing_of(uid) {
                if let Some(host) = room::host_udp_of(uid) {
                    tracing::info!(uid, host = %format!("{}:{}", host.ip, host.port), "late join: sending the host endpoint");
                    room::set_late_join(uid, false);
                    out.push(
                        channel::AckSuccessStartGame {
                            result: 0,
                            err_msg: String::new(),
                            is_host: 0,
                            host_ip: host.ip,
                            host_port: host.port,
                            loading_step: 4,
                        }
                        .packet(),
                    );
                }
            }
            out
        }

        // The host polls this a few seconds after punching started. Members that
        // never reported a received dummy are the failed ones. During a match the
        // punching is for a late joiner, which the answer names.
        channel::ReqGetHolePunchingResult::OPCODE => {
            let (reports, members) = room::punch_state(uid).unwrap_or((vec![], vec![uid]));
            let phrase = reports.iter().map(|r| r.1).max().unwrap_or(1);
            // Only a player that pressed Join during the match counts as an
            // intervention. (`playing` is set at Start, before punching, so it
            // cannot be the discriminator: a normal start would look like one.)
            let late = room::late_joiners_of(uid);
            let failed: Vec<i32> = members
                .iter()
                .copied()
                .filter(|&m| m != uid && !reports.iter().any(|r| r.0 == m))
                // with a late joiner only that player is being checked
                .filter(|m| late.is_empty() || late.contains(m))
                .collect();
            let intervention = late.first().copied();
            tracing::info!(uid, ?reports, ?failed, ?intervention, "ReqGetHolePunchingResult");
            vec![channel::AckGetHolePunchingResult {
                result: 0,
                phrase,
                intervention_user_id: intervention.unwrap_or(0),
                intervention: u8::from(intervention.is_some()),
                failed_user_i_ds: failed,
            }
            .packet()]
        }

        // The host can declare a guest's loading failed (seen with ErrCode 0 right
        // after the guest's UDP info arrived). Acknowledge; relay only real codes.
        channel::ReqForceFailedClientLoading::OPCODE => {
            let req = channel::ReqForceFailedClientLoading::decode(&pkt.payload)?;
            tracing::warn!(uid, failed_user = req.failed_user_id, err_code = req.err_code, "ReqForceFailedClientLoading");
            if req.err_code != 0 {
                session::send_to(req.failed_user_id, &[channel::ReqRecvForceFailedClientLoading { err_code: req.err_code }.packet()]);
            }
            vec![channel::AckForceFailedClientLoading { result: 0, err_msg: String::new() }.packet()]
        }

        // "Join" pressed in a room whose match is running. The joiner then
        // fetches the host-blocking flag, registers its UDP endpoint with
        // Intervention=1 and starts hole punching itself. The host gets the same
        // ack (`rfAckConnectGame` on the owner hands the joiner the booty seed).
        channel::ReqConnectGame::OPCODE => {
            let playing = room::is_playing_of(uid);
            tracing::info!(uid, playing, "ReqConnectGame");
            let ack = channel::AckConnectGame { user_id: uid, result: 0 }.packet();
            if playing {
                room::set_late_join(uid, true);
                if let Some(owner) = room::owner_of(uid).filter(|&o| o != uid) {
                    session::send_to(owner, std::slice::from_ref(&ack));
                }
            }
            vec![ack]
        }

        channel::ReqSuccessStartGame::OPCODE => {
            let req = channel::ReqSuccessStartGame::decode(&pkt.payload)?;
            let is_host = room::owner_of(uid).map(|o| o == uid).unwrap_or(true);
            let host = room::host_udp_of(uid).or_else(|| room::member(uid).and_then(|m| m.udp));
            tracing::info!(uid, step = req.loading_step, is_host, host = ?host, "ReqSuccessStartGame");
            let host_ip = host.as_ref().map(|u| u.ip.clone()).unwrap_or_else(|| local_ip());
            let host_port = host.as_ref().map(|u| u.port).unwrap_or(0);
            if is_host {
                // Guests wait at their loading step 4 for this: "the host is up at
                // HostIP:HostPort, connect" (20 s limit, error 5064 otherwise).
                let for_guests = channel::AckSuccessStartGame {
                    result: 0,
                    err_msg: String::new(),
                    is_host: 0,
                    host_ip: host_ip.clone(),
                    host_port,
                    loading_step: req.loading_step,
                }
                .packet();
                session::send_many(&room::others(uid), std::slice::from_ref(&for_guests));
            }
            vec![channel::AckSuccessStartGame {
                result: 0,
                err_msg: String::new(),
                is_host: u8::from(is_host),
                host_ip,
                host_port,
                loading_step: req.loading_step,
            }
            .packet()]
        }

        channel::ReqFailedStartGame::OPCODE => {
            let req = channel::ReqFailedStartGame::decode(&pkt.payload)?;
            tracing::warn!(uid, step = req.loading_step, "ReqFailedStartGame");
            let is_host = room::owner_of(uid).map(|o| o == uid).unwrap_or(true);
            vec![channel::AckFailedStartGame {
                result: 0,
                err_msg: String::new(),
                is_host: u8::from(is_host),
                failed_user_id: uid,
                loading_step: req.loading_step,
            }
            .packet()]
        }

        // A player left the match. When it is the host, the listen server is
        // gone with it, so the room is back to waiting - otherwise the room
        // stays "playing" and everyone entering it later gets a Join button
        // instead of Ready (matches quit early never send rfReqGameOver).
        channel::ReqDisconnectGame::OPCODE => {
            tracing::info!(uid, "ReqDisconnectGame");
            if room::is_owner(uid) && room::is_playing_of(uid) {
                // The guests' clients otherwise sit on the dead listen server
                // for ~90 s before giving up (5061); this push makes them drop
                // the UDP connection and come back to the room at once.
                let others = room::others(uid);
                tracing::info!(uid, guests = ?others, "host left the match - disconnecting the guests");
                session::send_many(&others, &[channel::ReqRecvForceDisconnectUDPFromHost { result: 0 }.packet()]);
                end_match(uid);
            }
            let ack = channel::AckDisconnectGame {
                result: 0,
                disconnect_user_id: uid,
                disconnect_user_name: ctx.name.clone(),
            }
            .packet();
            session::send_many(&room::others(uid), std::slice::from_ref(&ack));
            vec![ack]
        }

        // --- in the match (listen server on the host client)
        // The host reports each player that joined its listen server, itself
        // first: that is the moment its map load is over and it accepts late
        // joiners, so any held registration is replayed to it now.
        channel::ReqNotifyClientJoinTime::OPCODE => {
            let req = channel::ReqNotifyClientJoinTime::decode(&pkt.payload)?;
            tracing::info!(uid, joined = req.joined_client_id, "ReqNotifyClientJoinTime");
            if room::is_owner(uid) && req.joined_client_id == uid {
                host_ready(uid);
            }
            vec![channel::AckNotifyClientJoinTime { result: 0, err_msg: String::new() }.packet()]
        }

        // The round clock starting is another sign the host is in the match.
        channel::ReqGamePlaySetGameStartTime::OPCODE => {
            if room::is_owner(uid) && !room::is_host_ready_of(uid) {
                host_ready(uid);
            }
            vec![]
        }

        // Team balance asks the server for bot identities to fill the empty slots
        // (payloads seen: AF=0/RSA=6, AF=5/RSA=0). Hand out as many as requested;
        // the ids just need to be unique and not collide with real UIDs.
        channel::ReqGetBotID::OPCODE => {
            let req = channel::ReqGetBotID::decode(&pkt.payload)?;
            tracing::info!(af = req.af_count, rsa = req.rsa_count, "ReqGetBotID");
            vec![channel::AckGetBotID {
                result: 0,
                requested_af_count: req.af_count,
                requested_rsa_count: req.rsa_count,
                af_bot_id: (0..req.af_count as i32).map(|i| 1000 + i).collect(),
                rsa_bot_id: (0..req.rsa_count as i32).map(|i| 2000 + i).collect(),
            }
            .packet()]
        }

        // Fire-and-forget notifications from the host; no ack defined.
        channel::ReqGamePlaySetRoundStartTime::OPCODE
        | channel::ReqGamePlayChangeWeaponSet::OPCODE
        | channel::ReqNotifyLoadingStatus::OPCODE
        | channel::ReqNotifyRelayServerAddr::OPCODE
        | channel::ReqRelayServerOn::OPCODE
        | channel::ReqRelayServerOff::OPCODE => vec![],

        _ => return Ok(None),
    }))
}

/// The match of `uid`'s room is over: back to waiting, and every member and
/// the lobby hear about it (`rfReqNotifyChangeRoomState` flips the room page's
/// Start/Join button back to Ready).
pub fn end_match(uid: i32) {
    room::set_playing(uid, false);
    let mut packets = vec![channel::ReqNotifyChangeRoomState { room_state: 0 }.packet()];
    // everyone is "not ready" again, like the clients' own bReadyToGame reset
    if let Some((number, members)) = room::reset_states(uid) {
        let n = members.len();
        packets.push(
            channel::AckChangeUserState {
                result: 0,
                err_msg: String::new(),
                user_i_ds: members.clone(),
                positions: (0..n as u8).collect(),
                numbers: vec![number; n],
                old_states: vec![1; n],
                new_states: vec![0; n],
            }
            .packet(),
        );
    }
    session::send_many(&room::uids_of(uid), &packets);
    if let Some(number) = room::number_of(uid) {
        super::lobby::push_room_list(number);
    }
}

/// The host finished loading: replay the held late joiners to it (their UDP
/// registration and their hole-punching request), a moment later so the
/// host's own loading flag has certainly flipped.
fn host_ready(host: i32) {
    let pending = room::set_host_ready(host);
    if pending.is_empty() {
        return;
    }
    tracing::info!(host, ?pending, "host ready - replaying held late joiners");
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(1500));
        for uid in pending {
            let Some(m) = room::member(uid) else { continue };
            if room::owner_of(uid) != Some(host) {
                continue;
            }
            let mut packets = vec![];
            if let Some(reg) = other_udp_info(&m, session::nat_of(uid)) {
                packets.push(reg);
            }
            packets.push(channel::AckStartHolePunching { result: 0, err_msg: String::new(), game_type: 1, req_start_user_id: uid }.packet());
            tracing::info!(host, uid, "late join: replaying registration and hole punching");
            session::send_to(host, &packets);
        }
    });
}

/// `rfAckRegisterOtherUdpInfo` describing member `m` to another player.
fn other_udp_info(m: &Member, nat_type: u8) -> Option<Packet> {
    let u = m.udp.as_ref()?;
    Some(
        channel::AckRegisterOtherUdpInfo {
            uid: m.uid,
            ip: u.ip.clone(),
            port: u.port,
            nat_type,
            intervention: u8::from(u.intervention),
            host_inner_ip: u.inner_ip.clone(),
            host_inner_port: u.inner_port,
        }
        .packet(),
    )
}
