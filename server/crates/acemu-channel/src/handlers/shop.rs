//! The item shop: catalogue, cash balance, buying and selling, the luck shop
//! stubs and random ("supply") boxes.

use super::Reply;
use crate::inventory;
use crate::session::Ctx;
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqWebzenCash::OPCODE => inventory::read_named(&ctx.name, |inv| {
            vec![channel::AckWebzenCash {
                result: 0,
                err_msg: String::new(),
                p_coin_cash: inv.cash as f64,
                c_coin_cash: 0.0,
            }
            .packet()]
        }),

        // Cash-shop ("GP20") product feed. The client asks with the version it
        // has cached (0 on a fresh client). Order matters, per the client's own
        // page code (BTPageServerSelection): the ack CLEARS its product list, then
        // each rfReqNotifyShopItemData row is merged into a product keyed by
        // ProductCode (= item id; several rows with the same code become that
        // product's price options), then the end marker stores the version and
        // refreshes the shop/depot pages. A row is buyable only with SaleStatus
        // 10 or 11 and a non-expired SaleEndTime (0 = never). SaleType 2 = W
        // Coin(P), 1 = W Coin(C), else points. Chunked: the name array alone
        // would otherwise near the client's fixed receive buffer.
        // Coupons / item boxes ("Weapon Coupon I"): the client asks for the pick
        // list, shows a chooser, then redeems one pick.
        channel::ReqItemListInItemBox::OPCODE => {
            let req = channel::ReqItemListInItemBox::decode(&pkt.payload)?;
            let choices = inventory::item_box_choices(req.box_item_id);
            tracing::info!(box = req.box_item_id, uid = req.item_unique_id, choices = choices.len(), "ReqItemListInItemBox");
            vec![channel::AckItemListInItemBox {
                result: if choices.is_empty() { 1 } else { 0 },
                err_msg: if choices.is_empty() { "no item list".into() } else { String::new() },
                item_unique_id: req.item_unique_id,
                box_item_id: req.box_item_id,
                item_id: choices.iter().map(|c| c.0).collect(),
                part_id: choices.iter().map(|c| c.1).collect(),
                from_content: req.from_content,
            }
            .packet()]
        }

        channel::ReqGetItemFromItemBox::OPCODE => {
            let req = channel::ReqGetItemFromItemBox::decode(&pkt.payload)?;
            tracing::info!(box = req.item_id, uid = req.item_unique_id, choose = req.item_id_choose, part = req.part_id_choose, "ReqGetItemFromItemBox");
            vec![match inventory::take_from_box(&ctx.name, req.item_unique_id, req.item_id_choose) {
                Some((new_uid, remaining)) => channel::AckGetItemFromItemBox {
                    result: 0,
                    err_msg: String::new(),
                    item_id_choose: req.item_id_choose,
                    part_id_choose: req.part_id_choose,
                    box_item_idx: req.item_unique_id,
                    bxo_item_count: remaining,
                    b_have: 0,
                    item_idx: new_uid,
                    until_time: 0.0,
                    stack_count: 1,
                },
                None => channel::AckGetItemFromItemBox {
                    result: 1,
                    err_msg: "cannot redeem".into(),
                    item_id_choose: req.item_id_choose,
                    part_id_choose: req.part_id_choose,
                    box_item_idx: req.item_unique_id,
                    ..Default::default()
                },
            }
            .packet()]
        }

        // Balance refresh the client asks for after the tutorial and on some
        // page changes.
        channel::ReqMyMoney::OPCODE => {
            let (points, cash) = inventory::read_named(&ctx.name, |inv| (inv.points, inv.cash));
            vec![channel::AckMyMoney { result: 0, err_msg: String::new(), point: points, cash }.packet()]
        }

        channel::ReqShopItemData::OPCODE => {
            const CHUNK: usize = 25;
            const FEED_VERSION: i32 = 1;
            const SALE_STATUS_ON_SALE: i32 = 10;
            let req = channel::ReqShopItemData::decode(&pkt.payload)?;
            let products = inventory::products();
            tracing::info!(ver = req.ver, products = products.len(), "ReqShopItemData");
            let mut out = vec![channel::AckShopItemData.packet()];
            for chunk in products.chunks(CHUNK) {
                let n = chunk.len();
                out.push(
                    channel::ReqNotifyShopItemData {
                        product_no: chunk.iter().map(|p| p.no).collect(),
                        product_name: chunk.iter().map(|p| p.name.clone()).collect(),
                        product_code: chunk.iter().map(|p| p.id).collect(),
                        sale_type: chunk.iter().map(|p| p.sale_type).collect(),
                        product_cost: chunk.iter().map(|p| p.cost).collect(),
                        product_hour: chunk.iter().map(|p| p.hours).collect(),
                        product_quantity: vec![1; n],
                        sale_master_flag: vec![0; n],
                        sale_status: vec![SALE_STATUS_ON_SALE; n],
                        is_gift: vec![0; n],
                        sale_end_time: vec![0.0; n],
                        discountcost: vec![0; n],
                    }
                    .packet(),
                );
            }
            out.push(channel::ReqNotifyShopItemDataEnd { ver: FEED_VERSION }.packet());
            out
        }

        // Item shop: every id in the client's own shop table (weapons, gear, skills,
        // consumables). The client renders names, prices and stats from that table
        // and sorts the ids into its tabs; the ids are all it needs. The list is
        // sent in chunks because the client copies each array into a fixed stack
        // buffer (see PROTOCOL.md, "arrays").
        channel::ReqShopItemList::OPCODE => {
            const CHUNK: usize = 200;
            // Items sold through the product feed are left out: the Fortune
            // Depot lists every capsule-type id from this list at its (zero)
            // point price, and would show the box twice.
            let ids = inventory::shop_ids_for_points();
            let mut out = vec![channel::AckShopItemListStart { count: ids.len() as i32 }.packet()];
            for chunk in ids.chunks(CHUNK) {
                out.push(channel::AckShopItemList { item_id: chunk.to_vec() }.packet());
            }
            out.push(channel::AckShopItemListEnd.packet());
            out
        }

        channel::ReqBuyShopItem::OPCODE => {
            let req = channel::ReqBuyShopItem::decode(&pkt.payload)?;
            tracing::info!(item = req.item_id, opt1 = req.buy_option1, opt2 = req.buy_option2, "ReqBuyShopItem");
            // BuyOption2 (client's BTWindowItemBuyHK / BuyTerm / ItemExpired
            // windows): 0 new purchase, 1 extend the owned period item, 2 re-buy
            // an expired one. For 1 and 2 the owned instance is extended in place
            // and answered with its own unique id; the client's ack handler
            // updates an existing instance rather than adding one.
            let mode = if req.buy_option2 == 0 { inventory::BuyMode::New } else { inventory::BuyMode::Extend };
            let bought = inventory::buy_with(&ctx.name, req.item_id, req.buy_option1, mode);
            if bought.is_ok() && inventory::is_capsule(req.item_id) {
                let _ = inventory::db().extra_capsule_bought(&ctx.name, 1);
            }
            let (points, cash) = inventory::read_named(&ctx.name, |inv| (inv.points, inv.cash));
            if let Err(e) = bought {
                tracing::warn!(item = req.item_id, error = ?e, "purchase refused");
            }
            // Items report their expiry as stored (0 = permanent); skills are
            // period items to the client and need the wire form (see
            // inventory::PERMANENT_UNTIL).
            let until_of = |iuid: f64| {
                inventory::read_named(&ctx.name, |inv| {
                    inv.find(iuid).map(|i| i.until).or_else(|| {
                        inv.skills.iter().find(|s| s.unique_id as f64 == iuid).map(|s| inventory::skill_until_wire(s.until))
                    })
                })
                .unwrap_or(0.0)
            };
            vec![match bought {
                Ok(iuid) => channel::AckBuyShopItem {
                    item_count: 1,
                    result: vec![0],
                    bought_item_type_id: vec![req.item_id],
                    bought_item_unique_id: vec![iuid],
                    bought_part_id: vec![inventory::default_part_id(req.item_id)],
                    bought_until_t_ime: vec![until_of(iuid)],
                    bought_stack_cnt: vec![1],
                    point_remain: points,
                    cash_remain: cash,
                },
                Err(_) => channel::AckBuyShopItem {
                    item_count: 0,
                    result: vec![1],
                    point_remain: points,
                    cash_remain: cash,
                    ..Default::default()
                },
            }
            .packet()]
        }

        // Cash-shop purchase of a product-feed row (Fortune Depot boxes, packages,
        // rented costumes). The client sends the ProductNo we announced and the
        // coin type it chose; on Result 0 it re-requests its cash balance and
        // item list itself, so the item just has to be in the inventory by then.
        channel::ReqBuyItemByWebzenCashGP20::OPCODE => {
            let req = channel::ReqBuyItemByWebzenCashGP20::decode(&pkt.payload)?;
            tracing::info!(product_no = req.product_no, cointype = req.cointype, item = req.item_id, "ReqBuyItemByWebzenCashGP20");
            // The cash path carries no purchase mode: a rental product bought
            // while an instance is owned is the client's "extend term" flow.
            let bought = match inventory::product_by_no(req.product_no) {
                Some(p) => match inventory::buy_with(
                    &ctx.name,
                    p.id,
                    1,
                    if p.hours > 1 { inventory::BuyMode::Extend } else { inventory::BuyMode::New },
                ) {
                    Ok(uid) => {
                        // Fortune Depot capsules count towards the buying-count rewards.
                        if inventory::is_capsule(p.id) {
                            let _ = inventory::db().extra_capsule_bought(&ctx.name, 1);
                        }
                        Some((p.id, uid))
                    }
                    Err(e) => {
                        tracing::warn!(product = p.id, error = ?e, "cash purchase refused");
                        None
                    }
                },
                None => {
                    tracing::warn!(product_no = req.product_no, "unknown product");
                    None
                }
            };
            let (points, cash) = inventory::read_named(&ctx.name, |inv| (inv.points, inv.cash));
            match bought {
                // Success is reported as a *point* purchase on purpose. The client
                // dispatches rfAckBuyItemByWebzenCash to its page stack and only the
                // Item Shop page handles it; from the Fortune Depot (or before the
                // shop was ever opened) the server-selection page swallows it, the
                // buy window's 10 s timer fires and the client shows "Unable to buy
                // the cash item" (define 182) although the purchase went through.
                // rfAckBuyShopItem is handled by every page: it adds the instance,
                // stops that timer and shows the success note. The cash balance is
                // refreshed with an unsolicited rfAckWebzenCash.
                Some((item_id, uid)) => vec![
                    channel::AckBuyShopItem {
                        item_count: 1,
                        result: vec![0],
                        bought_item_type_id: vec![item_id],
                        bought_item_unique_id: vec![uid],
                        bought_part_id: vec![inventory::default_part_id(item_id)],
                        bought_until_t_ime: vec![inventory::read_named(&ctx.name, |inv| inv.find(uid).map(|i| i.until).unwrap_or(0.0))],
                        bought_stack_cnt: vec![1],
                        point_remain: points,
                        cash_remain: cash,
                    }
                    .packet(),
                    channel::AckWebzenCash {
                        result: 0,
                        err_msg: String::new(),
                        p_coin_cash: cash as f64,
                        c_coin_cash: 0.0,
                    }
                    .packet(),
                ],
                None => vec![channel::AckBuyItemByWebzenCash { result: 1, err_msg: "purchase failed".into() }.packet()],
            }
        }

        channel::ReqSellShopItemInfo::OPCODE => {
            let req = channel::ReqSellShopItemInfo::decode(&pkt.payload)?;
            let points = inventory::read_named(&ctx.name, |inv| inv.points);
            let refund = inventory::resale_value(&ctx.name, req.unique_item_id);
            vec![channel::AckSellShopItemInfo {
                result: 0,
                err_msg: String::new(),
                unique_item_id: req.unique_item_id,
                sell_point: refund,
                cur_point: points,
                after_point: points + refund,
            }
            .packet()]
        }

        channel::ReqSellShopItem::OPCODE => {
            let req = channel::ReqSellShopItem::decode(&pkt.payload)?;
            // `Count` in the ack is the stack that is LEFT: 0 makes the client drop
            // the instance, anything else becomes its displayed stack count.
            let sold = inventory::sell(&ctx.name, req.item_unique_id, req.count);
            let (points, cash) = inventory::read_named(&ctx.name, |inv| (inv.points, inv.cash));
            vec![channel::AckSellShopItem {
                result: if sold.is_some() { 0 } else { 1 },
                err_msg: if sold.is_some() { String::new() } else { "no such item".into() },
                point_remain: points,
                cash_remain: cash,
                count: sold.map(|(_, remaining)| remaining).unwrap_or(0),
            }
            .packet()]
        }

        channel::ReqLuckShopStorageInfo::OPCODE => {
            vec![channel::AckLuckShopStorageInfo::default().packet()]
        }

        channel::ReqLuckShopLotteryWinnerList::OPCODE => {
            let req = channel::ReqLuckShopLotteryWinnerList::decode(&pkt.payload)?;
            vec![channel::AckLuckShopLotteryWinnerList {
                capsule_item_id: req.capsule_item_id,
                ..Default::default()
            }
            .packet()]
        }

        // Random boxes ("supply boxes"): consume one, hand out one item from the
        // box's table. The client shows "loot could not be acquired" when this
        // goes unanswered.
        channel::ReqRandomBoxUse::OPCODE => {
            let req = channel::ReqRandomBoxUse::decode(&pkt.payload)?;
            tracing::info!(box_uid = req.item_idx_random_box, key_uid = req.item_idx_key, "ReqRandomBox_Use");
            vec![match inventory::open_box(&ctx.name, req.item_idx_random_box) {
                Some(r) => {
                    // one entry per reel: the weapon, then the bonus (Gold Bar /
                    // ticket) when the box has a second reel
                    let mut ids = vec![r.item_id];
                    let mut parts = vec![inventory::default_part_id(r.item_id)];
                    let mut uids = vec![r.unique_id];
                    let mut periods = vec![r.hours];
                    let mut untils = vec![r.until];
                    for b in &r.bonus {
                        ids.push(b.item_id);
                        parts.push(0);
                        uids.push(b.unique_id);
                        periods.push(0);
                        untils.push(0.0);
                    }
                    let n = ids.len();
                    channel::AckRandomBoxUse {
                        result: 0,
                        err_msg: String::new(),
                        random_box_item_idx: req.item_idx_random_box,
                        random_bxo_item_count: r.box_remaining,
                        key_item_idx: req.item_idx_key,
                        key_item_remain_count: 0,
                        item_id_choose: ids,
                        part_id_choose: parts,
                        period_on_choosen_item: periods,
                        stack_count_on_choosen_item: vec![1; n],
                        b_have: vec![0; n],
                        item_idx: uids,
                        until_time: untils,
                        stack_count: vec![1; n],
                    }
                }
                None => channel::AckRandomBoxUse {
                    result: 1,
                    err_msg: "no such box".into(),
                    random_box_item_idx: req.item_idx_random_box,
                    key_item_idx: req.item_idx_key,
                    ..Default::default()
                },
            }
            .packet()]
        }

        _ => return Ok(None),
    }))
}
