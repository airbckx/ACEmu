//! ACEmu admin service (port 55990).
//!
//! A JSON API over `save/acemu.db` for editing accounts, players (stats,
//! balance, inventory) and the item catalogue (prices, shop availability), and
//! a static file server for the admin web app (`server/webapp`, built with
//! `npm run build` into `webapp/out`). The game servers read the same database
//! on every request, so edits are live without a restart.
//!
//! There is no authentication: the service binds to 127.0.0.1 unless
//! `ACEMU_ADMIN_BIND` says otherwise.

mod auth;
mod friends;
mod matches;
mod servers;
mod social;

use acemu_db::model::*;

/// Unix seconds this process started, for the servers overview.
static STARTED_AT: std::sync::LazyLock<i64> = std::sync::LazyLock::new(|| {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
});
use acemu_db::Db;
use anyhow::Result;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, patch, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};

type App = Arc<Db>;

/// Every error becomes `{ "error": "..." }`.
struct ApiError(StatusCode, String);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.0, Json(serde_json::json!({ "error": self.1 }))).into_response()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(e: anyhow::Error) -> Self {
        ApiError(StatusCode::BAD_REQUEST, e.to_string())
    }
}

fn not_found(what: impl Into<String>) -> ApiError {
    ApiError(StatusCode::NOT_FOUND, what.into())
}

fn bad_request(why: impl Into<String>) -> ApiError {
    ApiError(StatusCode::BAD_REQUEST, why.into())
}

type ApiResult<T> = std::result::Result<Json<T>, ApiError>;

fn ok() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "ok": true }))
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "acemu_admin=info,acemu_db=info,tower_http=info".into()),
        )
        .with_target(false)
        .init();

    if std::env::args().any(|a| a == "--reset") {
        return reset();
    }
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--set-admin-password") {
        let (user, pass) = match (args.get(i + 1), args.get(i + 2)) {
            (Some(u), Some(p)) => (u.clone(), p.clone()),
            _ => anyhow::bail!("usage: acemu-admin --set-admin-password <user> <password>"),
        };
        let db = Db::open_default()?;
        auth::set_password(&db, &user, &pass)?;
        println!("password set for admin user {user}");
        return Ok(());
    }
    let _ = *STARTED_AT;

    let db = Db::open_default()?;
    // The login and channel servers seed too (docker-entrypoint.sh starts all
    // three at once), so losing that race is not a startup failure.
    let seeded = match db.seed_if_empty() {
        Ok(r) => r,
        Err(e) if acemu_db::is_concurrent_seed(&e) => {
            tracing::info!("another service is seeding the database; nothing to do");
            acemu_db::model::SeedReport::default()
        }
        Err(e) => return Err(e),
    };
    tracing::info!(home = %acemu_paths::home().display(), ?seeded, "database ready (save/acemu.db)");
    auth::init(&db)?;
    let db = Arc::new(db);

    let api = Router::new()
        .route("/auth/login", post(auth::login))
        .route("/auth/me", get(auth::me))
        .route("/auth/password", post(auth::change_password))
        .route("/summary", get(summary))
        .route("/servers", get(servers::servers))
        .route("/live", get(live))
        .route("/kinds", get(kinds))
        .route("/icons", get(icons))
        .route("/accounts", get(list_accounts).post(create_account))
        .route("/accounts/verify", post(verify_account))
        .route("/accounts/{id}", patch(update_account).delete(delete_account))
        .route("/players", get(list_players))
        .route("/players/{name}", get(get_player).patch(update_player).delete(delete_player))
        .route("/players/{name}/items", post(grant_item))
        .route("/players/{name}/items/{uid}", patch(update_item).delete(remove_item))
        .route("/players/{name}/skills", post(grant_skill))
        .route("/players/{name}/skills/{uid}", delete(remove_skill))
        .route("/players/{name}/matches", get(matches::player_matches))
        .route("/players/{name}/social", get(friends::get_social))
        .route("/players/{name}/friends", post(friends::add_friend))
        .route("/players/{name}/friends/{other}", delete(friends::remove_friend))
        .route("/players/{name}/blocks", post(friends::add_block))
        .route("/players/{name}/blocks/{other}", delete(friends::remove_block))
        .route("/players/{name}/repair", post(friends::repair_all))
        .route("/matches", get(matches::list_matches))
        .route("/matches/{id}", get(matches::get_match).delete(matches::delete_match))
        .route("/leaderboard", get(matches::leaderboard))
        .route("/items", get(search_items).post(create_catalog_item))
        .route("/items/{id}", get(get_item).patch(update_catalog_item))
        .route("/boxes", get(list_boxes))
        .route("/boxes/{id}", get(get_box).patch(update_box))
        .route("/coupons", get(social::list_coupons).post(social::create_coupon))
        .route("/coupons/{serial}", delete(social::delete_coupon))
        .route("/mail", get(social::list_mail).post(social::send_mail))
        .route("/mail/{id}", delete(social::delete_mail))
        .route("/quests", get(social::list_quests).post(social::create_quest))
        .route("/quests/{id}", get(social::get_quest).patch(social::update_quest).delete(social::delete_quest))
        .route("/players/{name}/quests", get(social::player_quests).post(social::assign_player_quest))
        .route("/players/{name}/quests/{quest_id}", patch(social::patch_player_quest).delete(social::remove_player_quest));

    let webapp = acemu_paths::home().join("webapp").join("out");
    let has_webapp = webapp.join("index.html").is_file();
    // every API route except the login needs a bearer token (see auth.rs)
    let api = api.layer(axum::middleware::from_fn(auth::require_token));
    let mut app = Router::new().nest("/api", api).with_state(db);
    if has_webapp {
        app = app.fallback_service(ServeDir::new(&webapp).not_found_service(ServeFile::new(webapp.join("index.html"))));
    } else {
        app = app.fallback(no_webapp);
    }
    let app = app.layer(CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any));

    let bind = std::env::var("ACEMU_ADMIN_BIND").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = std::env::var("ACEMU_ADMIN_PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(55990);
    let listener = tokio::net::TcpListener::bind((bind.as_str(), port)).await?;
    tracing::info!(webapp = %webapp.display(), has_webapp, "admin service listening on http://{bind}:{port}");
    axum::serve(listener, app).with_graceful_shutdown(async {
        let _ = tokio::signal::ctrl_c().await;
    })
    .await?;
    Ok(())
}

