//! Live view for the admin dashboard: who is connected and what the room is
//! doing. The snapshot is written to the `live` table of `save/acemu.db`
//! whenever it changes (`publish`, called after every handled packet and on
//! connect/disconnect), plus a heartbeat every few seconds so the admin can
//! tell a quiet server from a stopped one.

use crate::{inventory, room, session};
use serde::Serialize;
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Serialize)]
struct LiveSession {
    uid: i32,
    name: String,
    in_channel: bool,
    nat_type: u8,
    in_room: bool,
}

#[derive(Serialize)]
struct LiveMember {
    uid: i32,
    name: String,
    team: u16,
    state: u8,
    is_owner: bool,
    udp: Option<String>,
}

#[derive(Serialize)]
struct LiveRoom {
    number: i32,
    name: String,
    map: String,
    mode: String,
    mode_num: i32,
    max_users: i32,
    owner_uid: i32,
    owner_name: String,
    playing: bool,
    members: Vec<LiveMember>,
}

#[derive(Serialize)]
struct LiveState {
    sessions: Vec<LiveSession>,
    /// The first room (kept for older admin builds); `rooms` has them all.
    room: Option<LiveRoom>,
    rooms: Vec<LiveRoom>,
}

static LAST: Mutex<Option<(String, Instant)>> = Mutex::new(None);
const HEARTBEAT: Duration = Duration::from_secs(5);

fn snapshot() -> LiveState {
    let in_room = room::all_uids();
    let sessions = session::snapshot()
        .into_iter()
        .map(|(uid, name, in_channel, nat_type)| LiveSession { uid, name, in_channel, nat_type, in_room: in_room.contains(&uid) })
        .collect();
    let rooms: Vec<LiveRoom> = room::each(|r| LiveRoom {
        number: r.info.room_number,
        name: r.info.room_name.clone(),
        map: r.info.map_name.clone(),
        mode: r.info.game_class.clone(),
        mode_num: r.info.game_mode_num as i32,
        max_users: r.info.max_user_cnt as i32,
        owner_uid: r.owner,
        owner_name: r.info.room_owner.clone(),
        playing: r.playing,
        members: r
            .members
            .iter()
            .map(|m| LiveMember {
                uid: m.uid,
                name: m.name.clone(),
                team: m.team,
                state: m.state,
                is_owner: m.uid == r.owner,
                udp: m.udp.as_ref().map(|u| format!("{}:{}", u.ip, u.port)),
            })
            .collect(),
    });
    let room = rooms.first().map(|r| LiveRoom {
        number: r.number,
        name: r.name.clone(),
        map: r.map.clone(),
        mode: r.mode.clone(),
        mode_num: r.mode_num,
        max_users: r.max_users,
        owner_uid: r.owner_uid,
        owner_name: r.owner_name.clone(),
        playing: r.playing,
        members: r
            .members
            .iter()
            .map(|m| LiveMember { uid: m.uid, name: m.name.clone(), team: m.team, state: m.state, is_owner: m.is_owner, udp: m.udp.clone() })
            .collect(),
    });
    LiveState { sessions, room, rooms }
}

/// Marks the server as freshly started (no sessions, no room) and starts the
/// heartbeat thread, so an idle server still reads as online.
pub fn reset() {
    let started = chrono_now();
    let _ = inventory::db().set_live("channel_started_at", &started);
    *LAST.lock().unwrap_or_else(|p| p.into_inner()) = None;
    publish();
    std::thread::spawn(|| loop {
        std::thread::sleep(HEARTBEAT);
        publish();
    });
}

/// Writes the current snapshot if it changed since the last call, and a
/// heartbeat at least every `HEARTBEAT`.
pub fn publish() {
    let json = match serde_json::to_string(&snapshot()) {
        Ok(j) => j,
        Err(_) => return,
    };
    let mut last = LAST.lock().unwrap_or_else(|p| p.into_inner());
    let changed = last.as_ref().map(|(j, _)| j != &json).unwrap_or(true);
    let stale = last.as_ref().map(|(_, t)| t.elapsed() >= HEARTBEAT).unwrap_or(true);
    if !changed && !stale {
        return;
    }
    let db = inventory::db();
    if changed {
        if let Err(e) = db.set_live("channel", &json) {
            tracing::debug!(error = %e, "live state not published");
        }
    }
    if let Err(e) = db.set_live("channel_heartbeat", &chrono_now()) {
        tracing::debug!(error = %e, "heartbeat not published");
    }
    *last = Some((json, Instant::now()));
}

fn chrono_now() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_default()
}
