//! The game room lobby: create, query and re-configure a room, enter and
//! leave it, the member list, ready state and team changes.

use super::{lobby, Reply};
use crate::loadout;
use crate::nat;
use crate::player::level_of;
use crate::room::{self, Member};
use crate::session::{self, Ctx};
use crate::local_ip;
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        channel::ReqCreateGameRoom::OPCODE => {
            let req = channel::ReqCreateGameRoom::decode(&pkt.payload)?;
            let mut info = req.net_room_info;
            tracing::info!(
                uid,
                name = %info.room_name,
                map = %info.map_name,
                mode = %info.game_class,
                max = info.max_user_cnt,
                mode_num = info.game_mode_num,
                bots = info.play_with_bots,
                user_team = info.user_team_num,
                af = info.max_af_user_and_bot_num,
                rsa = info.max_rsa_user_and_bot_num,
                coop = room::is_coop(&info),
                "ReqCreateGameRoom"
            );
            let (result, err_msg) = match room::create(uid, &ctx.name, info.clone()) {
                Ok(number) => {
                    info = room::info(number).unwrap_or(info);
                    tracing::info!(uid, room = number, "room created");
                    lobby::push_room_list(number);
                    (0, String::new())
                }
                Err(e) => (1, e.to_string()),
            };
            vec![channel::AckCreateGameRoom {
                result,
                err_msg,
                user_id: uid,
                room_owner_ip: local_ip(),
                net_room_info: info,
            }
            .packet()]
        }

        channel::ReqGameRoomInfo::OPCODE => {
            let req = channel::ReqGameRoomInfo::decode(&pkt.payload)?;
            tracing::info!(room = req.game_num, uid = req.uid, "ReqGameRoomInfo");
            // the asker's own room when it is in one (the client sometimes sends 0)
            let info = room::info_of(uid).or_else(|| room::info(req.game_num as i32));
            vec![channel::AckGameRoomInfo {
                result: if info.is_some() { 0 } else { 1 },
                err_msg: if info.is_some() { String::new() } else { "no such room".into() },
                room_info: info.unwrap_or_default(),
            }
            .packet()]
        }

        channel::ReqChangeRoomSetting::OPCODE => {
            let req = channel::ReqChangeRoomSetting::decode(&pkt.payload)?;
            let (info, others) = match room::update_info(uid, req.room_info.clone()) {
                Some((info, members)) => (info, members.into_iter().filter(|&u| u != uid).collect::<Vec<_>>()),
                None => (req.room_info, vec![]),
            };
            let ack = channel::AckChangeRoomSetting { result: 0, err_msg: String::new(), room_info: info.clone() }.packet();
            session::send_many(&others, std::slice::from_ref(&ack));
            lobby::push_room_list(info.room_number);
            vec![ack]
        }

        channel::ReqEnterRoom::OPCODE => {
            let req = channel::ReqEnterRoom::decode(&pkt.payload)?;
            tracing::info!(uid, room = req.game_num, enter_type = req.room_enter_type, "ReqEnterRoom");
            match room::join(uid, &ctx.name, req.game_num as i32, &req.password) {
                Err(e) => {
                    tracing::warn!(uid, room = req.game_num, error = e, "enter room refused");
                    vec![enter_room_ack(req.game_num, &Member::new(uid, &ctx.name), 0, 1, e)]
                }
                Ok(j) => {
                    let ack = enter_room_ack(req.game_num, &j.member, j.owner, 0, "");
                    let mut out = vec![];
                    if j.is_new {
                        // tell the others who came in, and what they carry
                        let others = room::others(uid);
                        session::send_many(&others, std::slice::from_ref(&ack));
                        session::send_many(&others, &loadout::entry_lists_for(uid, &ctx.name));
                        lobby::push_room_list(j.number);
                        // The client never runs its NAT diagnosis against our
                        // servers, so its NatType stays -1 ("Not Determined"). The
                        // host's Start button tolerates that, but a guest's Ready
                        // button refuses to send anything ("NatType OK, Ready state
                        // Request" only), so a joiner gets a NAT type handed to it.
                        if j.owner != uid {
                            let nat_type = nat::guest_nat_type();
                            session::set_nat(uid, nat_type as u8);
                            out.push(channel::ReqForceNatTypeSetting { nat_type }.packet());
                        }
                    }
                    out.push(ack);
                    out
                }
            }
        }

        channel::ReqRoomUserList::OPCODE => {
            let _ = channel::ReqRoomUserList::decode(&pkt.payload)?;
            // With the user list, the requester gets every member's Entry lists -
            // its own included: BTPageRoomLobby.rfAckEntryQSlotIDList fills
            // UserInfos[UserNum].EquippedQSlots, which the host reads at spawn.
            // (Not rfAckMyEquipedItemList: see loadout::push_loadout_change.)
            let mut out = vec![room_user_list(uid).packet()];
            for m in room::members_of(uid) {
                out.extend(loadout::entry_lists_for(m.uid, &m.name));
            }
            out
        }

        channel::ReqNotifyMyCharInfoToRoomUser::OPCODE => {
            let m = room::member(uid).unwrap_or_else(|| Member::new(uid, &ctx.name));
            let ack = channel::AckNotifyMyCharInfoToRoomUser {
                result: 0,
                err_msg: String::new(),
                uid,
                user_name: m.name.clone(),
                grade: level_of(&m.name),
                team_num: m.team as i32,
                statue: m.state,
                is_host: u8::from(room::is_owner(uid)),
                user_ip: local_ip(),
                clan_name: String::new(),
                cm_pattern: 0,
                cm_bg: 0,
                cm_bl: 0,
                is_look_for: 0,
            }
            .packet();
            session::send_many(&room::others(uid), std::slice::from_ref(&ack));
            vec![ack]
        }

        channel::ReqLeaveRoom::OPCODE => {
            let req = channel::ReqLeaveRoom::decode(&pkt.payload)?;
            tracing::info!(uid, room = req.num, "ReqLeaveRoom");
            lobby::push_room_change(room::leave(uid, &ctx.name));
            vec![channel::AckLeaveRoom {
                num: req.num,
                uid,
                user_id: ctx.name.clone(),
                result: 0,
                err_msg: String::new(),
            }
            .packet()]
        }

        // User state (lobby / in room / ready ...). Inside a room the change is
        // shown to everyone, so it goes to every member.
        channel::ReqChangeUserState::OPCODE => {
            let req = channel::ReqChangeUserState::decode(&pkt.payload)?;
            tracing::info!(uid, new_state = req.new_state, check_playing = req.check_playing, "ReqChangeUserState");
            let (number, old, pos, members) = room::set_state(uid, req.new_state).unwrap_or((0, 0, 0, vec![]));
            let ack = channel::AckChangeUserState {
                result: 0,
                err_msg: String::new(),
                user_i_ds: vec![uid],
                positions: vec![pos],
                numbers: vec![number],
                old_states: vec![old],
                new_states: vec![req.new_state],
            }
            .packet();
            session::send_many(&members.into_iter().filter(|&u| u != uid).collect::<Vec<_>>(), std::slice::from_ref(&ack));
            vec![ack]
        }

        channel::ReqChangeTeam::OPCODE => {
            let req = channel::ReqChangeTeam::decode(&pkt.payload)?;
            room::set_team(uid, req.team_num);
            let ack = channel::AckChangeTeam {
                game_number: room::number_of(uid).unwrap_or(0) as u16,
                user_id: ctx.name.clone(),
                uid,
                team_num: req.team_num,
                result: 0,
                err_msg: String::new(),
            }
            .packet();
            session::send_many(&room::others(uid), std::slice::from_ref(&ack));
            vec![ack]
        }

        channel::ReqIsGameRoomPlaying::OPCODE => {
            let req = channel::ReqIsGameRoomPlaying::decode(&pkt.payload)?;
            vec![channel::AckIsGameRoomPlaying {
                game_room_num: req.game_room_num,
                is_playing: u8::from(room::is_playing(req.game_room_num as i32)),
            }
            .packet()]
        }

        _ => return Ok(None),
    }))
}

