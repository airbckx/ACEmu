//! The game rooms of this channel, keyed by room number (1, 2, ... - the
//! lowest free number is reused). Handlers go through the functions here
//! rather than the lock, so every rule about membership, ownership and the
//! start-game bookkeeping lives in one place.
//!
//! A player is in at most one room, so most queries take the player's uid and
//! act on "the room this player is in"; the lobby uses the room-number forms.

use crate::session::send_many;
use acemu_proto::channel;
use acemu_proto::GameRoomInfo;
use std::collections::BTreeMap;
use std::sync::Mutex;

/// The UDP endpoint a player registered for the match (`ReqRegisterMyUdpInfo`).
#[derive(Clone, Debug)]
pub struct Udp {
    pub ip: String,
    pub port: i32,
    pub inner_ip: String,
    pub inner_port: i32,
    #[allow(dead_code)]
    pub is_host: bool,
    /// Registered by a player joining a match already in progress
    /// (`rfReqRegisterMyUdpInfo.Intervention`); relayed to the host as-is.
    pub intervention: bool,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub uid: i32,
    pub name: String,
    pub team: u16,
    /// Last `rfReqChangeUserState` value (ready flag etc.), echoed in lists.
    pub state: u8,
    pub udp: Option<Udp>,
    /// Joining a match in progress (`rfReqConnectGame` while the room plays)
    /// and still waiting for the "host is up, connect" push.
    pub late_join: bool,
    /// This match's end-of-match loot: (item id, unique id, item type,
    /// rental expiry), once drawn. Cleared at the next start.
    pub booty: Option<(i32, f64, i32, f64)>,
}

impl Member {
    pub fn new(uid: i32, name: &str) -> Member {
        Member { uid, name: name.to_string(), team: 0, state: 0, udp: None, late_join: false, booty: None }
    }
}

/// One kill reported by the host (`rfReqLogGetScore`) during the current match.
#[derive(Clone, Debug)]
pub struct KillEvent {
    /// Seconds since the host pressed Start.
    pub at_secs: u32,
    pub killer: i32,
    pub victim: i32,
    pub weapon: i32,
    pub body_part: i32,
    pub assists: Vec<i32>,
}

pub struct Room {
    pub number: i32,
    pub info: GameRoomInfo,
    pub owner: i32,
    pub members: Vec<Member>,
    pub playing: bool,
    /// Hole-punching reports for the current start: (reporting uid, phrase),
    /// from `rfReqNotifyRecvDummyForHolePunching`. Cleared when punching starts.
    pub punch: Vec<(i32, u8)>,
    /// When the current match was started, for the kill log's timestamps.
    pub started: Option<std::time::Instant>,
    /// Kill feed of the current match, oldest first; cleared at the next start.
    pub kills: Vec<KillEvent>,
    /// The host has finished loading the map (it reported joining its own
    /// listen server). Until then the host's client refuses late joiners.
    pub host_ready: bool,
    /// Late joiners whose UDP registration is held back until `host_ready`.
    pub pending_late: Vec<i32>,
}

impl Room {
    pub fn uids(&self) -> Vec<i32> {
        self.members.iter().map(|m| m.uid).collect()
    }
    pub fn has(&self, uid: i32) -> bool {
        self.members.iter().any(|m| m.uid == uid)
    }
}

static ROOMS: Mutex<BTreeMap<i32, Room>> = Mutex::new(BTreeMap::new());

fn lock() -> std::sync::MutexGuard<'static, BTreeMap<i32, Room>> {
    ROOMS.lock().unwrap_or_else(|p| p.into_inner())
}

/// Runs `f` on room `number`, if it exists.
pub fn read<T>(number: i32, f: impl FnOnce(&Room) -> T) -> Option<T> {
    lock().get(&number).map(f)
}

/// Runs `f` on the room `uid` is in, if any.
pub fn read_of<T>(uid: i32, f: impl FnOnce(&Room) -> T) -> Option<T> {
    lock().values().find(|r| r.has(uid)).map(f)
}

fn with_of<T>(uid: i32, f: impl FnOnce(&mut Room) -> T) -> Option<T> {
    lock().values_mut().find(|r| r.has(uid)).map(f)
}

