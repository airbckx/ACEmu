//! Friends, block list, room invites and whispers.
//!
//! Flows, from the client behaviour (`BTPageLobby`, `BTPageRoomLobby`,
//! `BTPageServerSelection`):
//!
//! - **Friend invite**: A sends `rfReqSendFriendInvite(uid, name, msg)`; A gets
//!   `rfAckSendFriendInvite` and B (online) gets the push
//!   `rfReqRecvFriendInvite(A.uid, A.name, msg)`, which opens B's request
//!   window. B answers `rfReqConfirmFriendInvite(A.uid, A.name, IsPermit)` -
//!   0 accept, 1 decline - gets `rfAckConfirmFriendInvite`, and A gets the push
//!   `rfReqCompleteFriendInvite(B.name, IsPermit)`; both then refresh their
//!   lists. `rfReqDeleteFriend(names)` answers with the names that failed and
//!   pushes `rfReqRecvDeleteFriend(0, from)` to an online ex-friend.
//! - **Room invite**: a room member sends `rfReqInviteGameRoom(guest)`; the
//!   guest gets `rfReqRecvInviteGameRoom(...)` and, after the popup, sends
//!   `rfAckRecvInviteGameRoom(roomUniqueId, roomNumber, answer, inviter)` -
//!   0 accept (the client then auto-joins the room by number), 1 decline,
//!   3 busy. The inviter gets `rfAckInviteGameRoom(0, answer, guest)`, whose
//!   answer 2 we use for "not online"; the guest gets
//!   `rfAckRecvInviteGameRoomResult`.
//! - **Whisper**: `rfReqChatWispher(to, msg)` -> the target gets
//!   `rfReqRecvChatWispher(from, msg)`; the sender's ack result 252 means
//!   "not online", 182 "refused" (blocked or whispers switched off).
//! - **Friend position**: where a friend is, for the messenger's "move to"
//!   entry: CurPos 0 offline, 1 in a channel lobby, 2 in a room (with
//!   GameRoomNum); CurState 2 = playing.

use super::Reply;
use crate::room;
use crate::session::{self, Ctx};
use crate::{inventory, player};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

const WHISPER_OFFLINE: i32 = 252;
const WHISPER_REFUSED: i32 = 182;
const INVITE_ANSWER_OFFLINE: u8 = 2;

fn uid_by_name(name: &str) -> Option<i32> {
    session::snapshot().into_iter().find(|(_, n, _, _)| n == name).map(|(u, _, _, _)| u)
}

fn is_online(name: &str) -> bool {
    uid_by_name(name).is_some()
}

fn friend_list_packets(name: &str) -> Vec<Packet> {
    let friends = inventory::db().friends_of(name).unwrap_or_default();
    let mut out = vec![channel::AckFriendListStart { result: 0, total_count: friends.len() as i32 }.packet()];
    if friends.is_empty() {
        out.push(channel::AckFriendList::default().packet());
    }
    for chunk in friends.chunks(50) {
        let n = chunk.len();
        out.push(
            channel::AckFriendList {
                friend_name: chunk.to_vec(),
                is_log_on: chunk.iter().map(|f| is_online(f) as u8).collect(),
                clan_name: vec![String::new(); n],
                cm_pattern: vec![0; n],
                cm_bg: vec![0; n],
                cm_bl: vec![0; n],
                level_mark_id: vec![0; n],
                clan_level: vec![0; n],
                level: chunk.iter().map(|f| player::level_of(f)).collect(),
                is_look_for_clan: vec![0; n],
            }
            .packet(),
        );
    }
    out.push(channel::AckFriendListEnd.packet());
    out
}

fn block_list_packets(name: &str) -> Vec<Packet> {
    let blocks = inventory::db().blocks_of(name).unwrap_or_default();
    let mut out = vec![channel::AckBlockUserListStart { result: 0, total_count: blocks.len() as i32 }.packet()];
    if blocks.is_empty() {
        out.push(channel::AckBlockUserList::default().packet());
    }
    for chunk in blocks.chunks(100) {
        out.push(channel::AckBlockUserList { char_name: chunk.to_vec() }.packet());
    }
    out.push(channel::AckBlockUserListEnd.packet());
    out
}

