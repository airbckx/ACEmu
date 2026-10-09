//! Friend lists and block lists. A friendship is stored once per direction so
//! both players see each other; blocks are one-directional.

use crate::Db;
use anyhow::Result;
use rusqlite::{params, Connection};

pub const FRIENDS_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS friends (
    player  TEXT NOT NULL,
    friend  TEXT NOT NULL,
    since   TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (player, friend)
);
CREATE TABLE IF NOT EXISTS blocks (
    player  TEXT NOT NULL,
    blocked TEXT NOT NULL,
    since   TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (player, blocked)
);
"#;

pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(FRIENDS_SCHEMA)
}

impl Db {
    pub fn friends_of(&self, player: &str) -> Result<Vec<String>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT friend FROM friends WHERE player = ?1 ORDER BY friend")?;
        let rows = st.query_map(params![player], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn are_friends(&self, a: &str, b: &str) -> Result<bool> {
        let conn = self.lock();
        let n: i64 = conn.query_row("SELECT count(*) FROM friends WHERE player = ?1 AND friend = ?2", params![a, b], |r| r.get(0))?;
        Ok(n > 0)
    }

    /// Makes `a` and `b` friends (both directions).
    pub fn friend_add(&self, a: &str, b: &str) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        tx.execute("INSERT OR IGNORE INTO friends (player, friend) VALUES (?1, ?2)", params![a, b])?;
        tx.execute("INSERT OR IGNORE INTO friends (player, friend) VALUES (?1, ?2)", params![b, a])?;
        tx.commit()?;
        Ok(())
    }

    /// Ends the friendship in both directions; false if they were not friends.
    pub fn friend_remove(&self, a: &str, b: &str) -> Result<bool> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let n = tx.execute("DELETE FROM friends WHERE player = ?1 AND friend = ?2", params![a, b])?;
        tx.execute("DELETE FROM friends WHERE player = ?1 AND friend = ?2", params![b, a])?;
        tx.commit()?;
        Ok(n > 0)
    }

    pub fn blocks_of(&self, player: &str) -> Result<Vec<String>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT blocked FROM blocks WHERE player = ?1 ORDER BY blocked")?;
        let rows = st.query_map(params![player], |r| r.get::<_, String>(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    /// Whether `player` has blocked `other`.
    pub fn is_blocked(&self, player: &str, other: &str) -> Result<bool> {
        let conn = self.lock();
        let n: i64 = conn.query_row("SELECT count(*) FROM blocks WHERE player = ?1 AND blocked = ?2", params![player, other], |r| r.get(0))?;
        Ok(n > 0)
    }

    pub fn block_add(&self, player: &str, other: &str) -> Result<()> {
        let conn = self.lock();
        conn.execute("INSERT OR IGNORE INTO blocks (player, blocked) VALUES (?1, ?2)", params![player, other])?;
        Ok(())
    }

    pub fn block_remove(&self, player: &str, other: &str) -> Result<bool> {
        let conn = self.lock();
        Ok(conn.execute("DELETE FROM blocks WHERE player = ?1 AND blocked = ?2", params![player, other])? > 0)
    }
}
