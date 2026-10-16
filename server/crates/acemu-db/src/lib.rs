//! SQLite storage for ACEmu (`save/acemu.db`): accounts, players with their
//! inventories, and the item catalogue with prices and shop availability.
//!
//! The channel server, the login server and the admin service all open the
//! same file; SQLite's WAL mode and a busy timeout make that safe. Every call
//! here is one short transaction, so nothing is cached across processes: what
//! the admin changes, the game sees on its next request.
//!
//! On first use the tables are seeded from what was on disk before: the
//! client-extracted tables under `data/` (items, shop, products) and the
//! legacy `save/acemu-inventory*.json` / `save/acemu-account*.txt` files.

pub mod admin;
pub mod friends;
pub mod matches;
pub mod model;
pub mod social;

use anyhow::{anyhow, Context, Result};
use model::*;
use rusqlite::{params, Connection, OptionalExtension, Transaction};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

pub const DB_FILE: &str = "acemu.db";

pub struct Db {
    conn: Mutex<Connection>,
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS accounts (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    username      TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL DEFAULT '',
    character     TEXT UNIQUE,
    created_at    TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS players (
    name        TEXT PRIMARY KEY,
    points      INTEGER NOT NULL DEFAULT 0,
    cash        INTEGER NOT NULL DEFAULT 0,
    next_unique INTEGER NOT NULL DEFAULT 100000001,
    level       INTEGER NOT NULL DEFAULT 1,
    exp         INTEGER NOT NULL DEFAULT 0,
    kills       INTEGER NOT NULL DEFAULT 0,
    deaths      INTEGER NOT NULL DEFAULT 0,
    assists     INTEGER NOT NULL DEFAULT 0,
    headshots   INTEGER NOT NULL DEFAULT 0,
    team_kills  INTEGER NOT NULL DEFAULT 0,
    wins        INTEGER NOT NULL DEFAULT 0,
    losses      INTEGER NOT NULL DEFAULT 0,
    play_time   INTEGER NOT NULL DEFAULT 0,
    created_at  TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at  TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS items (
    player      TEXT NOT NULL REFERENCES players(name) ON DELETE CASCADE ON UPDATE CASCADE,
    unique_id   INTEGER NOT NULL,
    item_id     INTEGER NOT NULL,
    item_type   INTEGER NOT NULL,
    slot        INTEGER NOT NULL DEFAULT -1,
    stack       INTEGER NOT NULL DEFAULT 1,
    durability  INTEGER NOT NULL DEFAULT 100000,
    add_part_id INTEGER NOT NULL DEFAULT 0,
    paint_id    INTEGER NOT NULL DEFAULT 0,
    until       REAL NOT NULL DEFAULT 0,
    PRIMARY KEY (player, unique_id)
);
CREATE TABLE IF NOT EXISTS skills (
    player    TEXT NOT NULL REFERENCES players(name) ON DELETE CASCADE ON UPDATE CASCADE,
    unique_id INTEGER NOT NULL,
    skill_id  INTEGER NOT NULL,
    slot      INTEGER NOT NULL DEFAULT -1,
    until     REAL NOT NULL DEFAULT 0,
    PRIMARY KEY (player, unique_id)
);
CREATE TABLE IF NOT EXISTS qslots (
    player  TEXT NOT NULL REFERENCES players(name) ON DELETE CASCADE ON UPDATE CASCADE,
    id      INTEGER NOT NULL,
    name    TEXT NOT NULL,
    main    INTEGER NOT NULL DEFAULT 0,
    pistol  INTEGER NOT NULL DEFAULT 0,
    throw1  INTEGER NOT NULL DEFAULT 0,
    throw2  INTEGER NOT NULL DEFAULT 0,
    skill1  INTEGER NOT NULL DEFAULT 0,
    skill2  INTEGER NOT NULL DEFAULT 0,
    skill3  INTEGER NOT NULL DEFAULT 0,
    skill4  INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (player, id)
);
CREATE TABLE IF NOT EXISTS catalog (
    id            INTEGER PRIMARY KEY,
    name          TEXT NOT NULL,
    item_type     INTEGER NOT NULL,
    kind          INTEGER NOT NULL DEFAULT -1,
    category      TEXT NOT NULL DEFAULT '',
    sell_price    INTEGER NOT NULL DEFAULT 0,
    availability  TEXT NOT NULL DEFAULT 'shop' CHECK (availability IN ('shop', 'lootbox', 'hidden')),
    shop_order    INTEGER NOT NULL DEFAULT 0,
    in_shop_table INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS prices (
    item_id INTEGER NOT NULL REFERENCES catalog(id) ON DELETE CASCADE,
    idx     INTEGER NOT NULL,
    hours   INTEGER NOT NULL DEFAULT 0,
    points  INTEGER NOT NULL DEFAULT 0,
    cash    INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (item_id, idx)
);
CREATE INDEX IF NOT EXISTS catalog_availability ON catalog(availability, shop_order);
CREATE INDEX IF NOT EXISTS catalog_name ON catalog(name);
CREATE TABLE IF NOT EXISTS live (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (datetime('now'))
);
CREATE TABLE IF NOT EXISTS box_entries (
    box_id   INTEGER NOT NULL,
    item_id  INTEGER NOT NULL,
    reel     TEXT NOT NULL CHECK (reel IN ('main', 'bonus')),
    weight   REAL NOT NULL DEFAULT 1,
    featured INTEGER NOT NULL DEFAULT 0,
    hours    INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (box_id, reel, item_id)
);
"#;

/// Shop categories whose items are box spoils: never listed in the shop,
/// only random boxes hand them out.
const LOOTBOX_CATEGORIES: [&str; 3] = ["Superior", "Rare", "Legendary"];

/// True when a seeding error only means another process got there first: a
/// UNIQUE/PRIMARY KEY collision, or a write lock that stayed busy past the
/// timeout. The tables end up filled either way, so callers say so quietly
/// instead of warning about it (the three services seed in parallel; see
/// `Db::seed_if_empty`).
pub fn is_concurrent_seed(err: &anyhow::Error) -> bool {
    err.chain().filter_map(|e| e.downcast_ref::<rusqlite::Error>()).any(|e| {
        matches!(e, rusqlite::Error::SqliteFailure(f, _)
            if matches!(
                f.code,
                rusqlite::ErrorCode::ConstraintViolation | rusqlite::ErrorCode::DatabaseBusy | rusqlite::ErrorCode::DatabaseLocked
            ))
    })
}

pub fn hash_password(secret: &str) -> String {
    if secret.is_empty() {
        return String::new();
    }
    let digest = Sha256::digest(secret.as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

impl Db {
    /// Opens `save/acemu.db` (see `acemu_paths`), creating the schema if needed.
    pub fn open_default() -> Result<Db> {
        Self::open(&acemu_paths::save_file(DB_FILE))
    }

    pub fn open(path: &Path) -> Result<Db> {
        let conn = Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.execute_batch("PRAGMA journal_mode = WAL; PRAGMA foreign_keys = ON; PRAGMA synchronous = NORMAL;")?;
        conn.execute_batch(SCHEMA)?;
        migrate(&conn)?;
        matches::ensure_schema(&conn)?;
        social::ensure_schema(&conn)?;
        friends::ensure_schema(&conn)?;
        admin::ensure_schema(&conn)?;
        Ok(Db { conn: Mutex::new(conn) })
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.conn.lock().unwrap_or_else(|p| p.into_inner())
    }

    // -----------------------------------------------------------------------
    // seeding

    /// Imports the legacy JSON/TXT files the first time the database is used.
    /// Catalogue and players are seeded independently, each only while its
    /// tables are empty, so a fresh database next to old save files picks
    /// everything up and a populated one is left alone.
    ///
    /// The login, channel and admin services all call this at startup, and in
    /// the Docker image they start at the same moment (docker-entrypoint.sh),
    /// so the whole check-and-insert runs in one BEGIN IMMEDIATE transaction:
    /// the first process takes the write lock, the others wait for it (see the
    /// busy timeout in `open`), then read the committed rows and skip. No
    /// process sees an empty table another one is halfway through filling.
    pub fn seed_if_empty(&self) -> Result<SeedReport> {
        let mut report = SeedReport::default();
        let mut conn = self.lock();
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate)?;
        let catalog_empty: i64 = tx.query_row("SELECT count(*) FROM catalog", [], |r| r.get(0))?;
        if catalog_empty == 0 {
            let (c, p) = Self::seed_catalog(&tx)?;
            report.catalog = c;
            report.products = p;
        }
        let players_empty: i64 = tx.query_row("SELECT count(*) FROM players", [], |r| r.get(0))?;
        let accounts_empty: i64 = tx.query_row("SELECT count(*) FROM accounts", [], |r| r.get(0))?;
        if players_empty == 0 && accounts_empty == 0 {
            let (a, p) = Self::seed_players(&tx)?;
            report.accounts = a;
            report.players = p;
        }
        tx.commit()?;
        Ok(report)
    }

    fn seed_catalog(tx: &Transaction<'_>) -> Result<(usize, usize)> {
        #[derive(Deserialize)]
        struct WeaponRow {
            id: i32,
            name: String,
            #[serde(rename = "type")]
            item_type: i32,
        }
        #[derive(Deserialize)]
        struct ShopRow {
            id: i32,
            name: String,
            kind: i32,
            #[serde(default)]
            category: String,
            #[serde(default)]
            sell_price: i32,
            #[serde(default)]
            periods: Vec<i32>,
            #[serde(default)]
            cash: Vec<i32>,
            #[serde(default)]
            points: Vec<i32>,
        }
        #[derive(Deserialize)]
        struct ProductRow {
            id: i32,
            cost: i32,
            #[serde(default = "three")]
            sale_type: u8,
            #[serde(default)]
            hours: i32,
        }
        fn three() -> u8 {
            3
        }
        #[derive(Deserialize)]
        struct BoxFiller {
            id: i32,
            #[serde(default)]
            period: i32,
        }
        #[derive(Deserialize)]
        struct BoxRow {
            #[serde(rename = "box")]
            id: i32,
            #[serde(default)]
            featured: Vec<i32>,
            #[serde(default)]
            items: Vec<BoxFiller>,
            #[serde(default)]
            bonus: Vec<i32>,
        }

        let weapons: Vec<WeaponRow> = read_json_table("ACEMU_ITEMS", "items.json");
        let shop: Vec<ShopRow> = read_json_table("ACEMU_SHOP", "shop.json");
        let products: Vec<ProductRow> = read_json_table("ACEMU_PRODUCTS", "products.json");
        let boxes: Vec<BoxRow> = read_json_table("ACEMU_BOXES", "boxes.json");
        let featured_anywhere: std::collections::HashSet<i32> = boxes.iter().flat_map(|b| b.featured.iter().copied()).collect();

        struct Row {
            name: String,
            item_type: i32,
            kind: i32,
            category: String,
            sell_price: i32,
            avail: Availability,
            order: i32,
            in_table: bool,
            prices: Vec<PriceRow>,
        }
        let mut rows: BTreeMap<i32, Row> = BTreeMap::new();
        for w in weapons {
            rows.insert(
                w.id,
                Row {
                    name: w.name,
                    item_type: w.item_type,
                    kind: -1,
                    category: String::new(),
                    sell_price: 0,
                    avail: Availability::Lootbox,
                    order: 0,
                    in_table: false,
                    prices: vec![],
                },
            );
        }
        for (order, s) in shop.into_iter().enumerate() {
            let prices: Vec<PriceRow> = (0..5)
                .map(|k| PriceRow {
                    idx: 0,
                    hours: s.periods.get(k).copied().unwrap_or(0),
                    points: s.points.get(k).copied().unwrap_or(0),
                    cash: s.cash.get(k).copied().unwrap_or(0),
                })
                .filter(|p| p.points > 0 || p.cash > 0)
                .collect();
            // special items - the spoils tiers and the prizes printed on boxes -
            // are only obtainable from boxes
            let special = LOOTBOX_CATEGORIES.contains(&s.category.as_str()) || featured_anywhere.contains(&s.id);
            let e = rows.entry(s.id).or_insert(Row {
                name: s.name.clone(),
                item_type: s.kind,
                kind: s.kind,
                category: String::new(),
                sell_price: 0,
                avail: Availability::Shop,
                order: 0,
                in_table: true,
                prices: vec![],
            });
            e.kind = s.kind;
            e.category = s.category.clone();
            e.sell_price = s.sell_price;
            e.avail = if special { Availability::Lootbox } else { Availability::Shop };
            e.order = order as i32;
            e.in_table = true;
            e.prices = prices;
        }
        let mut product_count = 0;
        for p in &products {
            if let Some(e) = rows.get_mut(&p.id) {
                if e.prices.is_empty() {
                    e.prices = vec![if p.sale_type == 3 {
                        PriceRow { idx: 0, hours: p.hours, points: p.cost, cash: 0 }
                    } else {
                        PriceRow { idx: 0, hours: p.hours, points: 0, cash: p.cost }
                    }];
                    product_count += 1;
                }
            }
        }
        // Curated availability (data/availability.json, written by
        // hand-curated, see docs/GAME-DATA.md): the house rules for what the shop lists -
        // base weapons only, skins and event gear from boxes, packs and unimplemented
        // services hidden. Applied last so the derived defaults above are overridden
        // and the curation survives a database reset.
        #[derive(Deserialize)]
        struct PriceOverride {
            #[serde(default)]
            hours: i32,
            #[serde(default)]
            points: i32,
            #[serde(default)]
            cash: i32,
        }
        #[derive(Deserialize)]
        struct AvailRow {
            id: i32,
            availability: String,
            /// Curated purchase options (one per BuyOption1); absent = keep the table's.
            #[serde(default)]
            prices: Option<Vec<PriceOverride>>,
            /// Curated resale price; absent = keep the table's.
            #[serde(default)]
            sell_price: Option<i32>,
        }
        let overrides: Vec<AvailRow> = read_json_table("ACEMU_AVAILABILITY", "availability.json");
        let mut overridden = 0;
        for o in &overrides {
            if let Some(e) = rows.get_mut(&o.id) {
                if let Some(a) = Availability::parse(&o.availability) {
                    if e.avail != a {
                        e.avail = a;
                        overridden += 1;
                    }
                }
                if let Some(p) = &o.prices {
                    e.prices = p
                        .iter()
                        .map(|x| PriceRow { idx: 0, hours: x.hours, points: x.points, cash: x.cash })
                        .filter(|x| x.points > 0 || x.cash > 0)
                        .collect();
                }
                if let Some(s) = o.sell_price {
                    e.sell_price = s;
                }
            }
        }
        if !overrides.is_empty() {
            tracing::info!(rows = overrides.len(), overridden, "catalogue availability overrides applied (data/availability.json)");
        }
        let n = rows.len();
        for (id, r) in rows {
            tx.execute(
                "INSERT OR IGNORE INTO catalog (id, name, item_type, kind, category, sell_price, availability, shop_order, in_shop_table)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                params![id, r.name, r.item_type, r.kind, r.category, r.sell_price, r.avail.as_str(), r.order, r.in_table as i32],
            )?;
            for (idx, p) in r.prices.iter().enumerate() {
                tx.execute(
                    "INSERT OR IGNORE INTO prices (item_id, idx, hours, points, cash) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![id, idx as i32, p.hours, p.points, p.cash],
                )?;
            }
        }
        // Boxes: the featured prizes share half the odds, the fillers the other
        // half (the retail odds are unknown; this mirrors what the server did
        // before weights existed). Rental duplicates of a permanent filler start
        // at weight 0 so they stay editable without dropping.
        let mut box_count = 0;
        for b in &boxes {
            let mut entries: Vec<(i32, Reel, f64, bool, i32)> = vec![];
            let fillers: Vec<&BoxFiller> = b.items.iter().filter(|f| !b.featured.contains(&f.id)).collect();
            let has_permanent = fillers.iter().any(|f| f.id < 1_000_000);
            let kept: Vec<&BoxFiller> = fillers.iter().copied().filter(|f| !has_permanent || f.id < 1_000_000).collect();
            let featured_share = if b.featured.is_empty() { 0.0 } else if kept.is_empty() { 100.0 } else { 50.0 };
            let filler_share = 100.0 - featured_share;
            for &id in &b.featured {
                entries.push((id, Reel::Main, featured_share / b.featured.len() as f64, true, 0));
            }
            for f in &fillers {
                let weight = if kept.iter().any(|k| k.id == f.id) { filler_share / kept.len().max(1) as f64 } else { 0.0 };
                let hours = if f.period <= 1 { 0 } else { f.period };
                entries.push((f.id, Reel::Main, weight, false, hours));
            }
            for &id in &b.bonus {
                entries.push((id, Reel::Bonus, 1.0, false, 0));
            }
            for (item_id, reel, weight, featured, hours) in entries {
                tx.execute(
                    "INSERT OR REPLACE INTO box_entries (box_id, item_id, reel, weight, featured, hours) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    params![b.id, item_id, reel.as_str(), weight, featured as i32, hours],
                )?;
            }
            box_count += 1;
        }
        tx.execute("INSERT OR REPLACE INTO meta (key, value) VALUES ('catalog_seeded_at', datetime('now'))", [])?;
        tracing::info!(catalog = n, products = product_count, boxes = box_count, "catalogue and boxes seeded from data/*.json");
        Ok((n, product_count))
    }

    // -----------------------------------------------------------------------
    // random boxes

    /// Every box that has entries, with the catalogue name.
    pub fn boxes(&self) -> Result<Vec<BoxSummary>> {
        let conn = self.lock();
        let mut st = conn.prepare(
            "SELECT e.box_id, COALESCE(c.name, 'box ' || e.box_id),
                    SUM(CASE WHEN e.reel = 'main' THEN 1 ELSE 0 END),
                    SUM(CASE WHEN e.reel = 'bonus' THEN 1 ELSE 0 END)
             FROM box_entries e LEFT JOIN catalog c ON c.id = e.box_id
             GROUP BY e.box_id ORDER BY e.box_id",
        )?;
        let rows = st
            .query_map([], |r| Ok(BoxSummary { id: r.get(0)?, name: r.get(1)?, main_entries: r.get(2)?, bonus_entries: r.get(3)? }))?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn box_entries(&self, box_id: i32) -> Result<Vec<BoxEntryRow>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT item_id, reel, weight, featured, hours FROM box_entries WHERE box_id = ?1 ORDER BY reel, featured DESC, item_id")?;
        let rows = st
            .query_map(params![box_id], |r| {
                let reel: String = r.get(1)?;
                Ok(BoxEntryRow {
                    item_id: r.get(0)?,
                    reel: Reel::parse(&reel).unwrap_or(Reel::Main),
                    weight: r.get(2)?,
                    featured: r.get::<_, i32>(3)? != 0,
                    hours: r.get(4)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    /// Replaces every entry of `box_id`.
    pub fn box_replace(&self, box_id: i32, entries: &[BoxEntryRow]) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM box_entries WHERE box_id = ?1", params![box_id])?;
        for e in entries {
            tx.execute(
                "INSERT OR REPLACE INTO box_entries (box_id, item_id, reel, weight, featured, hours) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![box_id, e.item_id, e.reel.as_str(), e.weight.max(0.0), e.featured as i32, e.hours.max(0)],
            )?;
        }
        tx.commit()?;
        Ok(())
    }

    fn seed_players(tx: &Transaction<'_>) -> Result<(usize, usize)> {
        #[derive(Deserialize)]
        struct JItem {
            unique_id: f64,
            item_id: i32,
            item_type: i32,
            slot: i32,
            stack: i32,
            #[serde(default = "full")]
            durability: i32,
            #[serde(default)]
            add_part_id: i32,
            #[serde(default)]
            paint_id: i32,
            #[serde(default)]
            until: f64,
        }
        fn full() -> i32 {
            100_000
        }
        #[derive(Deserialize)]
        struct JSkill {
            unique_id: i32,
            skill_id: i32,
            slot: i32,
        }
        #[derive(Deserialize)]
        struct JQSlot {
            id: i32,
            name: String,
            main: f64,
            pistol: f64,
            throw1: f64,
            throw2: f64,
            skills: [i32; 4],
        }
        #[derive(Deserialize)]
        struct JInventory {
            points: i32,
            cash: i32,
            next_unique: i64,
            items: Vec<JItem>,
            #[serde(default)]
            skills: Vec<JSkill>,
            qslots: Vec<JQSlot>,
            #[serde(default)]
            stats: Stats,
        }

        let save_dir = acemu_paths::home().join("save");
        // accounts: acemu-account.txt -> "acemu", acemu-account-<id>.txt -> <id>
        let mut accounts: Vec<(String, String)> = vec![];
        let mut default_character = String::new();
        if let Ok(rd) = std::fs::read_dir(&save_dir) {
            for entry in rd.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                let Some(stem) = fname.strip_suffix(".txt") else { continue };
                let username = if stem == "acemu-account" {
                    "acemu".to_string()
                } else if let Some(id) = stem.strip_prefix("acemu-account-") {
                    id.to_string()
                } else {
                    continue;
                };
                let character = std::fs::read_to_string(entry.path()).map(|s| s.trim().to_string()).unwrap_or_default();
                if username == "acemu" {
                    default_character = character.clone();
                }
                accounts.push((username, character));
            }
        }
        // inventories: acemu-inventory.json -> default character, acemu-inventory-<name>.json -> <name>
        let mut players: Vec<(String, JInventory)> = vec![];
        if let Ok(rd) = std::fs::read_dir(&save_dir) {
            for entry in rd.flatten() {
                let fname = entry.file_name().to_string_lossy().to_string();
                let Some(stem) = fname.strip_suffix(".json") else { continue };
                let name = if stem == "acemu-inventory" {
                    if default_character.is_empty() {
                        continue;
                    }
                    default_character.clone()
                } else if let Some(n) = stem.strip_prefix("acemu-inventory-") {
                    n.to_string()
                } else {
                    continue;
                };
                let Ok(text) = std::fs::read_to_string(entry.path()) else { continue };
                match serde_json::from_str::<JInventory>(text.trim_start_matches('\u{feff}')) {
                    Ok(inv) => players.push((name, inv)),
                    Err(e) => tracing::warn!(file = %fname, error = %e, "legacy inventory not imported"),
                }
            }
        }

        let mut n_players = 0;
        for (name, inv) in players {
            let rec = PlayerRecord {
                name: name.clone(),
                points: inv.points,
                cash: inv.cash,
                next_unique: inv.next_unique,
                stats: inv.stats,
                items: inv
                    .items
                    .iter()
                    .map(|i| ItemRow {
                        unique_id: i.unique_id as i64,
                        item_id: i.item_id,
                        item_type: i.item_type,
                        slot: i.slot,
                        stack: i.stack,
                        durability: i.durability,
                        add_part_id: i.add_part_id,
                        paint_id: i.paint_id,
                        until: i.until,
                    })
                    .collect(),
                skills: inv.skills.iter().map(|s| SkillRow { unique_id: s.unique_id as i64, skill_id: s.skill_id, slot: s.slot, until: 0.0 }).collect(),
                qslots: inv
                    .qslots
                    .iter()
                    .map(|q| QSlotRow {
                        id: q.id,
                        name: q.name.clone(),
                        main: q.main as i64,
                        pistol: q.pistol as i64,
                        throw1: q.throw1 as i64,
                        throw2: q.throw2 as i64,
                        skills: q.skills,
                    })
                    .collect(),
            };
            write_player(tx, &rec)?;
            n_players += 1;
        }
        let mut n_accounts = 0;
        for (username, character) in accounts {
            let character = if character.is_empty() { None } else { Some(character) };
            tx.execute(
                "INSERT OR IGNORE INTO accounts (username, password_hash, character) VALUES (?1, '', ?2)",
                params![username, character],
            )?;
            n_accounts += 1;
        }
        tx.execute("INSERT OR REPLACE INTO meta (key, value) VALUES ('players_seeded_at', datetime('now'))", [])?;
        tracing::info!(players = n_players, accounts = n_accounts, "players and accounts seeded from save/*.json|txt");
        Ok((n_accounts, n_players))
    }

    // -----------------------------------------------------------------------
    // accounts

    pub fn accounts(&self) -> Result<Vec<Account>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT id, username, password_hash, character, created_at FROM accounts ORDER BY id")?;
        let rows = st.query_map([], account_from_row)?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn account_by_id(&self, id: i64) -> Result<Option<Account>> {
        Ok(self
            .lock()
            .query_row("SELECT id, username, password_hash, character, created_at FROM accounts WHERE id = ?1", params![id], account_from_row)
            .optional()?)
    }

    pub fn account_by_username(&self, username: &str) -> Result<Option<Account>> {
        Ok(self
            .lock()
            .query_row(
                "SELECT id, username, password_hash, character, created_at FROM accounts WHERE username = ?1",
                params![username],
                account_from_row,
            )
            .optional()?)
    }

    pub fn account_of_character(&self, character: &str) -> Result<Option<Account>> {
        Ok(self
            .lock()
            .query_row(
                "SELECT id, username, password_hash, character, created_at FROM accounts WHERE character = ?1",
                params![character],
                account_from_row,
            )
            .optional()?)
    }

    pub fn create_account(&self, username: &str, password: &str) -> Result<Account> {
        let username = username.trim();
        if username.is_empty() {
            return Err(anyhow!("username must not be empty"));
        }
        let id = {
            let conn = self.lock();
            conn.execute("INSERT INTO accounts (username, password_hash) VALUES (?1, ?2)", params![username, hash_password(password)])
                .map_err(|e| match e {
                    rusqlite::Error::SqliteFailure(f, _) if f.code == rusqlite::ErrorCode::ConstraintViolation => {
                        anyhow!("an account named {username} already exists")
                    }
                    e => e.into(),
                })?;
            conn.last_insert_rowid()
        };
        self.account_by_id(id)?.ok_or_else(|| anyhow!("account vanished"))
    }

    /// The account for `username`, created without a password if unknown.
    pub fn ensure_account(&self, username: &str) -> Result<Account> {
        if let Some(a) = self.account_by_username(username)? {
            return Ok(a);
        }
        self.create_account(username, "")
    }

    /// `password`: `Some("")` clears it. `character`: `Some(None)` unlinks.
    pub fn update_account(
        &self,
        id: i64,
        username: Option<&str>,
        password: Option<&str>,
        character: Option<Option<&str>>,
    ) -> Result<Account> {
        {
            let conn = self.lock();
            if let Some(u) = username {
                let u = u.trim();
                if u.is_empty() {
                    return Err(anyhow!("username must not be empty"));
                }
                conn.execute("UPDATE accounts SET username = ?1 WHERE id = ?2", params![u, id]).map_err(|e| match e {
                    rusqlite::Error::SqliteFailure(f, _) if f.code == rusqlite::ErrorCode::ConstraintViolation => {
                        anyhow!("an account named {u} already exists")
                    }
                    e => e.into(),
                })?;
            }
            if let Some(p) = password {
                conn.execute("UPDATE accounts SET password_hash = ?1 WHERE id = ?2", params![hash_password(p), id])?;
            }
            if let Some(c) = character {
                let c = c.map(str::trim).filter(|c| !c.is_empty());
                conn.execute("UPDATE accounts SET character = ?1 WHERE id = ?2", params![c, id]).map_err(|e| match e {
                    rusqlite::Error::SqliteFailure(f, _) if f.code == rusqlite::ErrorCode::ConstraintViolation => {
                        anyhow!("that character is already linked to another account")
                    }
                    e => e.into(),
                })?;
            }
        }
        self.account_by_id(id)?.ok_or_else(|| anyhow!("no account with id {id}"))
    }

    pub fn set_character(&self, username: &str, character: &str) -> Result<()> {
        let a = self.ensure_account(username)?;
        self.update_account(a.id, None, None, Some(Some(character)))?;
        Ok(())
    }

    pub fn delete_account(&self, id: i64, with_player: bool) -> Result<bool> {
        let Some(a) = self.account_by_id(id)? else { return Ok(false) };
        let conn = self.lock();
        if with_player {
            if let Some(c) = &a.character {
                conn.execute("DELETE FROM players WHERE name = ?1", params![c])?;
            }
        }
        conn.execute("DELETE FROM accounts WHERE id = ?1", params![id])?;
        Ok(true)
    }

    pub fn verify_password(&self, username: &str, secret: &str) -> Result<PasswordCheck> {
        let hash: Option<String> = self
            .lock()
            .query_row("SELECT password_hash FROM accounts WHERE username = ?1", params![username], |r| r.get(0))
            .optional()?;
        Ok(match hash {
            None => PasswordCheck::NoSuchAccount,
            Some(h) if h.is_empty() => PasswordCheck::NoPassword,
            Some(h) if h == hash_password(secret) => PasswordCheck::Ok,
            Some(_) => PasswordCheck::Wrong,
        })
    }

    // -----------------------------------------------------------------------
    // players

    pub fn players(&self) -> Result<Vec<PlayerSummary>> {
        let conn = self.lock();
        let mut st = conn.prepare(
            "SELECT p.name, p.level, p.exp, p.points, p.cash, p.kills, p.deaths, p.wins, p.losses, p.play_time,
                    (SELECT count(*) FROM items i WHERE i.player = p.name),
                    (SELECT a.username FROM accounts a WHERE a.character = p.name)
             FROM players p ORDER BY p.name",
        )?;
        let rows = st
            .query_map([], |r| {
                Ok(PlayerSummary {
                    name: r.get(0)?,
                    level: r.get(1)?,
                    exp: r.get(2)?,
                    points: r.get(3)?,
                    cash: r.get(4)?,
                    kills: r.get(5)?,
                    deaths: r.get(6)?,
                    wins: r.get(7)?,
                    losses: r.get(8)?,
                    play_time: r.get(9)?,
                    item_count: r.get(10)?,
                    account: r.get(11)?,
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(rows)
    }

    pub fn player(&self, name: &str) -> Result<Option<PlayerRecord>> {
        let conn = self.lock();
        let head = conn
            .query_row(
                "SELECT points, cash, next_unique, level, exp, kills, deaths, assists, headshots, team_kills, wins, losses, play_time
                 FROM players WHERE name = ?1",
                params![name],
                |r| {
                    Ok((
                        r.get::<_, i32>(0)?,
                        r.get::<_, i32>(1)?,
                        r.get::<_, i64>(2)?,
                        Stats {
                            level: r.get(3)?,
                            exp: r.get(4)?,
                            kills: r.get(5)?,
                            deaths: r.get(6)?,
                            assists: r.get(7)?,
                            headshots: r.get(8)?,
                            team_kills: r.get(9)?,
                            wins: r.get(10)?,
                            losses: r.get(11)?,
                            play_time: r.get(12)?,
                        },
                    ))
                },
            )
            .optional()?;
        let Some((points, cash, next_unique, stats)) = head else { return Ok(None) };
        let items = conn
            .prepare("SELECT unique_id, item_id, item_type, slot, stack, durability, add_part_id, paint_id, until FROM items WHERE player = ?1 ORDER BY unique_id")?
            .query_map(params![name], item_from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let skills = conn
            .prepare("SELECT unique_id, skill_id, slot, until FROM skills WHERE player = ?1 ORDER BY unique_id")?
            .query_map(params![name], skill_from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        let qslots = conn
            .prepare("SELECT id, name, main, pistol, throw1, throw2, skill1, skill2, skill3, skill4 FROM qslots WHERE player = ?1 ORDER BY id")?
            .query_map(params![name], |r| {
                Ok(QSlotRow {
                    id: r.get(0)?,
                    name: r.get(1)?,
                    main: r.get(2)?,
                    pistol: r.get(3)?,
                    throw1: r.get(4)?,
                    throw2: r.get(5)?,
                    skills: [r.get(6)?, r.get(7)?, r.get(8)?, r.get(9)?],
                })
            })?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(Some(PlayerRecord { name: name.to_string(), points, cash, next_unique, stats, items, skills, qslots }))
    }

    /// Writes the whole record: the player row is upserted and the item,
    /// skill and quick-slot tables are replaced, in one transaction.
    pub fn save_player(&self, rec: &PlayerRecord) -> Result<()> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        write_player(&tx, rec)?;
        tx.commit()?;
        Ok(())
    }

    pub fn delete_player(&self, name: &str) -> Result<bool> {
        let conn = self.lock();
        let n = conn.execute("DELETE FROM players WHERE name = ?1", params![name])?;
        conn.execute("UPDATE accounts SET character = NULL WHERE character = ?1", params![name])?;
        Ok(n > 0)
    }

    pub fn update_balance_and_stats(&self, name: &str, points: Option<i32>, cash: Option<i32>, stats: &StatsPatch) -> Result<Option<PlayerRecord>> {
        let Some(mut rec) = self.player(name)? else { return Ok(None) };
        if let Some(p) = points {
            rec.points = p;
        }
        if let Some(c) = cash {
            rec.cash = c;
        }
        rec.stats.apply(stats);
        self.save_player(&rec)?;
        Ok(Some(rec))
    }

    /// Adds `stack` of `item_id` to `name`'s bag as a new permanent instance.
    pub fn grant_item(&self, name: &str, item_id: i32, item_type: i32, stack: i32, durability: i32) -> Result<Option<ItemRow>> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let next: Option<i64> = tx.query_row("SELECT next_unique FROM players WHERE name = ?1", params![name], |r| r.get(0)).optional()?;
        let Some(uid) = next else { return Ok(None) };
        tx.execute("UPDATE players SET next_unique = next_unique + 1, updated_at = datetime('now') WHERE name = ?1", params![name])?;
        let row = ItemRow { unique_id: uid, item_id, item_type, slot: -1, stack: stack.max(1), durability, add_part_id: 0, paint_id: 0, until: 0.0 };
        tx.execute(
            "INSERT INTO items (player, unique_id, item_id, item_type, slot, stack, durability, add_part_id, paint_id, until)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![name, row.unique_id, row.item_id, row.item_type, row.slot, row.stack, row.durability, row.add_part_id, row.paint_id, row.until],
        )?;
        tx.commit()?;
        Ok(Some(row))
    }

    pub fn grant_skill(&self, name: &str, skill_id: i32) -> Result<Option<SkillRow>> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let next: Option<i64> = tx.query_row("SELECT next_unique FROM players WHERE name = ?1", params![name], |r| r.get(0)).optional()?;
        let Some(uid) = next else { return Ok(None) };
        tx.execute("UPDATE players SET next_unique = next_unique + 1, updated_at = datetime('now') WHERE name = ?1", params![name])?;
        let row = SkillRow { unique_id: uid, skill_id, slot: -1, until: 0.0 };
        tx.execute("INSERT INTO skills (player, unique_id, skill_id, slot, until) VALUES (?1, ?2, ?3, ?4, 0)", params![name, uid, skill_id, -1])?;
        tx.commit()?;
        Ok(Some(row))
    }

    pub fn update_item(&self, name: &str, unique_id: i64, stack: Option<i32>, durability: Option<i32>, slot: Option<i32>) -> Result<Option<ItemRow>> {
        let conn = self.lock();
        if let Some(s) = stack {
            conn.execute("UPDATE items SET stack = ?1 WHERE player = ?2 AND unique_id = ?3", params![s.max(1), name, unique_id])?;
        }
        if let Some(d) = durability {
            conn.execute("UPDATE items SET durability = ?1 WHERE player = ?2 AND unique_id = ?3", params![d.max(0), name, unique_id])?;
        }
        if let Some(s) = slot {
            conn.execute("UPDATE items SET slot = ?1 WHERE player = ?2 AND unique_id = ?3", params![s, name, unique_id])?;
        }
        Ok(conn
            .query_row(
                "SELECT unique_id, item_id, item_type, slot, stack, durability, add_part_id, paint_id, until FROM items WHERE player = ?1 AND unique_id = ?2",
                params![name, unique_id],
                item_from_row,
            )
            .optional()?)
    }

    /// Removes an item instance and clears quick-slot references to it.
    pub fn remove_item(&self, name: &str, unique_id: i64) -> Result<bool> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let n = tx.execute("DELETE FROM items WHERE player = ?1 AND unique_id = ?2", params![name, unique_id])?;
        for col in ["main", "pistol", "throw1", "throw2"] {
            tx.execute(&format!("UPDATE qslots SET {col} = 0 WHERE player = ?1 AND {col} = ?2"), params![name, unique_id])?;
        }
        tx.commit()?;
        Ok(n > 0)
    }

    pub fn remove_skill(&self, name: &str, unique_id: i64) -> Result<bool> {
        let mut conn = self.lock();
        let tx = conn.transaction()?;
        let skill_id: Option<i32> = tx
            .query_row("SELECT skill_id FROM skills WHERE player = ?1 AND unique_id = ?2", params![name, unique_id], |r| r.get(0))
            .optional()?;
        let Some(skill_id) = skill_id else { return Ok(false) };
        tx.execute("DELETE FROM skills WHERE player = ?1 AND unique_id = ?2", params![name, unique_id])?;
        for col in ["skill1", "skill2", "skill3", "skill4"] {
            tx.execute(&format!("UPDATE qslots SET {col} = 0 WHERE player = ?1 AND {col} = ?2"), params![name, skill_id])?;
        }
        tx.commit()?;
        Ok(true)
    }

    // -----------------------------------------------------------------------
    // catalogue

    pub fn catalog_item(&self, id: i32) -> Result<Option<CatalogRow>> {
        let conn = self.lock();
        let row = conn
            .query_row(
                "SELECT id, name, item_type, kind, sell_price, availability, shop_order, in_shop_table, category FROM catalog WHERE id = ?1",
                params![id],
                catalog_from_row,
            )
            .optional()?;
        let Some(mut row) = row else { return Ok(None) };
        row.prices = prices_of(&conn, id)?;
        Ok(Some(row))
    }

    pub fn item_name(&self, id: i32) -> Result<Option<String>> {
        Ok(self.lock().query_row("SELECT name FROM catalog WHERE id = ?1", params![id], |r| r.get(0)).optional()?)
    }

    /// Search by name substring or exact id, optionally filtered by kind and
    /// availability. Returns (total matches, one page).
    pub fn catalog_search(
        &self,
        q: &str,
        kind: Option<i32>,
        availability: Option<Availability>,
        limit: i64,
        offset: i64,
    ) -> Result<(i64, Vec<CatalogRow>)> {
        let conn = self.lock();
        let q = q.trim();
        let id_match: i32 = q.parse().unwrap_or(-1);
        let like = format!("%{q}%");
        let kind_v = kind.unwrap_or(i32::MIN);
        let avail_v = availability.map(|a| a.as_str().to_string()).unwrap_or_default();
        let where_sql = "(?1 = '' OR name LIKE ?2 OR id = ?3) AND (?4 = -2147483648 OR kind = ?4) AND (?5 = '' OR availability = ?5)";
        let total: i64 = conn.query_row(
            &format!("SELECT count(*) FROM catalog WHERE {where_sql}"),
            params![q, like, id_match, kind_v, avail_v],
            |r| r.get(0),
        )?;
        let mut st = conn.prepare(&format!(
            "SELECT id, name, item_type, kind, sell_price, availability, shop_order, in_shop_table, category FROM catalog
             WHERE {where_sql} ORDER BY id LIMIT ?6 OFFSET ?7"
        ))?;
        let mut rows = st
            .query_map(params![q, like, id_match, kind_v, avail_v, limit.clamp(1, 500), offset.max(0)], catalog_from_row)?
            .collect::<rusqlite::Result<Vec<_>>>()?;
        for r in &mut rows {
            r.prices = prices_of(&conn, r.id)?;
        }
        Ok((total, rows))
    }

    /// Adds a catalogue row. `None` when that id is already in the catalogue.
    ///
    /// This is how a server without the game's own tables gets a shop: add the
    /// ids you want to sell, with a type and a price. `shop_order` follows the
    /// highest one in use, so hand-added items list after any seeded ones.
    pub fn catalog_create(&self, new: &CatalogNew) -> Result<Option<CatalogRow>> {
        {
            let mut conn = self.lock();
            let tx = conn.transaction()?;
            let exists: Option<i32> = tx.query_row("SELECT id FROM catalog WHERE id = ?1", params![new.id], |r| r.get(0)).optional()?;
            if exists.is_some() {
                return Ok(None);
            }
            let order: i32 = tx.query_row("SELECT coalesce(max(shop_order), 0) + 1 FROM catalog", [], |r| r.get(0))?;
            let name = if new.name.trim().is_empty() { format!("Item {}", new.id) } else { new.name.trim().to_string() };
            let avail = new.availability.unwrap_or(Availability::Shop);
            tx.execute(
                "INSERT INTO catalog (id, name, item_type, kind, category, sell_price, availability, shop_order, in_shop_table)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 1)",
                params![
                    new.id,
                    name,
                    new.item_type,
                    new.kind.unwrap_or(new.item_type),
                    new.category.trim(),
                    new.sell_price.max(0),
                    avail.as_str(),
                    order
                ],
            )?;
            for (idx, p) in new.prices.iter().enumerate() {
                tx.execute(
                    "INSERT INTO prices (item_id, idx, hours, points, cash) VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![new.id, idx as i32, p.hours.max(0), p.points.max(0), p.cash.max(0)],
                )?;
            }
            tx.commit()?;
        }
        self.catalog_item(new.id)
    }

    pub fn catalog_update(&self, id: i32, patch: &CatalogPatch) -> Result<Option<CatalogRow>> {
        {
            let mut conn = self.lock();
            let tx = conn.transaction()?;
            let exists: Option<i32> = tx.query_row("SELECT id FROM catalog WHERE id = ?1", params![id], |r| r.get(0)).optional()?;
            if exists.is_none() {
                return Ok(None);
            }
            if let Some(n) = &patch.name {
                tx.execute("UPDATE catalog SET name = ?1 WHERE id = ?2", params![n.trim(), id])?;
            }
            if let Some(s) = patch.sell_price {
                tx.execute("UPDATE catalog SET sell_price = ?1 WHERE id = ?2", params![s.max(0), id])?;
            }
            if let Some(a) = patch.availability {
                tx.execute("UPDATE catalog SET availability = ?1 WHERE id = ?2", params![a.as_str(), id])?;
            }
            if let Some(prices) = &patch.prices {
                tx.execute("DELETE FROM prices WHERE item_id = ?1", params![id])?;
                for (idx, p) in prices.iter().enumerate() {
                    tx.execute(
                        "INSERT INTO prices (item_id, idx, hours, points, cash) VALUES (?1, ?2, ?3, ?4, ?5)",
                        params![id, idx as i32, p.hours.max(0), p.points.max(0), p.cash.max(0)],
                    )?;
                }
            }
            tx.commit()?;
        }
        self.catalog_item(id)
    }

    /// Ids the shop lists, in the client's shop-table order.
    pub fn shop_ids(&self) -> Result<Vec<i32>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT id FROM catalog WHERE availability = 'shop' ORDER BY in_shop_table DESC, shop_order, id")?;
        let ids = st.query_map([], |r| r.get(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(ids)
    }

    /// Obtainable ids of one kind (shop or lootbox availability).
    pub fn obtainable_ids_by_kind(&self, kind: i32) -> Result<Vec<i32>> {
        let conn = self.lock();
        let mut st = conn.prepare("SELECT id FROM catalog WHERE kind = ?1 AND availability <> 'hidden' ORDER BY id")?;
        let ids = st.query_map(params![kind], |r| r.get(0))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(ids)
    }

    pub fn is_hidden(&self, id: i32) -> Result<bool> {
        let a: Option<String> = self.lock().query_row("SELECT availability FROM catalog WHERE id = ?1", params![id], |r| r.get(0)).optional()?;
        Ok(a.as_deref() == Some("hidden"))
    }

    // -----------------------------------------------------------------------
    // live state (published by the channel server, read by the admin)

    /// Stores a live-state value (JSON text) under `key`, stamping the time.
    pub fn set_live(&self, key: &str, value: &str) -> Result<()> {
        self.lock().execute(
            "INSERT INTO live (key, value, updated_at) VALUES (?1, ?2, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = excluded.updated_at",
            params![key, value],
        )?;
        Ok(())
    }

    /// A live-state value and when it was written (UTC, `YYYY-MM-DD HH:MM:SS`),
    /// plus the seconds since then.
    pub fn get_live(&self, key: &str) -> Result<Option<(String, String, i64)>> {
        Ok(self
            .lock()
            .query_row(
                "SELECT value, updated_at, CAST(strftime('%s','now') AS INTEGER) - CAST(strftime('%s', updated_at) AS INTEGER) FROM live WHERE key = ?1",
                params![key],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .optional()?)
    }

    pub fn summary(&self) -> Result<Summary> {
        let conn = self.lock();
        let count = |sql: &str| -> Result<i64> { Ok(conn.query_row(sql, [], |r| r.get(0))?) };
        Ok(Summary {
            players: count("SELECT count(*) FROM players")?,
            accounts: count("SELECT count(*) FROM accounts")?,
            items: count("SELECT count(*) FROM items")?,
            catalog: count("SELECT count(*) FROM catalog")?,
            shop: count("SELECT count(*) FROM catalog WHERE availability = 'shop'")?,
            matches: count("SELECT count(*) FROM matches")?,
        })
    }
}

// ---------------------------------------------------------------------------
// helpers

fn read_json_table<T: serde::de::DeserializeOwned>(env: &str, file: &str) -> Vec<T> {
    let path = std::env::var(env).map(PathBuf::from).unwrap_or_else(|_| acemu_paths::data_file(file));
    match std::fs::read_to_string(&path) {
        Ok(s) => match serde_json::from_str::<Vec<T>>(s.trim_start_matches('\u{feff}')) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(path = %path.display(), error = %e, "table unreadable - not seeded");
                vec![]
            }
        },
        Err(e) => {
            tracing::warn!(path = %path.display(), error = %e, "table missing - not seeded");
            vec![]
        }
    }
}

fn write_player(tx: &Transaction<'_>, rec: &PlayerRecord) -> Result<()> {
    let s = &rec.stats;
    tx.execute(
        "INSERT INTO players (name, points, cash, next_unique, level, exp, kills, deaths, assists, headshots, team_kills, wins, losses, play_time)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
         ON CONFLICT(name) DO UPDATE SET
           points = excluded.points, cash = excluded.cash, next_unique = excluded.next_unique,
           level = excluded.level, exp = excluded.exp, kills = excluded.kills, deaths = excluded.deaths,
           assists = excluded.assists, headshots = excluded.headshots, team_kills = excluded.team_kills,
           wins = excluded.wins, losses = excluded.losses, play_time = excluded.play_time,
           updated_at = datetime('now')",
        params![
            rec.name, rec.points, rec.cash, rec.next_unique, s.level, s.exp, s.kills, s.deaths, s.assists, s.headshots, s.team_kills,
            s.wins, s.losses, s.play_time
        ],
    )?;
    tx.execute("DELETE FROM items WHERE player = ?1", params![rec.name])?;
    tx.execute("DELETE FROM skills WHERE player = ?1", params![rec.name])?;
    tx.execute("DELETE FROM qslots WHERE player = ?1", params![rec.name])?;
    for i in &rec.items {
        tx.execute(
            "INSERT OR REPLACE INTO items (player, unique_id, item_id, item_type, slot, stack, durability, add_part_id, paint_id, until)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![rec.name, i.unique_id, i.item_id, i.item_type, i.slot, i.stack, i.durability, i.add_part_id, i.paint_id, i.until],
        )?;
    }
    for sk in &rec.skills {
        tx.execute(
            "INSERT OR REPLACE INTO skills (player, unique_id, skill_id, slot, until) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![rec.name, sk.unique_id, sk.skill_id, sk.slot, sk.until],
        )?;
    }
    for q in &rec.qslots {
        tx.execute(
            "INSERT OR REPLACE INTO qslots (player, id, name, main, pistol, throw1, throw2, skill1, skill2, skill3, skill4)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![rec.name, q.id, q.name, q.main, q.pistol, q.throw1, q.throw2, q.skills[0], q.skills[1], q.skills[2], q.skills[3]],
        )?;
    }
    Ok(())
}

fn account_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<Account> {
    let hash: String = r.get(2)?;
    Ok(Account { id: r.get(0)?, username: r.get(1)?, has_password: !hash.is_empty(), character: r.get(3)?, created_at: r.get(4)? })
}

fn item_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<ItemRow> {
    Ok(ItemRow {
        unique_id: r.get(0)?,
        item_id: r.get(1)?,
        item_type: r.get(2)?,
        slot: r.get(3)?,
        stack: r.get(4)?,
        durability: r.get(5)?,
        add_part_id: r.get(6)?,
        paint_id: r.get(7)?,
        until: r.get(8)?,
    })
}

fn skill_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<SkillRow> {
    Ok(SkillRow { unique_id: r.get(0)?, skill_id: r.get(1)?, slot: r.get(2)?, until: r.get(3)? })
}

/// Columns added after the first databases were created (`CREATE TABLE IF NOT
/// EXISTS` leaves existing tables alone, so they are added here).
fn migrate(conn: &Connection) -> Result<()> {
    let has_column = |table: &str, col: &str| -> Result<bool> {
        let mut st = conn.prepare(&format!("PRAGMA table_info({table})"))?;
        let names = st.query_map([], |r| r.get::<_, String>(1))?.collect::<rusqlite::Result<Vec<_>>>()?;
        Ok(names.iter().any(|n| n == col))
    };
    if !has_column("skills", "until")? {
        conn.execute_batch("ALTER TABLE skills ADD COLUMN until REAL NOT NULL DEFAULT 0")?;
    }
    Ok(())
}

fn catalog_from_row(r: &rusqlite::Row<'_>) -> rusqlite::Result<CatalogRow> {
    let avail: String = r.get(5)?;
    Ok(CatalogRow {
        id: r.get(0)?,
        name: r.get(1)?,
        item_type: r.get(2)?,
        kind: r.get(3)?,
        sell_price: r.get(4)?,
        availability: Availability::parse(&avail).unwrap_or(Availability::Hidden),
        shop_order: r.get(6)?,
        in_shop_table: r.get::<_, i32>(7)? != 0,
        category: r.get(8)?,
        prices: vec![],
    })
}

fn prices_of(conn: &Connection, id: i32) -> Result<Vec<PriceRow>> {
    let mut st = conn.prepare("SELECT idx, hours, points, cash FROM prices WHERE item_id = ?1 ORDER BY idx")?;
    let rows = st
        .query_map(params![id], |r| Ok(PriceRow { idx: r.get(0)?, hours: r.get(1)?, points: r.get(2)?, cash: r.get(3)? }))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh() -> Db {
        Db::open(Path::new(":memory:")).unwrap()
    }

    #[test]
    fn accounts_round_trip_with_passwords() {
        let db = fresh();
        let a = db.create_account("maxa", "secret").unwrap();
        assert!(a.has_password);
        assert_eq!(db.verify_password("maxa", "secret").unwrap(), PasswordCheck::Ok);
        assert_eq!(db.verify_password("maxa", "nope").unwrap(), PasswordCheck::Wrong);
        assert_eq!(db.verify_password("nobody", "x").unwrap(), PasswordCheck::NoSuchAccount);
        db.update_account(a.id, None, Some(""), Some(Some("MaxA"))).unwrap();
        assert_eq!(db.verify_password("maxa", "anything").unwrap(), PasswordCheck::NoPassword);
        assert_eq!(db.account_of_character("MaxA").unwrap().unwrap().username, "maxa");
        assert!(db.create_account("maxa", "").is_err());
    }

    #[test]
    fn players_save_and_load() {
        let db = fresh();
        let rec = PlayerRecord {
            name: "MaxA".into(),
            points: 5,
            cash: 6,
            next_unique: 100_000_003,
            stats: Stats { level: 2, kills: 7, ..Default::default() },
            items: vec![ItemRow { unique_id: 100_000_001, item_id: 2000, item_type: 1, slot: 0, stack: 1, durability: 100_000, add_part_id: 0, paint_id: 0, until: 0.0 }],
            skills: vec![SkillRow { unique_id: 100_000_002, skill_id: 30001, slot: -1, until: 0.0 }],
            qslots: vec![QSlotRow { id: 0, name: "Slot 1".into(), main: 100_000_001, pistol: 0, throw1: 0, throw2: 0, skills: [30001, 0, 0, 0] }],
        };
        db.save_player(&rec).unwrap();
        let back = db.player("MaxA").unwrap().unwrap();
        assert_eq!(back.items.len(), 1);
        assert_eq!(back.stats.kills, 7);
        let granted = db.grant_item("MaxA", 1010, 2, 1, 100_000).unwrap().unwrap();
        assert_eq!(granted.unique_id, 100_000_003);
        assert!(db.remove_item("MaxA", 100_000_001).unwrap());
        let back = db.player("MaxA").unwrap().unwrap();
        assert_eq!(back.qslots[0].main, 0, "quick slot reference cleared");
        assert!(db.remove_skill("MaxA", 100_000_002).unwrap());
        assert_eq!(db.player("MaxA").unwrap().unwrap().qslots[0].skills[0], 0);
        assert_eq!(db.players().unwrap().len(), 1);
        assert!(db.delete_player("MaxA").unwrap());
        assert!(db.player("MaxA").unwrap().is_none());
    }

    #[test]
    fn catalog_availability_and_prices() {
        let db = fresh();
        {
            let conn = db.lock();
            conn.execute("INSERT INTO catalog (id, name, item_type, kind, availability, shop_order, in_shop_table) VALUES (2000, 'G3', 1, 1, 'shop', 3, 1)", []).unwrap();
            conn.execute("INSERT INTO catalog (id, name, item_type, kind, availability, shop_order, in_shop_table) VALUES (2001, 'G3 Gold', 1, -1, 'lootbox', 0, 0)", []).unwrap();
        }
        // boxes
        db.box_replace(
            17071,
            &[
                BoxEntryRow { item_id: 2001, reel: Reel::Main, weight: 25.0, featured: true, hours: 0 },
                BoxEntryRow { item_id: 2000, reel: Reel::Main, weight: 75.0, featured: false, hours: 168 },
                BoxEntryRow { item_id: 17003, reel: Reel::Bonus, weight: 1.0, featured: false, hours: 0 },
            ],
        )
        .unwrap();
        let entries = db.box_entries(17071).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries.iter().filter(|e| e.reel == Reel::Main).map(|e| e.weight).sum::<f64>(), 100.0);
        let boxes = db.boxes().unwrap();
        assert_eq!(boxes.len(), 1);
        assert_eq!((boxes[0].main_entries, boxes[0].bonus_entries), (2, 1));
        assert_eq!(db.shop_ids().unwrap(), vec![2000]);
        assert_eq!(db.obtainable_ids_by_kind(1).unwrap(), vec![2000]);
        let patch = CatalogPatch {
            availability: Some(Availability::Hidden),
            prices: Some(vec![PriceRow { idx: 0, hours: 0, points: 1234, cash: 0 }]),
            ..Default::default()
        };
        let row = db.catalog_update(2000, &patch).unwrap().unwrap();
        assert_eq!(row.prices[0].points, 1234);
        assert!(db.shop_ids().unwrap().is_empty());
        assert!(db.is_hidden(2000).unwrap());
        let (total, rows) = db.catalog_search("g3", None, None, 10, 0).unwrap();
        assert_eq!(total, 2);
        assert_eq!(rows.len(), 2);
        let (total, _) = db.catalog_search("", Some(1), Some(Availability::Hidden), 10, 0).unwrap();
        assert_eq!(total, 1);
    }

    /// All three services call `seed_if_empty` as they start, and in the Docker
    /// image they start at the same moment: exactly one fills the catalogue and
    /// the others find it already there instead of colliding on `catalog.id`.
    #[test]
    fn parallel_seeding_fills_the_catalogue_once() {
        let dir = std::env::temp_dir().join(format!("acemu-seed-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let items = dir.join("items.json");
        let shop = dir.join("shop.json");
        std::fs::write(&items, r#"[{"id":2000,"name":"G3","type":1},{"id":2001,"name":"G3 Gold","type":1}]"#).unwrap();
        std::fs::write(&shop, r#"[{"id":2000,"name":"G3","kind":1,"category":"Rifle","sell_price":10,"periods":[168],"points":[900],"cash":[0]}]"#).unwrap();
        std::env::set_var("ACEMU_ITEMS", &items);
        std::env::set_var("ACEMU_SHOP", &shop);
        std::env::set_var("ACEMU_PRODUCTS", dir.join("absent.json"));
        std::env::set_var("ACEMU_BOXES", dir.join("absent.json"));
        std::env::set_var("ACEMU_AVAILABILITY", dir.join("absent.json"));
        let path = dir.join("parallel.db");

        let seeded: Vec<usize> = std::thread::scope(|s| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let path = path.clone();
                    s.spawn(move || Db::open(&path).unwrap().seed_if_empty().unwrap().catalog)
                })
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        assert_eq!(seeded.iter().filter(|&&n| n > 0).count(), 1, "exactly one of them seeds, got {seeded:?}");

        let db = Db::open(&path).unwrap();
        assert_eq!(db.summary().unwrap().catalog, 2, "both items, each exactly once");
        assert_eq!(db.catalog_item(2000).unwrap().unwrap().prices.len(), 1);
        drop(db);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
