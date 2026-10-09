//! Packets that describe a player's loadout - equipped items, skills and quick
//! slots - built from the inventory.
//!
//! The "My" lists answer the player's own requests. The "Entry" lists describe
//! *another* room member to a client, keyed by that member's uid
//! (`UserNum`/`UserIDs`): the room-lobby page stores them in `UserInfos[]`,
//! which the host reads when it spawns that player's pawn.

use crate::inventory;
use crate::room;
use crate::session::{self, Ctx};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

/// `rfAckMyEquipedItemList`. Only requested by the client's tutorial flow
/// (`BTTcpHandler.BeginTutorial`): the handler stores the items in the room's
/// `UserInfos[].EquippedItems`, requests `MyEquipedSkillList`, whose handler
/// requests `MyEquipedQSlotList`, whose handler ends in `BeginTutorialCreateRoom`.
/// Answer it when asked; never push it (it switches the client to tutorial mode).
pub fn my_equiped_item_list(name: &str) -> Packet {
    inventory::read_named(name, |inv| {
        let eq = inv.equipped();
        channel::AckMyEquipedItemList {
            item_i_ds: eq.iter().map(|i| i.item_id).collect(),
            slot_positions: eq.iter().map(|i| i.slot).collect(),
            add_parts_id: eq.iter().map(|i| inv.effective_parts(i).0).collect(),
            paint_id: eq.iter().map(|i| inv.effective_parts(i).1).collect(),
            unique_id: eq.iter().map(|i| i.unique_id).collect(),
            result: 0,
        }
        .packet()
    })
}

/// `rfAckMyEquipedQSlotList`. Its handler writes `UserInfos[].EquippedQSlots`,
/// which is what `wMatchUserInfo.ResetQSlotItemUse` reads at spawn - i.e. the
/// weapons the pawn actually gets. `QSlotDispOrder` is 1-based (see PROTOCOL.md).
pub fn my_equiped_qslot_list(name: &str) -> Packet {
    inventory::read_named(name, |inv| {
        let q = &inv.qslots;
        let id_of = |uid: f64| inv.find(uid).map(|i| i.item_id).unwrap_or(0);
        channel::AckMyEquipedQSlotList {
            q_slot_disp_order: (1..=q.len() as u8).collect(),
            mw_unique_id: q.iter().map(|s| s.main).collect(),
            mw_item_id: q.iter().map(|s| id_of(s.main)).collect(),
            mw_part_id: q.iter().map(|s| inv.parts_of(s.main).0).collect(),
            mw_paint_id: q.iter().map(|s| inv.parts_of(s.main).1).collect(),
            pistol_unique_id: q.iter().map(|s| s.pistol).collect(),
            pistol_item_id: q.iter().map(|s| id_of(s.pistol)).collect(),
            pistol_part_id: q.iter().map(|s| inv.parts_of(s.pistol).0).collect(),
            pistol_paint_id: q.iter().map(|s| inv.parts_of(s.pistol).1).collect(),
            tw1unique_id: q.iter().map(|s| s.throw1).collect(),
            tw_item_id1: q.iter().map(|s| id_of(s.throw1)).collect(),
            tw2unique_id: q.iter().map(|s| s.throw2).collect(),
            tw_item_id2: q.iter().map(|s| id_of(s.throw2)).collect(),
            skill1id: q.iter().map(|s| s.skills[0]).collect(),
            skill2id: q.iter().map(|s| s.skills[1]).collect(),
            skill3id: q.iter().map(|s| s.skills[2]).collect(),
            skill4id: q.iter().map(|s| s.skills[3]).collect(),
            result: 0,
        }
        .packet()
    })
}

/// `rfAckEntryItemList`: member `uid`'s equipped items.
pub fn entry_item_list(uid: i32, name: &str) -> Packet {
    inventory::read_named(name, |inv| {
        let eq = inv.equipped();
        channel::AckEntryItemList {
            user_i_ds: vec![uid; eq.len()],
            item_i_ds: eq.iter().map(|i| i.item_id).collect(),
            slot_positions: eq.iter().map(|i| i.slot).collect(),
            add_parts_id: eq.iter().map(|i| inv.effective_parts(i).0).collect(),
            unique_id: eq.iter().map(|i| i.unique_id).collect(),
            paint_id: eq.iter().map(|i| inv.effective_parts(i).1).collect(),
            result: 0,
            err_msg: String::new(),
        }
        .packet()
    })
}

