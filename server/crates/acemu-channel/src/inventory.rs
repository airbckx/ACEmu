//! Item catalogue and the single account's inventory.
//!
//! The client owns the item *definitions*: names, prices, stats and models come
//! from its `System\Hong_GP.ebpx` parameter package (see `tools/ac-items.ps1`,
//! which extracts the tables into `server/data/*.json`). The server only deals
//! in item ids and per-instance state, so that is all this holds.
//!
//! Storage is the SQLite database `save/acemu.db` (`acemu_db`): the catalogue
//! with prices and shop availability (seeded from the JSON tables on first
//! run, editable through the admin service) and every player's inventory.
//! Nothing is cached across requests, so admin edits are live at once.
//!
//! Slot positions follow what the client itself sends in `rfReqEquipItem`: 0 for
//! the main weapon (G3), 1 for the pistol (Glock). Melee and throwables are
//! assumed to be 2 and 3.

use acemu_db::model as db_model;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

static DB: OnceLock<acemu_db::Db> = OnceLock::new();

/// The shared database, opened on first use.
pub fn db() -> &'static acemu_db::Db {
    DB.get_or_init(|| acemu_db::Db::open_default().expect("cannot open save/acemu.db"))
}

/// One catalogue entry: a row of the client's weapon table, its shop table, or
/// both (see `tools/ac-items.ps1`).
#[derive(Debug, Clone, Deserialize)]
pub struct CatalogItem {
    pub id: i32,
    pub name: String,
    /// Weapon-table type (`TYPE_*`); for gear/skills/consumables, which the
    /// weapon table does not know, this is the shop-table `kind` instead.
    #[serde(rename = "type")]
    pub item_type: i32,
    /// Shop-table kind (`KIND_*`), -1 when the item is not in the shop.
    #[serde(default = "no_kind")]
    pub kind: i32,
    /// Shop prices, one column per `BuyOption1` choice (see `Price`).
    #[serde(default)]
    pub prices: Vec<Price>,
    /// What the shop pays when the item is sold back.
    #[serde(default)]
    pub sell_price: i32,
}

fn no_kind() -> i32 {
    -1
}

/// One purchase option of a shop row: a duration in hours (0 = permanent) and
/// its price in points or cash. The client shows exactly these numbers.
#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct Price {
    pub hours: i32,
    pub points: i32,
    pub cash: i32,
}

/// Item types as they appear in the client's weapon table.
pub const TYPE_MAIN: i32 = 1;
pub const TYPE_PISTOL: i32 = 2;
pub const TYPE_MELEE: i32 = 3;
pub const TYPE_THROW: i32 = 5;

/// Kinds as they appear in the client's shop table.
pub const KIND_PISTOL: i32 = 0;
pub const KIND_MAIN: i32 = 1;
pub const KIND_THROW: i32 = 3;
pub const KIND_SKILL: i32 = 15;

pub fn slot_for_type(item_type: i32) -> i32 {
    match item_type {
        TYPE_MAIN => 0,
        TYPE_PISTOL => 1,
        TYPE_MELEE | 11 => 2,
        TYPE_THROW => 3,
        _ => -1,
    }
}

/// Reads a table from `data/<default>`, or from the path in `env` when set.
/// Reads a table from `data/<file>` (or the path in `env`); empty on error.
pub fn read_json_table<T: serde::de::DeserializeOwned>(env: &str, file: &str, what: &str) -> Vec<T> {
    read_json(env, file, what)
}

fn read_json<T: serde::de::DeserializeOwned>(env: &str, default: &str, what: &str) -> Vec<T> {
    let path = std::env::var(env).unwrap_or_else(|_| acemu_paths::data_file(default).display().to_string());
    match std::fs::read_to_string(&path) {
        Ok(s) => match serde_json::from_str::<Vec<T>>(s.trim_start_matches('\u{feff}')) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(path, error = %e, "{what} unreadable");
                Vec::new()
            }
        },
        Err(e) => {
            tracing::warn!(path, error = %e, "no {what}");
            Vec::new()
        }
    }
}

/// Opens the database, seeds it from the JSON tables and legacy save files if
/// it is empty, and loads the cash-shop product feed (`ACEMU_PRODUCTS`; see
/// tools/ac-products.ps1). The catalogue itself (names, kinds, prices, shop
/// availability) is read from the database on demand.
pub fn load_catalog() {
    let db = db();
    match db.seed_if_empty() {
        Ok(r) if r.catalog > 0 || r.players > 0 || r.accounts > 0 => {
            tracing::info!(catalog = r.catalog, products = r.products, players = r.players, accounts = r.accounts, "database seeded from data/ and save/")
        }
        Ok(_) => {}
        Err(e) => tracing::warn!(error = %e, "database seeding failed"),
    }
    let products = read_json::<Product>("ACEMU_PRODUCTS", PRODUCTS_FILE, "product feed");
    let shop = db.shop_ids().map(|v| v.len()).unwrap_or(0);
    let total = db.summary().map(|s| s.catalog).unwrap_or(0);
    tracing::info!(shop, products = products.len(), total, "item catalogue ready (save/acemu.db)");
    if shop == 0 {
        tracing::warn!("no items with shop availability - the shop will be empty");
    }
    *PRODUCTS.lock().unwrap() = products;
}

/// One row of the cash-shop product feed (`products.json`): what
/// `rfReqNotifyShopItemData` announces for items the shop table has no price for.
#[derive(Debug, Clone, Deserialize)]
pub struct Product {
    /// `ProductCode` on the wire: the item id.
    pub id: i32,
    pub name: String,
    pub cost: i32,
    /// `SaleType`, per the client's eExpiredBuyType: 1 W Coin(C), 2 W Coin(P), 3 Points.
    #[serde(default = "sale_type_points")]
    pub sale_type: u8,
    /// Rental hours, 0 = permanent.
    #[serde(default)]
    pub hours: i32,
}

fn sale_type_points() -> u8 {
    3
}

pub const SALE_TYPE_POINTS: u8 = 3;

impl Product {
    fn price(&self) -> Price {
        if self.sale_type == SALE_TYPE_POINTS {
            Price { hours: self.hours, points: self.cost, cash: 0 }
        } else {
            Price { hours: self.hours, points: 0, cash: self.cost }
        }
    }
}

const PRODUCTS_FILE: &str = "products.json";
static PRODUCTS: Mutex<Vec<Product>> = Mutex::new(Vec::new());

/// `ProductNo` announced for the product at `index` of `products.json`; the
/// client hands it back in `rfReqBuyItemByWebzenCashGP20`. Stable per file
/// position, whatever the catalogue filters out.
pub fn product_no(index: usize) -> i32 {
    1000 + index as i32
}

/// One entry of the feed as announced: the file row, its number, and the
/// price actually charged.
#[derive(Debug, Clone)]
pub struct FeedProduct {
    pub no: i32,
    pub id: i32,
    pub name: String,
    pub cost: i32,
    pub sale_type: u8,
    pub hours: i32,
}

fn feed_product(index: usize, p: &Product) -> Option<FeedProduct> {
    // The catalogue decides: rows the admin marked lootbox-only or hidden are
    // not offered, and a price edited there replaces the file's default.
    let row = db().catalog_item(p.id).ok().flatten()?;
    if row.availability != db_model::Availability::Shop {
        return None;
    }
    let (cost, sale_type, hours) = match row.prices.first() {
        Some(pr) if pr.cash > 0 => (pr.cash, SALE_TYPE_PCOIN, pr.hours),
        Some(pr) if pr.points > 0 => (pr.points, SALE_TYPE_POINTS, pr.hours),
        _ => (p.cost, p.sale_type, p.hours),
    };
    Some(FeedProduct { no: product_no(index), id: p.id, name: row.name, cost, sale_type, hours })
}

