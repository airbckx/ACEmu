//! Connected clients. Every connection is a session with a uid and, once logged
//! in, a character name. Sessions are kept in a registry so a handler can push
//! packets to *other* players (room broadcasts, UDP info fan-out).

use acemu_proto::channel;
use acemu_proto::frame::Packet;
use std::io::Write;
use std::net::TcpStream;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};

/// One connected client. `tx` is a second handle on the socket so other threads
/// can push packets to it; every write goes through its mutex.
struct Conn {
    uid: i32,
    name: String,
    in_channel: bool,
    /// The NAT type the client reported in `rfAckStartNatDiag` (1 until known).
    nat_type: u8,
    tx: Arc<Mutex<TcpStream>>,
}

static CONNS: Mutex<Vec<Conn>> = Mutex::new(Vec::new());
static NEXT_UID: AtomicI32 = AtomicI32::new(1);

/// Per-thread view of the session a handler acts for.
pub struct Ctx {
    pub uid: i32,
    /// Character name; empty until `ReqLogin`/`ReqLogin2`.
    pub name: String,
}

/// Adds a connection to the registry and returns its uid.
pub fn register(tx: Arc<Mutex<TcpStream>>) -> i32 {
    let uid = NEXT_UID.fetch_add(1, Ordering::Relaxed);
    CONNS.lock().unwrap().push(Conn { uid, name: String::new(), in_channel: false, nat_type: 1, tx });
    uid
}

pub fn unregister(uid: i32) {
    CONNS.lock().unwrap().retain(|c| c.uid != uid);
}

pub fn set_name(uid: i32, name: &str) {
    if let Some(c) = CONNS.lock().unwrap().iter_mut().find(|c| c.uid == uid) {
        c.name = name.to_string();
    }
}

pub fn set_in_channel(uid: i32, in_channel: bool) {
    if let Some(c) = CONNS.lock().unwrap().iter_mut().find(|c| c.uid == uid) {
        c.in_channel = in_channel;
    }
}

pub fn set_nat(uid: i32, nat_type: u8) {
    if let Some(c) = CONNS.lock().unwrap().iter_mut().find(|c| c.uid == uid) {
        c.nat_type = nat_type;
    }
}

pub fn nat_of(uid: i32) -> u8 {
    CONNS.lock().unwrap().iter().find(|c| c.uid == uid).map(|c| c.nat_type).unwrap_or(1)
}

/// The character name of a logged-in session.
pub fn name_of(uid: i32) -> Option<String> {
    CONNS.lock().unwrap().iter().find(|c| c.uid == uid).map(|c| c.name.clone()).filter(|n| !n.is_empty())
}

/// Every connection as (uid, name, in_channel, nat_type), for the live view.
pub fn snapshot() -> Vec<(i32, String, bool, u8)> {
    CONNS.lock().unwrap().iter().map(|c| (c.uid, c.name.clone(), c.in_channel, c.nat_type)).collect()
}

/// (uid, name) of every logged-in session that is in the channel lobby/rooms.
pub fn channel_users() -> Vec<(i32, String)> {
    CONNS
        .lock()
        .unwrap()
        .iter()
        .filter(|c| c.in_channel && !c.name.is_empty())
        .map(|c| (c.uid, c.name.clone()))
        .collect()
}

/// Pushes packets to one session (no-op if it is gone).
pub fn send_to(uid: i32, pkts: &[Packet]) {
    let tx = CONNS.lock().unwrap().iter().find(|c| c.uid == uid).map(|c| c.tx.clone());
    let Some(tx) = tx else { return };
    let mut s = tx.lock().unwrap();
    for p in pkts {
        tracing::debug!(
            to = uid,
            opcode = p.opcode,
            name = channel::opcode_name(p.opcode).unwrap_or("?"),
            len = p.payload.len(),
            "-> push"
        );
        if let Err(e) = s.write_all(&p.encode()) {
            tracing::warn!(to = uid, error = %e, "push failed");
            break;
        }
    }
}

pub fn send_many(uids: &[i32], pkts: &[Packet]) {
    for &u in uids {
        send_to(u, pkts);
    }
}
