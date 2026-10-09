//! The mailbox ("memo" in the protocol): send, list, read, delete, take an
//! attachment. Rows live in the database (`acemu_db::social`), keyed by the
//! character that owns the tab; a sent message is stored twice, once in the
//! recipient's inbox and once in the sender's Sent tab.
//!
//! Client behaviour (`BTPageMail` / `BTTPMailHK`): tab 0 is the
//! inbox, 1 the Sent tab, 2 the Webzen gift box (handled elsewhere);
//! `MemoType` in the list is 1 text, 2 item attached, 4 points; `Status` 0 is
//! unread; `dateRecv` must be `YYYY-MM-DD` (it is re-split on `-`);
//! `UntilExpire` is in hours. Sending answers with the names that failed and
//! the client's own codes 100351 (no such player) / 100352 (mailbox full); an
//! empty failure list means success. An online recipient gets
//! `rfReqRecvMemo` pushed so its lobby shows "Message received".

use super::Reply;
use crate::inventory;
use crate::session::{self, Ctx};
use acemu_db::social::{self as db_social, MailAttachment};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    Ok(Some(match pkt.opcode {
        channel::ReqGetMyMailBoxState::OPCODE => {
            let (unread, total) = inventory::db().mail_state(&ctx.name).unwrap_or((0, 0));
            vec![channel::AckGetMyMailBoxState {
                result: 0,
                is_full: (total >= db_social::MAILBOX_CAPACITY) as u8,
                new_mail_count: unread.min(u16::MAX as i64) as u16,
            }
            .packet()]
        }

        channel::ReqMemoList::OPCODE => {
            let req = channel::ReqMemoList::decode(&pkt.payload)?;
            let rows = inventory::db().mail_list(&ctx.name, req.type_ as i32).unwrap_or_default();
            tracing::info!(tab = req.type_, kind = req.memo_type, count = rows.len(), "ReqMemoList");
            let mut out = vec![channel::AckStartMemoList { result: 0, err_msg: String::new(), type_: req.type_, total_count: rows.len() as i32 }.packet()];
            // one row's text can be long; keep each array well under the client's buffer
            for chunk in rows.chunks(20) {
                out.push(
                    channel::AckMemoList {
                        memo_idx: chunk.iter().map(|m| m.id as i32).collect(),
                        cname_from: chunk.iter().map(|m| if m.r#box == db_social::BOX_SENT { m.recipient.clone() } else { m.sender.clone() }).collect(),
                        date_recv: chunk.iter().map(|m| m.sent_date.clone()).collect(),
                        text: chunk.iter().map(|m| m.text.clone()).collect(),
                        status: chunk.iter().map(|m| m.status as u8).collect(),
                        until_expire: chunk.iter().map(|m| m.hours_left).collect(),
                        item_uid: chunk.iter().map(|m| m.item_uid as f64).collect(),
                        item_id: chunk.iter().map(|m| if m.msg_type == db_social::MAIL_POINTS { m.points } else { m.item_id }).collect(),
                        msg_type: chunk.iter().map(|m| m.msg_type as u8).collect(),
                        item_part_id: chunk.iter().map(|m| if m.msg_type == db_social::MAIL_ITEM { m.item_count } else { m.item_part }).collect(),
                    }
                    .packet(),
                );
            }
            out.push(channel::AckEndMemoList.packet());
            out
        }

        channel::ReqSendMemo::OPCODE => {
            let req = channel::ReqSendMemo::decode(&pkt.payload)?;
            tracing::info!(to = ?req.cnames, item_uid = req.item_uid, len = req.msg.len(), "ReqSendMemo");
            // An attached item is moved out of the sender's bag first; the
            // recipient takes it from the message.
            let attachment = if req.item_uid != 0 {
                inventory::take_item(&ctx.name, req.item_uid as f64)
                    .map(|it| MailAttachment { item_uid: it.unique_id as i64, item_id: it.item_id, item_part: it.add_part_id, item_count: it.stack, points: 0 })
            } else {
                None
            };
            let recipients: Vec<String> = req.cnames.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
            let failed = inventory::db().mail_send(&ctx.name, &recipients, &req.msg, attachment.as_ref()).unwrap_or_default();
            // If nobody could receive an attached item, give it back.
            if let Some(a) = &attachment {
                if failed.len() == recipients.len() {
                    inventory::grant(&ctx.name, a.item_id, a.item_count.max(1));
                }
            }
            let delivered: Vec<&String> = recipients.iter().filter(|r| !failed.iter().any(|(f, _)| f == *r)).collect();
            for to in delivered {
                if let Some((uid, _)) = session::snapshot().into_iter().map(|(u, n, _, _)| (u, n)).find(|(_, n)| n == to) {
                    let kind = attachment.as_ref().map(|_| db_social::MAIL_ITEM).unwrap_or(db_social::MAIL_TEXT) as u8;
                    session::send_to(uid, &[channel::ReqRecvMemo { cname_from: ctx.name.clone(), memo_type: kind }.packet()]);
                }
            }
            vec![channel::AckSendMemo {
                cnames_to_fail: failed.iter().map(|(n, _)| n.clone()).collect(),
                result_array: failed.iter().map(|(_, c)| *c).collect(),
            }
            .packet()]
        }

        channel::ReqReadMemo::OPCODE => {
            let req = channel::ReqReadMemo::decode(&pkt.payload)?;
            let row = inventory::db().mail_read(&ctx.name, req.memo_idx as i64).ok().flatten();
            vec![match row {
                Some(m) => channel::AckReadMemo { cname_from: m.sender, text: m.text, date_recv: m.sent_date, item_uid: m.item_uid as f64 }.packet(),
                None => channel::AckReadMemo::default().packet(),
            }]
        }

        channel::ReqDelMemo::OPCODE => {
            let req = channel::ReqDelMemo::decode(&pkt.payload)?;
            let ids: Vec<i64> = req.memo_idx.iter().map(|&i| i as i64).collect();
            let failed = inventory::db().mail_delete(&ctx.name, req.type_ as i32, &ids).unwrap_or_default();
            vec![channel::AckDelMemo { result: 0, err_msg: String::new(), failed_memo_id: failed.iter().map(|&i| i as i32).collect() }.packet()]
        }

        channel::ReqDeleteMailAll::OPCODE => {
            let req = channel::ReqDeleteMailAll::decode(&pkt.payload)?;
            let n = inventory::db().mail_delete_all(&ctx.name, req.mail_type as i32).unwrap_or(0);
            vec![channel::AckDeleteMailAll { result: 0, mail_type: req.mail_type, count: n as i32 }.packet()]
        }

        // Taking an attachment: the item (or points) moves into the inventory
        // and the message becomes plain text. The client then refreshes its
        // item list and balance itself.
        channel::ReqGetAttachedItemFromMemo::OPCODE => {
            let req = channel::ReqGetAttachedItemFromMemo::decode(&pkt.payload)?;
            let taken = inventory::db().mail_take_attachment(&ctx.name, req.memo_idx as i64).ok().flatten();
            tracing::info!(memo = req.memo_idx, item = req.item_id, taken = taken.is_some(), "ReqGetAttachedItemFromMemo");
            vec![match taken {
                Some(a) if a.item_id > 0 => {
                    let uid = inventory::grant(&ctx.name, a.item_id, a.item_count.max(1)).unwrap_or(0.0);
                    channel::AckGetAttachedItemFromMemo { result: 0, err_msg: String::new(), item_idx: uid, item_id: a.item_id }.packet()
                }
                Some(a) => {
                    inventory::add_points(&ctx.name, a.points, 0);
                    channel::AckGetAttachedItemFromMemo { result: 0, err_msg: String::new(), item_idx: 0.0, item_id: 0 }.packet()
                }
                None => channel::AckGetAttachedItemFromMemo { result: 1, err_msg: "nothing attached".into(), item_idx: 0.0, item_id: req.item_id }.packet(),
            }]
        }

        _ => return Ok(None),
    }))
}