/// `--reset`: moves the database and the legacy save files into
/// `save/backup-<unix time>/` and creates a fresh database seeded from the
/// `data/` tables only (no players, no accounts). Refuses while a server has
/// the database open.
fn reset() -> Result<()> {
    let save = acemu_paths::home().join("save");
    std::fs::create_dir_all(&save)?;
    let stamp = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let backup = save.join(format!("backup-{stamp}"));
    let mut moved = vec![];
    for entry in std::fs::read_dir(&save)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let is_db = name == acemu_db::DB_FILE || name.starts_with(&format!("{}-", acemu_db::DB_FILE));
        let is_legacy = (name.starts_with("acemu-inventory") && name.ends_with(".json"))
            || (name.starts_with("acemu-account") && name.ends_with(".txt"));
        if !is_db && !is_legacy {
            continue;
        }
        std::fs::create_dir_all(&backup)?;
        let target = backup.join(&name);
        std::fs::rename(entry.path(), &target).map_err(|e| {
            anyhow::anyhow!("could not move {name} (is a server still running?): {e}")
        })?;
        moved.push(name);
    }
    let db = Db::open_default()?;
    let seeded = db.seed_if_empty()?;
    let summary = db.summary()?;
    println!("moved {} file(s) to {}", moved.len(), if moved.is_empty() { "(nothing)".to_string() } else { backup.display().to_string() });
    for m in &moved {
        println!("  {m}");
    }
    println!(
        "fresh database at {}: catalogue {} items ({} in shop, {} with product-feed prices), {} players, {} accounts",
        acemu_paths::save_file(acemu_db::DB_FILE).display(),
        seeded.catalog,
        summary.shop,
        seeded.products,
        summary.players,
        summary.accounts
    );
    Ok(())
}