pub const SALE_TYPE_PCOIN: u8 = 2;

/// The product feed: `products.json` rows that the catalogue still offers.
pub fn products() -> Vec<FeedProduct> {
    let list = PRODUCTS.lock().unwrap().clone();
    list.iter().enumerate().filter_map(|(i, p)| feed_product(i, p)).collect()
}

/// The product a `ProductNo` refers to.
pub fn product_by_no(product_no: i32) -> Option<FeedProduct> {
    let idx = usize::try_from(product_no.checked_sub(1000)?).ok()?;
    let p = PRODUCTS.lock().unwrap().get(idx).cloned()?;
    feed_product(idx, &p)
}

/// Shop ids for the point shop list: everything in the shop table except the
/// rows sold through the product feed.
pub fn shop_ids_for_points() -> Vec<i32> {
    let feed: std::collections::HashSet<i32> = PRODUCTS.lock().unwrap().iter().map(|p| p.id).collect();
    shop_ids().into_iter().filter(|id| !feed.contains(id)).collect()
}

/// Every id the shop offers (weapons, gear, skills and consumables): the
/// catalogue rows with shop availability, in shop-table order.
pub fn shop_ids() -> Vec<i32> {
    db().shop_ids().unwrap_or_else(|e| {
        tracing::warn!(error = %e, "shop ids unavailable");
        vec![]
    })
}

/// One row of the client's promotion table (`promotions.json`, class
/// wPromotionParam / BT_Promotion): what a player receives on reaching `level`.
/// Level 1 is the starter kit; later levels are rentals (`hours` 360/720/1080).
#[derive(Debug, Clone, Deserialize)]
struct PromotionRow {
    level: i32,
    item: i32,
    #[serde(default)]
    hours: i32,
}

const PROMOTIONS_FILE: &str = "promotions.json";
static PROMOTIONS: Mutex<Option<Vec<PromotionRow>>> = Mutex::new(None);

fn promotions_for(level: i32) -> Vec<(i32, i32)> {
    let mut g = PROMOTIONS.lock().unwrap();
    let rows = g.get_or_insert_with(|| {
        let rows = read_json::<PromotionRow>("ACEMU_PROMOTIONS", PROMOTIONS_FILE, "promotion table");
        tracing::info!(rows = rows.len(), "promotion table loaded");
        rows
    });
    rows.iter().filter(|r| r.level == level).map(|r| (r.item, r.hours)).collect()
}

/// One reward handed out by `grant_promotions`.
#[derive(Debug, Clone)]
pub struct Granted {
    /// Unique id of the new instance (skills: their int id as f64).
    pub unique_id: f64,
    pub item_id: i32,
    /// Rental hours, 0 = permanent; goes out as `PartID` in the level-up packets.
    pub hours: i32,
}

/// Grants the promotion-table rewards for reaching `level` (skills go to the
/// skill list, everything else to the bag as a rental when the row says so).
/// Permanent rewards the player already owns are not duplicated.
pub fn grant_promotions(name: &str, level: i32) -> Vec<Granted> {
    let rewards = promotions_for(level);
    if rewards.is_empty() {
        return vec![];
    }
    with_named(name, |inv| {
        let mut out = vec![];
        for (item_id, hours) in rewards {
            let Some(cat) = catalog_item(item_id) else { continue };
            if hours == 0 && inv.items.iter().any(|i| i.item_id == item_id) {
                continue;
            }
            let unique_id = if cat.kind == KIND_SKILL {
                if inv.skills.iter().any(|s| s.skill_id == item_id) {
                    continue;
                }
                inv.add_skill(item_id, hours) as f64
            } else {
                inv.add_rental(item_id, cat.item_type, hours)
            };
            out.push(Granted { unique_id, item_id, hours });
        }
        tracing::info!(name, level, granted = ?out.iter().map(|g| g.item_id).collect::<Vec<_>>(), "promotion rewards");
        let changed = !out.is_empty();
        (out, changed)
    })
}

/// One pick offered by an item box / coupon (`itemboxes.json`, BT_ItemBox).
#[derive(Debug, Clone, Deserialize)]
struct ItemBoxChoice {
    id: i32,
    #[serde(default)]
    part: i32,
}

#[derive(Debug, Clone, Deserialize)]
struct ItemBoxRow {
    #[serde(rename = "box")]
    id: i32,
    #[serde(default)]
    choices: Vec<ItemBoxChoice>,
}

const ITEMBOXES_FILE: &str = "itemboxes.json";
static ITEMBOXES: Mutex<Option<BTreeMap<i32, Vec<(i32, i32)>>>> = Mutex::new(None);

/// The (item id, part id) picks a coupon/item box offers; empty when unknown.
pub fn item_box_choices(box_id: i32) -> Vec<(i32, i32)> {
    let mut g = ITEMBOXES.lock().unwrap();
    let map = g.get_or_insert_with(|| {
        let rows = read_json::<ItemBoxRow>("ACEMU_ITEMBOXES", ITEMBOXES_FILE, "item-box table");
        tracing::info!(boxes = rows.len(), "item-box table loaded");
        rows.into_iter().map(|r| (r.id, r.choices.iter().map(|c| (c.id, c.part)).collect())).collect()
    });
    map.get(&box_id).cloned().unwrap_or_default()
}

/// Redeems one owned coupon/item box for the chosen item. Returns the new
/// instance's unique id and how many of the box are left; None when the box is
/// not owned or the choice is not on its list.
pub fn take_from_box(name: &str, box_unique_id: f64, item_id: i32) -> Option<(f64, i32)> {
    with_named(name, |inv| {
        let Some(pos) = inv.items.iter().position(|i| i.unique_id == box_unique_id) else {
            return (None, false);
        };
        let box_item_id = inv.items[pos].item_id;
        let Some(&(_, second)) = item_box_choices(box_item_id).iter().find(|&&(id, _)| id == item_id) else {
            tracing::warn!(box_item_id, item_id, "choice not in the box's list");
            return (None, false);
        };
        // The pair's second value is a rental period in hours for gear coupons
        // (72 = 3 days, 168 = 7 days); other values are part/group ids, ignored.
        const PERIODS: [i32; 7] = [24, 72, 168, 360, 720, 1080, 2160];
        let hours = if PERIODS.contains(&second) { second } else { 0 };
        let item_type = catalog_item(item_id).map(|c| c.item_type).unwrap_or(TYPE_MAIN);
        if inv.items[pos].stack > 1 {
            inv.items[pos].stack -= 1;
        } else {
            inv.items.remove(pos);
        }
        let remaining = inv.items.iter().filter(|i| i.item_id == box_item_id).map(|i| i.stack).sum();
        let uid = inv.add_rental(item_id, item_type, hours);
        tracing::info!(box_item_id, item_id, uid, remaining, "item box redeemed");
        (Some((uid, remaining)), true)
    })
}

