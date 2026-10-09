//! Inventory and loadout: item, skill and quick-slot lists (own and other
//! members'), equipping, weapon remodelling and quick-slot updates.

use super::Reply;
use crate::inventory;
use crate::loadout;
use crate::session::{self, Ctx};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

/// The full item list of `name` (`rfAckItemListStart/List/End`), also pushed
/// unasked after a server-side grant the client would not reload for itself.
pub fn item_list_packets(name: &str) -> Vec<Packet> {
    inventory::with_named(name, |inv| {
        // rentals whose time is up are dropped before the list goes out
        let gone = inv.purge_expired();
        if !gone.is_empty() {
            tracing::info!(count = gone.len(), ids = ?gone.iter().map(|i| i.item_id).collect::<Vec<_>>(), "expired rentals removed");
        }
        let n = inv.items.len();
        let packets = vec![
            channel::AckItemListStart { count: n as i32, result: 0, err_msg: String::new() }.packet(),
            channel::AckItemList {
                unique_id: inv.items.iter().map(|i| i.unique_id).collect(),
                item_id: inv.items.iter().map(|i| i.item_id).collect(),
                item_type: inv.items.iter().map(|i| i.item_type).collect(),
                durability: inv.items.iter().map(|i| i.durability).collect(),
                until_time: inv.items.iter().map(|i| i.until).collect(),
                slot_position: inv.items.iter().map(|i| i.slot).collect(),
                add_parts_id: inv.items.iter().map(|i| inv.effective_parts(i).0).collect(),
                damage_degree: vec![0; n],
                item_state: vec![0; n],
                stack_count: inv.items.iter().map(|i| i.stack).collect(),
                paint_id: inv.items.iter().map(|i| inv.effective_parts(i).1).collect(),
                trade_count: vec![0; n],
            }
            .packet(),
            channel::AckItemListEnd.packet(),
        ];
        (packets, !gone.is_empty())
    })
}

