//! Item upkeep: repairing worn weapons, the room's check for worn-out
//! equipment, and consumables used during a match. Wear itself is applied
//! from the host's end-of-match report (`rfReqUpdateWeaponDurability`, in
//! `results.rs`), which also pushes `rfReqNotifyDurabilityZeroItem` to the
//! owner of anything that hit zero (see `durability_zero_notice`).
//!
//! Client behaviour: `rfAckRepairItem`'s `RepairCost` is the
//! player's point balance *after* the repair (`MM.kPoint = RepairCost`), and
//! every listed instance is set to 100 000 durability with the returned
//! `DamageDegree`. `rfAckCheckEquipedWeaponDurability` lists equipped
//! instances with no durability left and their slots; the room page uses it
//! to warn before a match.

use super::Reply;
use crate::inventory;
use crate::session::Ctx;
use acemu_proto::channel;
use acemu_proto::frame::Packet;

/// The push telling `owner` which of their weapons just wore out.
pub fn durability_zero_notice(owner_uid: i32, owner: &str, unique_ids: &[f64]) -> Option<Packet> {
    let briefs: Vec<(f64, i32, i32, i32)> = unique_ids.iter().filter_map(|&u| inventory::item_brief(owner, u).map(|(id, part, slot)| (u, id, part, slot))).collect();
    if briefs.is_empty() {
        return None;
    }
    Some(
        channel::ReqNotifyDurabilityZeroItem {
            result: 0,
            err_msg: String::new(),
            user_id: owner_uid,
            item_unique_i_ds: briefs.iter().map(|b| b.0).collect(),
            item_i_ds: briefs.iter().map(|b| b.1).collect(),
            item_part_i_ds: briefs.iter().map(|b| b.2).collect(),
            item_slot_pos: briefs.iter().map(|b| if b.3 < 0 { inventory::SLOT_NONE as u8 } else { b.3 as u8 }).collect(),
        }
        .packet(),
    )
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqRepairItem::OPCODE => {
            let req = channel::ReqRepairItem::decode(&pkt.payload)?;
            tracing::info!(items = ?req.unique_item_id, "ReqRepairItem");
            vec![match inventory::repair(&ctx.name, &req.unique_item_id) {
                Some((_cost, points_left)) => channel::AckRepairItem {
                    result: 0,
                    err_msg: String::new(),
                    unique_item_id: req.unique_item_id.clone(),
                    repair_cost: points_left,
                    damage_degree: vec![0; req.unique_item_id.len()],
                }
                .packet(),
                None => channel::AckRepairItem {
                    // 204 is the client's "not enough points" define
                    result: 204,
                    err_msg: "not enough points".into(),
                    unique_item_id: req.unique_item_id,
                    repair_cost: inventory::read_named(&ctx.name, |inv| inv.points),
                    damage_degree: vec![],
                }
                .packet(),
            }]
        }

        channel::ReqCheckEquipedWeaponDurability::OPCODE => {
            let worn = inventory::equipped_worn_out(&ctx.name);
            vec![channel::AckCheckEquipedWeaponDurability {
                result: 0,
                item_unique_id: worn.iter().map(|w| w.0).collect(),
                item_slot_num: worn.iter().map(|w| w.1).collect(),
            }
            .packet()]
        }

        // A consumable used in a match (the host reports for any player).
        channel::ReqUseItemDuringGame::OPCODE => {
            let req = channel::ReqUseItemDuringGame::decode(&pkt.payload)?;
            let who = if req.char_name.is_empty() { ctx.name.clone() } else { req.char_name.clone() };
            let left = inventory::consume(&who, req.item_unique_id, req.using_count as i32);
            tracing::info!(player = %who, item = req.item_id, uid = req.item_unique_id, count = req.using_count, left = ?left, "ReqUseItemDuringGame");
            vec![channel::AckUseItemDuringGame {
                error_code: if left.is_some() { 0 } else { 1 },
                char_name: who,
                item_id: req.item_id,
                item_unique_id: req.item_unique_id,
                remain_count: left.unwrap_or(0),
            }
            .packet()]
        }

        _ => return Ok(None),
    }))
}