/// Draws one entry of a reel by weight (entries with weight 0 never drop).
/// Returns (item id, rental hours).
fn draw_weighted(entries: &[db_model::BoxEntryRow]) -> Option<(i32, i32)> {
    let total: f64 = entries.iter().map(|e| e.weight.max(0.0)).sum();
    if total <= 0.0 {
        return None;
    }
    let mut r = roll(1_000_000) as f64 / 1_000_000.0 * total;
    for e in entries {
        let w = e.weight.max(0.0);
        if w <= 0.0 {
            continue;
        }
        if r < w {
            return Some((e.item_id, e.hours));
        }
        r -= w;
    }
    entries.iter().rev().find(|e| e.weight > 0.0).map(|e| (e.item_id, e.hours))
}

/// Per-weapon default part set (`weapon-parts.json`, class wWeaponCustomInfoParam).
#[derive(Debug, Clone, Deserialize)]
struct WeaponPartsRow {
    id: i32,
    default_part: i32,
}

const WEAPON_PARTS_FILE: &str = "weapon-parts.json";
static WEAPON_PARTS: Mutex<Option<BTreeMap<i32, i32>>> = Mutex::new(None);

/// The unioned part id a weapon ships with (sight | barrel << 10 | silencer << 17
/// | gunstock << 24, as the client's `wWeapon.AddPartsGroup` decodes it). The
/// client has no table for this - the retail server sent it with every item -
/// so without it gold/dragon/custom models spawn with the stock attachments.
pub fn default_part_id(item_id: i32) -> i32 {
    let mut g = WEAPON_PARTS.lock().unwrap();
    let map = g.get_or_insert_with(|| {
        let rows = read_json::<WeaponPartsRow>("ACEMU_WEAPON_PARTS", WEAPON_PARTS_FILE, "weapon parts table");
        tracing::info!(weapons = rows.len(), "weapon parts table loaded");
        rows.into_iter().map(|r| (r.id, r.default_part)).collect()
    });
    map.get(&item_id).copied().unwrap_or(0)
}

/// Cheap PRNG; nothing here needs to be unpredictable.
fn roll(n: usize) -> usize {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos()).unwrap_or(1);
    let mut x = (t as u64) | 1;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    (x % n.max(1) as u64) as usize
}

/// What opening a box produced.
#[derive(Debug, Clone)]
pub struct BoxResult {
    pub box_item_id: i32,
    /// Boxes of this kind still owned (0 when the last one was used).
    pub box_remaining: i32,
    pub item_id: i32,
    pub unique_id: f64,
    /// Rental period of the award in hours (0 = permanent) and its expiry stamp.
    pub hours: i32,
    pub until: f64,
    /// Second-reel award, if the box has one.
    pub bonus: Option<BoxBonus>,
}

/// What the bonus reel produced: a real instance in the bag. Gold Bars are
/// items too - the client files every id in `rfAckRandomBox_Use` as an
/// instance (`BTWindowRandBoxHK` → `AddInstanceItemByParameter`), so crediting
/// them as points left a ghost bar with unique id 0 that sold for nothing. The
/// bar is worth its catalogue resale price (`Gold Bar (15000)` → 15000) when
/// the player sells it, which is the original mechanic.
#[derive(Debug, Clone)]
pub struct BoxBonus {
    pub item_id: i32,
    pub unique_id: f64,
}

/// Opens one owned box instance: consumes one and draws from the box's item
/// reel by weight (`box_entries` in the database, editable in the admin), plus
/// one draw from its bonus reel if it has one. Boxes with no entries hand out a
/// random obtainable main weapon. Items the admin hid never drop.
pub fn open_box(name: &str, box_unique_id: f64) -> Option<BoxResult> {
    with_named(name, |inv| {
        let Some(pos) = inv.items.iter().position(|i| i.unique_id == box_unique_id) else {
            return (None, false);
        };
        let box_item_id = inv.items[pos].item_id;
        let entries = db().box_entries(box_item_id).unwrap_or_default();
        let main: Vec<db_model::BoxEntryRow> = entries
            .iter()
            .filter(|e| e.reel == db_model::Reel::Main && e.weight > 0.0 && !db().is_hidden(e.item_id).unwrap_or(false))
            .cloned()
            .collect();
        let drawn = draw_weighted(&main).or_else(|| {
            let fallback: Vec<db_model::BoxEntryRow> = db()
                .obtainable_ids_by_kind(KIND_MAIN)
                .unwrap_or_default()
                .into_iter()
                .map(|id| db_model::BoxEntryRow { item_id: id, reel: db_model::Reel::Main, weight: 1.0, featured: false, hours: 0 })
                .collect();
            draw_weighted(&fallback)
        });
        let Some((item_id, hours)) = drawn else {
            return (None, false);
        };
        let item_type = catalog_item(item_id).map(|c| c.item_type).unwrap_or(TYPE_MAIN);
        // consume the box
        if inv.items[pos].stack > 1 {
            inv.items[pos].stack -= 1;
        } else {
            inv.items.remove(pos);
        }
        let box_remaining = inv.items.iter().filter(|i| i.item_id == box_item_id).map(|i| i.stack).sum();
        let unique_id = inv.add_rental(item_id, item_type, hours);
        let until = inv.find(unique_id).map(|i| i.until).unwrap_or(0.0);
        // bonus reel: Gold Bars and tickets alike go into the bag as items
        let bonus_pool: Vec<db_model::BoxEntryRow> = entries.iter().filter(|e| e.reel == db_model::Reel::Bonus && e.weight > 0.0).cloned().collect();
        let bonus = if bonus_pool.is_empty() {
            None
        } else {
            let Some((bonus_id, _)) = draw_weighted(&bonus_pool) else { unreachable!("non-empty weighted pool") };
            let t = catalog_item(bonus_id).map(|c| c.item_type).unwrap_or(16);
            Some(BoxBonus { item_id: bonus_id, unique_id: inv.add(bonus_id, t) })
        };
        tracing::info!(box_item_id, item_id, unique_id, hours, bonus = ?bonus, points = inv.points, "random box opened");
        (Some(BoxResult { box_item_id, box_remaining, item_id, unique_id, hours, until, bonus }), true)
    })
}

/// Raw durability lost per second of use (`ACEMU_WEAR`; 0 disables wear). At the
/// default a fresh weapon (100 000) lasts about five and a half hours of use.
fn wear_per_sec() -> i32 {
    std::env::var("ACEMU_WEAR").ok().and_then(|v| v.parse().ok()).unwrap_or(5)
}

/// Applies end-of-match wear to one weapon owned by `owner` (the host reports
/// every room member's weapons). Returns the new raw durability and whether it
/// hit zero; None for an unknown instance.
pub fn wear(owner: &str, unique_id: f64, using_sec: i32) -> Option<(i32, bool)> {
    with_named(owner, |inv| {
        let Some(it) = inv.items.iter_mut().find(|i| i.unique_id == unique_id) else {
            return (None, false);
        };
        let loss = using_sec.max(0).saturating_mul(wear_per_sec());
        it.durability = (it.durability - loss).max(0);
        (Some((it.durability, it.durability == 0)), loss > 0)
    })
}

/// Points to repair one instance: one per 100 raw durability missing, so a
/// fully worn weapon costs 1000. The same formula backs the repair quote.
pub fn repair_cost_of(durability: i32) -> i32 {
    ((full_durability() - durability).max(0) / 100).max(0)
}

/// Repairs the given owned instances to full. Charges the summed quote; None
/// when the player cannot afford it. Returns (cost charged, points left).
pub fn repair(name: &str, unique_ids: &[f64]) -> Option<(i32, i32)> {
    with_named(name, |inv| {
        let cost: i32 = unique_ids.iter().filter_map(|&u| inv.find(u)).map(|i| repair_cost_of(i.durability)).sum();
        if cost > inv.points {
            return (None, false);
        }
        inv.points -= cost;
        let full = full_durability();
        for it in inv.items.iter_mut().filter(|i| unique_ids.contains(&i.unique_id)) {
            it.durability = full;
        }
        tracing::info!(player = name, count = unique_ids.len(), cost, points = inv.points, "repaired");
        (Some((cost, inv.points)), true)
    })
}