/// Runs `f` on every room, in room-number order.
pub fn each<T>(mut f: impl FnMut(&Room) -> T) -> Vec<T> {
    lock().values().map(|r| f(r)).collect()
}

// --- queries (by the player)

/// The number of the room `uid` is in.
pub fn number_of(uid: i32) -> Option<i32> {
    read_of(uid, |r| r.number)
}

pub fn contains(uid: i32) -> bool {
    number_of(uid).is_some()
}

/// Every member of `uid`'s room (empty when not in one).
pub fn members_of(uid: i32) -> Vec<Member> {
    read_of(uid, |r| r.members.clone()).unwrap_or_default()
}

/// Every member uid of `uid`'s room, `uid` included.
pub fn uids_of(uid: i32) -> Vec<i32> {
    read_of(uid, |r| r.uids()).unwrap_or_default()
}

/// Every member of `uid`'s room except `uid`.
pub fn others(uid: i32) -> Vec<i32> {
    uids_of(uid).into_iter().filter(|&u| u != uid).collect()
}

pub fn owner_of(uid: i32) -> Option<i32> {
    read_of(uid, |r| r.owner)
}

pub fn is_owner(uid: i32) -> bool {
    owner_of(uid) == Some(uid)
}

pub fn member(uid: i32) -> Option<Member> {
    read_of(uid, |r| r.members.iter().find(|m| m.uid == uid).cloned()).flatten()
}

pub fn info_of(uid: i32) -> Option<GameRoomInfo> {
    read_of(uid, |r| r.info.clone())
}

pub fn host_udp_of(uid: i32) -> Option<Udp> {
    read_of(uid, |r| r.members.iter().find(|m| m.uid == r.owner).and_then(|m| m.udp.clone())).flatten()
}

pub fn is_playing_of(uid: i32) -> bool {
    read_of(uid, |r| r.playing).unwrap_or(false)
}

// --- queries (by the room)

pub fn info(number: i32) -> Option<GameRoomInfo> {
    read(number, |r| r.info.clone())
}

pub fn is_playing(number: i32) -> bool {
    read(number, |r| r.playing).unwrap_or(false)
}

/// Every uid sitting in any room.
pub fn all_uids() -> Vec<i32> {
    lock().values().flat_map(|r| r.uids()).collect()
}

pub fn numbers() -> Vec<i32> {
    lock().keys().copied().collect()
}

// --- lobby: create / join / leave / settings

/// Creates a room with `uid` as its owner and only member and returns its
/// number (`info.room_number` and `room_owner` are set here). A player who is
/// still in another room leaves it first.
pub fn create(uid: i32, name: &str, mut info: GameRoomInfo) -> Result<i32, &'static str> {
    if contains(uid) {
        leave(uid, name);
    }
    let mut g = lock();
    let number = (1..=i32::from(u16::MAX)).find(|n| !g.contains_key(n)).ok_or("no free room number")?;
    info.room_number = number;
    info.room_owner = name.to_string();
    g.insert(
        number,
        Room {
            number,
            info,
            owner: uid,
            members: vec![Member::new(uid, name)],
            playing: false,
            punch: vec![],
            started: None,
            kills: vec![],
            host_ready: false,
            pending_late: vec![],
        },
    );
    Ok(number)
}

pub struct Joined {
    pub number: i32,
    pub member: Member,
    pub owner: i32,
    /// False when the player was already a member (re-entry).
    pub is_new: bool,
}

/// Co-op modes: every human is on one side and the bots on the other. Defence
/// (`wMission.wNDFGameInfo`) is one; any room flagged "play with bots" counts too.
/// Balancing players across both teams in these modes makes them fight each other.
pub fn is_coop(info: &GameRoomInfo) -> bool {
    info.play_with_bots != 0 || info.game_class.contains("NDF") || info.game_class.to_lowercase().contains("defen")
}

