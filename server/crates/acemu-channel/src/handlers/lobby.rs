//! The channel lobby: channel list and enter/leave, user lists, the room list,
//! the player's own character sheet, chat and system messages.

use super::Reply;
use crate::player::{char_info, level_of};
use crate::session::{self, Ctx};
use crate::{room, local_ip};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

/// One `rfAckRoomList` row per room in `rooms` (an empty list for none).
fn room_rows(rooms: &[&room::Room]) -> channel::AckRoomList {
    channel::AckRoomList {
        game_num: rooms.iter().map(|r| r.info.room_number).collect(),
        title: rooms.iter().map(|r| r.info.room_name.clone()).collect(),
        map_name: rooms.iter().map(|r| r.info.map_name.clone()).collect(),
        map_num: rooms.iter().map(|r| r.info.map_num as i32).collect(),
        user_count: rooms.iter().map(|r| r.members.len() as u8).collect(),
        max_count: rooms.iter().map(|r| r.info.max_user_cnt).collect(),
        mode_name: rooms.iter().map(|r| r.info.game_class.clone()).collect(),
        mode_num: rooms.iter().map(|r| r.info.game_mode_num as i32).collect(),
        weapon_limit: rooms.iter().map(|r| r.info.weapon_limit).collect(),
        is_team_balance: rooms.iter().map(|r| r.info.is_team_valance).collect(),
        status: rooms.iter().map(|r| u8::from(r.playing)).collect(),
        room_owner_ip: rooms.iter().map(|_| local_ip()).collect(),
        game_minute: rooms.iter().map(|r| r.info.game_minute).collect(),
        room_owner_name: rooms.iter().map(|r| r.info.room_owner.clone()).collect(),
        is_special: rooms.iter().map(|_| 0).collect(),
        is_password: rooms.iter().map(|r| r.info.is_password).collect(),
        bot_difficult: rooms.iter().map(|r| r.info.bot_mode_difficulty).collect(),
        bot_mode_user_team_num: rooms.iter().map(|r| r.info.user_team_num).collect(),
    }
}

/// The lobby's full room list, one row per room.
fn room_list_all() -> channel::AckRoomList {
    let rows = room::each(|r| room_rows(&[r]));
    let mut all = channel::AckRoomList::default();
    for row in rows {
        all.game_num.extend(row.game_num);
        all.title.extend(row.title);
        all.map_name.extend(row.map_name);
        all.map_num.extend(row.map_num);
        all.user_count.extend(row.user_count);
        all.max_count.extend(row.max_count);
        all.mode_name.extend(row.mode_name);
        all.mode_num.extend(row.mode_num);
        all.weapon_limit.extend(row.weapon_limit);
        all.is_team_balance.extend(row.is_team_balance);
        all.status.extend(row.status);
        all.room_owner_ip.extend(row.room_owner_ip);
        all.game_minute.extend(row.game_minute);
        all.room_owner_name.extend(row.room_owner_name);
        all.is_special.extend(row.is_special);
        all.is_password.extend(row.is_password);
        all.bot_difficult.extend(row.bot_difficult);
        all.bot_mode_user_team_num.extend(row.bot_mode_user_team_num);
    }
    all
}

/// Sessions sitting in the channel lobby, i.e. in the channel but in no room.
fn lobby_uids() -> Vec<i32> {
    let in_room = room::all_uids();
    session::channel_users().into_iter().map(|(u, _)| u).filter(|u| !in_room.contains(u)).collect()
}

/// Pushes room `number`'s current row to everyone in the lobby, so the list
/// refreshes without the player pressing Refresh. The row is preceded by the
/// destroy notice for the same number: `BTPageLobby.rfAckRoomList` is meant to
/// replace a row it already has, but the list class in this build
/// (`BTACLLobbyHKCN`) stores the room number in column 0 while its
/// `FindRoomNo` still checks column 1, so a repeated row was *added* and the
/// room showed twice once it started. `rfAckGameRoomDestroy` removes by the
/// right column, so destroy + add is a clean replace.
pub fn push_room_list(number: i32) {
    let Some(list) = room::read(number, |r| room_rows(&[r])) else { return };
    let to = lobby_uids();
    if !to.is_empty() {
        tracing::debug!(room = number, to = ?to, "lobby refresh: room row");
        session::send_many(&to, &[channel::AckGameRoomDestroy { game_room_num: number as u16 }.packet(), list.packet()]);
    }
}

/// Tells everyone in the lobby that room `number` is gone
/// (`BTPageLobby.rfAckGameRoomDestroy` drops the row).
pub fn push_room_destroyed(number: i32) {
    let to = lobby_uids();
    if !to.is_empty() {
        tracing::debug!(room = number, to = ?to, "lobby refresh: room destroyed");
        session::send_many(&to, &[channel::AckGameRoomDestroy { game_room_num: number as u16 }.packet()]);
    }
}