/// Equipped items whose durability is gone: (unique id, slot).
pub fn equipped_worn_out(name: &str) -> Vec<(f64, i32)> {
    read_named(name, |inv| inv.items.iter().filter(|i| i.slot >= 0 && i.durability <= 0).map(|i| (i.unique_id, i.slot)).collect())
}

/// Item id, part id and slot of an owned instance (for the zero-durability notice).
pub fn item_brief(name: &str, unique_id: f64) -> Option<(i32, i32, i32)> {
    read_named(name, |inv| inv.find(unique_id).map(|i| (i.item_id, i.add_part_id, i.slot)))
}

/// Uses `count` of a stackable instance in a match; returns what is left, or
/// None when the player does not own enough.
pub fn consume(name: &str, unique_id: f64, count: i32) -> Option<i32> {
    with_named(name, |inv| {
        let Some(pos) = inv.items.iter().position(|i| i.unique_id == unique_id) else { return (None, false) };
        let n = count.max(1);
        if inv.items[pos].stack < n {
            return (None, false);
        }
        inv.items[pos].stack -= n;
        let left = inv.items[pos].stack;
        if left == 0 {
            inv.items.remove(pos);
        }
        (Some(left), true)
    })
}

/// Gives `count` of `item_id` (a skill for skill ids; stackables stack).
/// Returns the unique id, None for an unknown item.
pub fn grant(name: &str, item_id: i32, count: i32) -> Option<f64> {
    let cat = catalog_item(item_id)?;
    with_named(name, |inv| {
        if cat.kind == KIND_SKILL {
            let uid = inv.add_skill(item_id, 0) as f64;
            tracing::info!(item_id, uid, "skill granted");
            return (Some(uid), true);
        }
        let uid = inv.add(item_id, cat.item_type);
        if count > 1 {
            if let Some(it) = inv.items.iter_mut().find(|i| i.unique_id == uid) {
                it.stack = count;
            }
        }
        tracing::info!(item_id, count, uid, "item granted");
        (Some(uid), true)
    })
}

/// Removes an owned instance from the bag (mail attachment); None if not owned.
pub fn take_item(name: &str, unique_id: f64) -> Option<Item> {
    with_named(name, |inv| {
        let Some(pos) = inv.items.iter().position(|i| i.unique_id == unique_id) else { return (None, false) };
        let it = inv.items.remove(pos);
        for q in &mut inv.qslots {
            for f in [&mut q.main, &mut q.pistol, &mut q.throw1, &mut q.throw2] {
                if *f == unique_id {
                    *f = 0.0;
                }
            }
        }
        (Some(it), true)
    })
}

/// Credits points and EXP (level follows the EXP table); returns (points, level).
pub fn add_points(name: &str, points: i32, exp: i32) -> (i32, i32) {
    with_named(name, |inv| {
        inv.points += points.max(0);
        inv.stats.exp += exp.max(0);
        inv.stats.level = level_for_exp(inv.stats.exp).max(inv.stats.level);
        ((inv.points, inv.stats.level), points > 0 || exp > 0)
    })
}

/// A Fortune Depot capsule: a box with a drop table.
pub fn is_capsule(item_id: i32) -> bool {
    db().box_entries(item_id).map(|e| !e.is_empty()).unwrap_or(false)
}

/// One end-of-match booty award, already in the bag.
#[derive(Debug, Clone)]
pub struct Booty {
    pub item_id: i32,
    pub unique_id: f64,
    pub item_type: i32,
    /// Rental period in hours (0 = permanent) and its expiry stamp.
    pub hours: i32,
    pub until: f64,
}

/// Where a booty draw takes its item from.
enum BootySource {
    /// A weapon from the main reel of one of the client's spoils boxes
    /// (17071 legendary / 17072 rare / 17073 superior), as a rental.
    FromBox { box_id: i32, hours: i32 },
    /// One of these catalogue ids, as a rental (0 = permanent).
    OneOf { ids: &'static [i32], hours: i32 },
}

/// The booty pool, weighted. Handing out the same "Random Superior Spoils
/// Box" after every match got old fast, so the box is now the rare outcome
/// and the usual prize is a short weapon rental drawn from the box tables, a
/// one-day booster or a Gold Bar (sold for its face value).
const BOOTY_POOL: &[(u32, BootySource)] = &[
    (40, BootySource::FromBox { box_id: 17073, hours: 72 }),
    (15, BootySource::FromBox { box_id: 17072, hours: 72 }),
    (5, BootySource::FromBox { box_id: 17071, hours: 24 }),
    // 5% / 10% point boosters, 10% / 50% exp boosters, 20% point booster
    (20, BootySource::OneOf { ids: &[16008, 16009, 16010, 16001, 16003], hours: 24 }),
    // Gold Bar (1000) / (2000) / (3000)
    (10, BootySource::OneOf { ids: &[17022, 17035, 17052], hours: 0 }),
    (7, BootySource::OneOf { ids: &[17073], hours: 0 }),
    (3, BootySource::OneOf { ids: &[17072], hours: 0 }),
];

/// Draws one entry from the booty pool: (item id, rental hours).
fn draw_booty() -> (i32, i32) {
    let total: u32 = BOOTY_POOL.iter().map(|(w, _)| *w).sum();
    let mut r = roll(total as usize) as u32;
    let mut chosen = &BOOTY_POOL[BOOTY_POOL.len() - 1].1;
    for (w, src) in BOOTY_POOL {
        if r < *w {
            chosen = src;
            break;
        }
        r -= *w;
    }
    match chosen {
        BootySource::FromBox { box_id, hours } => {
            // The box table's second value is not a period for the spoils
            // boxes (it looks like a model index), so the rental length is ours.
            let pool: Vec<db_model::BoxEntryRow> = db()
                .box_entries(*box_id)
                .unwrap_or_default()
                .into_iter()
                .filter(|e| e.reel == db_model::Reel::Main && e.weight > 0.0 && !db().is_hidden(e.item_id).unwrap_or(false))
                .collect();
            match draw_weighted(&pool) {
                Some((item_id, _)) => (item_id, *hours),
                None => (*box_id, 0),
            }
        }
        BootySource::OneOf { ids, hours } => (ids[roll(ids.len())], *hours),
    }
}

/// End-of-match booty: one draw from `BOOTY_POOL`, delivered to the bag.
pub fn grant_booty(name: &str) -> Option<Booty> {
    let (item_id, hours) = draw_booty();
    let item_type = catalog_item(item_id).map(|c| c.item_type).unwrap_or(16);
    with_named(name, |inv| {
        let unique_id = inv.add_rental(item_id, item_type, hours);
        let until = inv.find(unique_id).map(|i| i.until).unwrap_or(0.0);
        tracing::info!(item_id, unique_id, hours, "booty granted");
        (Some(Booty { item_id, unique_id, item_type, hours, until }), true)
    })
}

/// Opens the database and lists every player; used by `--check-inventory`.
pub fn check_file() -> std::result::Result<String, String> {
    let db = acemu_db::Db::open_default().map_err(|e| e.to_string())?;
    let players = db.players().map_err(|e| e.to_string())?;
    let mut out = format!("{}: ok - {} players", acemu_paths::save_file(acemu_db::DB_FILE).display(), players.len());
    for p in players {
        out.push_str(&format!("\n  {} level={} kills={} points={} cash={} items={}", p.name, p.level, p.kills, p.points, p.cash, p.item_count));
    }
    Ok(out)
}

/// A catalogue row from the database. Rows the client's shop table prices at
/// zero fall back to the product feed's price (see `Product`).
pub fn catalog_item(id: i32) -> Option<CatalogItem> {
    let row = db().catalog_item(id).ok().flatten()?;
    let mut prices: Vec<Price> = row.prices.iter().map(|p| Price { hours: p.hours, points: p.points, cash: p.cash }).collect();
    if prices.is_empty() {
        if let Some(p) = PRODUCTS.lock().unwrap().iter().find(|p| p.id == id) {
            prices = vec![p.price()];
        }
    }
    Some(CatalogItem { id: row.id, name: row.name, item_type: row.item_type, kind: row.kind, prices, sell_price: row.sell_price })
}

/// Raw durability of a new item. The client divides the wire value by a per-item
/// weight from its table before showing it, so a small number reads as 0 and the
/// weapon is refused as broken ("equipped with default weapons with 0 durability").
/// Observed: 10 000 shows as 10%, so 100 000 is full. `ACEMU_DURABILITY`
/// overrides it at runtime.
pub fn full_durability() -> i32 {
    std::env::var("ACEMU_DURABILITY").ok().and_then(|v| v.parse().ok()).unwrap_or(100_000)
}

/// Slot position the client sends to *unequip* an item or skill (it logs the
/// server's -1 as 13 as well).
pub const SLOT_NONE: i32 = 13;

/// One item instance the account owns.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// `BtrDouble` on the wire; unique per instance.
    pub unique_id: f64,
    pub item_id: i32,
    pub item_type: i32,
    /// Equipped slot, or -1 when it sits in the bag.
    pub slot: i32,
    pub stack: i32,
    #[serde(default = "full_durability")]
    pub durability: i32,
    /// Weapon remodel: attached part group (`AddPartsID` on the wire), 0 = stock.
    #[serde(default)]
    pub add_part_id: i32,
    /// Weapon paint/skin id, 0 = none.
    #[serde(default)]
    pub paint_id: i32,
    /// Rental expiry as an OLE date (`UntilTime` on the wire); 0 = permanent.
    #[serde(default)]
    pub until: f64,
}

