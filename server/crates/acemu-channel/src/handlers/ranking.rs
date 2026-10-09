//! The My Info page's ranking tab, another player's record sheet
//! (`rfReqDBCharInfo`), and the community options (whisper / invite
//! permissions).
//!
//! Ranking (`BTTPMyInfoRankHK`): the main combo picks the table -
//! 0 Overall, 1 Overall monthly, 2/3 Women's, 4/5 Headshot, 6-11 clan tables;
//! the sub combo the scope - 0 Friends & clan, 1 PC cafe, 2 Self, 3 Top. Each
//! row carries the rank position (`Ranking_Accum`), the previous position
//! (`Ranking_Accum_Prev`, the arrow is their difference) and the ranked value
//! (`Ranking_Accum_Exp`). There is no monthly history yet, so the monthly
//! tables repeat the all-time ones; clan tables are empty.

use super::Reply;
use crate::inventory;
use crate::session::Ctx;
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqGetRankingInfoList::OPCODE => {
            let req = channel::ReqGetRankingInfoList::decode(&pkt.payload)?;
            let players = inventory::db().players().unwrap_or_default();
            let mut ranked: Vec<(String, i32, i32)> = match req.ranking_type {
                0 | 1 => players.iter().map(|p| (p.name.clone(), p.level, p.exp)).collect(),
                4 | 5 => inventory::db()
                    .leaderboard()
                    .unwrap_or_default()
                    .iter()
                    .map(|l| (l.name.clone(), l.level, l.headshots))
                    .collect(),
                _ => vec![],
            };
            ranked.sort_by(|a, b| b.2.cmp(&a.2).then_with(|| a.0.cmp(&b.0)));
            let rows: Vec<(usize, &(String, i32, i32))> = ranked
                .iter()
                .enumerate()
                .filter(|(_, r)| match req.community_type {
                    2 => r.0 == ctx.name,
                    _ => true,
                })
                .take(100)
                .collect();
            tracing::info!(ranking_type = req.ranking_type, community_type = req.community_type, rows = rows.len(), "ReqGetRankingInfoList");
            let mut out = vec![channel::AckGetRankingInfoListStart {
                result: 0,
                ranking_type: req.ranking_type,
                community_type: req.community_type,
                total_count: rows.len() as i32,
            }
            .packet()];
            for chunk in rows.chunks(25) {
                let n = chunk.len();
                out.push(
                    channel::AckGetRankingInfoList {
                        result: 0,
                        ranking_type: req.ranking_type,
                        community_type: req.community_type,
                        char_name: chunk.iter().map(|(_, r)| r.0.clone()).collect(),
                        char_level: chunk.iter().map(|(_, r)| r.1).collect(),
                        clan_name: vec![String::new(); n],
                        clan_mark_pattern: vec![0; n],
                        clan_mark_bg: vec![0; n],
                        clan_mark_bl: vec![0; n],
                        ranking_accum: chunk.iter().map(|(i, _)| *i as i32 + 1).collect(),
                        ranking_accum_prev: chunk.iter().map(|(i, _)| *i as i32 + 1).collect(),
                        ranking_accum_exp: chunk.iter().map(|(_, r)| r.2).collect(),
                        level_mark_id: vec![0; n],
                        title_mark_id: vec![0; n],
                        reserved1: vec![0; n],
                        reserved2: vec![0; n],
                        reserved3: vec![0; n],
                    }
                    .packet(),
                );
            }
            out.push(channel::AckGetRankingInfoListEnd.packet());
            out
        }

        // Another player's record sheet: career stats plus what they have
        // equipped (main and sub weapon with parts, the special costume).
        channel::ReqDBCharInfo::OPCODE => {
            let req = channel::ReqDBCharInfo::decode(&pkt.payload)?;
            let name = req.char_name.trim().to_string();
            let exists = inventory::db().player(&name).ok().flatten().is_some();
            tracing::info!(char = %name, exists, "ReqDBCharInfo");
            if !exists {
                return Ok(Some(vec![channel::AckDBCharInfo { result: 1, err_msg: "no such player".into(), char_name: name, ..Default::default() }.packet()]));
            }
            let (main, sub, costume) = inventory::read_named(&name, |inv| {
                let at = |slot: i32| inv.items.iter().find(|i| i.slot == slot).map(|i| (i.item_id, i.add_part_id)).unwrap_or((0, 0));
                let costume = inv.items.iter().find(|i| i.slot == 22).map(|i| i.item_id).unwrap_or(0);
                (at(0), at(1), costume)
            });
            vec![channel::AckDBCharInfo {
                result: 0,
                err_msg: String::new(),
                char_name: name.clone(),
                clan_name: String::new(),
                cm_pattern: 0,
                cm_bg: 0,
                cm_bl: 0,
                clan_person_intro: String::new(),
                main_weapon_item_id: main.0,
                main_weapon_part_id: main.1,
                sub_weapon_item_id: sub.0,
                sub_weapon_part_id: sub.1,
                af_special_costume_item_id: costume,
                rsa_special_costume_item_id: costume,
                st_char_info: crate::player::char_info(&name),
                clan_level: 0,
            }
            .packet()]
        }

        channel::ReqCommunityOptionList::OPCODE => {
            let e = inventory::db().extra(&ctx.name).unwrap_or_default();
            vec![channel::AckCommunityOptionList {
                opt_wispher: e.opt_whisper as u16,
                opt_game_invite: e.opt_game_invite as u16,
                opt_friend_invite: e.opt_friend_invite as u16,
                opt_clan_invite: e.opt_clan_invite as u16,
            }
            .packet()]
        }

        channel::ReqSetCommunityOption::OPCODE => {
            let req = channel::ReqSetCommunityOption::decode(&pkt.payload)?;
            let ok = inventory::db()
                .extra_set_options(&ctx.name, req.opt_wispher as i32, req.opt_game_invite as i32, req.opt_friend_invite as i32, req.opt_clan_invite as i32)
                .is_ok();
            vec![channel::AckSetCommunityOption { result: if ok { 0 } else { 1 }, err_str: String::new() }.packet()]
        }

        _ => return Ok(None),
    }))
}
