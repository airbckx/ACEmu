//! Opcode handlers, one module per area of the protocol. Each module answers
//! the opcodes it owns and returns `None` for the rest; [`dispatch`] tries
//! them in order and logs whatever nobody claimed, with its payload, so the
//! next handler can be written from real traffic.

pub mod daily;
pub mod friends;
pub mod items;
pub mod lobby;
pub mod login;
pub mod mail;
pub mod matchlog;
pub mod netgame;
pub mod quests;
pub mod ranking;
pub mod results;
pub mod room;
pub mod shop;
pub mod stubs;
pub mod upkeep;

use crate::hex;
use crate::session::Ctx;
use acemu_proto::channel;
use acemu_proto::frame::Packet;
use anyhow::Result;

/// `Ok(None)`: not this module's opcode. `Ok(Some(replies))`: handled (an empty
/// vector is a valid answer). `Err`: the payload did not decode.
pub type Reply = Result<Option<Vec<Packet>>>;

type Handler = fn(&Packet, &mut Ctx) -> Reply;

// mail / quests / ranking sit before shop and stubs on purpose: they take
// over opcodes those modules used to answer with defaults (mailbox state,
// quest list, community options, luck-shop storage info).
const CHAIN: [Handler; 15] = [
    login::handle,
    mail::handle,
    quests::handle,
    daily::handle,
    ranking::handle,
    friends::handle,
    upkeep::handle,
    lobby::handle,
    items::handle,
    shop::handle,
    room::handle,
    netgame::handle,
    matchlog::handle,
    results::handle,
    stubs::handle,
];

pub fn dispatch(pkt: &Packet, ctx: &mut Ctx) -> Result<Vec<Packet>> {
    let name = channel::opcode_name(pkt.opcode).unwrap_or("<unknown>");
    tracing::debug!(uid = ctx.uid, opcode = pkt.opcode, name, len = pkt.payload.len(), "<- recv");
    tracing::trace!(payload = %hex(&pkt.payload), "raw");
    for handle in CHAIN {
        if let Some(replies) = handle(pkt, ctx)? {
            crate::live::publish();
            return Ok(replies);
        }
    }
    tracing::warn!(
        uid = ctx.uid,
        opcode = pkt.opcode,
        name,
        len = pkt.payload.len(),
        payload = %hex(&pkt.payload),
        "no handler"
    );
    Ok(vec![])
}
