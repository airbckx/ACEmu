//! Quests, event quests and their special rewards, coupon codes, the Fortune
//! Depot's capsule-count rewards, and the odd lobby notifications around them.
//!
//! How quests work (client behaviour): the quest *definitions* are the
//! client's own table (`data/quests.json`, seeded into `quests`); the server
//! only says which quest ids a player has and how far along they are
//! (`rfAckQuestList`). The client counts progress during a match
//! (`wMatchMaker.UpdateQuest`) and at the end reports the new totals with
//! `rfReqQuestUpdate`. Here that is stored, completed quests pay their reward
//! (points, EXP, items) and unlock the next quests of their chain
//! (`RewardQuestID`), and `rfReqQuestNewAvailable` tells the client, which
//! then re-requests the list. Daily quests (type 2) reset every day.
//!
//! Event quests (type 1, with a date window) show up in the lobby through
//! `rfAckQuestGetAvailableEventList`; their reward is claimed with
//! `rfReqReceiveSpecialReward`. The shipped table has none active; they can
//! be enabled from the database (`quests.event_start/event_end`, OLE dates).

use super::Reply;
use crate::inventory;
use crate::session::Ctx;
use acemu_db::social::{self as db_social, QuestRow};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

/// Quest categories the client only offers in PC cafes / through events we do
/// not emulate; they are never assigned.
fn assignable(q: &QuestRow) -> bool {
    q.enabled
        && q.quest_type != db_social::QUEST_EVENT
        && !q.category.starts_with("PC Cafe")
        && !q.category.starts_with("Puzzle")
        && !q.category.starts_with("Roll Call")
        && !q.category.starts_with("Special")
}

/// Today's ISO date from the OLE clock the rest of the server uses
/// (OLE day 0 is 1899-12-30; 25569 is 1970-01-01). Civil-from-days after
/// Howard Hinnant.
pub fn today() -> String {
    let days_since_epoch = inventory::ole_now().floor() as i64 - 25569;
    let z = days_since_epoch + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{y:04}-{m:02}-{d:02}")
}

/// Makes sure the player has a quest list: chain heads of every assignable
/// type at or below the player's level, plus the daily set for today.
fn ensure_assigned(name: &str) {
    let db = inventory::db();
    let Ok(all) = db.quests() else { return };
    let level = crate::player::level_of(name);
    let extra = db.extra(name).unwrap_or_default();
    let day = today();
    if extra.daily_day != day {
        let dailies: Vec<i32> = all.iter().filter(|q| q.quest_type == db_social::QUEST_DAILY).map(|q| q.id).collect();
        let _ = db.quest_remove(name, &dailies);
        let _ = db.extra_set_daily_day(name, &day);
    }
    let have: std::collections::HashSet<i32> = db.quest_progress(name).unwrap_or_default().iter().map(|p| p.quest_id).collect();
    let mut assign = Vec::new();
    for q in all.iter().filter(|q| assignable(q) && q.link_step <= 1 && q.create_level <= level) {
        if !have.contains(&q.id) {
            assign.push(q.id);
        }
    }
    // the day's attendance, puzzle and special quests (client rotation table)
    for id in super::daily::today_quests(name) {
        if !have.contains(&id) && all.iter().any(|q| q.id == id && q.enabled) {
            assign.push(id);
        }
    }
    if !assign.is_empty() {
        tracing::info!(player = name, quests = ?assign, "quests assigned");
        let _ = db.quest_assign(name, &assign);
    }
}

fn quest_list_packets(name: &str) -> Vec<Packet> {
    ensure_assigned(name);
    let db = inventory::db();
    let defs = db.quests().unwrap_or_default();
    let progress = db.quest_progress(name).unwrap_or_default();
    let rows: Vec<(i32, i32, u8, u8)> = progress
        .iter()
        .filter_map(|p| {
            let q = defs.iter().find(|q| q.id == p.quest_id)?;
            Some((p.quest_id, p.progress.min(q.mission_count), (q.quest_type == db_social::QUEST_DAILY) as u8, p.complete as u8))
        })
        .collect();
    let mut out = vec![channel::AckQuestListStart { result: 0, err_msg: String::new() }.packet()];
    for chunk in rows.chunks(100) {
        out.push(
            channel::AckQuestList {
                quest_id: chunk.iter().map(|r| r.0).collect(),
                count_prograss: chunk.iter().map(|r| r.1).collect(),
                quest_type: chunk.iter().map(|r| r.2).collect(),
                is_complete: chunk.iter().map(|r| r.3).collect(),
            }
            .packet(),
        );
    }
    out.push(channel::AckQuestListEnd.packet());
    out
}