async fn no_webapp() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "ACEmu admin API is up at /api. The web app is not built yet: run `npm install` and `npm run build` in server/webapp, then restart.",
    )
}

// ---------------------------------------------------------------------------
// summary / kinds

async fn summary(State(db): State<App>) -> ApiResult<Summary> {
    Ok(Json(db.summary()?))
}

/// What the channel server last published: connected sessions and the room.
/// `online` is false when the heartbeat is older than 30 s (server stopped)
/// or was never written.
#[derive(Serialize)]
struct Live {
    online: bool,
    /// Unix seconds, from the channel server's own clock.
    started_at: Option<i64>,
    heartbeat_age_secs: Option<i64>,
    sessions: serde_json::Value,
    /// The first open room (older shape); `rooms` lists every open room.
    room: serde_json::Value,
    rooms: serde_json::Value,
}

async fn live(State(db): State<App>) -> ApiResult<Live> {
    let heartbeat = db.get_live("channel_heartbeat")?;
    let started = db.get_live("channel_started_at")?.and_then(|(v, _, _)| v.parse().ok());
    let state: serde_json::Value = db
        .get_live("channel")?
        .and_then(|(v, _, _)| serde_json::from_str(&v).ok())
        .unwrap_or(serde_json::json!({ "sessions": [], "room": null, "rooms": [] }));
    let age = heartbeat.map(|(_, _, age)| age);
    let online = age.map(|a| a <= 30).unwrap_or(false);
    Ok(Json(Live {
        online,
        started_at: started,
        heartbeat_age_secs: age,
        sessions: if online { state["sessions"].clone() } else { serde_json::json!([]) },
        room: if online { state["room"].clone() } else { serde_json::Value::Null },
        rooms: if online && state["rooms"].is_array() { state["rooms"].clone() } else { serde_json::json!([]) },
    }))
}

#[derive(Serialize)]
struct Kind {
    kind: i32,
    label: &'static str,
}

async fn kinds() -> Json<Vec<Kind>> {
    Json(
        [
            (0, "pistol"),
            (1, "main"),
            (3, "throw"),
            (6, "helmet"),
            (7, "face"),
            (8, "accessory"),
            (9, "backpack"),
            (10, "costume"),
            (11, "sight"),
            (12, "silencer"),
            (15, "skill"),
            (16, "item"),
            (-1, "not in shop"),
        ]
        .into_iter()
        .map(|(kind, label)| Kind { kind, label })
        .collect(),
    )
}

/// Item id -> shop icon base name, from `data/icons.json` (extracted from the
/// client's item resource table). The web app renders `/icons/<name>.png`
/// (256x64) and `/icons-hd/<name>.png` next to item names. An empty object
/// when the file is missing, so the web app just shows placeholders.
async fn icons() -> Json<serde_json::Map<String, serde_json::Value>> {
    #[derive(Deserialize)]
    struct Row {
        id: i64,
        icon: String,
    }
    let path = acemu_paths::data_file("icons.json");
    let rows: Vec<Row> = std::fs::read_to_string(&path).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    let mut map = serde_json::Map::new();
    for r in rows.into_iter().filter(|r| !r.icon.is_empty()) {
        map.insert(r.id.to_string(), serde_json::Value::String(r.icon));
    }
    Json(map)
}

// ---------------------------------------------------------------------------
// accounts

async fn list_accounts(State(db): State<App>) -> ApiResult<Vec<Account>> {
    Ok(Json(db.accounts()?))
}

#[derive(Deserialize)]
struct NewAccount {
    username: String,
    #[serde(default)]
    password: String,
}

async fn create_account(State(db): State<App>, Json(body): Json<NewAccount>) -> ApiResult<Account> {
    Ok(Json(db.create_account(&body.username, &body.password)?))
}

#[derive(Serialize)]
struct Verified {
    /// `ok`, `wrong`, `no_password` (the account accepts anything) or
    /// `no_such_account`.
    result: &'static str,
    account: Option<Account>,
}