/// Now as an OLE Automation date: days since 1899-12-30, fraction = time of
/// day. That is what `BtrDouble` times are on the wire (`rfAckGetServerTime`,
/// `UntilTime`): the client's `BtrDoubleToBtrTime` printed 1899-12-30 for 0.
pub fn ole_now() -> f64 {
    const UNIX_EPOCH_OLE: f64 = 25569.0; // 1970-01-01
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0);
    UNIX_EPOCH_OLE + secs / 86400.0
}

/// Expiry stamp for a rental of `hours` (0 or the table's "1 = permanent" -> 0).
pub fn until_for_hours(hours: i32) -> f64 {
    if hours <= 1 {
        0.0
    } else {
        ole_now() + hours as f64 / 24.0
    }
}

/// One skill the account owns. Skills live in their own list on the wire
/// (`rfAckUserSkillList`), with `int` unique ids and a byte slot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub unique_id: i32,
    pub skill_id: i32,
    /// Equipped slot, or -1 when not equipped.
    pub slot: i32,
    /// Rental expiry as an OLE date (`UntilTime` on the wire); 0 = permanent.
    #[serde(default)]
    pub until: f64,
}

/// `UntilTime` put on the wire for a permanent skill: 2099-01-01 as an OLE
/// date. Skills are period items in the client's table (`CostType` 1/4), so it
/// always computes a remaining time from `UntilTime` and shows 0 as "expired".
pub const PERMANENT_UNTIL: f64 = 72_686.0;

/// The wire form of a skill's expiry (see `PERMANENT_UNTIL`).
pub fn skill_until_wire(until: f64) -> f64 {
    if until > 0.0 { until } else { PERMANENT_UNTIL }
}

/// A weapon loadout the client can switch between in the lobby.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickSlot {
    pub id: i32,
    pub name: String,
    pub main: f64,
    pub pistol: f64,
    pub throw1: f64,
    pub throw2: f64,
    pub skills: [i32; 4],
}

/// Career stats behind the lobby's My Info / Records panel. EXP is cumulative;
/// the level is derived from the client's own level table (`levels.json`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub level: i32,
    pub exp: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub headshots: i32,
    pub team_kills: i32,
    pub wins: i32,
    pub losses: i32,
    pub play_time: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats { level: 1, exp: 0, kills: 0, deaths: 0, assists: 0, headshots: 0, team_kills: 0, wins: 0, losses: 0, play_time: 0 }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelRow {
    pub level: i32,
    pub name: String,
    pub upper_exp: i32,
}

static LEVELS: Mutex<Vec<LevelRow>> = Mutex::new(Vec::new());
const LEVELS_FILE: &str = "levels.json";

pub fn load_levels() {
    let path = std::env::var("ACEMU_LEVELS").unwrap_or_else(|_| acemu_paths::data_file(LEVELS_FILE).display().to_string());
    let rows: Vec<LevelRow> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(s.trim_start_matches('\u{feff}')).ok())
        .unwrap_or_default();
    let rows = if rows.is_empty() {
        let fallback = fallback_levels();
        tracing::warn!(path, levels = fallback.len(), "no level table - using the built-in curve");
        fallback
    } else {
        tracing::info!(levels = rows.len(), top = %rows.last().map(|r| r.name.as_str()).unwrap_or(""), "level table loaded");
        rows
    };
    *LEVELS.lock().unwrap() = rows;
}

/// A level ladder for servers running without `levels.json`, so progression
/// still works: 100 levels, each costing 10 % more experience than the last,
/// starting at 1000. The client draws the rank from its own table, so only the
/// thresholds matter here - the names are for our own pages.
fn fallback_levels() -> Vec<LevelRow> {
    let mut rows = Vec::with_capacity(100);
    let mut total = 0f64;
    let mut step = 1_000f64;
    for level in 1..=100 {
        total += step;
        step *= 1.1;
        rows.push(LevelRow { level, name: format!("Level {level}"), upper_exp: total.min(i32::MAX as f64) as i32 });
    }
    rows
}

/// Level reached with `exp` cumulative EXP: each row's `upper_exp` is the total
/// needed to leave that level.
pub fn level_for_exp(exp: i32) -> i32 {
    let rows = LEVELS.lock().unwrap();
    let mut level = 1;
    for r in rows.iter() {
        if exp >= r.upper_exp {
            level = r.level + 1;
        } else {
            break;
        }
    }
    level.min(rows.len().max(1) as i32)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub points: i32,
    pub cash: i32,
    pub next_unique: u64,
    pub items: Vec<Item>,
    #[serde(default)]
    pub skills: Vec<Skill>,
    pub qslots: Vec<QuickSlot>,
    #[serde(default)]
    pub stats: Stats,
}

/// One player's line from `rfReqUpdateScore`.
#[derive(Debug, Clone, Copy, Default)]
pub struct MatchLine {
    pub kills: i32,
    pub assists: i32,
    pub deaths: i32,
    pub exp: i32,
    pub points: i32,
    pub playtime: i32,
    /// Some(true) won, Some(false) lost, None undecided (draw / left early).
    pub won: Option<bool>,
}

