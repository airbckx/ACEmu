//! Match history: one row per finished match (`matches`), one per participant
//! (`match_players`, bots included), the kill feed (`match_kills`) and the
//! objective events the host reports during a match (`match_events`: round
//! ends, bomb plants/defuses, points taken, domination score, chopper kills).
//! The channel server records a match when the host reports the score table
//! (`rfReqUpdateScore`); events are written as they arrive with a NULL
//! `match_id` and attached to the match once it is recorded. The admin
//! service reads the history and derives the leaderboard from the players'
//! career stats plus these rows. `host_priority` keeps the last host-priority
//! point each player reported (`rfReqUpdateMyHostPriorityPoint`; lower is a
//! better host).

use crate::Db;
use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

pub const MATCH_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS matches (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    played_at   TEXT NOT NULL DEFAULT (datetime('now')),
    room_name   TEXT NOT NULL DEFAULT '',
    map         TEXT NOT NULL DEFAULT '',
    mode        TEXT NOT NULL DEFAULT '',
    host        TEXT NOT NULL DEFAULT '',
    duration    INTEGER NOT NULL DEFAULT 0,
    end_type    INTEGER NOT NULL DEFAULT 0,
    team0_score INTEGER NOT NULL DEFAULT 0,
    team1_score INTEGER NOT NULL DEFAULT 0,
    winner      INTEGER NOT NULL DEFAULT -1
);
CREATE TABLE IF NOT EXISTS match_players (
    match_id  INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    uid       INTEGER NOT NULL,
    name      TEXT NOT NULL,
    team      INTEGER NOT NULL DEFAULT 0,
    is_bot    INTEGER NOT NULL DEFAULT 0,
    score     INTEGER NOT NULL DEFAULT 0,
    kills     INTEGER NOT NULL DEFAULT 0,
    assists   INTEGER NOT NULL DEFAULT 0,
    deaths    INTEGER NOT NULL DEFAULT 0,
    exp       INTEGER NOT NULL DEFAULT 0,
    points    INTEGER NOT NULL DEFAULT 0,
    headshots INTEGER NOT NULL DEFAULT 0,
    won       INTEGER NOT NULL DEFAULT -1,
    PRIMARY KEY (match_id, uid)
);
CREATE TABLE IF NOT EXISTS match_kills (
    match_id    INTEGER NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    seq         INTEGER NOT NULL,
    at_secs     INTEGER NOT NULL DEFAULT 0,
    killer_uid  INTEGER NOT NULL,
    killer_name TEXT NOT NULL DEFAULT '',
    victim_uid  INTEGER NOT NULL,
    victim_name TEXT NOT NULL DEFAULT '',
    weapon      INTEGER NOT NULL DEFAULT 0,
    body_part   INTEGER NOT NULL DEFAULT 0,
    headshot    INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (match_id, seq)
);
CREATE TABLE IF NOT EXISTS match_events (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    match_id    INTEGER REFERENCES matches(id) ON DELETE CASCADE,
    room_number INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    at_secs     INTEGER NOT NULL DEFAULT 0,
    kind        TEXT NOT NULL,
    actor_uid   INTEGER NOT NULL DEFAULT 0,
    actor_name  TEXT NOT NULL DEFAULT '',
    team        INTEGER NOT NULL DEFAULT -1,
    extra       TEXT NOT NULL DEFAULT '{}'
);
CREATE INDEX IF NOT EXISTS match_events_match ON match_events(match_id);
CREATE INDEX IF NOT EXISTS match_events_room ON match_events(room_number, match_id);
CREATE TABLE IF NOT EXISTS host_priority (
    name       TEXT PRIMARY KEY,
    uid        INTEGER NOT NULL DEFAULT 0,
    point      INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE INDEX IF NOT EXISTS matches_played_at ON matches(played_at);
CREATE INDEX IF NOT EXISTS match_players_name ON match_players(name);
"#;

/// Creates the match tables; called from `Db::open`.
pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(MATCH_SCHEMA)
}