/// Whether `target` accepts contact from `from`: not blocked and, for
/// whispers, the option switched on.
fn accepts(target: &str, from: &str, whisper: bool) -> bool {
    let db = inventory::db();
    if db.is_blocked(target, from).unwrap_or(false) {
        return false;
    }
    if whisper {
        return db.extra(target).map(|e| e.opt_whisper != 0).unwrap_or(true);
    }
    true
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqFriendList::OPCODE => friend_list_packets(&ctx.name),
        channel::ReqBlockUserList::OPCODE => block_list_packets(&ctx.name),

        channel::ReqSendFriendInvite::OPCODE => {
            let req = channel::ReqSendFriendInvite::decode(&pkt.payload)?;
            let target = req.invite_name.trim().to_string();
            tracing::info!(to = %target, uid = req.invite_user_id, "ReqSendFriendInvite");
            let db = inventory::db();
            let result = if target.is_empty() || target == ctx.name {
                1
            } else if db.player(&target).ok().flatten().is_none() {
                db_social_no_such_player()
            } else if db.are_friends(&ctx.name, &target).unwrap_or(false) {
                0 // already friends: nothing to do, but not an error the client knows
            } else if !accepts(&target, &ctx.name, false) || !db.extra(&target).map(|e| e.opt_friend_invite != 0).unwrap_or(true) {
                WHISPER_REFUSED
            } else if let Some(uid) = uid_by_name(&target) {
                session::send_to(uid, &[channel::ReqRecvFriendInvite { req_user_id: ctx.uid, req_charname: ctx.name.clone(), invite_msg: req.invite_msg }.packet()]);
                0
            } else {
                WHISPER_OFFLINE
            };
            vec![channel::AckSendFriendInvite { result, err_msg: String::new() }.packet()]
        }

        channel::ReqConfirmFriendInvite::OPCODE => {
            let req = channel::ReqConfirmFriendInvite::decode(&pkt.payload)?;
            let requester = req.reqed_charname.trim().to_string();
            tracing::info!(from = %requester, permit = req.is_permit, "ReqConfirmFriendInvite");
            if req.is_permit == 0 {
                let _ = inventory::db().friend_add(&ctx.name, &requester);
            }
            // tell the requester; prefer the uid the client echoed, fall back to the name
            let target_uid = session::name_of(req.req_user_id).filter(|n| *n == requester).map(|_| req.req_user_id).or_else(|| uid_by_name(&requester));
            if let Some(uid) = target_uid {
                session::send_to(uid, &[channel::ReqCompleteFriendInvite { charname_to_be_invited: ctx.name.clone(), is_permit: req.is_permit }.packet()]);
            }
            vec![channel::AckConfirmFriendInvite { result: 0, err_msg: String::new() }.packet()]
        }

        channel::ReqDeleteFriend::OPCODE => {
            let req = channel::ReqDeleteFriend::decode(&pkt.payload)?;
            let mut failed = Vec::new();
            for name in &req.delete_char_name {
                match inventory::db().friend_remove(&ctx.name, name) {
                    Ok(true) => {
                        if let Some(uid) = uid_by_name(name) {
                            session::send_to(uid, &[channel::ReqRecvDeleteFriend { result: 0, from_char_name: ctx.name.clone() }.packet()]);
                        }
                    }
                    _ => failed.push(name.clone()),
                }
            }
            vec![channel::AckDeleteFriend { result: 0, failed_charname: failed }.packet()]
        }

        channel::ReqAddBlockUserList::OPCODE => {
            let req = channel::ReqAddBlockUserList::decode(&pkt.payload)?;
            let target = req.char_name.trim().to_string();
            let ok = !target.is_empty() && target != ctx.name && inventory::db().block_add(&ctx.name, &target).is_ok();
            vec![channel::AckAddBlockUserList { result: if ok { 0 } else { 1 }, err_msg: String::new() }.packet()]
        }

        channel::ReqRemoveBlockUserList::OPCODE => {
            let req = channel::ReqRemoveBlockUserList::decode(&pkt.payload)?;
            let _ = inventory::db().block_remove(&ctx.name, req.char_name.trim());
            vec![channel::AckRemoveBlockUserList { result: 0, err_msg: String::new() }.packet()]
        }

        channel::ReqFriendPosition::OPCODE => {
            let req = channel::ReqFriendPosition::decode(&pkt.payload)?;
            let target = req.target_char_name.trim().to_string();
            let (cur_pos, cur_state, room_num) = match uid_by_name(&target) {
                Some(uid) if room::contains(uid) => (2u8, if room::is_playing_of(uid) { 2u8 } else { 0 }, room::number_of(uid).unwrap_or(0)),
                Some(_) => (1, 0, -1),
                None => (0, 0, -1),
            };
            vec![channel::AckFriendPosition {
                result: 0,
                target_char_name: target,
                server_type: 0,
                f_servername: "ACEmu".into(),
                f_server_short_name: "AC".into(),
                f_server_ip: crate::local_ip(),
                f_server_port: 55999,
                cur_pos,
                cur_state,
                channel_num: 0,
                channel_nick_name: "CH1".into(),
                game_room_num: room_num,
            }
            .packet()]
        }

        channel::ReqInviteGameRoom::OPCODE => {
            let req = channel::ReqInviteGameRoom::decode(&pkt.payload)?;
            let guest = req.guest_char_name.trim().to_string();
            tracing::info!(guest = %guest, "ReqInviteGameRoom");
            let Some(info) = room::info_of(ctx.uid) else {
                return Ok(Some(vec![channel::AckInviteGameRoom { result: 1, answer: 0, guest_char_name: guest }.packet()]));
            };
            match uid_by_name(&guest) {
                Some(uid) if !room::contains(uid) && accepts(&guest, &ctx.name, false) && inventory::db().extra(&guest).map(|e| e.opt_game_invite != 0).unwrap_or(true) => {
                    let owner = room::owner_of(ctx.uid).and_then(session::name_of).unwrap_or_default();
                    session::send_to(
                        uid,
                        &[channel::ReqRecvInviteGameRoom {
                            invite_user_server_name: "ACEmu".into(),
                            invite_user_server_id: 0,
                            invite_user_channel_number: 0,
                            room_unique_id: info.room_number.to_string(),
                            room_number: info.room_number as u16,
                            room_name: info.room_name.clone(),
                            room_state: room::is_playing_of(ctx.uid) as u8,
                            map_name: info.map_name.clone(),
                            owner_name: owner,
                            game_mode: info.game_mode_num,
                            weapon_limit: info.weapon_limit,
                            is_team_balance: info.is_team_valance,
                            password: info.password.clone(),
                            invite_char_name: ctx.name.clone(),
                        }
                        .packet()],
                    );
                    // the inviter hears back when the guest answers
                    vec![]
                }
                _ => vec![channel::AckInviteGameRoom { result: 0, answer: INVITE_ANSWER_OFFLINE, guest_char_name: guest }.packet()],
            }
        }

        // The guest's answer to a room invite (sent by the client as an "ack").
        channel::AckRecvInviteGameRoom::OPCODE => {
            let req = channel::AckRecvInviteGameRoom::decode(&pkt.payload)?;
            tracing::info!(inviter = %req.invite_char_name, answer = req.answer, room = req.room_number, "AckRecvInviteGameRoom");
            if let Some(uid) = uid_by_name(req.invite_char_name.trim()) {
                session::send_to(uid, &[channel::AckInviteGameRoom { result: 0, answer: req.answer, guest_char_name: ctx.name.clone() }.packet()]);
            }
            vec![channel::AckRecvInviteGameRoomResult { result: 0 }.packet()]
        }

        channel::ReqChatWispher::OPCODE => {
            let req = channel::ReqChatWispher::decode(&pkt.payload)?;
            let to = req.char_name.trim().to_string();
            let result = match uid_by_name(&to) {
                Some(uid) if accepts(&to, &ctx.name, true) => {
                    session::send_to(uid, &[channel::ReqRecvChatWispher { charname_from: ctx.name.clone(), message: req.message }.packet()]);
                    0
                }
                Some(_) => WHISPER_REFUSED,
                None => WHISPER_OFFLINE,
            };
            vec![channel::AckChatWispher { result, err_msg: String::new(), to_char_name: to }.packet()]
        }

        // Clan-wide whisper: no clans yet, so acknowledge without delivery.
        channel::ReqChatWispherMyCommunity::OPCODE => {
            vec![channel::AckChatWispherMyCommunity { result: 0, err_msg: String::new() }.packet()]
        }

        _ => return Ok(None),
    }))
}

/// The client's own code for "no such player" (shared with mail).
fn db_social_no_such_player() -> i32 {
    acemu_db::social::SEND_NO_SUCH_PLAYER
}