/// Pays a completed quest's reward and returns the chain successors it unlocks.
fn complete_quest(name: &str, q: &QuestRow) -> Vec<i32> {
    let db = inventory::db();
    let _ = db.quest_mark_complete(name, q.id);
    inventory::add_points(name, q.reward_point, q.reward_exp);
    for &item in q.reward_items.iter().filter(|&&i| i > 0) {
        inventory::grant(name, item, 1);
    }
    let unlocked: Vec<i32> = q.next_quests.iter().copied().filter(|n| *n > 0).collect();
    if !unlocked.is_empty() {
        let _ = db.quest_assign(name, &unlocked);
    }
    tracing::info!(player = name, quest = q.id, desc = %q.description, points = q.reward_point, exp = q.reward_exp, unlocked = ?unlocked, "quest completed");
    unlocked
}

/// Active event quests: type 1 whose window contains now.
fn active_event_quests() -> Vec<QuestRow> {
    let now = inventory::ole_now();
    inventory::db()
        .quests()
        .unwrap_or_default()
        .into_iter()
        .filter(|q| q.enabled && q.quest_type == db_social::QUEST_EVENT && q.event_start > 0.0 && q.event_start <= now && now <= q.event_end)
        .collect()
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqQuestList::OPCODE => quest_list_packets(&ctx.name),

        // End-of-match report: new totals per quest.
        channel::ReqQuestUpdate::OPCODE => {
            let req = channel::ReqQuestUpdate::decode(&pkt.payload)?;
            tracing::info!(quests = ?req.quest_id, progress = ?req.count_prograss, "ReqQuestUpdate");
            let db = inventory::db();
            let defs = db.quests().unwrap_or_default();
            let done: std::collections::HashSet<i32> =
                db.quest_progress(&ctx.name).unwrap_or_default().iter().filter(|p| p.complete).map(|p| p.quest_id).collect();
            let mut achieved = Vec::new();
            let mut unlocked = Vec::new();
            let mut daily_pushes = Vec::new();
            for (id, progress) in req.quest_id.iter().zip(req.count_prograss.iter()) {
                let Some(q) = defs.iter().find(|q| q.id == *id) else { continue };
                let stored = db.quest_set_progress(&ctx.name, *id, *progress).unwrap_or(0);
                if stored >= q.mission_count && !done.contains(id) {
                    achieved.push(*id);
                    unlocked.extend(complete_quest(&ctx.name, q));
                    daily_pushes.extend(super::daily::on_quest_completed(&ctx.name, *id));
                }
            }
            let mut out = vec![channel::AckQuestUpdate { result: 0, err_msg: String::new() }.packet()];
            // attendance check-in / puzzle piece notices go first: the
            // quest-complete window the next packet opens draws the new pieces
            out.extend(daily_pushes);
            if !achieved.is_empty() {
                let is_daily: Vec<u8> = unlocked
                    .iter()
                    .map(|n| defs.iter().find(|q| q.id == *n).map(|q| (q.quest_type == db_social::QUEST_DAILY) as u8).unwrap_or(0))
                    .collect();
                out.push(
                    channel::ReqQuestNewAvailable { achived_quest_id: achieved, new_quest_id: unlocked, is_daily_quest: is_daily, expired_quest: vec![] }
                        .packet(),
                );
            }
            out
        }

        channel::ReqQuestGetAvailableEventList::OPCODE => {
            let ids: Vec<i32> = active_event_quests().iter().map(|q| q.id).collect();
            if !ids.is_empty() {
                let _ = inventory::db().quest_assign(&ctx.name, &ids);
            }
            vec![channel::AckQuestGetAvailableEventList { result: 0, err_msg: String::new(), quest_id: ids }.packet()]
        }

        // Who completed an event quest lately (the lobby prints the newest one).
        channel::ReqSpecialQuestRank::OPCODE => {
            let ids: Vec<i32> = inventory::db().quests().unwrap_or_default().iter().filter(|q| q.quest_type == db_social::QUEST_EVENT).map(|q| q.id).collect();
            let names = inventory::db().quest_completers(&ids).unwrap_or_default();
            vec![channel::AckSpecialQuestRank { char_list: names }.packet()]
        }

        channel::ReqReceiveSpecialReward::OPCODE => {
            let req = channel::ReqReceiveSpecialReward::decode(&pkt.payload)?;
            let db = inventory::db();
            let q = db.quests().unwrap_or_default().into_iter().find(|q| q.id == req.quest_id);
            let p = db.quest_progress(&ctx.name).unwrap_or_default().into_iter().find(|p| p.quest_id == req.quest_id);
            tracing::info!(quest = req.quest_id, found = q.is_some(), "ReqReceiveSpecialReward");
            vec![match (q, p) {
                (Some(q), Some(p)) if p.complete && !p.claimed => {
                    let _ = db.quest_mark_claimed(&ctx.name, q.id);
                    // the quest's own item, else the special mission's reward
                    // (index 21 of the daily reward list)
                    let (item, count) = q
                        .reward_items
                        .iter()
                        .copied()
                        .find(|&i| i > 0)
                        .map(|i| (i, 1))
                        .or_else(super::daily::special_reward)
                        .unwrap_or((0, 0));
                    let uid = if item > 0 { inventory::grant(&ctx.name, item, count).unwrap_or(0.0) } else { 0.0 };
                    channel::AckReceiveSpecialReward {
                        result: 0,
                        rewarded_quest_id: q.id,
                        rewarded_quest_is_complete: 1,
                        item_idx: uid,
                        reward_item_id: item,
                        reward_item_count: count,
                    }
                    .packet()
                }
                _ => channel::AckReceiveSpecialReward { result: 1, rewarded_quest_id: req.quest_id, ..Default::default() }.packet(),
            }]
        }

        channel::ReqEventNotify::OPCODE => vec![channel::AckEventNotify { type_: 0 }.packet()],

        channel::ReqSetUserBootySeedValue::OPCODE => {
            let req = channel::ReqSetUserBootySeedValue::decode(&pkt.payload)?;
            let _ = inventory::db().extra_set_booty_seed(&ctx.name, req.seed_value);
            vec![channel::AckSetUserBootySeedValue { user_id: req.user_id, result: 0, err_msg: String::new() }.packet()]
        }

        // Coupon codes (admin-created rows in `coupons`): items, points or cash.
        channel::ReqCouponUse::OPCODE => {
            let req = channel::ReqCouponUse::decode(&pkt.payload)?;
            let grant = inventory::db().coupon_redeem(&req.serial, &ctx.name).ok().flatten();
            tracing::info!(serial = %req.serial, ok = grant.is_some(), "ReqCouponUse");
            match grant {
                Some(g) => {
                    if g.item_id > 0 {
                        inventory::grant(&ctx.name, g.item_id, g.item_count.max(1));
                    }
                    if g.points > 0 || g.cash > 0 {
                        inventory::with_named(&ctx.name, |inv| {
                            inv.points += g.points.max(0);
                            inv.cash += g.cash.max(0);
                            ((), true)
                        });
                    }
                    // The ack carries no amounts and the client asks for nothing
                    // afterwards, so its balance and bag would stay stale until
                    // the next shop visit: push both now.
                    let mut out = vec![channel::AckCouponUse { result: 0, err_msg: String::new() }.packet()];
                    out.extend(super::items::balance_packets(&ctx.name));
                    if g.item_id > 0 {
                        out.extend(super::items::item_list_packets(&ctx.name));
                    }
                    out
                }
                None => vec![channel::AckCouponUse { result: 1, err_msg: "invalid coupon".into() }.packet()],
            }
        }

        // Fortune Depot > Box Storage: capsules bought so far and which
        // count-rewards were already taken (bit per reward index).
        channel::ReqLuckShopStorageInfo::OPCODE => {
            let extra = inventory::db().extra(&ctx.name).unwrap_or_default();
            vec![channel::AckLuckShopStorageInfo {
                error_code: 0,
                buying_capsule_count: extra.capsule_count.clamp(0, u16::MAX as i32) as u16,
                capsule_reward_state: extra.capsule_reward_state,
            }
            .packet()]
        }

        // Claim the reward for having bought N capsules: the chosen item from
        // the reward box's list is granted and the reward's bit is set.
        channel::ReqLuckShopRewardBuyingCount::OPCODE => {
            let req = channel::ReqLuckShopRewardBuyingCount::decode(&pkt.payload)?;
            let db = inventory::db();
            let extra = db.extra(&ctx.name).unwrap_or_default();
            let reward = db.capsule_rewards().unwrap_or_default().into_iter().find(|r| r.reward_type == 0 && r.need == req.choose_num as i32);
            tracing::info!(need = req.choose_num, box_id = req.item_id, choose = req.item_id_choose, "ReqLuckShop_RewardBuyingCount");
            vec![match reward {
                Some(r) if extra.capsule_count >= r.need && (extra.capsule_reward_state & (1 << r.index)) == 0 => {
                    let state = extra.capsule_reward_state | (1 << r.index);
                    let _ = db.extra_set_reward_state(&ctx.name, state);
                    inventory::grant(&ctx.name, req.item_id_choose, 1);
                    channel::AckLuckShopRewardBuyingCount { error_code: 0, final_capsule_reward_state: state }.packet()
                }
                _ => channel::AckLuckShopRewardBuyingCount { error_code: 1, final_capsule_reward_state: extra.capsule_reward_state }.packet(),
            }]
        }

        // Trade N Supply Box Tickets (item 17700) for the chosen item.
        channel::ReqLuckShopRewardCoupon::OPCODE => {
            let req = channel::ReqLuckShopRewardCoupon::decode(&pkt.payload)?;
            const TICKET: i32 = 17700;
            let need = req.choose_num as i32;
            let taken = inventory::with_named(&ctx.name, |inv| {
                let have: i32 = inv.items.iter().filter(|i| i.item_id == TICKET).map(|i| i.stack).sum();
                if have < need || need <= 0 {
                    return (None, false);
                }
                let mut left = need;
                inv.items.retain_mut(|i| {
                    if i.item_id != TICKET || left == 0 {
                        return true;
                    }
                    let use_n = i.stack.min(left);
                    i.stack -= use_n;
                    left -= use_n;
                    i.stack > 0
                });
                let (uid, remaining) = inv
                    .items
                    .iter()
                    .find(|i| i.item_id == TICKET)
                    .map(|i| (i.unique_id, i.stack))
                    .unwrap_or((0.0, 0));
                (Some((uid, remaining)), true)
            });
            tracing::info!(need, choose = req.item_id_choose, ok = taken.is_some(), "ReqLuckShop_RewardCoupon");
            vec![match taken {
                Some((uid, remaining)) => {
                    inventory::grant(&ctx.name, req.item_id_choose, 1);
                    channel::AckLuckShopRewardCoupon { error_code: 0, coupon_item_type_id: TICKET, coupon_item_unique_id: uid, coupon_count_after_reduction: remaining }
                        .packet()
                }
                None => channel::AckLuckShopRewardCoupon { error_code: 1, coupon_item_type_id: TICKET, ..Default::default() }.packet(),
            }]
        }

        _ => return Ok(None),
    }))
}

