//! Lobby lists the client asks for once and will not proceed without, answered
//! with empty or benign defaults: quests, events, PC-bang, mailbox, chat block,
//! community options and the per-weapon records page. When one of these grows real behaviour, move
//! it to the module that owns the feature.

use super::Reply;
use crate::session::Ctx;
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, _ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqCharBlockInfo::OPCODE => vec![channel::AckCharBlockInfo {
            result: 0,
            chat_block_start_time: 0.0,
            chat_block_end_time: 0.0,
            blocked_item_idx: vec![],
        }
        .packet()],

        channel::ReqGetMyMailBoxState::OPCODE => {
            vec![channel::AckGetMyMailBoxState { result: 0, is_full: 0, new_mail_count: 0 }.packet()]
        }

        // Mailbox contents: no mail system yet, so an empty list of the requested type.
        channel::ReqMemoList::OPCODE => {
            let req = channel::ReqMemoList::decode(&pkt.payload)?;
            vec![
                channel::AckStartMemoList { result: 0, err_msg: String::new(), type_: req.type_, total_count: 0 }.packet(),
                channel::AckMemoList::default().packet(),
            ]
        }

        channel::ReqQuestList::OPCODE => vec![
            channel::AckQuestListStart { result: 0, err_msg: String::new() }.packet(),
            channel::AckQuestList::default().packet(),
            channel::AckQuestListEnd.packet(),
        ],

        channel::ReqSpecialQuestRank::OPCODE => vec![channel::AckSpecialQuestRank::default().packet()],

        channel::ReqRegisteredPCBangInfo::OPCODE => vec![channel::AckRegisteredPCBangInfo {
            result: 0,
            err_msg: String::new(),
            is_registered_pc_bang: 0,
            is_main_pc_bang: 0,
            pc_bang_name: String::new(),
        }
        .packet()],

        channel::ReqCommunityOptionList::OPCODE => vec![channel::AckCommunityOptionList {
            opt_wispher: 1,
            opt_game_invite: 1,
            opt_friend_invite: 1,
            opt_clan_invite: 1,
        }
        .packet()],

        channel::ReqLimitedListInChannel::OPCODE => {
            vec![channel::AckLimitedListInChannel::default().packet()]
        }

        channel::ReqQuestGetAvailableEventList::OPCODE => {
            vec![channel::AckQuestGetAvailableEventList::default().packet()]
        }

        // Records page: per-weapon-class kill counters. Nothing tracked yet beyond
        // the headline numbers, so report zeros rather than nothing.
        channel::ReqCharDetailInfo::OPCODE => vec![channel::AckCharDetailInfo::default().packet()],

        _ => return Ok(None),
    }))
}