/// What the login server will decide for these credentials, so a launcher can
/// tell the player before the game starts (and create the account if needed).
async fn verify_account(State(db): State<App>, Json(body): Json<NewAccount>) -> ApiResult<Verified> {
    let username = body.username.trim();
    let result = match db.verify_password(username, &body.password)? {
        PasswordCheck::Ok => "ok",
        PasswordCheck::Wrong => "wrong",
        PasswordCheck::NoPassword => "no_password",
        PasswordCheck::NoSuchAccount => "no_such_account",
    };
    Ok(Json(Verified { result, account: db.account_by_username(username)? }))
}

#[derive(Deserialize)]
struct AccountPatch {
    username: Option<String>,
    password: Option<String>,
    /// Absent: unchanged. `null`: unlink. String: link to that character.
    #[serde(default)]
    character: Option<serde_json::Value>,
}

async fn update_account(State(db): State<App>, Path(id): Path<i64>, Json(body): Json<AccountPatch>) -> ApiResult<Account> {
    let character = match &body.character {
        None => None,
        Some(serde_json::Value::Null) => Some(None),
        Some(serde_json::Value::String(s)) => Some(Some(s.as_str())),
        Some(_) => return Err(ApiError(StatusCode::BAD_REQUEST, "character must be a string or null".into())),
    };
    if db.account_by_id(id)?.is_none() {
        return Err(not_found(format!("no account with id {id}")));
    }
    Ok(Json(db.update_account(id, body.username.as_deref(), body.password.as_deref(), character)?))
}

#[derive(Deserialize)]
struct DeleteAccountQuery {
    #[serde(default)]
    with_player: bool,
}

async fn delete_account(State(db): State<App>, Path(id): Path<i64>, Query(q): Query<DeleteAccountQuery>) -> ApiResult<serde_json::Value> {
    if !db.delete_account(id, q.with_player)? {
        return Err(not_found(format!("no account with id {id}")));
    }
    Ok(ok())
}

// ---------------------------------------------------------------------------
// players

async fn list_players(State(db): State<App>) -> ApiResult<Vec<PlayerSummary>> {
    Ok(Json(db.players()?))
}

#[derive(Serialize)]
struct ItemOut {
    unique_id: i64,
    item_id: i32,
    name: String,
    item_type: i32,
    slot: i32,
    stack: i32,
    durability: i32,
    add_part_id: i32,
    paint_id: i32,
    until: f64,
}

#[derive(Serialize)]
struct SkillOut {
    unique_id: i64,
    skill_id: i32,
    name: String,
    slot: i32,
}

#[derive(Serialize)]
struct PlayerOut {
    name: String,
    points: i32,
    cash: i32,
    next_unique: i64,
    stats: Stats,
    items: Vec<ItemOut>,
    skills: Vec<SkillOut>,
    qslots: Vec<QSlotRow>,
    account: Option<String>,
}

fn item_out(db: &Db, i: &ItemRow) -> ItemOut {
    ItemOut {
        unique_id: i.unique_id,
        item_id: i.item_id,
        name: db.item_name(i.item_id).ok().flatten().unwrap_or_else(|| format!("item {}", i.item_id)),
        item_type: i.item_type,
        slot: i.slot,
        stack: i.stack,
        durability: i.durability,
        add_part_id: i.add_part_id,
        paint_id: i.paint_id,
        until: i.until,
    }
}

fn skill_out(db: &Db, s: &SkillRow) -> SkillOut {
    SkillOut {
        unique_id: s.unique_id,
        skill_id: s.skill_id,
        name: db.item_name(s.skill_id).ok().flatten().unwrap_or_else(|| format!("skill {}", s.skill_id)),
        slot: s.slot,
    }
}

fn player_out(db: &Db, rec: PlayerRecord) -> Result<PlayerOut> {
    let account = db.account_of_character(&rec.name)?.map(|a| a.username);
    Ok(PlayerOut {
        items: rec.items.iter().map(|i| item_out(db, i)).collect(),
        skills: rec.skills.iter().map(|s| skill_out(db, s)).collect(),
        name: rec.name,
        points: rec.points,
        cash: rec.cash,
        next_unique: rec.next_unique,
        stats: rec.stats,
        qslots: rec.qslots,
        account,
    })
}

