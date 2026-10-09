//! Row types shared by the channel server, the login server and the admin
//! service. They mirror the tables one to one and carry no game logic.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub has_password: bool,
    /// Character name, once the client created one. Unique across accounts.
    pub character: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordCheck {
    NoSuchAccount,
    /// The account has no password set: anything is accepted.
    NoPassword,
    Ok,
    Wrong,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

/// Fields of a `PATCH /players/{name}`: every field optional.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct StatsPatch {
    pub level: Option<i32>,
    pub exp: Option<i32>,
    pub kills: Option<i32>,
    pub deaths: Option<i32>,
    pub assists: Option<i32>,
    pub headshots: Option<i32>,
    pub team_kills: Option<i32>,
    pub wins: Option<i32>,
    pub losses: Option<i32>,
    pub play_time: Option<i32>,
}

impl Stats {
    pub fn apply(&mut self, p: &StatsPatch) {
        macro_rules! set {
            ($($f:ident),*) => { $( if let Some(v) = p.$f { self.$f = v; } )* };
        }
        set!(level, exp, kills, deaths, assists, headshots, team_kills, wins, losses, play_time);
    }
}

/// One item instance a player owns. `unique_id` is a `BtrDouble` on the wire
/// but always integral, so it is stored as an integer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemRow {
    pub unique_id: i64,
    pub item_id: i32,
    pub item_type: i32,
    /// Equipped slot, or -1 when it sits in the bag.
    pub slot: i32,
    pub stack: i32,
    pub durability: i32,
    pub add_part_id: i32,
    pub paint_id: i32,
    /// Rental expiry as an OLE date; 0 = permanent.
    pub until: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillRow {
    pub unique_id: i64,
    pub skill_id: i32,
    pub slot: i32,
    /// Rental expiry as an OLE date (`UntilTime` on the wire); 0 = permanent.
    #[serde(default)]
    pub until: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QSlotRow {
    pub id: i32,
    pub name: String,
    pub main: i64,
    pub pistol: i64,
    pub throw1: i64,
    pub throw2: i64,
    pub skills: [i32; 4],
}

/// A player with everything they own.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRecord {
    pub name: String,
    pub points: i32,
    pub cash: i32,
    pub next_unique: i64,
    pub stats: Stats,
    pub items: Vec<ItemRow>,
    pub skills: Vec<SkillRow>,
    pub qslots: Vec<QSlotRow>,
}

/// One line of the players list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSummary {
    pub name: String,
    pub level: i32,
    pub exp: i32,
    pub points: i32,
    pub cash: i32,
    pub kills: i32,
    pub deaths: i32,
    pub wins: i32,
    pub losses: i32,
    pub play_time: i32,
    pub item_count: i64,
    pub account: Option<String>,
}

/// Where an item can be obtained.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Availability {
    /// Listed in the item shop (and allowed in boxes).
    Shop,
    /// Not listed; only random boxes hand it out.
    Lootbox,
    /// Neither listed nor handed out.
    Hidden,
}

impl Availability {
    pub fn as_str(self) -> &'static str {
        match self {
            Availability::Shop => "shop",
            Availability::Lootbox => "lootbox",
            Availability::Hidden => "hidden",
        }
    }

    pub fn parse(s: &str) -> Option<Availability> {
        match s {
            "shop" => Some(Availability::Shop),
            "lootbox" => Some(Availability::Lootbox),
            "hidden" => Some(Availability::Hidden),
            _ => None,
        }
    }
}

/// One purchase option: a duration in hours (0 = permanent) and its price in
/// points or cash.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub struct PriceRow {
    #[serde(default)]
    pub idx: i32,
    pub hours: i32,
    pub points: i32,
    pub cash: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogRow {
    pub id: i32,
    pub name: String,
    /// Weapon-table type for weapons, else the shop-table kind.
    pub item_type: i32,
    /// Shop-table kind, -1 when the client's shop table does not list it.
    pub kind: i32,
    /// The client's shop category ("Store", "Cash Item", "Superior", "Rare",
    /// "Legendary", ...); empty when not in the shop table.
    pub category: String,
    pub sell_price: i32,
    pub availability: Availability,
    /// Position in the client's shop table (the order the shop lists ids in).
    pub shop_order: i32,
    pub in_shop_table: bool,
    pub prices: Vec<PriceRow>,
}

/// A catalogue row added by hand, for servers running without the game's shop
/// table: the id is the client's, everything else is the operator's choice.
#[derive(Debug, Clone, Deserialize)]
pub struct CatalogNew {
    pub id: i32,
    #[serde(default)]
    pub name: String,
    /// Weapon-table type, which decides the inventory slot the item takes.
    pub item_type: i32,
    /// Shop-table kind; defaults to `item_type`, which is right for weapons.
    #[serde(default)]
    pub kind: Option<i32>,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub sell_price: i32,
    #[serde(default)]
    pub availability: Option<Availability>,
    #[serde(default)]
    pub prices: Vec<PriceRow>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct CatalogPatch {
    pub name: Option<String>,
    pub sell_price: Option<i32>,
    pub availability: Option<Availability>,
    pub prices: Option<Vec<PriceRow>>,
}

/// Which reel of a random box an entry belongs to: the item reel (what the
/// player wins) or the bonus reel (a Gold Bar voucher or ticket awarded
/// alongside).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reel {
    Main,
    Bonus,
}

impl Reel {
    pub fn as_str(self) -> &'static str {
        match self {
            Reel::Main => "main",
            Reel::Bonus => "bonus",
        }
    }

    pub fn parse(s: &str) -> Option<Reel> {
        match s {
            "main" => Some(Reel::Main),
            "bonus" => Some(Reel::Bonus),
            _ => None,
        }
    }
}

/// One possible drop of a random box. `weight` is relative within the reel
/// (0 = never drops); `hours` is the drop's rental period (0 = permanent);
/// `featured` marks the prizes printed on the box.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoxEntryRow {
    pub item_id: i32,
    pub reel: Reel,
    pub weight: f64,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub hours: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct BoxSummary {
    pub id: i32,
    pub name: String,
    pub main_entries: i64,
    pub bonus_entries: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct Summary {
    pub players: i64,
    pub accounts: i64,
    pub items: i64,
    pub catalog: i64,
    pub shop: i64,
    pub matches: i64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SeedReport {
    pub catalog: usize,
    pub products: usize,
    pub players: usize,
    pub accounts: usize,
}