/// Adds `uid` to room `number`: on the owner's team in co-op modes, otherwise
/// on the smaller team. Fails for a wrong password, a full room, or when the
/// player is in a different room.
pub fn join(uid: i32, name: &str, number: i32, password: &str) -> Result<Joined, &'static str> {
    let mut g = lock();
    if g.values().any(|r| r.number != number && r.has(uid)) {
        return Err("already in another room");
    }
    match g.get_mut(&number) {
        None => Err("no such room"),
        Some(r) if r.has(uid) => {
            let member = r.members.iter().find(|m| m.uid == uid).cloned().unwrap();
            Ok(Joined { number, member, owner: r.owner, is_new: false })
        }
        Some(r) if r.info.is_password != 0 && !r.info.password.is_empty() && r.info.password != password => Err("wrong password"),
        Some(r) if r.members.len() >= r.info.max_user_cnt.max(1) as usize => Err("room is full"),
        Some(r) => {
            let mut member = Member::new(uid, name);
            member.team = if is_coop(&r.info) {
                r.members.iter().find(|m| m.uid == r.owner).map(|m| m.team).unwrap_or(0)
            } else {
                // team balance: join the smaller side
                let t0 = r.members.iter().filter(|m| m.team == 0).count();
                let t1 = r.members.len() - t0;
                if t1 < t0 { 1 } else { 0 }
            };
            r.members.push(member.clone());
            Ok(Joined { number, member, owner: r.owner, is_new: true })
        }
    }
}

/// What `leave` did: the room the player was in and whether it is now gone.
pub struct Left {
    pub number: i32,
    pub closed: bool,
}

/// Removes `uid` from its room, hands the room to the next member if the
/// owner left, and tells the remaining members. None if the user was in no room.
pub fn leave(uid: i32, name: &str) -> Option<Left> {
    let (number, closed, others) = {
        let mut g = lock();
        let number = g.values().find(|r| r.has(uid))?.number;
        let r = g.get_mut(&number)?;
        r.members.retain(|m| m.uid != uid);
        r.pending_late.retain(|&u| u != uid);
        if r.members.is_empty() {
            tracing::info!(room = number, "room closed - last member left");
            g.remove(&number);
            return Some(Left { number, closed: true });
        }
        if r.owner == uid {
            let new_owner = &r.members[0];
            tracing::info!(room = number, new_owner = new_owner.uid, name = %new_owner.name, "room owner left - ownership moved");
            r.owner = new_owner.uid;
            r.info.room_owner = new_owner.name.clone();
            // the host's listen server left with it
            r.playing = false;
            r.host_ready = false;
            r.pending_late.clear();
        }
        (number, false, r.uids())
    };
    send_many(
        &others,
        &[channel::AckLeaveRoom {
            num: number as u16,
            uid,
            user_id: name.to_string(),
            result: 0,
            err_msg: String::new(),
        }
        .packet()],
    );
    Some(Left { number, closed })
}

/// Replaces the settings of `uid`'s room (number and owner are kept). Returns
/// the stored info and every member's uid.
pub fn update_info(uid: i32, mut info: GameRoomInfo) -> Option<(GameRoomInfo, Vec<i32>)> {
    with_of(uid, |r| {
        info.room_number = r.number;
        info.room_owner = r.info.room_owner.clone();
        r.info = info;
        (r.info.clone(), r.uids())
    })
}

/// Records a member's new state. Returns (room number, old state, member
/// position, every member's uid) when the player is in a room.
pub fn set_state(uid: i32, new_state: u8) -> Option<(i32, u8, u8, Vec<i32>)> {
    with_of(uid, |r| {
        let pos = r.members.iter().position(|m| m.uid == uid)?;
        let old = r.members[pos].state;
        r.members[pos].state = new_state;
        Some((r.number, old, pos as u8, r.uids()))
    })
    .flatten()
}

/// Members of `uid`'s room other than the host that pressed Ready (state 1):
/// the players a Start actually takes along.
pub fn ready_guests_of(uid: i32) -> Vec<i32> {
    read_of(uid, |r| r.members.iter().filter(|m| m.uid != r.owner && m.state == 1).map(|m| m.uid).collect()).unwrap_or_default()
}

/// Puts every member of `uid`'s room back to "not ready". Returns
/// (room number, member uids in list order) for the state broadcast.
pub fn reset_states(uid: i32) -> Option<(i32, Vec<i32>)> {
    with_of(uid, |r| {
        for m in &mut r.members {
            m.state = 0;
        }
        (r.number, r.uids())
    })
}

/// Records a member's loot for this match.
pub fn set_booty(uid: i32, booty: (i32, f64, i32, f64)) {
    with_of(uid, |r| {
        if let Some(m) = r.members.iter_mut().find(|m| m.uid == uid) {
            m.booty = Some(booty);
        }
    });
}