/// `rfAckEntrySkillList`: member `uid`'s equipped skills.
pub fn entry_skill_list(uid: i32, name: &str) -> Packet {
    inventory::read_named(name, |inv| {
        let sk = inv.equipped_skills();
        channel::AckEntrySkillList {
            user_i_ds: vec![uid; sk.len()],
            skill_i_ds: sk.iter().map(|s| s.skill_id).collect(),
            slot_positions: sk.iter().map(|s| s.slot).collect(),
            result: 0,
            err_msg: String::new(),
        }
        .packet()
    })
}

/// `rfAckEntryQSlotIDList`: member `uid`'s quick slots by item id.
pub fn entry_qslot_id_list(uid: i32, name: &str) -> Packet {
    inventory::read_named(name, |inv| {
        let q = &inv.qslots;
        let id_of = |u: f64| inv.find(u).map(|i| i.item_id).unwrap_or(0);
        channel::AckEntryQSlotIDList {
            user_num: uid,
            q_slot_disp_order: (1..=q.len() as u8).collect(),
            mw_item_id: q.iter().map(|s| id_of(s.main)).collect(),
            mw_part_id: q.iter().map(|s| inv.parts_of(s.main).0).collect(),
            mw_paint_id: q.iter().map(|s| inv.parts_of(s.main).1).collect(),
            pistol_item_id: q.iter().map(|s| id_of(s.pistol)).collect(),
            pistol_part_id: q.iter().map(|s| inv.parts_of(s.pistol).0).collect(),
            pistol_paint_id: q.iter().map(|s| inv.parts_of(s.pistol).1).collect(),
            tw_item_id1: q.iter().map(|s| id_of(s.throw1)).collect(),
            tw_item_id2: q.iter().map(|s| id_of(s.throw2)).collect(),
            skill1id: q.iter().map(|s| s.skills[0]).collect(),
            skill2id: q.iter().map(|s| s.skills[1]).collect(),
            skill3id: q.iter().map(|s| s.skills[2]).collect(),
            skill4id: q.iter().map(|s| s.skills[3]).collect(),
        }
        .packet()
    })
}

/// `rfAckEntryQSlotUniqueIDList`: member `uid`'s quick slots by instance id.
pub fn entry_qslot_unique_list(uid: i32, name: &str) -> Packet {
    inventory::read_named(name, |inv| {
        let q = &inv.qslots;
        channel::AckEntryQSlotUniqueIDList {
            user_num: uid,
            q_slot_disp_order: (1..=q.len() as u8).collect(),
            mw_unique_id: q.iter().map(|s| s.main).collect(),
            pistol_unique_id: q.iter().map(|s| s.pistol).collect(),
            tw1unique_id: q.iter().map(|s| s.throw1).collect(),
            tw2unique_id: q.iter().map(|s| s.throw2).collect(),
        }
        .packet()
    })
}

/// All four Entry lists for one member, in the order the room page expects.
pub fn entry_lists_for(uid: i32, name: &str) -> Vec<Packet> {
    vec![
        entry_item_list(uid, name),
        entry_skill_list(uid, name),
        entry_qslot_id_list(uid, name),
        entry_qslot_unique_list(uid, name),
    ]
}

/// After a loadout change in a room: every member (the player included) gets the
/// player's Entry lists again, so the room's per-user info stays current.
///
/// Never push `rfAckMyEquipedItemList` for this: that chain ends in
/// `BTTcpHandler.BeginTutorialCreateRoom`, which flips the client into tutorial
/// mode, and in tutorial mode the client fakes the server for room traffic (Ready
/// sends nothing, a guest's start handler logs "Start GameRoom Without Me").
pub fn push_loadout_change(ctx: &Ctx) {
    if room::contains(ctx.uid) {
        session::send_many(&room::uids_of(ctx.uid), &entry_lists_for(ctx.uid, &ctx.name));
    }
}