fn room_user_list(uid: i32) -> channel::AckRoomUserList {
    let owner = room::owner_of(uid).unwrap_or(0);
    let m = room::members_of(uid);
    let n = m.len();
    channel::AckRoomUserList {
        uid: m.iter().map(|x| x.uid).collect(),
        user_name: m.iter().map(|x| x.name.clone()).collect(),
        grade: m.iter().map(|x| level_of(&x.name)).collect(),
        team_num: m.iter().map(|x| x.team as i32).collect(),
        statue: m.iter().map(|x| x.state).collect(),
        is_host: m.iter().map(|x| u8::from(x.uid == owner)).collect(),
        user_ip: vec![local_ip(); n],
        clan_name: vec![String::new(); n],
        cm_pattern: vec![0; n],
        cm_bg: vec![0; n],
        cm_bl: vec![0; n],
        is_looking_for: vec![0; n],
        gmuid: vec![0; n],
        clan_level: vec![0; n],
        level_mark_id: vec![0; n],
        title_mark_id: vec![0; n],
        reserved1: vec![0; n],
        reserved2: vec![0; n],
        reserved3: vec![0; n],
    }
}

fn enter_room_ack(game_num: u16, m: &Member, owner: i32, result: i32, err: &str) -> Packet {
    channel::AckEnterRoom {
        game_num,
        owner_id: owner,
        uid: m.uid,
        user_id: m.name.clone(),
        team_num: m.team,
        grade: level_of(&m.name) as u16,
        user_ip: local_ip(),
        clan_name: String::new(),
        clan_mark_pattern: 0,
        clan_mark_bg: 0,
        clan_mark_bl: 0,
        is_look_for: 0,
        result,
        err_msg: err.to_string(),
        clan_level: 0,
        level_mark_id: 0,
        title_mark_id: 0,
        reserved1: 0,
        reserved2: 0,
        reserved3: 0,
    }
    .packet()
}