/// Loads the quest and capsule-reward tables into the database once.
pub fn seed_tables() {
    let db = inventory::db();
    let quests: Vec<QuestRow> = inventory::read_json_table("ACEMU_QUESTS", "quests.json", "quest table");
    match db.quests_seed(&quests) {
        Ok(n) if n > 0 => tracing::info!(quests = n, "quest table seeded"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "quest table seeding failed"),
    }
    let rewards: Vec<db_social::CapsuleRewardRow> = inventory::read_json_table("ACEMU_CAPSULE_REWARDS", "capsule-rewards.json", "capsule-reward table");
    match db.capsule_rewards_seed(&rewards) {
        Ok(n) if n > 0 => tracing::info!(rewards = n, "capsule-reward table seeded"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "capsule-reward table seeding failed"),
    }
    let daily: Vec<db_social::DailyQuestRow> = inventory::read_json_table("ACEMU_DAILY_QUESTS", "daily-quests.json", "daily-quest table");
    match db.daily_quests_seed(&daily) {
        Ok(n) if n > 0 => tracing::info!(rows = n, "daily-quest table seeded"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "daily-quest table seeding failed"),
    }
    let daily_rewards: Vec<db_social::DailyRewardRow> = inventory::read_json_table("ACEMU_DAILY_REWARDS", "daily-rewards.json", "daily-reward table");
    match db.daily_rewards_seed(&daily_rewards) {
        Ok(n) if n > 0 => tracing::info!(rows = n, "daily-reward table seeded"),
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "daily-reward table seeding failed"),
    }
    // A code everyone can redeem once, so the coupon path can be tried without
    // touching the database (Item Shop > Redeem Codes: eight boxes of four).
    if let Ok(n) = db.coupons_prune_invalid() {
        if n > 0 {
            tracing::info!(removed = n, "coupon codes the client cannot enter were removed");
        }
    }
    if let Err(e) = db.coupon_put("ACEM-UWEL-COME-2026-FREE-5000-POIN-TS00", 0, 1, 5000, 0, 1_000_000, "welcome gift: 5000 points, once per player") {
        tracing::warn!(error = %e, "welcome coupon");
    }
}