async fn get_player(State(db): State<App>, Path(name): Path<String>) -> ApiResult<PlayerOut> {
    let rec = db.player(&name)?.ok_or_else(|| not_found(format!("no player named {name}")))?;
    Ok(Json(player_out(&db, rec)?))
}

#[derive(Deserialize)]
struct PlayerPatch {
    points: Option<i32>,
    cash: Option<i32>,
    #[serde(default)]
    stats: StatsPatch,
}

async fn update_player(State(db): State<App>, Path(name): Path<String>, Json(body): Json<PlayerPatch>) -> ApiResult<PlayerOut> {
    let rec = db
        .update_balance_and_stats(&name, body.points, body.cash, &body.stats)?
        .ok_or_else(|| not_found(format!("no player named {name}")))?;
    Ok(Json(player_out(&db, rec)?))
}

async fn delete_player(State(db): State<App>, Path(name): Path<String>) -> ApiResult<serde_json::Value> {
    if !db.delete_player(&name)? {
        return Err(not_found(format!("no player named {name}")));
    }
    Ok(ok())
}

#[derive(Deserialize)]
struct GrantItem {
    item_id: i32,
    #[serde(default = "one")]
    stack: i32,
}

fn one() -> i32 {
    1
}

/// Raw durability of a fresh item (see the channel server's `full_durability`).
fn full_durability() -> i32 {
    std::env::var("ACEMU_DURABILITY").ok().and_then(|v| v.parse().ok()).unwrap_or(100_000)
}

async fn grant_item(State(db): State<App>, Path(name): Path<String>, Json(body): Json<GrantItem>) -> ApiResult<ItemOut> {
    let cat = db.catalog_item(body.item_id)?.ok_or_else(|| not_found(format!("no catalogue item {}", body.item_id)))?;
    if cat.kind == 15 {
        return Err(ApiError(StatusCode::BAD_REQUEST, "that id is a skill - use the skills endpoint".into()));
    }
    let row = db
        .grant_item(&name, body.item_id, cat.item_type, body.stack, full_durability())?
        .ok_or_else(|| not_found(format!("no player named {name}")))?;
    Ok(Json(item_out(&db, &row)))
}

#[derive(Deserialize)]
struct ItemPatch {
    stack: Option<i32>,
    durability: Option<i32>,
    slot: Option<i32>,
}

async fn update_item(State(db): State<App>, Path((name, uid)): Path<(String, i64)>, Json(body): Json<ItemPatch>) -> ApiResult<ItemOut> {
    let row = db
        .update_item(&name, uid, body.stack, body.durability, body.slot)?
        .ok_or_else(|| not_found(format!("{name} has no item {uid}")))?;
    Ok(Json(item_out(&db, &row)))
}

async fn remove_item(State(db): State<App>, Path((name, uid)): Path<(String, i64)>) -> ApiResult<serde_json::Value> {
    if !db.remove_item(&name, uid)? {
        return Err(not_found(format!("{name} has no item {uid}")));
    }
    Ok(ok())
}

#[derive(Deserialize)]
struct GrantSkill {
    skill_id: i32,
}

async fn grant_skill(State(db): State<App>, Path(name): Path<String>, Json(body): Json<GrantSkill>) -> ApiResult<SkillOut> {
    let row = db.grant_skill(&name, body.skill_id)?.ok_or_else(|| not_found(format!("no player named {name}")))?;
    Ok(Json(skill_out(&db, &row)))
}

async fn remove_skill(State(db): State<App>, Path((name, uid)): Path<(String, i64)>) -> ApiResult<serde_json::Value> {
    if !db.remove_skill(&name, uid)? {
        return Err(not_found(format!("{name} has no skill {uid}")));
    }
    Ok(ok())
}

// ---------------------------------------------------------------------------
// catalogue

