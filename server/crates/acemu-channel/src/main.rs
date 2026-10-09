//! ACEmu channel server (port 55999).
//!
//! The client connects here after the login server answers
//! `rfReqConnectChannelServer`. The Channel link has 544 RPCs; this answers what
//! players need to log in, sit in the lobby, create/join a room and start a match
//! (one of them as listen-server host), and logs everything else by name with its
//! payload so the next handler can be written from real traffic. The start-game
//! path (relay-server announcement, UDP address responder, `ClientsCount`
//! semantics) and the quick-slot rules are documented in docs/PROTOCOL.md.
//!
//! Layout:
//! - `session`: connected clients, the per-thread `Ctx`, pushes to other players;
//! - `room`: the one game room and every rule about it;
//! - `nat`: the UDP NAT-diagnosis responder and guest NAT type;
//! - `inventory`: item data, per character, persisted under `save/`;
//! - `player`, `loadout`: packets built from the inventory (character sheet,
//!   equipped/entry lists);
//! - `handlers::*`: the opcode handlers, one module per area of the protocol,
//!   tried in turn by `handlers::dispatch`;
//! - `live`: publishes sessions and the room to the database for the admin.

mod handlers;
mod inventory;
mod live;
mod loadout;
mod nat;
mod player;
mod room;
mod session;

use acemu_proto::channel;
use acemu_proto::frame::Decoder;
use acemu_proto::wire::{self, StringEncoding};
use anyhow::Result;
use session::Ctx;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

const BIND: &str = "0.0.0.0:55999";
/// Address every client is told for everything server-side (relay/NAT servers,
/// room owner IPs): `ACEMU_PUBLIC_IP`, default 127.0.0.1.
pub fn local_ip() -> String {
    acemu_paths::public_ip()
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "acemu_channel=debug,acemu_proto=debug".into()),
        )
        .with_target(false)
        .init();
    wire::set_string_encoding(StringEncoding::NullTerminatedUtf16);
    if std::env::args().any(|a| a == "--check-inventory") {
        return match inventory::check_file() {
            Ok(msg) => {
                println!("{msg}");
                Ok(())
            }
            Err(e) => {
                eprintln!("{e}");
                std::process::exit(1)
            }
        };
    }
    tracing::info!(home = %acemu_paths::home().display(), "server folder (data/ and save/)");
    inventory::load_catalog();
    inventory::load_levels();
    handlers::quests::seed_tables();
    inventory::set_default_name(&player::default_character());
    live::reset();

    for p in nat::NAT_PORTS {
        std::thread::spawn(move || nat::udp_nat_server(p));
    }

    let listener = TcpListener::bind(BIND)?;
    tracing::info!(character = %player::default_character(), "channel server listening on {BIND}");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                std::thread::spawn(move || {
                    let peer = s.peer_addr().map(|a| a.to_string()).unwrap_or_default();
                    tracing::info!(%peer, "client connected");
                    if let Err(e) = serve(s) {
                        tracing::warn!(%peer, error = %e, "session ended");
                    } else {
                        tracing::info!(%peer, "client disconnected");
                    }
                });
            }
            Err(e) => tracing::warn!(error = %e, "accept failed"),
        }
    }
    Ok(())
}

fn serve(stream: TcpStream) -> Result<()> {
    stream.set_nodelay(true)?;
    let tx = Arc::new(Mutex::new(stream.try_clone()?));
    let uid = session::register(tx.clone());
    live::publish();
    let mut ctx = Ctx { uid, name: String::new() };
    let result = serve_loop(stream, &tx, &mut ctx);
    // cleanup: leave the room (telling the others) and forget the session
    let left = if ctx.name.is_empty() { None } else { room::leave(ctx.uid, &ctx.name) };
    session::unregister(uid);
    handlers::lobby::push_room_change(left);
    live::publish();
    tracing::info!(uid, name = %ctx.name, "session removed");
    result
}

fn serve_loop(mut stream: TcpStream, tx: &Arc<Mutex<TcpStream>>, ctx: &mut Ctx) -> Result<()> {
    let mut dec = Decoder::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        dec.feed(&buf[..n]);

        while let Some(pkt) = dec.next_packet()? {
            let replies = match handlers::dispatch(&pkt, ctx) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!(
                        uid = ctx.uid,
                        opcode = pkt.opcode,
                        name = channel::opcode_name(pkt.opcode).unwrap_or("?"),
                        len = pkt.payload.len(),
                        error = %e,
                        payload = %hex(&pkt.payload),
                        "decode failed"
                    );
                    continue;
                }
            };
            if replies.is_empty() {
                continue;
            }
            let mut s = tx.lock().unwrap();
            for reply in replies {
                tracing::debug!(
                    uid = ctx.uid,
                    opcode = reply.opcode,
                    name = channel::opcode_name(reply.opcode).unwrap_or("?"),
                    len = reply.payload.len(),
                    "-> send"
                );
                s.write_all(&reply.encode())?;
            }
        }
    }
}

pub fn unix_now() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i32)
        .unwrap_or(0)
}

/// Hex + printable rendering, for payloads whose layout we do not yet know.
pub fn hex(b: &[u8]) -> String {
    let h: Vec<String> = b.iter().map(|x| format!("{x:02X}")).collect();
    let a: String = b
        .iter()
        .map(|&c| if (32..127).contains(&c) { c as char } else { '.' })
        .collect();
    format!("{} | {a}", h.join(" "))
}