/// After a member left (the result of `room::leave`): the row with the new
/// count, or the destroy notice. Nothing for `None` (was in no room).
pub fn push_room_change(left: Option<room::Left>) {
    match left {
        Some(l) if l.closed => push_room_destroyed(l.number),
        Some(l) => push_room_list(l.number),
        None => {}
    }
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        channel::ReqChannelList::OPCODE => {
            let req = channel::ReqChannelList::decode(&pkt.payload)?;
            tracing::info!(channel_type = req.b_channel_type, "ReqChannelList");
            vec![channel::AckChannelList {
                id: vec![0],
                title: vec!["Channel 1".into()],
                cur_user: vec![session::channel_users().len() as i32],
                max_user: vec![100],
            }
            .packet()]
        }

        channel::ReqEnterChannel::OPCODE => {
            let req = channel::ReqEnterChannel::decode(&pkt.payload)?;
            tracing::info!(uid, id = req.id, "ReqEnterChannel");
            session::set_in_channel(uid, true);
            vec![channel::AckEnterChannel {
                id: req.id,
                uid,
                user_id: ctx.name.clone(),
                level: level_of(&ctx.name),
                clan_name: String::new(),
                clan_mark_pattern: 0,
                clan_mark_bg: 0,
                clan_mark_bl: 0,
                clan_grade: 0,
                clan_win: 0,
                clan_lose: 0,
                is_look_for: 0,
                result: 0,
                err_msg: String::new(),
                clan_level: 0,
                level_mark_id: 0,
                title_mark_id: 0,
                reserved1: 0,
                reserved2: 0,
                reserved3: 0,
            }
            .packet()]
        }

        // Sent right after a room is created/joined: the client leaves the channel
        // lobby list before it switches to the room page.
        channel::ReqLeaveChannel::OPCODE => {
            let req = channel::ReqLeaveChannel::decode(&pkt.payload)?;
            tracing::info!(uid, id = req.id, "ReqLeaveChannel");
            vec![channel::AckLeaveChannel { result: 0, user_id: ctx.name.clone(), uid }.packet()]
        }

        channel::ReqRoomList::OPCODE => {
            let req = channel::ReqRoomList::decode(&pkt.payload)?;
            tracing::info!(channel = req.id, start = req.start_index, count = req.count, "ReqRoomList");
            let list = room_list_all();
            let n = list.game_num.len() as i32;
            vec![
                channel::AckRoomListStart { result: 0, total_count: n, count: n, start_index: req.start_index }.packet(),
                list.packet(),
                channel::AckRoomListEnd.packet(),
            ]
        }

        channel::ReqChannelUserList::OPCODE => {
            let req = channel::ReqChannelUserList::decode(&pkt.payload)?;
            let users = session::channel_users();
            let n = users.len();
            vec![channel::AckChannelUserList {
                total_count: n as i32,
                count: n as i32,
                start_index: req.start_index as i32,
                uid: users.iter().map(|u| u.0).collect(),
                user_id: users.iter().map(|u| u.1.clone()).collect(),
                user_levels: users.iter().map(|u| level_of(&u.1)).collect(),
                clan_name: vec![String::new(); n],
                clan_pattern: vec![0; n],
                clan_bg: vec![0; n],
                clan_bl: vec![0; n],
                clan_grade: vec![0; n],
                clan_win: vec![0; n],
                clan_lose: vec![0; n],
                is_look_for_clan: vec![0; n],
                result: 0,
                err_msg: String::new(),
                clan_level: vec![0; n],
                level_mark_id: vec![0; n],
                title_mark_id: vec![0; n],
                reserved1: vec![0; n],
                reserved2: vec![0; n],
                reserved3: vec![0; n],
            }
            .packet()]
        }

        channel::ReqChannelUserListInGameRoom::OPCODE => {
            let req = channel::ReqChannelUserListInGameRoom::decode(&pkt.payload)?;
            let users = session::channel_users();
            let n = users.len();
            vec![channel::AckChannelUserListInGameRoom {
                result: 0,
                total_count: n as i32,
                count: n as i32,
                start_index: req.start_index as i32,
                uid: users.iter().map(|u| u.0).collect(),
                char_name: users.iter().map(|u| u.1.clone()).collect(),
                user_level: users.iter().map(|u| level_of(&u.1)).collect(),
                clan_name: vec![String::new(); n],
                clan_pattern: vec![0; n],
                clan_bg: vec![0; n],
                clan_bl: vec![0; n],
                is_look_for_clan: vec![0; n],
                clan_level: vec![0; n],
                level_mark_id: vec![0; n],
                title_mark_id: vec![0; n],
                reserved1: vec![0; n],
                reserved2: vec![0; n],
                reserved3: vec![0; n],
            }
            .packet()]
        }

        channel::ReqCharInfo::OPCODE => vec![channel::AckCharInfo {
            result: 0,
            err_msg: String::new(),
            user_id: uid,
            char_name: ctx.name.clone(),
            clan_name: String::new(),
            cm_pattern: 0,
            cm_bg: 0,
            cm_bl: 0,
            st_char_info: char_info(&ctx.name),
            in_clan_rank: 0,
            clan_level: 0,
        }
        .packet()],

        // The client announces its own arrival to itself (RecvType=1, MsgCode=5).
        channel::ReqSendSystemMsg::OPCODE => {
            let req = channel::ReqSendSystemMsg::decode(&pkt.payload)?;
            tracing::info!(
                recv_type = req.recv_type,
                from = %req.send_char_name,
                to = %req.recv_charname,
                code = req.msg_info.msg_code,
                "ReqSendSystemMsg"
            );
            vec![channel::AckSendSystemMsg { result: 0 }.packet()]
        }

        // Chat: to the room when in one, otherwise to everyone in the lobby.
        channel::ReqChatNormal::OPCODE => {
            let req = channel::ReqChatNormal::decode(&pkt.payload)?;
            tracing::info!(uid, from = %ctx.name, msg = %req.message, "chat");
            let (id, targets) = if let Some(number) = room::number_of(uid) {
                (number as u16, room::uids_of(uid))
            } else {
                (0, session::channel_users().into_iter().map(|u| u.0).collect())
            };
            let ack = channel::AckChatNormal { id, sender: ctx.name.clone(), message: req.message, result: 0 }.packet();
            session::send_many(&targets.into_iter().filter(|&u| u != uid).collect::<Vec<_>>(), std::slice::from_ref(&ack));
            vec![ack]
        }

        _ => return Ok(None),
    }))
}