#[derive(Deserialize)]
struct SearchQuery {
    #[serde(default)]
    q: String,
    kind: Option<i32>,
    availability: Option<Availability>,
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Serialize)]
struct SearchOut {
    total: i64,
    items: Vec<CatalogRow>,
}

async fn search_items(State(db): State<App>, Query(q): Query<SearchQuery>) -> ApiResult<SearchOut> {
    let (total, items) = db.catalog_search(&q.q, q.kind, q.availability, q.limit.unwrap_or(50), q.offset.unwrap_or(0))?;
    Ok(Json(SearchOut { total, items }))
}

/// Adds a catalogue row. Needed on a server running without the game's shop
/// table: without a catalogue row an id cannot be sold, granted or boxed.
async fn create_catalog_item(State(db): State<App>, Json(body): Json<CatalogNew>) -> ApiResult<CatalogRow> {
    if body.id <= 0 {
        return Err(bad_request("id must be a positive item id"));
    }
    Ok(Json(db.catalog_create(&body)?.ok_or_else(|| bad_request(format!("catalogue item {} already exists", body.id)))?))
}

async fn get_item(State(db): State<App>, Path(id): Path<i32>) -> ApiResult<CatalogRow> {
    Ok(Json(db.catalog_item(id)?.ok_or_else(|| not_found(format!("no catalogue item {id}")))?))
}

async fn update_catalog_item(State(db): State<App>, Path(id): Path<i32>, Json(body): Json<CatalogPatch>) -> ApiResult<CatalogRow> {
    Ok(Json(db.catalog_update(id, &body)?.ok_or_else(|| not_found(format!("no catalogue item {id}")))?))
}

// ---------------------------------------------------------------------------
// random boxes

#[derive(Serialize)]
struct BoxEntryOut {
    item_id: i32,
    name: String,
    reel: Reel,
    weight: f64,
    featured: bool,
    hours: i32,
    /// Share of this reel's total weight, in percent.
    percent: f64,
}

#[derive(Serialize)]
struct BoxOut {
    id: i32,
    name: String,
    entries: Vec<BoxEntryOut>,
}

fn box_out(db: &Db, id: i32, entries: Vec<BoxEntryRow>) -> Result<BoxOut> {
    let name = db.item_name(id)?.unwrap_or_else(|| format!("box {id}"));
    let total = |reel: Reel| entries.iter().filter(|e| e.reel == reel).map(|e| e.weight.max(0.0)).sum::<f64>();
    let totals = (total(Reel::Main), total(Reel::Bonus));
    let out = entries
        .iter()
        .map(|e| {
            let t = if e.reel == Reel::Main { totals.0 } else { totals.1 };
            BoxEntryOut {
                item_id: e.item_id,
                name: db.item_name(e.item_id).ok().flatten().unwrap_or_else(|| format!("item {}", e.item_id)),
                reel: e.reel,
                weight: e.weight,
                featured: e.featured,
                hours: e.hours,
                percent: if t > 0.0 { (e.weight.max(0.0) / t * 1000.0).round() / 10.0 } else { 0.0 },
            }
        })
        .collect();
    Ok(BoxOut { id, name, entries: out })
}

async fn list_boxes(State(db): State<App>) -> ApiResult<Vec<BoxSummary>> {
    Ok(Json(db.boxes()?))
}

async fn get_box(State(db): State<App>, Path(id): Path<i32>) -> ApiResult<BoxOut> {
    let entries = db.box_entries(id)?;
    if entries.is_empty() && db.item_name(id)?.is_none() {
        return Err(not_found(format!("no box {id}")));
    }
    Ok(Json(box_out(&db, id, entries)?))
}

#[derive(Deserialize)]
struct BoxPatch {
    entries: Vec<BoxEntryRow>,
}

async fn update_box(State(db): State<App>, Path(id): Path<i32>, Json(body): Json<BoxPatch>) -> ApiResult<BoxOut> {
    if db.item_name(id)?.is_none() {
        return Err(not_found(format!("no catalogue item {id} to be a box")));
    }
    db.box_replace(id, &body.entries)?;
    Ok(Json(box_out(&db, id, db.box_entries(id)?)?))
}
