//! Admin web app sign-in: the admin users (separate from game accounts) and a
//! small key/value store for the admin service (the JWT signing secret).

use crate::Db;
use anyhow::Result;
use rusqlite::{params, Connection, OptionalExtension};

pub const ADMIN_SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS admin_users (
    username      TEXT PRIMARY KEY,
    salt          TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    created_at    TEXT NOT NULL DEFAULT (datetime('now')),
    last_login    TEXT
);
CREATE TABLE IF NOT EXISTS admin_settings (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
"#;

pub fn ensure_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(ADMIN_SCHEMA)
}

#[derive(Debug, Clone)]
pub struct AdminUser {
    pub username: String,
    pub salt: String,
    pub password_hash: String,
    pub created_at: String,
    pub last_login: Option<String>,
}

impl Db {
    pub fn admin_users_count(&self) -> Result<i64> {
        Ok(self.lock().query_row("SELECT count(*) FROM admin_users", [], |r| r.get(0))?)
    }

    pub fn admin_user(&self, username: &str) -> Result<Option<AdminUser>> {
        Ok(self
            .lock()
            .query_row(
                "SELECT username, salt, password_hash, created_at, last_login FROM admin_users WHERE username = ?1",
                params![username],
                |r| Ok(AdminUser { username: r.get(0)?, salt: r.get(1)?, password_hash: r.get(2)?, created_at: r.get(3)?, last_login: r.get(4)? }),
            )
            .optional()?)
    }

    /// Creates or replaces an admin user's credentials.
    pub fn admin_user_set(&self, username: &str, salt: &str, password_hash: &str) -> Result<()> {
        self.lock().execute(
            "INSERT INTO admin_users (username, salt, password_hash) VALUES (?1, ?2, ?3)
             ON CONFLICT(username) DO UPDATE SET salt = excluded.salt, password_hash = excluded.password_hash",
            params![username, salt, password_hash],
        )?;
        Ok(())
    }

    pub fn admin_user_touch(&self, username: &str) -> Result<()> {
        self.lock().execute("UPDATE admin_users SET last_login = datetime('now') WHERE username = ?1", params![username])?;
        Ok(())
    }

    pub fn admin_setting(&self, key: &str) -> Result<Option<String>> {
        Ok(self.lock().query_row("SELECT value FROM admin_settings WHERE key = ?1", params![key], |r| r.get(0)).optional()?)
    }

    pub fn admin_setting_set(&self, key: &str, value: &str) -> Result<()> {
        self.lock().execute(
            "INSERT INTO admin_settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}