pub fn set_team(uid: i32, team: u16) {
    with_of(uid, |r| {
        if let Some(m) = r.members.iter_mut().find(|m| m.uid == uid) {
            m.team = team;
        }
    });
}

// --- match start (all keyed by a member's uid, normally the host's)

/// Marks `uid`'s room as playing and forgets the previous match's UDP
/// endpoints. Returns every member's uid.
pub fn start(uid: i32) -> Option<Vec<i32>> {
    with_of(uid, |r| {
        r.playing = true;
        r.started = Some(std::time::Instant::now());
        r.kills.clear();
        r.host_ready = false;
        r.pending_late.clear();
        for m in &mut r.members {
            m.udp = None;
            m.late_join = false;
            m.booty = None;
        }
        r.uids()
    })
}

/// Appends a kill to the current match's feed of the reporter's room
/// (timestamped from the start).
pub fn record_kill(reporter: i32, killer: i32, victim: i32, weapon: i32, body_part: i32, assists: Vec<i32>) {
    with_of(reporter, |r| {
        let at_secs = r.started.map(|s| s.elapsed().as_secs() as u32).unwrap_or(0);
        r.kills.push(KillEvent { at_secs, killer, victim, weapon, body_part, assists });
    });
}

/// Seconds since the match of `uid`'s room started and its kill feed so far.
pub fn match_log(uid: i32) -> (u32, Vec<KillEvent>) {
    read_of(uid, |r| (r.started.map(|s| s.elapsed().as_secs() as u32).unwrap_or(0), r.kills.clone())).unwrap_or((0, vec![]))
}

pub fn set_playing(uid: i32, playing: bool) {
    with_of(uid, |r| r.playing = playing);
}

/// Stores a member's UDP endpoint; false when `uid` is in no room.
pub fn set_udp(uid: i32, udp: Udp) -> bool {
    with_of(uid, |r| {
        if let Some(m) = r.members.iter_mut().find(|m| m.uid == uid) {
            m.udp = Some(udp);
        }
    })
    .is_some()
}

/// Flags `uid` as joining the running match (or clears it once connected).
pub fn set_late_join(uid: i32, late: bool) {
    with_of(uid, |r| {
        if let Some(m) = r.members.iter_mut().find(|m| m.uid == uid) {
            m.late_join = late;
        }
    });
}

/// Whether the host of `uid`'s room has finished loading.
pub fn is_host_ready_of(uid: i32) -> bool {
    read_of(uid, |r| r.host_ready).unwrap_or(false)
}

/// Holds a late joiner's registration back until the host is ready.
pub fn queue_late(uid: i32) {
    with_of(uid, |r| {
        if !r.pending_late.contains(&uid) {
            r.pending_late.push(uid);
        }
    });
}

pub fn is_queued_late(uid: i32) -> bool {
    read_of(uid, |r| r.pending_late.contains(&uid)).unwrap_or(false)
}

/// Marks the host of `uid`'s room as loaded and returns the late joiners
/// that were waiting for it (their queue is cleared).
pub fn set_host_ready(uid: i32) -> Vec<i32> {
    with_of(uid, |r| {
        r.host_ready = true;
        std::mem::take(&mut r.pending_late)
    })
    .unwrap_or_default()
}

pub fn is_late_join(uid: i32) -> bool {
    member(uid).map(|m| m.late_join).unwrap_or(false)
}

/// Members of `uid`'s room still waiting to join the running match.
pub fn late_joiners_of(uid: i32) -> Vec<i32> {
    read_of(uid, |r| r.members.iter().filter(|m| m.late_join).map(|m| m.uid).collect()).unwrap_or_default()
}

pub fn punch_reset(uid: i32) {
    with_of(uid, |r| r.punch.clear());
}

pub fn punch_report(uid: i32, phrase: u8) {
    with_of(uid, |r| r.punch.push((uid, phrase)));
}

/// The hole-punching reports so far in `uid`'s room and every member's uid.
pub fn punch_state(uid: i32) -> Option<(Vec<(i32, u8)>, Vec<i32>)> {
    read_of(uid, |r| (r.punch.clone(), r.uids()))
}
