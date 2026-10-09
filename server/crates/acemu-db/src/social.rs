//! Player-to-player and progression features that live beside the core
//! tables: the mailbox (`mail`), coupon codes (`coupons`, `coupon_uses`),
//! per-player extras the lobby asks for (`player_extra`: community options,
//! Fortune Depot capsule counters, booty seed, daily-quest day), the quest
//! definitions seeded from the client's table (`quests`), each player's quest
//! progress (`quest_progress`) and the capsule-reward table
//! (`capsule_rewards`). The channel server's `handlers/{mail,quests,ranking}`
//! are the consumers.

use crate::Db;
use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};

pub const SOCIAL_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS mail (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    owner       TEXT NOT NULL,
    box         INTEGER NOT NULL DEFAULT 0,
    sender      TEXT NOT NULL DEFAULT '',
    recipient   TEXT NOT NULL DEFAULT '',
    text        TEXT NOT NULL DEFAULT '',
    msg_type    INTEGER NOT NULL DEFAULT 1,
    item_uid    INTEGER NOT NULL DEFAULT 0,
    item_id     INTEGER NOT NULL DEFAULT 0,
    item_part   INTEGER NOT NULL DEFAULT 0,
    item_count  INTEGER NOT NULL DEFAULT 1,
    points      INTEGER NOT NULL DEFAULT 0,
    status      INTEGER NOT NULL DEFAULT 0,
    sent_at     TEXT NOT NULL DEFAULT (datetime('now')),
    expires_at  TEXT NOT NULL DEFAULT (datetime('now', '+30 days'))
);
CREATE INDEX IF NOT EXISTS mail_owner ON mail (owner, box);
CREATE TABLE IF NOT EXISTS coupons (
    serial      TEXT PRIMARY KEY,
    item_id     INTEGER NOT NULL DEFAULT 0,
    item_count  INTEGER NOT NULL DEFAULT 1,
    points      INTEGER NOT NULL DEFAULT 0,
    cash        INTEGER NOT NULL DEFAULT 0,
    uses_left   INTEGER NOT NULL DEFAULT 1,
    expires_at  TEXT,
    note        TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS coupon_uses (
    serial      TEXT NOT NULL,
    player      TEXT NOT NULL,
    used_at     TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (serial, player)
);
CREATE TABLE IF NOT EXISTS player_extra (
    player               TEXT PRIMARY KEY,
    opt_whisper          INTEGER NOT NULL DEFAULT 1,
    opt_game_invite      INTEGER NOT NULL DEFAULT 1,
    opt_friend_invite    INTEGER NOT NULL DEFAULT 1,
    opt_clan_invite      INTEGER NOT NULL DEFAULT 1,
    capsule_count        INTEGER NOT NULL DEFAULT 0,
    capsule_reward_state INTEGER NOT NULL DEFAULT 0,
    booty_seed           INTEGER NOT NULL DEFAULT 0,
    daily_day            TEXT NOT NULL DEFAULT ''
);
CREATE TABLE IF NOT EXISTS daily_state (
    player          TEXT PRIMARY KEY,
    attend_month    TEXT NOT NULL DEFAULT '',
    attend_count    INTEGER NOT NULL DEFAULT 0,
    attend_claimed  INTEGER NOT NULL DEFAULT 0,
    attend_day      TEXT NOT NULL DEFAULT '',
    puzzle_tier     INTEGER NOT NULL DEFAULT 1,
    puzzle_pieces   INTEGER NOT NULL DEFAULT 0,
    puzzle_day      TEXT NOT NULL DEFAULT '',
    puzzles_done    INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS daily_quests (
    daily_type  INTEGER NOT NULL,
    step        INTEGER NOT NULL DEFAULT 1,
    quest_id    INTEGER NOT NULL,
    PRIMARY KEY (daily_type, step, quest_id)
);
CREATE TABLE IF NOT EXISTS daily_rewards (
    reward_type INTEGER NOT NULL,
    num         INTEGER NOT NULL,
    item_id     INTEGER NOT NULL DEFAULT 0,
    count       INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (reward_type, num)
);
CREATE TABLE IF NOT EXISTS quests (
    id              INTEGER PRIMARY KEY,
    quest_type      INTEGER NOT NULL DEFAULT 3,
    link_id         INTEGER NOT NULL DEFAULT 0,
    link_step       INTEGER NOT NULL DEFAULT 1,
    create_level    INTEGER NOT NULL DEFAULT 0,
    display_order   INTEGER NOT NULL DEFAULT 0,
    category        TEXT NOT NULL DEFAULT '',
    description     TEXT NOT NULL DEFAULT '',
    mission_type    INTEGER NOT NULL DEFAULT 0,
    mission_count   INTEGER NOT NULL DEFAULT 1,
    reward_exp      INTEGER NOT NULL DEFAULT 0,
    reward_point    INTEGER NOT NULL DEFAULT 0,
    reward_item1    INTEGER NOT NULL DEFAULT 0,
    reward_item2    INTEGER NOT NULL DEFAULT 0,
    reward_part1    INTEGER NOT NULL DEFAULT 0,
    reward_part2    INTEGER NOT NULL DEFAULT 0,
    next_quests     TEXT NOT NULL DEFAULT '',
    event_start     REAL NOT NULL DEFAULT 0,
    event_end       REAL NOT NULL DEFAULT 0,
    enabled         INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE IF NOT EXISTS quest_progress (
    player       TEXT NOT NULL,
    quest_id     INTEGER NOT NULL,
    progress     INTEGER NOT NULL DEFAULT 0,
    complete     INTEGER NOT NULL DEFAULT 0,
    claimed      INTEGER NOT NULL DEFAULT 0,
    assigned_at  TEXT NOT NULL DEFAULT (datetime('now')),
    completed_at TEXT,
    PRIMARY KEY (player, quest_id)
);
CREATE TABLE IF NOT EXISTS capsule_rewards (
    reward_type INTEGER NOT NULL,
    idx         INTEGER NOT NULL,
    need        INTEGER NOT NULL,
    reward_id   INTEGER NOT NULL,
    PRIMARY KEY (reward_type, idx)
);
"#;

pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(SOCIAL_SCHEMA)
}

// ---------------------------------------------------------------------------
// mail

/// Message types as the client's mailbox renders them (`BTTPMailHK.AddMail`):
/// 1 plain text, 2 an attached item, 4 points, 7 cash.
pub const MAIL_TEXT: i32 = 1;
pub const MAIL_ITEM: i32 = 2;
pub const MAIL_POINTS: i32 = 4;

/// Mailbox tabs as the client requests them (`rfReqMemoList` Type).
pub const BOX_INBOX: i32 = 0;
pub const BOX_SENT: i32 = 1;

/// The client shows an inbox as full from this many messages.
pub const MAILBOX_CAPACITY: i64 = 50;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailRow {
    pub id: i64,
    pub owner: String,
    pub r#box: i32,
    pub sender: String,
    pub recipient: String,
    pub text: String,
    pub msg_type: i32,
    pub item_uid: i64,
    pub item_id: i32,
    pub item_part: i32,
    pub item_count: i32,
    pub points: i32,
    /// 0 unread, 1 read.
    pub status: i32,
    /// `YYYY-MM-DD` (the client's `GetDatefromString` splits on `-`).
    pub sent_date: String,
    /// Hours until the message expires.
    pub hours_left: i32,
}

fn mail_from_row(r: &rusqlite::Row) -> rusqlite::Result<MailRow> {
    Ok(MailRow {
        id: r.get(0)?,
        owner: r.get(1)?,
        r#box: r.get(2)?,
        sender: r.get(3)?,
        recipient: r.get(4)?,
        text: r.get(5)?,
        msg_type: r.get(6)?,
        item_uid: r.get(7)?,
        item_id: r.get(8)?,
        item_part: r.get(9)?,
        item_count: r.get(10)?,
        points: r.get(11)?,
        status: r.get(12)?,
        sent_date: r.get(13)?,
        hours_left: r.get::<_, f64>(14)?.max(0.0) as i32,
    })
}

const MAIL_COLS: &str = "id, owner, box, sender, recipient, text, msg_type, item_uid, item_id, item_part, item_count, points, status, \
    substr(sent_at, 1, 10), (julianday(expires_at) - julianday('now')) * 24.0";

/// Why a message could not be delivered; the values are the client's own
/// codes (`BTPageMail.rfAckSendMemo`).
pub const SEND_NO_SUCH_PLAYER: i32 = 100351;
pub const SEND_MAILBOX_FULL: i32 = 100352;

#[derive(Debug, Clone)]
pub struct MailAttachment {
    pub item_uid: i64,
    pub item_id: i32,
    pub item_part: i32,
    pub item_count: i32,
    pub points: i32,
}

impl Db {
    /// Delivers `text` from `sender` to each recipient: an inbox row for them
    /// and one sent row for the sender. Returns the recipients that failed with
    /// the client's error code.
    pub fn mail_send(&self, sender: &str, recipients: &[String], text: &str, attachment: Option<&MailAttachment>) -> Result<Vec<(String, i32)>> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let mut failed = Vec::new();
        let (msg_type, uid, item, part, count, points) = match attachment {
            Some(a) if a.item_id > 0 => (MAIL_ITEM, a.item_uid, a.item_id, a.item_part, a.item_count, 0),
            Some(a) if a.points > 0 => (MAIL_POINTS, 0, 0, 0, 1, a.points),
            _ => (MAIL_TEXT, 0, 0, 0, 1, 0),
        };
        for to in recipients {
            let exists: bool = tx.query_row("SELECT count(*) FROM players WHERE name = ?1", params![to], |r| r.get::<_, i64>(0))? > 0;
            if !exists {
                failed.push((to.clone(), SEND_NO_SUCH_PLAYER));
                continue;
            }
            let count_inbox: i64 = tx.query_row("SELECT count(*) FROM mail WHERE owner = ?1 AND box = 0", params![to], |r| r.get(0))?;
            if count_inbox >= MAILBOX_CAPACITY {
                failed.push((to.clone(), SEND_MAILBOX_FULL));
                continue;
            }
            tx.execute(
                "INSERT INTO mail (owner, box, sender, recipient, text, msg_type, item_uid, item_id, item_part, item_count, points)
                 VALUES (?1, 0, ?2, ?1, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![to, sender, text, msg_type, uid, item, part, count, points],
            )?;
            tx.execute(
                "INSERT INTO mail (owner, box, sender, recipient, text, msg_type, item_uid, item_id, item_part, item_count, points, status)
                 VALUES (?1, 1, ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, 1)",
                params![sender, to, text, msg_type, uid, item, part, count, points],
            )?;
        }
        tx.commit()?;
        Ok(failed)
    }

    /// A system message (no sender account) into a player's inbox.
    pub fn mail_system(&self, to: &str, sender: &str, text: &str, attachment: Option<&MailAttachment>) -> Result<()> {
        let conn = self.lock();
        let (msg_type, uid, item, part, count, points) = match attachment {
            Some(a) if a.item_id > 0 => (MAIL_ITEM, a.item_uid, a.item_id, a.item_part, a.item_count, 0),
            Some(a) if a.points > 0 => (MAIL_POINTS, 0, 0, 0, 1, a.points),
            _ => (MAIL_TEXT, 0, 0, 0, 1, 0),
        };
        conn.execute(
            "INSERT INTO mail (owner, box, sender, recipient, text, msg_type, item_uid, item_id, item_part, item_count, points)
             VALUES (?1, 0, ?2, ?1, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![to, sender, text, msg_type, uid, item, part, count, points],
        )?;
        Ok(())
    }

    /// One mailbox tab, newest first; expired messages are dropped first.
    pub fn mail_list(&self, owner: &str, r#box: i32) -> Result<Vec<MailRow>> {
        let conn = self.lock();
        conn.execute("DELETE FROM mail WHERE expires_at <= datetime('now')", [])?;
        let mut st = conn.prepare(&format!("SELECT {MAIL_COLS} FROM mail WHERE owner = ?1 AND box = ?2 ORDER BY id DESC"))?;
        let rows = st.query_map(params![owner, r#box], mail_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn mail_get(&self, owner: &str, id: i64) -> Result<Option<MailRow>> {
        let conn = self.lock();
        let row = conn
            .query_row(&format!("SELECT {MAIL_COLS} FROM mail WHERE owner = ?1 AND id = ?2"), params![owner, id], mail_from_row)
            .optional()?;
        Ok(row)
    }

    /// Marks a message read and returns it.
    pub fn mail_read(&self, owner: &str, id: i64) -> Result<Option<MailRow>> {
        {
            let conn = self.lock();
            conn.execute("UPDATE mail SET status = 1 WHERE owner = ?1 AND id = ?2", params![owner, id])?;
        }
        self.mail_get(owner, id)
    }

    /// Deletes the given messages from one tab; returns the ids that were not found.
    pub fn mail_delete(&self, owner: &str, r#box: i32, ids: &[i64]) -> Result<Vec<i64>> {
        let conn = self.lock();
        let mut failed = Vec::new();
        for id in ids {
            let n = conn.execute("DELETE FROM mail WHERE owner = ?1 AND box = ?2 AND id = ?3", params![owner, r#box, id])?;
            if n == 0 {
                failed.push(*id);
            }
        }
        Ok(failed)
    }

    /// Empties one tab; returns how many messages went. Messages that still
    /// carry an attachment are kept, as the retail client expects (it warns
    /// before deleting one).
    pub fn mail_delete_all(&self, owner: &str, r#box: i32) -> Result<i64> {
        let conn = self.lock();
        let n = conn.execute(
            "DELETE FROM mail WHERE owner = ?1 AND box = ?2 AND NOT (msg_type IN (2, 4) AND (item_id > 0 OR points > 0))",
            params![owner, r#box],
        )?;
        Ok(n as i64)
    }

    /// (unread inbox messages, inbox total).
    pub fn mail_state(&self, owner: &str) -> Result<(i64, i64)> {
        let conn = self.lock();
        conn.execute("DELETE FROM mail WHERE expires_at <= datetime('now')", [])?;
        let unread: i64 = conn.query_row("SELECT count(*) FROM mail WHERE owner = ?1 AND box = 0 AND status = 0", params![owner], |r| r.get(0))?;
        let total: i64 = conn.query_row("SELECT count(*) FROM mail WHERE owner = ?1 AND box = 0", params![owner], |r| r.get(0))?;
        Ok((unread, total))
    }

    /// Detaches the item or points from a message (the caller grants them);
    /// None when there is nothing to take.
    pub fn mail_take_attachment(&self, owner: &str, id: i64) -> Result<Option<MailAttachment>> {
        let conn = self.lock();
        let row = conn
            .query_row(
                "SELECT item_uid, item_id, item_part, item_count, points FROM mail WHERE owner = ?1 AND id = ?2 AND box = 0",
                params![owner, id],
                |r| Ok(MailAttachment { item_uid: r.get(0)?, item_id: r.get(1)?, item_part: r.get(2)?, item_count: r.get(3)?, points: r.get(4)? }),
            )
            .optional()?;
        let Some(a) = row else { return Ok(None) };
        if a.item_id <= 0 && a.points <= 0 {
            return Ok(None);
        }
        conn.execute(
            "UPDATE mail SET item_uid = 0, item_id = 0, item_part = 0, item_count = 1, points = 0, msg_type = 1, status = 1 WHERE owner = ?1 AND id = ?2",
            params![owner, id],
        )?;
        Ok(Some(a))
    }

    // -----------------------------------------------------------------------
    // coupons

    /// Creates or replaces a coupon code (admin use). The code is stored
    /// normalised (see `coupon_key`).
    pub fn coupon_put(&self, serial: &str, item_id: i32, item_count: i32, points: i32, cash: i32, uses_left: i32, note: &str) -> Result<()> {
        let key = coupon_key(serial);
        if key.len() != COUPON_LEN {
            anyhow::bail!("a coupon code must be {COUPON_LEN} letters or digits (dashes and spaces are ignored); got {}", key.len());
        }
        let conn = self.lock();
        conn.execute(
            "INSERT OR REPLACE INTO coupons (serial, item_id, item_count, points, cash, uses_left, note) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![key, item_id, item_count, points, cash, uses_left, note],
        )?;
        Ok(())
    }

    /// Redeems a code for `player`: None when the code is unknown, exhausted,
    /// expired or already used by this player; otherwise what it grants
    /// (the caller applies it).
    pub fn coupon_redeem(&self, serial: &str, player: &str) -> Result<Option<CouponGrant>> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let key = coupon_key(serial);
        let row = tx
            .query_row(
                "SELECT item_id, item_count, points, cash, uses_left, expires_at FROM coupons WHERE serial = ?1",
                params![key],
                |r| Ok((r.get::<_, i32>(0)?, r.get::<_, i32>(1)?, r.get::<_, i32>(2)?, r.get::<_, i32>(3)?, r.get::<_, i32>(4)?, r.get::<_, Option<String>>(5)?)),
            )
            .optional()?;
        let Some((item_id, item_count, points, cash, uses_left, expires_at)) = row else { return Ok(None) };
        if uses_left <= 0 {
            return Ok(None);
        }
        if let Some(exp) = expires_at {
            let expired: bool = tx.query_row("SELECT datetime(?1) <= datetime('now')", params![exp], |r| r.get(0))?;
            if expired {
                return Ok(None);
            }
        }
        let used: i64 = tx.query_row("SELECT count(*) FROM coupon_uses WHERE serial = ?1 AND player = ?2", params![key, player], |r| r.get(0))?;
        if used > 0 {
            return Ok(None);
        }
        tx.execute("INSERT INTO coupon_uses (serial, player) VALUES (?1, ?2)", params![key, player])?;
        tx.execute("UPDATE coupons SET uses_left = uses_left - 1 WHERE serial = ?1", params![key])?;
        tx.commit()?;
        Ok(Some(CouponGrant { item_id, item_count, points, cash }))
    }

    // -----------------------------------------------------------------------
    // per-player extras

    pub fn extra(&self, player: &str) -> Result<PlayerExtra> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO player_extra (player) VALUES (?1)", params![player])?;
        let row = conn.query_row(
            "SELECT opt_whisper, opt_game_invite, opt_friend_invite, opt_clan_invite, capsule_count, capsule_reward_state, booty_seed, daily_day
             FROM player_extra WHERE player = ?1",
            params![player],
            |r| {
                Ok(PlayerExtra {
                    opt_whisper: r.get(0)?,
                    opt_game_invite: r.get(1)?,
                    opt_friend_invite: r.get(2)?,
                    opt_clan_invite: r.get(3)?,
                    capsule_count: r.get(4)?,
                    capsule_reward_state: r.get(5)?,
                    booty_seed: r.get(6)?,
                    daily_day: r.get(7)?,
                })
            },
        )?;
        Ok(row)
    }

    pub fn extra_set_options(&self, player: &str, whisper: i32, game_invite: i32, friend_invite: i32, clan_invite: i32) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO player_extra (player) VALUES (?1)", params![player])?;
        conn.execute(
            "UPDATE player_extra SET opt_whisper = ?2, opt_game_invite = ?3, opt_friend_invite = ?4, opt_clan_invite = ?5 WHERE player = ?1",
            params![player, whisper, game_invite, friend_invite, clan_invite],
        )?;
        Ok(())
    }

    pub fn extra_set_booty_seed(&self, player: &str, seed: i32) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO player_extra (player) VALUES (?1)", params![player])?;
        conn.execute("UPDATE player_extra SET booty_seed = ?2 WHERE player = ?1", params![player, seed])?;
        Ok(())
    }

    /// Counts capsules bought in the Fortune Depot; returns the new count.
    pub fn extra_capsule_bought(&self, player: &str, n: i32) -> Result<i32> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO player_extra (player) VALUES (?1)", params![player])?;
        conn.execute("UPDATE player_extra SET capsule_count = capsule_count + ?2 WHERE player = ?1", params![player, n])?;
        let c: i32 = conn.query_row("SELECT capsule_count FROM player_extra WHERE player = ?1", params![player], |r| r.get(0))?;
        Ok(c)
    }

    pub fn extra_set_reward_state(&self, player: &str, state: i32) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO player_extra (player) VALUES (?1)", params![player])?;
        conn.execute("UPDATE player_extra SET capsule_reward_state = ?2 WHERE player = ?1", params![player, state])?;
        Ok(())
    }

    pub fn extra_set_daily_day(&self, player: &str, day: &str) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO player_extra (player) VALUES (?1)", params![player])?;
        conn.execute("UPDATE player_extra SET daily_day = ?2 WHERE player = ?1", params![player, day])?;
        Ok(())
    }

    // -----------------------------------------------------------------------
    // daily attendance / puzzle

    pub fn daily_state(&self, player: &str) -> Result<DailyState> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO daily_state (player) VALUES (?1)", params![player])?;
        let row = conn.query_row(
            "SELECT attend_month, attend_count, attend_claimed, attend_day, puzzle_tier, puzzle_pieces, puzzle_day, puzzles_done
             FROM daily_state WHERE player = ?1",
            params![player],
            |r| {
                Ok(DailyState {
                    attend_month: r.get(0)?,
                    attend_count: r.get(1)?,
                    attend_claimed: r.get(2)?,
                    attend_day: r.get(3)?,
                    puzzle_tier: r.get(4)?,
                    puzzle_pieces: r.get(5)?,
                    puzzle_day: r.get(6)?,
                    puzzles_done: r.get(7)?,
                })
            },
        )?;
        Ok(row)
    }

    pub fn daily_state_set(&self, player: &str, s: &DailyState) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO daily_state (player) VALUES (?1)", params![player])?;
        conn.execute(
            "UPDATE daily_state SET attend_month = ?2, attend_count = ?3, attend_claimed = ?4, attend_day = ?5,
                    puzzle_tier = ?6, puzzle_pieces = ?7, puzzle_day = ?8, puzzles_done = ?9 WHERE player = ?1",
            params![
                player,
                s.attend_month,
                s.attend_count,
                s.attend_claimed,
                s.attend_day,
                s.puzzle_tier,
                s.puzzle_pieces,
                s.puzzle_day,
                s.puzzles_done
            ],
        )?;
        Ok(())
    }

    /// Loads the client's daily-quest rotation table once (`data/daily-quests.json`).
    pub fn daily_quests_seed(&self, rows: &[DailyQuestRow]) -> Result<usize> {
        let mut conn = self.lock();
        let existing: i64 = conn.query_row("SELECT count(*) FROM daily_quests", [], |r| r.get(0))?;
        if existing > 0 || rows.is_empty() {
            return Ok(0);
        }
        let tx = conn.transaction()?;
        for r in rows {
            tx.execute("INSERT OR REPLACE INTO daily_quests (daily_type, step, quest_id) VALUES (?1, ?2, ?3)", params![r.daily_type, r.step, r.quest_id])?;
        }
        tx.commit()?;
        Ok(rows.len())
    }

    pub fn daily_quests(&self) -> Result<Vec<DailyQuestRow>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT daily_type, step, quest_id FROM daily_quests ORDER BY daily_type, step, rowid")?;
        let rows = st
            .query_map([], |r| Ok(DailyQuestRow { daily_type: r.get(0)?, step: r.get(1)?, quest_id: r.get(2)? }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    /// Loads the daily reward table once (`data/daily-rewards.json`).
    pub fn daily_rewards_seed(&self, rows: &[DailyRewardRow]) -> Result<usize> {
        let mut conn = self.lock();
        let existing: i64 = conn.query_row("SELECT count(*) FROM daily_rewards", [], |r| r.get(0))?;
        if existing > 0 || rows.is_empty() {
            return Ok(0);
        }
        let tx = conn.transaction()?;
        for r in rows {
            tx.execute(
                "INSERT OR REPLACE INTO daily_rewards (reward_type, num, item_id, count) VALUES (?1, ?2, ?3, ?4)",
                params![r.reward_type, r.num, r.item_id, r.count],
            )?;
        }
        tx.commit()?;
        Ok(rows.len())
    }

    pub fn daily_rewards(&self) -> Result<Vec<DailyRewardRow>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT reward_type, num, item_id, count FROM daily_rewards ORDER BY reward_type, num")?;
        let rows = st
            .query_map([], |r| Ok(DailyRewardRow { reward_type: r.get(0)?, num: r.get(1)?, item_id: r.get(2)?, count: r.get(3)? }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    // -----------------------------------------------------------------------
    // capsule rewards

    pub fn capsule_rewards_seed(&self, rows: &[CapsuleRewardRow]) -> Result<usize> {
        let mut conn = self.lock();
        let existing: i64 = conn.query_row("SELECT count(*) FROM capsule_rewards", [], |r| r.get(0))?;
        if existing > 0 || rows.is_empty() {
            return Ok(0);
        }
        let tx = conn.transaction()?;
        for r in rows {
            tx.execute(
                "INSERT OR REPLACE INTO capsule_rewards (reward_type, idx, need, reward_id) VALUES (?1, ?2, ?3, ?4)",
                params![r.reward_type, r.index, r.need, r.reward_id],
            )?;
        }
        tx.commit()?;
        Ok(rows.len())
    }

    pub fn capsule_rewards(&self) -> Result<Vec<CapsuleRewardRow>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT reward_type, idx, need, reward_id FROM capsule_rewards ORDER BY reward_type, idx")?;
        let rows = st
            .query_map([], |r| Ok(CapsuleRewardRow { reward_type: r.get(0)?, index: r.get(1)?, need: r.get(2)?, reward_id: r.get(3)? }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    // -----------------------------------------------------------------------
    // quests

    /// Loads the quest table once (rows from `data/quests.json`).
    pub fn quests_seed(&self, rows: &[QuestRow]) -> Result<usize> {
        let mut conn = self.lock();
        let existing: i64 = conn.query_row("SELECT count(*) FROM quests", [], |r| r.get(0))?;
        if existing > 0 || rows.is_empty() {
            return Ok(0);
        }
        let tx = conn.transaction()?;
        for q in rows {
            let next: Vec<String> = q.next_quests.iter().map(|n| n.to_string()).collect();
            tx.execute(
                "INSERT OR REPLACE INTO quests (id, quest_type, link_id, link_step, create_level, display_order, category, description,
                    mission_type, mission_count, reward_exp, reward_point, reward_item1, reward_item2, reward_part1, reward_part2,
                    next_quests, event_start, event_end)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
                params![
                    q.id,
                    q.quest_type,
                    q.link_id,
                    q.link_step,
                    q.create_level,
                    q.display_order,
                    q.category,
                    q.description,
                    q.mission_type,
                    q.mission_count,
                    q.reward_exp,
                    q.reward_point,
                    q.reward_items.first().copied().unwrap_or(0),
                    q.reward_items.get(1).copied().unwrap_or(0),
                    q.reward_parts.first().copied().unwrap_or(0),
                    q.reward_parts.get(1).copied().unwrap_or(0),
                    next.join(","),
                    q.event_start,
                    q.event_end
                ],
            )?;
        }
        tx.commit()?;
        Ok(rows.len())
    }

    pub fn quests(&self) -> Result<Vec<QuestRow>> {
        let conn = self.lock();
        let mut st = conn.prepare(
            "SELECT id, quest_type, link_id, link_step, create_level, display_order, category, description, mission_type, mission_count,
                    reward_exp, reward_point, reward_item1, reward_item2, reward_part1, reward_part2, next_quests, event_start, event_end, enabled
             FROM quests ORDER BY display_order, id",
        )?;
        let rows = st
            .query_map([], |r| {
                let next: String = r.get(16)?;
                Ok(QuestRow {
                    id: r.get(0)?,
                    quest_type: r.get(1)?,
                    link_id: r.get(2)?,
                    link_step: r.get(3)?,
                    create_level: r.get(4)?,
                    display_order: r.get(5)?,
                    category: r.get(6)?,
                    description: r.get(7)?,
                    mission_type: r.get(8)?,
                    mission_count: r.get(9)?,
                    reward_exp: r.get(10)?,
                    reward_point: r.get(11)?,
                    reward_items: vec![r.get(12)?, r.get(13)?],
                    reward_parts: vec![r.get(14)?, r.get(15)?],
                    next_quests: next.split(',').filter_map(|s| s.trim().parse().ok()).collect(),
                    event_start: r.get(17)?,
                    event_end: r.get(18)?,
                    enabled: r.get::<_, i32>(19)? != 0,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn quest_progress(&self, player: &str) -> Result<Vec<QuestProgressRow>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT quest_id, progress, complete, claimed FROM quest_progress WHERE player = ?1 ORDER BY quest_id")?;
        let rows = st
            .query_map(params![player], |r| {
                Ok(QuestProgressRow { quest_id: r.get(0)?, progress: r.get(1)?, complete: r.get::<_, i32>(2)? != 0, claimed: r.get::<_, i32>(3)? != 0 })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    /// Adds quests to a player's list (existing rows untouched).
    pub fn quest_assign(&self, player: &str, ids: &[i32]) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        for id in ids {
            tx.execute("INSERT OR IGNORE INTO quest_progress (player, quest_id) VALUES (?1, ?2)", params![player, id])?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Removes the given quests from a player's list (daily reset).
    pub fn quest_remove(&self, player: &str, ids: &[i32]) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        for id in ids {
            tx.execute("DELETE FROM quest_progress WHERE player = ?1 AND quest_id = ?2", params![player, id])?;
        }
        tx.commit()?;
        Ok(())
    }

    /// Stores reported progress (never lowering it); returns the stored value.
    pub fn quest_set_progress(&self, player: &str, quest_id: i32, progress: i32) -> Result<i32> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO quest_progress (player, quest_id) VALUES (?1, ?2)", params![player, quest_id])?;
        conn.execute(
            "UPDATE quest_progress SET progress = max(progress, ?3) WHERE player = ?1 AND quest_id = ?2",
            params![player, quest_id, progress],
        )?;
        let p: i32 = conn.query_row("SELECT progress FROM quest_progress WHERE player = ?1 AND quest_id = ?2", params![player, quest_id], |r| r.get(0))?;
        Ok(p)
    }

    pub fn quest_mark_complete(&self, player: &str, quest_id: i32) -> Result<()> {
        let conn = self.lock();
        conn.execute(
            "UPDATE quest_progress SET complete = 1, completed_at = datetime('now') WHERE player = ?1 AND quest_id = ?2",
            params![player, quest_id],
        )?;
        Ok(())
    }

    pub fn quest_mark_claimed(&self, player: &str, quest_id: i32) -> Result<()> {
        let conn = self.lock();
        conn.execute("UPDATE quest_progress SET claimed = 1 WHERE player = ?1 AND quest_id = ?2", params![player, quest_id])?;
        Ok(())
    }

    /// Players who completed any of the given quests, most recent first.
    pub fn quest_completers(&self, quest_ids: &[i32]) -> Result<Vec<String>> {
        if quest_ids.is_empty() {
            return Ok(vec![]);
        }
        let conn = self.lock();
        let list: Vec<String> = quest_ids.iter().map(|i| i.to_string()).collect();
        let sql = format!(
            "SELECT DISTINCT player FROM quest_progress WHERE complete = 1 AND quest_id IN ({}) ORDER BY completed_at DESC LIMIT 20",
            list.join(",")
        );
        let mut st = conn.prepare(&sql)?;
        let rows = st.query_map([], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }
}

/// The client's Redeem Codes window has eight boxes of four characters and
/// sends them joined, upper-cased, only when exactly 32 long.
pub const COUPON_LEN: usize = 32;

/// A code as the client sends it: letters and digits only, upper-case.
pub fn coupon_key(serial: &str) -> String {
    serial.chars().filter(|c| c.is_ascii_alphanumeric()).map(|c| c.to_ascii_uppercase()).collect()
}

/// The stored code in eight dash-separated groups, for display.
pub fn coupon_display(serial: &str) -> String {
    serial.as_bytes().chunks(4).map(|c| String::from_utf8_lossy(c).to_string()).collect::<Vec<_>>().join("-")
}

// ---------------------------------------------------------------------------
// admin views

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponRow {
    pub serial: String,
    pub item_id: i32,
    pub item_name: Option<String>,
    pub item_count: i32,
    pub points: i32,
    pub cash: i32,
    pub uses_left: i32,
    pub expires_at: Option<String>,
    pub note: String,
    pub used: i64,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct QuestPatch {
    pub enabled: Option<bool>,
    pub quest_type: Option<i32>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub mission_count: Option<i32>,
    pub reward_exp: Option<i32>,
    pub reward_point: Option<i32>,
    pub reward_items: Option<Vec<i32>>,
    pub next_quests: Option<Vec<i32>>,
    pub create_level: Option<i32>,
    /// OLE dates; 0 clears the window.
    pub event_start: Option<f64>,
    pub event_end: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerQuestRow {
    pub quest_id: i32,
    pub category: String,
    pub description: String,
    pub quest_type: i32,
    pub mission_count: i32,
    pub progress: i32,
    pub complete: bool,
    pub claimed: bool,
    pub assigned_at: String,
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PlayerQuestPatch {
    pub progress: Option<i32>,
    pub complete: Option<bool>,
    pub claimed: Option<bool>,
}

impl Db {
    pub fn coupons(&self) -> Result<Vec<CouponRow>> {
        let conn = self.lock();
        let mut st = conn.prepare(
            "SELECT c.serial, c.item_id, i.name, c.item_count, c.points, c.cash, c.uses_left, c.expires_at, c.note,
                    (SELECT count(*) FROM coupon_uses u WHERE u.serial = c.serial)
             FROM coupons c LEFT JOIN catalog i ON i.id = c.item_id ORDER BY c.serial",
        )?;
        let rows = st
            .query_map([], |r| {
                Ok(CouponRow {
                    serial: r.get(0)?,
                    item_id: r.get(1)?,
                    item_name: r.get(2)?,
                    item_count: r.get(3)?,
                    points: r.get(4)?,
                    cash: r.get(5)?,
                    uses_left: r.get(6)?,
                    expires_at: r.get(7)?,
                    note: r.get(8)?,
                    used: r.get(9)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn coupon_set_expiry(&self, serial: &str, expires_at: Option<&str>) -> Result<()> {
        let conn = self.lock();
        conn.execute("UPDATE coupons SET expires_at = ?2 WHERE serial = ?1", params![coupon_key(serial), expires_at])?;
        Ok(())
    }

    /// Removes codes the client could never send (not 32 alphanumerics);
    /// returns how many went.
    pub fn coupons_prune_invalid(&self) -> Result<usize> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT serial FROM coupons")?;
        let all = st.query_map([], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        let mut n = 0;
        for s in all {
            if s.len() != COUPON_LEN || coupon_key(&s) != s {
                conn.execute("DELETE FROM coupon_uses WHERE serial = ?1", params![s])?;
                n += conn.execute("DELETE FROM coupons WHERE serial = ?1", params![s])?;
            }
        }
        Ok(n)
    }

    pub fn coupon_delete(&self, serial: &str) -> Result<bool> {
        let conn = self.lock();
        let key = coupon_key(serial);
        conn.execute("DELETE FROM coupon_uses WHERE serial = ?1", params![key])?;
        Ok(conn.execute("DELETE FROM coupons WHERE serial = ?1", params![key])? > 0)
    }

    /// Every message in the system (admin view), newest first; optional
    /// owner filter.
    pub fn mail_all(&self, owner: Option<&str>, limit: i64) -> Result<Vec<MailRow>> {
        let conn = self.lock();
        let mut st = conn.prepare(&format!("SELECT {MAIL_COLS} FROM mail WHERE (?1 = '' OR owner = ?1) ORDER BY id DESC LIMIT ?2"))?;
        let rows = st.query_map(params![owner.unwrap_or(""), limit], mail_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn mail_delete_by_id(&self, id: i64) -> Result<bool> {
        let conn = self.lock();
        Ok(conn.execute("DELETE FROM mail WHERE id = ?1", params![id])? > 0)
    }

    pub fn player_names(&self) -> Result<Vec<String>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT name FROM players ORDER BY name")?;
        let rows = st.query_map([], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn quest_get(&self, id: i32) -> Result<Option<QuestRow>> {
        Ok(self.quests()?.into_iter().find(|q| q.id == id))
    }

    /// Creates a quest row (a custom challenge). The client only knows its own
    /// table, so a new id shows with the client's default text; reusing an
    /// existing id is refused.
    pub fn quest_create(&self, q: &QuestRow) -> Result<()> {
        if self.quest_get(q.id)?.is_some() {
            anyhow::bail!("quest {} already exists", q.id);
        }
        self.quests_insert(std::slice::from_ref(q))
    }

    fn quests_insert(&self, rows: &[QuestRow]) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        for q in rows {
            let next: Vec<String> = q.next_quests.iter().map(|n| n.to_string()).collect();
            tx.execute(
                "INSERT OR REPLACE INTO quests (id, quest_type, link_id, link_step, create_level, display_order, category, description,
                    mission_type, mission_count, reward_exp, reward_point, reward_item1, reward_item2, reward_part1, reward_part2,
                    next_quests, event_start, event_end, enabled)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)",
                params![
                    q.id,
                    q.quest_type,
                    q.link_id,
                    q.link_step,
                    q.create_level,
                    q.display_order,
                    q.category,
                    q.description,
                    q.mission_type,
                    q.mission_count,
                    q.reward_exp,
                    q.reward_point,
                    q.reward_items.first().copied().unwrap_or(0),
                    q.reward_items.get(1).copied().unwrap_or(0),
                    q.reward_parts.first().copied().unwrap_or(0),
                    q.reward_parts.get(1).copied().unwrap_or(0),
                    next.join(","),
                    q.event_start,
                    q.event_end,
                    q.enabled as i32
                ],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn quest_update(&self, id: i32, p: &QuestPatch) -> Result<Option<QuestRow>> {
        let Some(mut q) = self.quest_get(id)? else { return Ok(None) };
        if let Some(v) = p.enabled {
            q.enabled = v;
        }
        if let Some(v) = p.quest_type {
            q.quest_type = v;
        }
        if let Some(v) = &p.category {
            q.category = v.clone();
        }
        if let Some(v) = &p.description {
            q.description = v.clone();
        }
        if let Some(v) = p.mission_count {
            q.mission_count = v.max(1);
        }
        if let Some(v) = p.reward_exp {
            q.reward_exp = v.max(0);
        }
        if let Some(v) = p.reward_point {
            q.reward_point = v.max(0);
        }
        if let Some(v) = &p.reward_items {
            q.reward_items = v.clone();
        }
        if let Some(v) = &p.next_quests {
            q.next_quests = v.clone();
        }
        if let Some(v) = p.create_level {
            q.create_level = v.max(0);
        }
        if let Some(v) = p.event_start {
            q.event_start = v.max(0.0);
        }
        if let Some(v) = p.event_end {
            q.event_end = v.max(0.0);
        }
        self.quests_insert(std::slice::from_ref(&q))?;
        self.quest_get(id)
    }

    pub fn quest_delete(&self, id: i32) -> Result<bool> {
        let conn = self.lock();
        conn.execute("DELETE FROM quest_progress WHERE quest_id = ?1", params![id])?;
        Ok(conn.execute("DELETE FROM quests WHERE id = ?1", params![id])? > 0)
    }

    /// A player's quest list with the definitions joined in.
    pub fn player_quests(&self, player: &str) -> Result<Vec<PlayerQuestRow>> {
        let conn = self.lock();
        let mut st = conn.prepare(
            "SELECT p.quest_id, coalesce(q.category, ''), coalesce(q.description, ''), coalesce(q.quest_type, 0), coalesce(q.mission_count, 1),
                    p.progress, p.complete, p.claimed, p.assigned_at, p.completed_at
             FROM quest_progress p LEFT JOIN quests q ON q.id = p.quest_id WHERE p.player = ?1 ORDER BY p.quest_id",
        )?;
        let rows = st
            .query_map(params![player], |r| {
                Ok(PlayerQuestRow {
                    quest_id: r.get(0)?,
                    category: r.get(1)?,
                    description: r.get(2)?,
                    quest_type: r.get(3)?,
                    mission_count: r.get(4)?,
                    progress: r.get(5)?,
                    complete: r.get::<_, i32>(6)? != 0,
                    claimed: r.get::<_, i32>(7)? != 0,
                    assigned_at: r.get(8)?,
                    completed_at: r.get(9)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    /// Who has a quest and how far (admin view of one quest).
    pub fn quest_players(&self, quest_id: i32) -> Result<Vec<(String, i32, bool, bool)>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT player, progress, complete, claimed FROM quest_progress WHERE quest_id = ?1 ORDER BY complete DESC, progress DESC, player")?;
        let rows = st
            .query_map(params![quest_id], |r| Ok((r.get(0)?, r.get(1)?, r.get::<_, i32>(2)? != 0, r.get::<_, i32>(3)? != 0)))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn player_quest_patch(&self, player: &str, quest_id: i32, p: &PlayerQuestPatch) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO quest_progress (player, quest_id) VALUES (?1, ?2)", params![player, quest_id])?;
        if let Some(v) = p.progress {
            conn.execute("UPDATE quest_progress SET progress = ?3 WHERE player = ?1 AND quest_id = ?2", params![player, quest_id, v.max(0)])?;
        }
        if let Some(v) = p.complete {
            if v {
                conn.execute(
                    "UPDATE quest_progress SET complete = 1, completed_at = coalesce(completed_at, datetime('now')) WHERE player = ?1 AND quest_id = ?2",
                    params![player, quest_id],
                )?;
            } else {
                conn.execute("UPDATE quest_progress SET complete = 0, completed_at = NULL, claimed = 0 WHERE player = ?1 AND quest_id = ?2", params![player, quest_id])?;
            }
        }
        if let Some(v) = p.claimed {
            conn.execute("UPDATE quest_progress SET claimed = ?3 WHERE player = ?1 AND quest_id = ?2", params![player, quest_id, v as i32])?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponGrant {
    pub item_id: i32,
    pub item_count: i32,
    pub points: i32,
    pub cash: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PlayerExtra {
    pub opt_whisper: i32,
    pub opt_game_invite: i32,
    pub opt_friend_invite: i32,
    pub opt_clan_invite: i32,
    pub capsule_count: i32,
    pub capsule_reward_state: i32,
    pub booty_seed: i32,
    pub daily_day: String,
}

/// A player's attendance and puzzle progress (`daily_state`).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DailyState {
    /// `YYYY-MM` the attendance count belongs to; a new month starts over.
    pub attend_month: String,
    /// Days checked in this month (one per calendar day).
    pub attend_count: i32,
    /// Bit `n` set: attendance reward `n+1` was taken.
    pub attend_claimed: i32,
    /// `YYYY-MM-DD` of the last check-in.
    pub attend_day: String,
    /// Which puzzle (1..3) is being collected.
    pub puzzle_tier: i32,
    /// Bit `n` set: puzzle piece `n+1` collected.
    pub puzzle_pieces: i32,
    /// `YYYY-MM-DD` of the last piece (one per day).
    pub puzzle_day: String,
    pub puzzles_done: i32,
}

/// One row of the client's daily-quest rotation table (`wDailyQuestParam`):
/// the quest of a daily type on a given day is `candidates[day % n]`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyQuestRow {
    pub daily_type: i32,
    #[serde(default = "one")]
    pub step: i32,
    pub quest_id: i32,
}

/// Daily quest types as the client's `wDailyQuestParam.DailyQuestType` uses them.
pub const DAILY_ATTENDANCE: i32 = 2;
pub const DAILY_PUZZLE_FIRST: i32 = 3;
pub const DAILY_PUZZLE_LAST: i32 = 5;
pub const DAILY_SPECIAL: i32 = 7;

/// One row of the daily reward table (`wDailyQuestRewardParam`): type 0 =
/// after `num` attendance check-ins, 1 = for finishing puzzle `num`, 2 = the
/// special mission's reward (our own row, index 21 of the client's list).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyRewardRow {
    pub reward_type: i32,
    pub num: i32,
    pub item_id: i32,
    #[serde(default = "one")]
    pub count: i32,
}

pub const REWARD_ATTENDANCE: i32 = 0;
pub const REWARD_PUZZLE: i32 = 1;
pub const REWARD_SPECIAL: i32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsuleRewardRow {
    pub reward_type: i32,
    pub index: i32,
    pub need: i32,
    pub reward_id: i32,
}

/// One row of the client's quest table (see `docs/GAME-DATA.md`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestRow {
    pub id: i32,
    pub quest_type: i32,
    #[serde(default)]
    pub link_id: i32,
    #[serde(default = "one")]
    pub link_step: i32,
    #[serde(default)]
    pub create_level: i32,
    #[serde(default)]
    pub display_order: i32,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub mission_type: i32,
    #[serde(default = "one")]
    pub mission_count: i32,
    #[serde(default)]
    pub reward_exp: i32,
    #[serde(default)]
    pub reward_point: i32,
    #[serde(default)]
    pub reward_items: Vec<i32>,
    #[serde(default)]
    pub reward_parts: Vec<i32>,
    #[serde(default)]
    pub next_quests: Vec<i32>,
    #[serde(default)]
    pub event_start: f64,
    #[serde(default)]
    pub event_end: f64,
    #[serde(default = "yes")]
    pub enabled: bool,
}

fn one() -> i32 {
    1
}
fn yes() -> bool {
    true
}

/// Quest types as the client's `wQuestParam.QuestType` uses them.
pub const QUEST_EVENT: i32 = 1;
pub const QUEST_DAILY: i32 = 2;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestProgressRow {
    pub quest_id: i32,
    pub progress: i32,
    pub complete: bool,
    pub claimed: bool,
}