// ---------------------------------------------------------------------------
// input

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMatchPlayer {
    pub uid: i32,
    pub name: String,
    pub team: i32,
    pub is_bot: bool,
    pub score: i32,
    pub kills: i32,
    pub assists: i32,
    pub deaths: i32,
    pub exp: i32,
    pub points: i32,
    pub headshots: i32,
    /// `Some(true)` won, `Some(false)` lost, `None` draw / undecided.
    pub won: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewKill {
    pub at_secs: u32,
    pub killer_uid: i32,
    pub killer_name: String,
    pub victim_uid: i32,
    pub victim_name: String,
    pub weapon: i32,
    pub body_part: i32,
    pub headshot: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMatch {
    pub room_name: String,
    pub map: String,
    pub mode: String,
    pub host: String,
    /// Seconds.
    pub duration: i32,
    pub end_type: i32,
    pub team0_score: i32,
    pub team1_score: i32,
    /// 0 or 1; -1 for a draw.
    pub winner: i32,
    pub players: Vec<NewMatchPlayer>,
    pub kills: Vec<NewKill>,
}

/// One objective event reported by the host while a match is running. It is
/// stored right away against the room; [`Db::attach_match_events`] links it to
/// the match once that is recorded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewMatchEvent {
    pub room_number: i32,
    /// Seconds since the host pressed Start.
    pub at_secs: u32,
    /// `round_end`, `bomb_planted`, `bomb_defused`, `point_taken`, `dom_score`,
    /// `chopper_kill`, `udp_kick`, `host_lost`.
    pub kind: String,
    /// The player the event is about (0 when it has none, e.g. a score update).
    pub actor_uid: i32,
    pub actor_name: String,
    /// 0 AF, 1 RSA, -1 none / unknown.
    pub team: i32,
    /// Kind-specific detail, stored as JSON text.
    pub extra: serde_json::Value,
}

// ---------------------------------------------------------------------------
// output

#[derive(Debug, Clone, Serialize)]
pub struct MatchSummary {
    pub id: i64,
    /// UTC, `YYYY-MM-DD HH:MM:SS`.
    pub played_at: String,
    pub room_name: String,
    pub map: String,
    pub mode: String,
    pub host: String,
    pub duration: i32,
    pub end_type: i32,
    pub team0_score: i32,
    pub team1_score: i32,
    pub winner: i32,
    /// Human participants.
    pub players: i64,
    pub bots: i64,
    /// Names of the human participants.
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchPlayerRow {
    pub uid: i32,
    pub name: String,
    pub team: i32,
    pub is_bot: bool,
    pub score: i32,
    pub kills: i32,
    pub assists: i32,
    pub deaths: i32,
    pub exp: i32,
    pub points: i32,
    pub headshots: i32,
    /// 1 won, 0 lost, -1 draw / undecided.
    pub won: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchKillRow {
    pub seq: i32,
    pub at_secs: i32,
    pub killer_uid: i32,
    pub killer_name: String,
    pub victim_uid: i32,
    pub victim_name: String,
    pub weapon: i32,
    pub weapon_name: Option<String>,
    pub body_part: i32,
    pub headshot: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchEventRow {
    pub id: i64,
    pub at_secs: i32,
    pub kind: String,
    pub actor_uid: i32,
    pub actor_name: String,
    pub team: i32,
    pub extra: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct MatchDetail {
    #[serde(flatten)]
    pub summary: MatchSummary,
    pub scoreboard: Vec<MatchPlayerRow>,
    pub kill_feed: Vec<MatchKillRow>,
    /// Objective timeline (round ends, bomb plants, points taken, ...), oldest first.
    pub events: Vec<MatchEventRow>,
}

/// A player's last reported host-priority point (lower is a better host).
#[derive(Debug, Clone, Serialize)]
pub struct HostPriorityRow {
    pub name: String,
    pub uid: i32,
    pub point: i32,
    pub updated_at: String,
}

/// One leaderboard line: the career stats plus what the match history adds.
#[derive(Debug, Clone, Serialize)]
pub struct LeaderRow {
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub headshots: i32,
    pub wins: i32,
    pub losses: i32,
    pub play_time: i32,
    pub matches: i64,
    pub best_kills: i32,
    pub last_played: Option<String>,
}

impl Db {
    /// Stores a finished match; returns its id.
    pub fn record_match(&self, m: &NewMatch) -> Result<i64> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        tx.execute(
            "INSERT INTO matches (room_name, map, mode, host, duration, end_type, team0_score, team1_score, winner)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![m.room_name, m.map, m.mode, m.host, m.duration, m.end_type, m.team0_score, m.team1_score, m.winner],
        )?;
        let id = tx.last_insert_rowid();
        for p in &m.players {
            let won = match p.won {
                Some(true) => 1,
                Some(false) => 0,
                None => -1,
            };
            tx.execute(
                "INSERT OR REPLACE INTO match_players (match_id, uid, name, team, is_bot, score, kills, assists, deaths, exp, points, headshots, won)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                params![id, p.uid, p.name, p.team, p.is_bot as i32, p.score, p.kills, p.assists, p.deaths, p.exp, p.points, p.headshots, won],
            )?;
        }
        for (seq, k) in m.kills.iter().enumerate() {
            tx.execute(
                "INSERT INTO match_kills (match_id, seq, at_secs, killer_uid, killer_name, victim_uid, victim_name, weapon, body_part, headshot)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                params![id, seq as i32, k.at_secs, k.killer_uid, k.killer_name, k.victim_uid, k.victim_name, k.weapon, k.body_part, k.headshot as i32],
            )?;
        }
        tx.commit()?;
        Ok(id)
    }

    /// Newest first. `player` restricts to matches that player took part in.
    /// Returns (total, one page).
    pub fn matches(&self, limit: i64, offset: i64, player: Option<&str>) -> Result<(i64, Vec<MatchSummary>)> {
        let conn = self.lock();
        let who = player.unwrap_or("");
        let filter = "(?1 = '' OR EXISTS (SELECT 1 FROM match_players f WHERE f.match_id = m.id AND f.name = ?1 AND f.is_bot = 0))";
        let total: i64 = conn.query_row(&format!("SELECT count(*) FROM matches m WHERE {filter}"), params![who], |r| r.get(0))?;
        let mut st = conn.prepare(&format!(
            "SELECT m.id, m.played_at, m.room_name, m.map, m.mode, m.host, m.duration, m.end_type, m.team0_score, m.team1_score, m.winner,
                    (SELECT count(*) FROM match_players p WHERE p.match_id = m.id AND p.is_bot = 0),
                    (SELECT count(*) FROM match_players p WHERE p.match_id = m.id AND p.is_bot = 1),
                    (SELECT group_concat(p.name, char(10)) FROM match_players p WHERE p.match_id = m.id AND p.is_bot = 0)
             FROM matches m WHERE {filter} ORDER BY m.id DESC LIMIT ?2 OFFSET ?3"
        ))?;
        let rows = st
            .query_map(params![who, limit.clamp(1, 500), offset.max(0)], summary_from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok((total, rows))
    }

    pub fn match_detail(&self, id: i64) -> Result<Option<MatchDetail>> {
        let conn = self.lock();
        let summary = conn
            .query_row(
                "SELECT m.id, m.played_at, m.room_name, m.map, m.mode, m.host, m.duration, m.end_type, m.team0_score, m.team1_score, m.winner,
                        (SELECT count(*) FROM match_players p WHERE p.match_id = m.id AND p.is_bot = 0),
                        (SELECT count(*) FROM match_players p WHERE p.match_id = m.id AND p.is_bot = 1),
                        (SELECT group_concat(p.name, char(10)) FROM match_players p WHERE p.match_id = m.id AND p.is_bot = 0)
                 FROM matches m WHERE m.id = ?1",
                params![id],
                summary_from_row,
            )
            .optional()?;
        let Some(summary) = summary else { return Ok(None) };
        let scoreboard = conn
            .prepare(
                "SELECT uid, name, team, is_bot, score, kills, assists, deaths, exp, points, headshots, won
                 FROM match_players WHERE match_id = ?1 ORDER BY team, score DESC, kills DESC, name",
            )?
            .query_map(params![id], |r| {
                Ok(MatchPlayerRow {
                    uid: r.get(0)?,
                    name: r.get(1)?,
                    team: r.get(2)?,
                    is_bot: r.get::<_, i32>(3)? != 0,
                    score: r.get(4)?,
                    kills: r.get(5)?,
                    assists: r.get(6)?,
                    deaths: r.get(7)?,
                    exp: r.get(8)?,
                    points: r.get(9)?,
                    headshots: r.get(10)?,
                    won: r.get(11)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let kill_feed = conn
            .prepare(
                "SELECT k.seq, k.at_secs, k.killer_uid, k.killer_name, k.victim_uid, k.victim_name, k.weapon, c.name, k.body_part, k.headshot
                 FROM match_kills k LEFT JOIN catalog c ON c.id = k.weapon
                 WHERE k.match_id = ?1 ORDER BY k.seq",
            )?
            .query_map(params![id], |r| {
                Ok(MatchKillRow {
                    seq: r.get(0)?,
                    at_secs: r.get(1)?,
                    killer_uid: r.get(2)?,
                    killer_name: r.get(3)?,
                    victim_uid: r.get(4)?,
                    victim_name: r.get(5)?,
                    weapon: r.get(6)?,
                    weapon_name: r.get(7)?,
                    body_part: r.get(8)?,
                    headshot: r.get::<_, i32>(9)? != 0,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let events = events_of(&conn, id)?;
        Ok(Some(MatchDetail { summary, scoreboard, kill_feed, events }))
    }

    pub fn delete_match(&self, id: i64) -> Result<bool> {
        let n = self.lock().execute("DELETE FROM matches WHERE id = ?1", params![id])?;
        Ok(n > 0)
    }

    // --- in-match events

    /// Stores an event of a running match (no match id yet); returns its row id.
    pub fn record_match_event(&self, e: &NewMatchEvent) -> Result<i64> {
        let conn = self.lock();
        conn.execute(
            "INSERT INTO match_events (room_number, at_secs, kind, actor_uid, actor_name, team, extra)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![e.room_number, e.at_secs, e.kind, e.actor_uid, e.actor_name, e.team, e.extra.to_string()],
        )?;
        Ok(conn.last_insert_rowid())
    }

    /// Links the unattached events of `room_number` written in the last
    /// `window_secs` seconds to match `match_id` and drops older unattached
    /// ones of that room (left over from a match that was never recorded).
    /// Returns how many were attached.
    pub fn attach_match_events(&self, match_id: i64, room_number: i32, window_secs: i64) -> Result<usize> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let since = format!("-{} seconds", window_secs.max(0));
        let attached = tx.execute(
            "UPDATE match_events SET match_id = ?1
             WHERE match_id IS NULL AND room_number = ?2 AND created_at >= datetime('now', ?3)",
            params![match_id, room_number, since],
        )?;
        tx.execute("DELETE FROM match_events WHERE match_id IS NULL AND room_number = ?1", params![room_number])?;
        tx.commit()?;
        Ok(attached)
    }

    /// The events of a recorded match, oldest first.
    pub fn match_events(&self, match_id: i64) -> Result<Vec<MatchEventRow>> {
        events_of(&self.lock(), match_id)
    }

    // --- host priority

    /// Remembers the host-priority point `name` (connected as `uid`) reported.
    pub fn set_host_priority(&self, name: &str, uid: i32, point: i32) -> Result<()> {
        self.lock().execute(
            "INSERT INTO host_priority (name, uid, point, updated_at) VALUES (?1, ?2, ?3, datetime('now'))
             ON CONFLICT(name) DO UPDATE SET uid = excluded.uid, point = excluded.point, updated_at = excluded.updated_at",
            params![name, uid, point],
        )?;
        Ok(())
    }

    /// The last host-priority point `name` reported, if any.
    pub fn host_priority(&self, name: &str) -> Result<Option<HostPriorityRow>> {
        Ok(self
            .lock()
            .query_row("SELECT name, uid, point, updated_at FROM host_priority WHERE name = ?1", params![name], |r| {
                Ok(HostPriorityRow { name: r.get(0)?, uid: r.get(1)?, point: r.get(2)?, updated_at: r.get(3)? })
            })
            .optional()?)
    }

    /// Every saved player with career stats and match-history aggregates.
    /// Unsorted; the caller ranks by whatever column it shows.
    pub fn leaderboard(&self) -> Result<Vec<LeaderRow>> {
        let conn = self.lock();
        let mut st = conn.prepare(
            "SELECT p.name, p.level, p.exp, p.kills, p.deaths, p.assists, p.headshots, p.wins, p.losses, p.play_time,
                    COALESCE(x.matches, 0), COALESCE(x.best_kills, 0), x.last_played
             FROM players p
             LEFT JOIN (
                 SELECT mp.name, count(*) AS matches, max(mp.kills) AS best_kills, max(m.played_at) AS last_played
                 FROM match_players mp JOIN matches m ON m.id = mp.match_id
                 WHERE mp.is_bot = 0 GROUP BY mp.name
             ) x ON x.name = p.name
             ORDER BY p.kills DESC, p.name",
        )?;
        let rows = st
            .query_map([], |r| {
                Ok(LeaderRow {
                    name: r.get(0)?,
                    level: r.get(1)?,
                    exp: r.get(2)?,
                    kills: r.get(3)?,
                    deaths: r.get(4)?,
                    assists: r.get(5)?,
                    headshots: r.get(6)?,
                    wins: r.get(7)?,
                    losses: r.get(8)?,
                    play_time: r.get(9)?,
                    matches: r.get(10)?,
                    best_kills: r.get(11)?,
                    last_played: r.get(12)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    /// Whether `name` has a recorded match of `mode` (used for one-off rewards
    /// such as the tutorial).
    pub fn has_played_mode(&self, name: &str, mode: &str) -> Result<bool> {
        let n: i64 = self.lock().query_row(
            "SELECT count(*) FROM match_players p JOIN matches m ON m.id = p.match_id WHERE p.name = ?1 AND m.mode = ?2",
            params![name, mode],
            |r| r.get(0),
        )?;
        Ok(n > 0)
    }

    pub fn match_count(&self) -> Result<i64> {
        Ok(self.lock().query_row("SELECT count(*) FROM matches", [], |r| r.get(0))?)
    }
}

fn events_of(conn: &Connection, match_id: i64) -> Result<Vec<MatchEventRow>> {
    let rows = conn
        .prepare("SELECT id, at_secs, kind, actor_uid, actor_name, team, extra FROM match_events WHERE match_id = ?1 ORDER BY id")?
        .query_map(params![match_id], |r| {
            let extra: String = r.get(6)?;
            Ok(MatchEventRow {
                id: r.get(0)?,
                at_secs: r.get(1)?,
                kind: r.get(2)?,
                actor_uid: r.get(3)?,
                actor_name: r.get(4)?,
                team: r.get(5)?,
                extra: serde_json::from_str(&extra).unwrap_or(serde_json::Value::Null),
            })
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

fn summary_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<MatchSummary> {
    let names: Option<String> = r.get(13)?;
    Ok(MatchSummary {
        id: r.get(0)?,
        played_at: r.get(1)?,
        room_name: r.get(2)?,
        map: r.get(3)?,
        mode: r.get(4)?,
        host: r.get(5)?,
        duration: r.get(6)?,
        end_type: r.get(7)?,
        team0_score: r.get(8)?,
        team1_score: r.get(9)?,
        winner: r.get(10)?,
        players: r.get(11)?,
        bots: r.get(12)?,
        names: names.map(|s| s.split('\n').map(str::to_string).collect()).unwrap_or_default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn matches_round_trip_and_leaderboard() {
        let db = Db::open(Path::new(":memory:")).unwrap();
        {
            let conn = db.lock();
            conn.execute("INSERT INTO players (name, kills, deaths, wins) VALUES ('MaxA', 10, 2, 1)", []).unwrap();
            conn.execute("INSERT INTO players (name, kills, deaths, losses) VALUES ('MaxB', 3, 9, 1)", []).unwrap();
            conn.execute("INSERT INTO catalog (id, name, item_type) VALUES (2000, 'G3', 1)", []).unwrap();
        }
        let m = NewMatch {
            room_name: "Test".into(),
            map: "dm-CQBhouse".into(),
            mode: "WGame.wTeamGame".into(),
            host: "MaxA".into(),
            duration: 300,
            end_type: 1,
            team0_score: 50,
            team1_score: 20,
            winner: 0,
            players: vec![
                NewMatchPlayer { uid: 1, name: "MaxA".into(), team: 0, is_bot: false, score: 50, kills: 10, assists: 0, deaths: 2, exp: 100, points: 200, headshots: 4, won: Some(true) },
                NewMatchPlayer { uid: 3, name: "MaxB".into(), team: 1, is_bot: false, score: 20, kills: 2, assists: 1, deaths: 10, exp: 40, points: 80, headshots: 0, won: Some(false) },
                NewMatchPlayer { uid: 1000, name: "Bot".into(), team: 1, is_bot: true, score: 0, kills: 0, assists: 0, deaths: 3, exp: 0, points: 0, headshots: 0, won: Some(false) },
            ],
            kills: vec![NewKill { at_secs: 12, killer_uid: 1, killer_name: "MaxA".into(), victim_uid: 3, victim_name: "MaxB".into(), weapon: 2000, body_part: 1, headshot: true }],
        };
        // an orphan from an earlier, never-recorded match and one real event
        db.record_match_event(&NewMatchEvent {
            room_number: 1,
            at_secs: 5,
            kind: "bomb_planted".into(),
            actor_uid: 1,
            actor_name: "MaxA".into(),
            team: 0,
            extra: serde_json::json!({ "site": 0 }),
        })
        .unwrap();
        db.lock().execute("UPDATE match_events SET created_at = datetime('now', '-1 day')", []).unwrap();
        db.record_match_event(&NewMatchEvent {
            room_number: 1,
            at_secs: 40,
            kind: "round_end".into(),
            actor_uid: 0,
            actor_name: String::new(),
            team: 0,
            extra: serde_json::json!({ "round": 1, "af": 1, "rsa": 0 }),
        })
        .unwrap();
        let id = db.record_match(&m).unwrap();
        assert_eq!(db.attach_match_events(id, 1, 600).unwrap(), 1);
        let ev = db.match_events(id).unwrap();
        assert_eq!(ev.len(), 1);
        assert_eq!(ev[0].kind, "round_end");
        assert_eq!(ev[0].extra["round"], 1);
        let orphans: i64 = db.lock().query_row("SELECT count(*) FROM match_events WHERE match_id IS NULL", [], |r| r.get(0)).unwrap();
        assert_eq!(orphans, 0);
        db.set_host_priority("MaxA", 1, 45).unwrap();
        db.set_host_priority("MaxA", 7, 120).unwrap();
        assert_eq!(db.host_priority("MaxA").unwrap().map(|h| (h.uid, h.point)), Some((7, 120)));
        assert!(db.host_priority("Nobody").unwrap().is_none());
        let (total, page) = db.matches(10, 0, None).unwrap();
        assert_eq!(total, 1);
        assert_eq!(page[0].players, 2);
        assert_eq!(page[0].bots, 1);
        assert_eq!(page[0].names, vec!["MaxA", "MaxB"]);
        let (only_b, _) = db.matches(10, 0, Some("MaxB")).unwrap();
        assert_eq!(only_b, 1);
        let (none, _) = db.matches(10, 0, Some("Nobody")).unwrap();
        assert_eq!(none, 0);
        let d = db.match_detail(id).unwrap().unwrap();
        assert_eq!(d.scoreboard.len(), 3);
        assert_eq!(d.events.len(), 1);
        assert_eq!(d.kill_feed[0].weapon_name.as_deref(), Some("G3"));
        assert!(d.kill_feed[0].headshot);
        let lb = db.leaderboard().unwrap();
        assert_eq!(lb[0].name, "MaxA");
        assert_eq!(lb[0].matches, 1);
        assert_eq!(lb[0].best_kills, 10);
        assert!(db.delete_match(id).unwrap());
        assert_eq!(db.match_count().unwrap(), 0);
    }
}