/// Applies a match result to `name`'s account; returns (old level, new level).
pub fn apply_match(name: &str, m: MatchLine) -> (i32, i32) {
    with_named(name, |inv| (apply_match_in(inv, m), true))
}

/// Every access names the character explicitly (`with_named`/`read_named`);
/// an empty name means the default character. Accesses are serialised so two
/// packets for the same player never interleave a load and a save.
static INV_LOCK: Mutex<()> = Mutex::new(());
static DEFAULT_NAME: Mutex<String> = Mutex::new(String::new());

/// Names the character an empty name resolves to (the original single account).
pub fn set_default_name(name: &str) {
    *DEFAULT_NAME.lock().unwrap() = name.to_string();
}

fn resolve_name(name: &str) -> String {
    if name.is_empty() {
        DEFAULT_NAME.lock().unwrap().clone()
    } else {
        name.to_string()
    }
}

fn from_record(rec: db_model::PlayerRecord) -> Inventory {
    Inventory {
        points: rec.points,
        cash: rec.cash,
        next_unique: rec.next_unique.max(0) as u64,
        items: rec
            .items
            .into_iter()
            .map(|i| Item {
                unique_id: i.unique_id as f64,
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
        skills: rec
            .skills
            .into_iter()
            .map(|s| Skill { unique_id: s.unique_id as i32, skill_id: s.skill_id, slot: s.slot, until: s.until })
            .collect(),
        qslots: rec
            .qslots
            .into_iter()
            .map(|q| QuickSlot {
                id: q.id,
                name: q.name,
                main: q.main as f64,
                pistol: q.pistol as f64,
                throw1: q.throw1 as f64,
                throw2: q.throw2 as f64,
                skills: q.skills,
            })
            .collect(),
        stats: Stats {
            level: rec.stats.level,
            exp: rec.stats.exp,
            kills: rec.stats.kills,
            deaths: rec.stats.deaths,
            assists: rec.stats.assists,
            headshots: rec.stats.headshots,
            team_kills: rec.stats.team_kills,
            wins: rec.stats.wins,
            losses: rec.stats.losses,
            play_time: rec.stats.play_time,
        },
    }
}

fn to_record(name: &str, inv: &Inventory) -> db_model::PlayerRecord {
    db_model::PlayerRecord {
        name: name.to_string(),
        points: inv.points,
        cash: inv.cash,
        next_unique: inv.next_unique as i64,
        stats: db_model::Stats {
            level: inv.stats.level,
            exp: inv.stats.exp,
            kills: inv.stats.kills,
            deaths: inv.stats.deaths,
            assists: inv.stats.assists,
            headshots: inv.stats.headshots,
            team_kills: inv.stats.team_kills,
            wins: inv.stats.wins,
            losses: inv.stats.losses,
            play_time: inv.stats.play_time,
        },
        items: inv
            .items
            .iter()
            .map(|i| db_model::ItemRow {
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
        skills: inv
            .skills
            .iter()
            .map(|s| db_model::SkillRow { unique_id: s.unique_id as i64, skill_id: s.skill_id, slot: s.slot, until: s.until })
            .collect(),
        qslots: inv
            .qslots
            .iter()
            .map(|q| db_model::QSlotRow {
                id: q.id,
                name: q.name.clone(),
                main: q.main as i64,
                pistol: q.pistol as i64,
                throw1: q.throw1 as i64,
                throw2: q.throw2 as i64,
                skills: q.skills,
            })
            .collect(),
    }
}

impl Inventory {
    /// A fresh account: the two default weapons the client equips on its own
    /// (G3 2000, Glock 17 1010), one quick-slot, and the starting balance
    /// (1000 points / 150 cash; `ACEMU_START_POINTS` / `ACEMU_START_CASH` override).
    fn starter() -> Inventory {
        let env = |k: &str, d: i32| std::env::var(k).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
        let mut inv = Inventory {
            points: env("ACEMU_START_POINTS", 1_000),
            cash: env("ACEMU_START_CASH", 150),
            next_unique: 100_000_001,
            items: vec![],
            skills: vec![],
            qslots: vec![],
            stats: Stats::default(),
        };
        // No melee item: the client gives every pawn the generic melee strike
        // (9000, type 11, class wMeleeWeapon) and the default knife (9002, type 3,
        // class wKnife) by itself. Equipping 9000 in slot 2 made the pawn spawn
        // with a second wMeleeWeapon and the knife key stopped working.
        let g3 = inv.add(2000, TYPE_MAIN);
        let glock = inv.add(1010, TYPE_PISTOL);
        for it in &mut inv.items {
            it.slot = slot_for_type(it.item_type);
        }
        // Default throwables: a frag grenade in the throwable slot and a flash bang
        // in the bag; both go into the loadout, which is what the pawn spawns with.
        let grenade = inv.add(6000, TYPE_THROW);
        let flash = inv.add(6002, TYPE_THROW);
        if let Some(it) = inv.items.iter_mut().find(|i| i.unique_id == grenade) {
            it.slot = slot_for_type(TYPE_THROW);
        }
        // QSlotID is a dense 0-based index: the client keeps `SaveQS[QSlotID]` in an
        // array sized by the number of slots received. QSlotDispOrder is 1-based.
        inv.qslots.push(QuickSlot {
            id: 0,
            name: "Slot 1".into(),
            main: g3,
            pistol: glock,
            throw1: grenade,
            throw2: flash,
            skills: [0; 4],
        });
        inv
    }

    fn add(&mut self, item_id: i32, item_type: i32) -> f64 {
        self.add_rental(item_id, item_type, 0)
    }

    /// Adds an instance that expires after `hours` (0 / 1 = permanent).
    fn add_rental(&mut self, item_id: i32, item_type: i32, hours: i32) -> f64 {
        let uid = self.next_unique as f64;
        self.next_unique += 1;
        self.items.push(Item {
            unique_id: uid,
            item_id,
            item_type,
            slot: -1,
            stack: 1,
            durability: full_durability(),
            add_part_id: 0,
            paint_id: 0,
            until: until_for_hours(hours),
        });
        uid
    }

    /// Removes every rental whose time is up and returns them (for
    /// `rfAckNotifyExpiredItemIDs`); quick-slot references to them are cleared.
    pub fn purge_expired(&mut self) -> Vec<Item> {
        let now = ole_now();
        let (gone, keep): (Vec<Item>, Vec<Item>) =
            self.items.drain(..).partition(|i| i.until > 0.0 && i.until <= now);
        self.items = keep;
        for it in &gone {
            for q in &mut self.qslots {
                for f in [&mut q.main, &mut q.pistol, &mut q.throw1, &mut q.throw2] {
                    if *f == it.unique_id {
                        *f = 0.0;
                    }
                }
            }
        }
        gone
    }

    /// Part and paint to put on the wire for an item: its own remodel if it has
    /// one, else the weapon's shipped default (see `default_part_id`).
    pub fn effective_parts(&self, it: &Item) -> (i32, i32) {
        let part = if it.add_part_id != 0 { it.add_part_id } else { default_part_id(it.item_id) };
        (part, it.paint_id)
    }

    /// `effective_parts` by unique id; (0, 0) for an unknown instance.
    pub fn parts_of(&self, unique_id: f64) -> (i32, i32) {
        self.find(unique_id).map(|i| self.effective_parts(i)).unwrap_or((0, 0))
    }

    /// Adds a skill that expires after `hours` (0 / 1 = permanent).
    fn add_skill(&mut self, skill_id: i32, hours: i32) -> i32 {
        let uid = self.next_unique as i32;
        self.next_unique += 1;
        self.skills.push(Skill { unique_id: uid, skill_id, slot: -1, until: until_for_hours(hours) });
        uid
    }

    /// Removes every skill whose time is up and returns them (for
    /// `rfAckNotifyExpiredSkillIDs`); quick-slot references to them are cleared.
    pub fn purge_expired_skills(&mut self) -> Vec<Skill> {
        let now = ole_now();
        let (gone, keep): (Vec<Skill>, Vec<Skill>) =
            self.skills.drain(..).partition(|s| s.until > 0.0 && s.until <= now);
        self.skills = keep;
        for sk in &gone {
            for q in &mut self.qslots {
                for f in &mut q.skills {
                    if *f == sk.skill_id {
                        *f = 0;
                    }
                }
            }
        }
        gone
    }

    pub fn equipped(&self) -> Vec<&Item> {
        self.items.iter().filter(|i| i.slot >= 0).collect()
    }

    pub fn equipped_skills(&self) -> Vec<&Skill> {
        self.skills.iter().filter(|s| s.slot >= 0).collect()
    }

    pub fn find(&self, unique_id: f64) -> Option<&Item> {
        self.items.iter().find(|i| i.unique_id == unique_id)
    }

    /// The client keeps exactly five quick slots (`wMyPlayerStatus.QuickSlotList`
    /// is a static array of 5 and `wMatchUserInfo.ResetQSlotItemUse` walks 0..4),
    /// indexed by `QSlotID`. Make sure ids 0..4 all exist, in order.
    pub fn ensure_qslots(&mut self) {
        for id in 0..QSLOT_COUNT {
            if !self.qslots.iter().any(|s| s.id == id) {
                self.qslots.push(QuickSlot {
                    id,
                    name: format!("Slot {}", id + 1),
                    main: 0.0,
                    pistol: 0.0,
                    throw1: 0.0,
                    throw2: 0.0,
                    skills: [0; 4],
                });
            }
        }
        self.qslots.retain(|s| (0..QSLOT_COUNT).contains(&s.id));
        self.qslots.sort_by_key(|s| s.id);
    }
}

/// Number of quick slots the client expects per account.
pub const QSLOT_COUNT: i32 = 5;

/// `name`'s inventory from the database; a starter inventory for a character
/// without a row yet (it is written on the first change).
fn load(name: &str) -> Inventory {
    let mut inv = match db().player(name) {
        Ok(Some(rec)) => from_record(rec),
        Ok(None) => {
            tracing::info!(character = name, "no player row - creating starter inventory");
            Inventory::starter()
        }
        Err(e) => {
            tracing::warn!(character = name, error = %e, "player row unreadable - using a starter inventory for this request");
            Inventory::starter()
        }
    };
    inv.ensure_qslots();
    inv
}

fn save(name: &str, inv: &Inventory) {
    if let Err(e) = db().save_player(&to_record(name, inv)) {
        tracing::warn!(character = name, error = %e, "could not persist inventory");
    }
}

/// Makes sure `name` has a player row: a new character gets its starter
/// inventory saved right away, so the admin app lists it from the first login
/// on instead of only after the first purchase or equip.
pub fn ensure_exists(name: &str) {
    let _serialised = INV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let name = resolve_name(name);
    match db().player(&name) {
        Ok(Some(_)) => {}
        Ok(None) => {
            tracing::info!(character = %name, "new character - saving a starter inventory");
            let mut inv = Inventory::starter();
            inv.ensure_qslots();
            save(&name, &inv);
        }
        Err(e) => tracing::warn!(character = %name, error = %e, "could not check for the player row"),
    }
}

/// Runs `f` with `name`'s inventory; persists afterwards if `f` says it changed.
pub fn with_named<T>(name: &str, f: impl FnOnce(&mut Inventory) -> (T, bool)) -> T {
    let _serialised = INV_LOCK.lock().unwrap_or_else(|p| p.into_inner());
    let name = resolve_name(name);
    let mut inv = load(&name);
    let (out, changed) = f(&mut inv);
    if changed {
        save(&name, &inv);
    }
    out
}

/// Read-only access to `name`'s inventory.
pub fn read_named<T>(name: &str, f: impl FnOnce(&Inventory) -> T) -> T {
    with_named(name, |inv| (f(inv), false))
}

/// Applies a match line to an inventory; returns (old level, new level).
pub fn apply_match_in(inv: &mut Inventory, m: MatchLine) -> (i32, i32) {
    let s = &mut inv.stats;
    let old = s.level;
    s.kills += m.kills;
    s.assists += m.assists;
    s.deaths += m.deaths;
    s.exp += m.exp.max(0);
    s.play_time += m.playtime.max(0);
    match m.won {
        Some(true) => s.wins += 1,
        Some(false) => s.losses += 1,
        None => {}
    }
    s.level = level_for_exp(s.exp).max(old);
    inv.points += m.points.max(0);
    tracing::info!(
        level = s.level, exp = s.exp, kills = s.kills, deaths = s.deaths, wins = s.wins, losses = s.losses,
        points = inv.points, "stats updated"
    );
    (old, s.level)
}

/// Buys `item_id` from the catalogue. Prices live only in the client's table, so a
/// flat cost is charged. Returns the new unique id, or None if unknown/unaffordable.
/// Why a purchase was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuyError {
    UnknownItem,
    NotForSale,
    NotEnoughPoints,
    NotEnoughCash,
}

/// Buys `item_id` at the client's own price: `option` is the 1-based
/// `BuyOption1` (which duration column the player picked). Points are charged
/// when the column has a point price, cash otherwise. Returns the new unique id.
pub fn buy(name: &str, item_id: i32, option: u8) -> std::result::Result<f64, BuyError> {
    buy_with(name, item_id, option, BuyMode::New)
}

/// How a purchase applies to what the player already owns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuyMode {
    /// A new instance, always.
    New,
    /// Extend the owned instance of this item (the client's "extend term"
    /// dialog, `BuyOption2` 1, and the expired-item re-buy, `BuyOption2` 2):
    /// the bought period is added to that instance's expiry and the same
    /// unique id is returned, which the client updates in place. Falls back to
    /// a new instance when nothing is owned.
    Extend,
}

/// Buys `item_id` at the client's own price: `option` is the 1-based
/// `BuyOption1` (which duration column the player picked). Points are charged
/// when the column has a point price, cash otherwise. Returns the unique id of
/// the instance bought or extended.
pub fn buy_with(name: &str, item_id: i32, option: u8, mode: BuyMode) -> std::result::Result<f64, BuyError> {
    let item = catalog_item(item_id).ok_or(BuyError::UnknownItem)?;
    let idx = (option.max(1) - 1) as usize;
    // Rows without a price anywhere (neither the shop table nor the product
    // feed) are what the client shows at 0, so charge what it shows.
    const FALLBACK: Price = Price { hours: 0, points: 0, cash: 0 };
    let price = item.prices.get(idx).or_else(|| item.prices.first()).copied().unwrap_or(FALLBACK);
    with_named(name, |inv| {
        if price.points > 0 {
            if inv.points < price.points {
                return (Err(BuyError::NotEnoughPoints), false);
            }
            inv.points -= price.points;
        } else if inv.cash < price.cash {
            return (Err(BuyError::NotEnoughCash), false);
        } else {
            inv.cash -= price.cash;
        }
        let extended = if mode == BuyMode::Extend { inv.extend_owned(&item, price.hours) } else { None };
        let uid = match extended {
            Some(uid) => uid,
            None if item.kind == KIND_SKILL => inv.add_skill(item.id, price.hours) as f64,
            None => inv.add_rental(item.id, item.item_type, price.hours),
        };
        tracing::info!(
            item = %item.name, id = item.id, kind = item.kind, uid, option, hours = price.hours, extended = extended.is_some(),
            cost_points = price.points, cost_cash = price.cash, points = inv.points, cash = inv.cash, "bought"
        );
        (Ok(uid), true)
    })
}

/// Expiry after adding `hours` to an instance that currently expires at
/// `until`: counted from the later of now and the old expiry, so extending a
/// running rental adds to it and re-buying an expired one starts fresh.
/// A permanent option (hours <= 1) makes the item permanent.
fn extended_until(until: f64, hours: i32) -> f64 {
    if hours <= 1 || until == 0.0 {
        return 0.0;
    }
    let base = if until > ole_now() { until } else { ole_now() };
    base + hours as f64 / 24.0
}

impl Inventory {
    /// Extends the owned instance of `item` (the one expiring soonest) by
    /// `hours`; returns its unique id, or None when none is owned.
    fn extend_owned(&mut self, item: &CatalogItem, hours: i32) -> Option<f64> {
        if item.kind == KIND_SKILL {
            let sk = self
                .skills
                .iter_mut()
                .filter(|s| s.skill_id == item.id)
                .min_by(|a, b| a.until.partial_cmp(&b.until).unwrap_or(std::cmp::Ordering::Equal))?;
            sk.until = extended_until(sk.until, hours);
            return Some(sk.unique_id as f64);
        }
        let it = self
            .items
            .iter_mut()
            .filter(|i| i.item_id == item.id)
            .min_by(|a, b| a.until.partial_cmp(&b.until).unwrap_or(std::cmp::Ordering::Equal))?;
        it.until = extended_until(it.until, hours);
        Some(it.unique_id)
    }
}

/// What the shop pays for an owned instance: the table's resale price, else a
/// fifth of the purchase price (the table's own ratio), else nothing.
pub fn resale_value(name: &str, unique_id: f64) -> i32 {
    read_named(name, |inv| {
        let id = inv
            .find(unique_id)
            .map(|i| i.item_id)
            .or_else(|| inv.skills.iter().find(|s| s.unique_id as f64 == unique_id).map(|s| s.skill_id));
        id.and_then(catalog_item)
            .map(|c| if c.sell_price > 0 { c.sell_price } else { c.prices.first().map(|p| p.points / 5).unwrap_or(0) })
            .unwrap_or(0)
    })
}

/// Sells `count` of an owned instance (item or skill) back for its resale
/// value. Returns `(points, remaining stack)`; the stack goes to 0 when the
/// instance is gone. The client removes the instance only when the ack's
/// `Count` is 0 and otherwise sets the displayed stack to it
/// (`BTPageInventory.rfAckSellShopItem`).
pub fn sell(name: &str, unique_id: f64, count: i32) -> Option<(i32, i32)> {
    let each = resale_value(name, unique_id);
    with_named(name, |inv| {
        let Some(pos) = inv.items.iter().position(|i| i.unique_id == unique_id) else {
            if let Some(pos) = inv.skills.iter().position(|s| s.unique_id as f64 == unique_id) {
                let sk = inv.skills.remove(pos);
                inv.points += each;
                for q in &mut inv.qslots {
                    for f in &mut q.skills {
                        if *f == sk.skill_id {
                            *f = 0;
                        }
                    }
                }
                tracing::info!(skill = sk.skill_id, uid = sk.unique_id, points = inv.points, "sold skill");
                return (Some((inv.points, 0)), true);
            }
            return (None, false);
        };
        let stack = inv.items[pos].stack.max(1);
        let sold = count.clamp(1, stack);
        inv.points += each * sold;
        let remaining = stack - sold;
        if remaining > 0 {
            inv.items[pos].stack = remaining;
            let it = &inv.items[pos];
            tracing::info!(id = it.item_id, uid = unique_id, sold, remaining, points = inv.points, "sold part of a stack");
            return (Some((inv.points, remaining)), true);
        }
        let it = inv.items.remove(pos);
        for q in &mut inv.qslots {
            for f in [&mut q.main, &mut q.pistol, &mut q.throw1, &mut q.throw2] {
                if *f == unique_id {
                    *f = 0.0;
                }
            }
        }
        tracing::info!(id = it.item_id, uid = unique_id, sold, points = inv.points, "sold");
        (Some((inv.points, 0)), true)
    })
}

/// Remodels an owned weapon: attaches part group `add_part_id` and paint
/// `paint_id` (the client validates combinations against its own parts table).
/// Prices live only client-side, so a flat cost is charged per change.
/// Returns `(points, cash)` after the change, or None if unowned/unaffordable.
pub fn modify(name: &str, unique_id: f64, add_part_id: i32, paint_id: i32) -> Option<(i32, i32)> {
    const FLAT_COST: i32 = 500;
    with_named(name, |inv| {
        let Some(pos) = inv.items.iter().position(|i| i.unique_id == unique_id) else {
            return (None, false);
        };
        let it = &inv.items[pos];
        let changed = it.add_part_id != add_part_id || it.paint_id != paint_id;
        if changed && inv.points < FLAT_COST {
            return (None, false);
        }
        if changed {
            inv.points -= FLAT_COST;
        }
        let it = &mut inv.items[pos];
        it.add_part_id = add_part_id;
        it.paint_id = paint_id;
        tracing::info!(id = it.item_id, uid = unique_id, add_part_id, paint_id, points = inv.points, "weapon modified");
        (Some((inv.points, inv.cash)), changed)
    })
}

/// Equips an owned skill into `slot`, unequipping whatever held that slot.
/// Returns `(ok, previous skill in that slot)`.
pub fn equip_skill(name: &str, unique_id: i32, slot: i32) -> (bool, Option<Skill>) {
    with_named(name, |inv| {
        if !inv.skills.iter().any(|s| s.unique_id == unique_id) {
            return ((false, None), false);
        }
        if slot < 0 || slot == SLOT_NONE {
            for s in &mut inv.skills {
                if s.unique_id == unique_id {
                    s.slot = -1;
                }
            }
            return ((true, None), true);
        }
        let old = inv.skills.iter().find(|s| s.slot == slot && s.unique_id != unique_id).cloned();
        for s in &mut inv.skills {
            if s.slot == slot {
                s.slot = -1;
            }
            if s.unique_id == unique_id {
                s.slot = slot;
            }
        }
        ((true, old), true)
    })
}

/// Equips an owned instance into `slot`, unequipping whatever held that slot.
/// Returns the previously equipped instance, if any.
pub fn equip(name: &str, unique_id: f64, slot: i32) -> Option<Item> {
    with_named(name, |inv| {
        if !inv.items.iter().any(|i| i.unique_id == unique_id) {
            return (None, false);
        }
        if slot < 0 || slot == SLOT_NONE {
            for it in &mut inv.items {
                if it.unique_id == unique_id {
                    it.slot = -1;
                }
            }
            return (None, true);
        }
        let old = inv.items.iter().find(|i| i.slot == slot && i.unique_id != unique_id).cloned();
        for it in &mut inv.items {
            if it.slot == slot {
                it.slot = -1;
            }
            if it.unique_id == unique_id {
                it.slot = slot;
            }
        }
        (old, true)
    })
}