/// The player's balance as the client's money widgets expect it: points and
/// cash (`rfAckMyMoney`) plus the cash-shop wallet (`rfAckWebzenCash`).
pub fn balance_packets(name: &str) -> Vec<Packet> {
    let (points, cash) = inventory::read_named(name, |inv| (inv.points, inv.cash));
    vec![
        channel::AckMyMoney { result: 0, err_msg: String::new(), point: points, cash }.packet(),
        channel::AckWebzenCash { result: 0, err_msg: String::new(), p_coin_cash: cash as f64, c_coin_cash: 0.0 }.packet(),
    ]
}

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        channel::ReqItemList::OPCODE => item_list_packets(&ctx.name),

        channel::ReqMyEquipedItemList::OPCODE => vec![loadout::my_equiped_item_list(&ctx.name)],

        channel::ReqEntryItemList::OPCODE => {
            let req = channel::ReqEntryItemList::decode(&pkt.payload)?;
            let who = session::name_of(req.user_id).unwrap_or_else(|| ctx.name.clone());
            vec![loadout::entry_item_list(req.user_id, &who)]
        }

        // Weapon remodel: the client builds the part/paint choice from its own
        // parts table and sends the chosen ids; we store them on the instance and
        // echo them in every item/quick-slot list from then on.
        channel::ReqModifyWeapon::OPCODE => {
            let req = channel::ReqModifyWeapon::decode(&pkt.payload)?;
            tracing::info!(uid = req.unique_id, part = req.add_part_id, paint = req.paint_id, "ReqModifyWeapon");
            let (points, cash) = inventory::read_named(&ctx.name, |inv| (inv.points, inv.cash));
            vec![match inventory::modify(&ctx.name, req.unique_id, req.add_part_id, req.paint_id) {
                Some((point, cash)) => channel::AckModifyWeapon {
                    result: 0,
                    err_msg: String::new(),
                    unique_id: req.unique_id,
                    add_part_id: req.add_part_id,
                    paint_id: req.paint_id,
                    point,
                    cash,
                },
                None => channel::AckModifyWeapon {
                    result: 1,
                    err_msg: "cannot modify".into(),
                    unique_id: req.unique_id,
                    add_part_id: 0,
                    paint_id: 0,
                    point: points,
                    cash,
                },
            }
            .packet()]
        }

        // Skills have their own inventory list, int unique ids and byte slots.
        // Skills are period items in the client's table: it computes a remaining
        // time from UntilTime and shows 0 as "expired", so permanent ones go out
        // with a far-future stamp (`inventory::skill_until_wire`).
        channel::ReqUserSkillList::OPCODE => inventory::with_named(&ctx.name, |inv| {
            let gone = inv.purge_expired_skills();
            if !gone.is_empty() {
                tracing::info!(count = gone.len(), ids = ?gone.iter().map(|s| s.skill_id).collect::<Vec<_>>(), "expired skills removed");
            }
            let sk = &inv.skills;
            let packets = vec![
                channel::AckUserSkillListStart { count: sk.len() as i32, result: 0, err_msg: String::new() }.packet(),
                channel::AckUserSkillList {
                    unique_skill_id: sk.iter().map(|s| s.unique_id).collect(),
                    skill_id: sk.iter().map(|s| s.skill_id).collect(),
                    until_time: sk.iter().map(|s| inventory::skill_until_wire(s.until)).collect(),
                    slot_position: sk
                        .iter()
                        .map(|s| if s.slot < 0 { inventory::SLOT_NONE as u8 } else { s.slot as u8 })
                        .collect(),
                }
                .packet(),
                channel::AckUserSkillListEnd.packet(),
            ];
            (packets, !gone.is_empty())
        }),

        channel::ReqMyEquipedSkillList::OPCODE => inventory::read_named(&ctx.name, |inv| {
            let eq = inv.equipped_skills();
            vec![channel::AckMyEquipedSkillList {
                skill_i_ds: eq.iter().map(|s| s.skill_id).collect(),
                slot_positions: eq.iter().map(|s| s.slot).collect(),
                result: 0,
            }
            .packet()]
        }),

        channel::ReqEntrySkillList::OPCODE => {
            let req = channel::ReqEntrySkillList::decode(&pkt.payload)?;
            let who = session::name_of(req.user_id).unwrap_or_else(|| ctx.name.clone());
            vec![loadout::entry_skill_list(req.user_id, &who)]
        }

        channel::ReqEquipSkill::OPCODE => {
            let req = channel::ReqEquipSkill::decode(&pkt.payload)?;
            tracing::info!(skill = req.skill_id, uid = req.skill_unique_id, slot = req.new_slot_position, "ReqEquipSkill");
            let suid = if req.skill_unique_id != 0 {
                req.skill_unique_id
            } else {
                inventory::read_named(&ctx.name, |inv| {
                    inv.skills.iter().find(|s| s.skill_id == req.skill_id).map(|s| s.unique_id)
                })
                .unwrap_or(0)
            };
            let (ok, old) = inventory::equip_skill(&ctx.name, suid, req.new_slot_position as i32);
            vec![channel::AckEquipSkill {
                changed_new_skill_id: req.skill_id,
                changed_new_skill_unique_id: suid,
                changed_new_slot_position: req.new_slot_position,
                changed_old_skill_id: old.as_ref().map(|o| o.skill_id).unwrap_or(0),
                changed_old_skill_unique_id: old.as_ref().map(|o| o.unique_id).unwrap_or(0),
                changed_old_slot_position: old.as_ref().map(|_| req.new_slot_position).unwrap_or(0),
                result: if ok { 0 } else { 1 },
                err_msg: if ok { String::new() } else { "no such skill".into() },
            }
            .packet()]
        }

        channel::ReqConfirmSkillChanged::OPCODE => {
            vec![channel::AckConfirmSkillChanged { result: 0, err_msg: String::new() }.packet()]
        }

        channel::ReqQSlotList::OPCODE => inventory::read_named(&ctx.name, |inv| {
            let q = &inv.qslots;
            vec![channel::AckQSlotList {
                result: 0,
                err_msg: String::new(),
                q_slot_id: q.iter().map(|s| s.id).collect(),
                q_slot_name: q.iter().map(|s| s.name.clone()).collect(),
                q_slot_disp_order: (1..=q.len() as u8).collect(),
                mw_item_id: q.iter().map(|s| s.main).collect(),
                pistol_item_id: q.iter().map(|s| s.pistol).collect(),
                tw_item_id1: q.iter().map(|s| s.throw1).collect(),
                tw_item_id2: q.iter().map(|s| s.throw2).collect(),
                skill1: q.iter().map(|s| s.skills[0]).collect(),
                skill2: q.iter().map(|s| s.skills[1]).collect(),
                skill3: q.iter().map(|s| s.skills[2]).collect(),
                skill4: q.iter().map(|s| s.skills[3]).collect(),
            }
            .packet()]
        }),

        channel::ReqMyEquipedQSlotList::OPCODE => vec![loadout::my_equiped_qslot_list(&ctx.name)],

        channel::ReqEntryQSlotIDList::OPCODE => {
            let req = channel::ReqEntryQSlotIDList::decode(&pkt.payload)?;
            let who = session::name_of(req.user_num).unwrap_or_else(|| ctx.name.clone());
            vec![loadout::entry_qslot_id_list(req.user_num, &who)]
        }

        channel::ReqEntryQSlotUniqueIDList::OPCODE => {
            let req = channel::ReqEntryQSlotUniqueIDList::decode(&pkt.payload)?;
            let who = session::name_of(req.user_num).unwrap_or_else(|| ctx.name.clone());
            vec![loadout::entry_qslot_unique_list(req.user_num, &who)]
        }

        channel::ReqUpdateQSlot::OPCODE => {
            let req = channel::ReqUpdateQSlot::decode(&pkt.payload)?;
            tracing::info!(slot = req.q_slot_id, name = %req.q_slot_name, main = req.mw_item_id, pistol = req.pistol_item_id, "ReqUpdateQSlot");
            inventory::with_named(&ctx.name, |inv| {
                let slot = inventory::QuickSlot {
                    id: req.q_slot_id,
                    name: req.q_slot_name.clone(),
                    main: req.mw_item_id,
                    pistol: req.pistol_item_id,
                    throw1: req.tw_item_id1,
                    throw2: req.tw_item_id2,
                    skills: [req.skill1, req.skill2, req.skill3, req.skill4],
                };
                match inv.qslots.iter_mut().find(|s| s.id == req.q_slot_id) {
                    Some(s) => *s = slot,
                    None => inv.qslots.push(slot),
                }
                ((), true)
            });
            loadout::push_loadout_change(ctx);
            vec![channel::AckUpdateQSlot { result: 0, err_msg: String::new() }.packet()]
        }

        // Equip an owned instance. On entry the client also asks for its default
        // weapons by item id with UniqueID 0 (G3 2000 -> slot 0, Glock 1010 -> slot
        // 1); resolve those to whichever instance of that item we hold.
        channel::ReqEquipItem::OPCODE => {
            let req = channel::ReqEquipItem::decode(&pkt.payload)?;
            tracing::info!(item = req.item_id, uid = req.unique_id, slot = req.slot_position, "ReqEquipItem");
            let iuid = if req.unique_id != 0.0 {
                req.unique_id
            } else {
                inventory::read_named(&ctx.name, |inv| {
                    inv.items.iter().find(|i| i.item_id == req.item_id).map(|i| i.unique_id)
                })
                .unwrap_or(0.0)
            };
            let old = if iuid != 0.0 { inventory::equip(&ctx.name, iuid, req.slot_position) } else { None };
            let ok = iuid != 0.0;
            let out = vec![channel::AckEquipItem {
                changed_new_item_id: req.item_id,
                changed_new_unique_id: iuid,
                changed_new_slot_position: req.slot_position,
                changed_old_item_id: old.as_ref().map(|o| o.item_id).unwrap_or(0),
                changed_old_unique_id: old.as_ref().map(|o| o.unique_id).unwrap_or(0.0),
                changed_old_slot_position: old.as_ref().map(|_| req.slot_position).unwrap_or(0),
                result: if ok { 0 } else { 1 },
                err_msg: if ok { String::new() } else { "no such item".into() },
            }
            .packet()];
            loadout::push_loadout_change(ctx);
            out
        }

        channel::ReqConfirmItemChanged::OPCODE => {
            vec![channel::AckConfirmItemChanged { result: 0, err_msg: String::new() }.packet()]
        }

        // Repair quote for the inventory's repair window: one point per 100 raw
        // durability missing (a fully worn weapon costs 1000).
        channel::ReqRepairCost::OPCODE => {
            let req = channel::ReqRepairCost::decode(&pkt.payload)?;
            let full = inventory::full_durability();
            let costs = inventory::read_named(&ctx.name, |inv| {
                req.unique_item_id
                    .iter()
                    .map(|&u| inv.find(u).map(|i| ((full - i.durability).max(0) / 100).max(0)).unwrap_or(0))
                    .collect::<Vec<i32>>()
            });
            vec![channel::AckRepairCost {
                result: 0,
                err_msg: String::new(),
                unique_item_id: req.unique_item_id,
                repair_cost: costs,
            }
            .packet()]
        }

        // Asked after a match and on page changes: report (and drop) rentals whose
        // time is up so the client can show its "expired items" window.
        channel::ReqNotifyExpiredItemIDs::OPCODE => inventory::with_named(&ctx.name, |inv| {
            let gone = inv.purge_expired();
            if !gone.is_empty() {
                tracing::info!(count = gone.len(), ids = ?gone.iter().map(|i| i.item_id).collect::<Vec<_>>(), "expired rentals reported");
            }
            let packets = vec![channel::AckNotifyExpiredItemIDs {
                user_id: uid,
                expired_item_unique_i_ds: gone.iter().map(|i| i.unique_id).collect(),
                expired_item_i_ds: gone.iter().map(|i| i.item_id).collect(),
                expired_item_part_i_ds: gone.iter().map(|i| i.add_part_id).collect(),
                expired_item_slot_pos: gone.iter().map(|i| if i.slot < 0 { inventory::SLOT_NONE as u8 } else { i.slot as u8 }).collect(),
                result: 0,
                err_msg: String::new(),
            }
            .packet()];
            (packets, !gone.is_empty())
        }),

        channel::ReqNotifyExpiredSkillIDs::OPCODE => inventory::with_named(&ctx.name, |inv| {
            let gone = inv.purge_expired_skills();
            if !gone.is_empty() {
                tracing::info!(count = gone.len(), ids = ?gone.iter().map(|s| s.skill_id).collect::<Vec<_>>(), "expired skills reported");
            }
            let packets = vec![channel::AckNotifyExpiredSkillIDs {
                user_id: uid,
                expired_skill_unique_i_ds: gone.iter().map(|s| s.unique_id as f64).collect(),
                expired_skill_i_ds: gone.iter().map(|s| s.skill_id).collect(),
                result: 0,
                err_msg: String::new(),
                expired_skill_slot_positions: gone.iter().map(|s| if s.slot < 0 { inventory::SLOT_NONE as u8 } else { s.slot as u8 }).collect(),
            }
            .packet()];
            (packets, !gone.is_empty())
        }),

        _ => return Ok(None),
    }))
}
