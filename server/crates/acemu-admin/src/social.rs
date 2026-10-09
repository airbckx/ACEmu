//! Admin endpoints for the social/progression tables (`acemu_db::social`):
//! coupon codes, the mailbox (including system mail to one or every player),
//! quests ("challenges") with their event windows, and per-player quest
//! progress.

use crate::{not_found, ok, ApiResult, App};
use acemu_db::social::{CouponRow, MailAttachment, MailRow, PlayerQuestPatch, PlayerQuestRow, QuestPatch, QuestRow};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// coupons

pub async fn list_coupons(State(db): State<App>) -> ApiResult<Vec<CouponRow>> {
    Ok(Json(db.coupons()?))
}

#[derive(Deserialize)]
pub struct NewCoupon {
    serial: String,
    #[serde(default)]
    item_id: i32,
    #[serde(default = "one")]
    item_count: i32,
    #[serde(default)]
    points: i32,
    #[serde(default)]
    cash: i32,
    #[serde(default = "one")]
    uses_left: i32,
    /// `YYYY-MM-DD HH:MM:SS` (UTC) or null.
    #[serde(default)]
    expires_at: Option<String>,
    #[serde(default)]
    note: String,
}

fn one() -> i32 {
    1
}

pub async fn create_coupon(State(db): State<App>, Json(body): Json<NewCoupon>) -> ApiResult<CouponRow> {
    let serial = body.serial.trim().to_uppercase();
    if serial.is_empty() {
        return Err(anyhow::anyhow!("serial is empty").into());
    }
    if body.item_id <= 0 && body.points <= 0 && body.cash <= 0 {
        return Err(anyhow::anyhow!("a coupon needs an item, points or cash").into());
    }
    if body.item_id > 0 && db.item_name(body.item_id)?.is_none() {
        return Err(not_found(format!("no catalogue item {}", body.item_id)));
    }
    db.coupon_put(&serial, body.item_id, body.item_count.max(1), body.points, body.cash, body.uses_left, &body.note)?;
    db.coupon_set_expiry(&serial, body.expires_at.as_deref().filter(|s| !s.trim().is_empty()))?;
    let row = db.coupons()?.into_iter().find(|c| c.serial == serial).ok_or_else(|| not_found("coupon vanished"))?;
    Ok(Json(row))
}

pub async fn delete_coupon(State(db): State<App>, Path(serial): Path<String>) -> ApiResult<serde_json::Value> {
    if !db.coupon_delete(&serial)? {
        return Err(not_found(format!("no coupon {serial}")));
    }
    Ok(ok())
}

// ---------------------------------------------------------------------------
// mail

#[derive(Deserialize)]
pub struct MailQuery {
    #[serde(default)]
    player: Option<String>,
    #[serde(default)]
    limit: Option<i64>,
}

pub async fn list_mail(State(db): State<App>, Query(q): Query<MailQuery>) -> ApiResult<Vec<MailRow>> {
    Ok(Json(db.mail_all(q.player.as_deref().filter(|s| !s.is_empty()), q.limit.unwrap_or(200).clamp(1, 2000))?))
}

/// `to`: a list of character names, or `["*"]` for every player.
#[derive(Deserialize)]
pub struct SendMail {
    to: Vec<String>,
    #[serde(default = "system_sender")]
    sender: String,
    text: String,
    #[serde(default)]
    item_id: i32,
    #[serde(default = "one")]
    item_count: i32,
    #[serde(default)]
    points: i32,
}

fn system_sender() -> String {
    "ACEmu".into()
}

#[derive(Serialize)]
pub struct SendMailOut {
    delivered: Vec<String>,
    failed: Vec<String>,
}

pub async fn send_mail(State(db): State<App>, Json(body): Json<SendMail>) -> ApiResult<SendMailOut> {
    let everyone = body.to.iter().any(|t| t.trim() == "*");
    let recipients: Vec<String> = if everyone {
        db.player_names()?
    } else {
        body.to.iter().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect()
    };
    if recipients.is_empty() {
        return Err(anyhow::anyhow!("no recipients").into());
    }
    if body.item_id > 0 && db.item_name(body.item_id)?.is_none() {
        return Err(not_found(format!("no catalogue item {}", body.item_id)));
    }
    let attachment = if body.item_id > 0 || body.points > 0 {
        Some(MailAttachment { item_uid: 0, item_id: body.item_id, item_part: 0, item_count: body.item_count.max(1), points: body.points.max(0) })
    } else {
        None
    };
    let known = db.player_names()?;
    let mut delivered = vec![];
    let mut failed = vec![];
    for to in recipients {
        if !known.contains(&to) {
            failed.push(to);
            continue;
        }
        match db.mail_system(&to, &body.sender, &body.text, attachment.as_ref()) {
            Ok(()) => delivered.push(to),
            Err(_) => failed.push(to),
        }
    }
    Ok(Json(SendMailOut { delivered, failed }))
}

pub async fn delete_mail(State(db): State<App>, Path(id): Path<i64>) -> ApiResult<serde_json::Value> {
    if !db.mail_delete_by_id(id)? {
        return Err(not_found(format!("no mail {id}")));
    }
    Ok(ok())
}

// ---------------------------------------------------------------------------
// quests

#[derive(Deserialize)]
pub struct QuestQuery {
    #[serde(default)]
    q: Option<String>,
    #[serde(default)]
    quest_type: Option<i32>,
    /// `events`: only rows with a date window; `enabled` / `disabled`.
    #[serde(default)]
    filter: Option<String>,
}

#[derive(Serialize)]
pub struct QuestOut {
    #[serde(flatten)]
    quest: QuestRow,
    reward_item_names: Vec<String>,
    /// Now inside the event window (or no window set).
    active: bool,
    players: i64,
    completed: i64,
}

fn quest_out(db: &acemu_db::Db, q: QuestRow, stats: Option<(i64, i64)>) -> QuestOut {
    let now = ole_now();
    let active = q.event_start <= 0.0 && q.event_end <= 0.0 || (q.event_start <= now && now <= q.event_end);
    let reward_item_names = q
        .reward_items
        .iter()
        .filter(|&&i| i > 0)
        .map(|&i| db.item_name(i).ok().flatten().unwrap_or_else(|| format!("item {i}")))
        .collect();
    let (players, completed) = stats.unwrap_or((0, 0));
    QuestOut { quest: q, reward_item_names, active, players, completed }
}

/// Now as an OLE date (days since 1899-12-30), matching what the quests table
/// stores for event windows.
fn ole_now() -> f64 {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs_f64()).unwrap_or(0.0);
    25569.0 + secs / 86400.0
}

pub async fn list_quests(State(db): State<App>, Query(q): Query<QuestQuery>) -> ApiResult<Vec<QuestOut>> {
    let needle = q.q.unwrap_or_default().to_lowercase();
    let rows = db
        .quests()?
        .into_iter()
        .filter(|r| q.quest_type.map(|t| r.quest_type == t).unwrap_or(true))
        .filter(|r| match q.filter.as_deref() {
            Some("events") => r.event_start > 0.0 || r.event_end > 0.0,
            Some("enabled") => r.enabled,
            Some("disabled") => !r.enabled,
            _ => true,
        })
        .filter(|r| {
            needle.is_empty()
                || r.description.to_lowercase().contains(&needle)
                || r.category.to_lowercase().contains(&needle)
                || r.id.to_string() == needle
        })
        .map(|r| quest_out(&db, r, None))
        .collect();
    Ok(Json(rows))
}

#[derive(Serialize)]
pub struct QuestDetail {
    #[serde(flatten)]
    quest: QuestOut,
    progress: Vec<QuestPlayerProgress>,
}

#[derive(Serialize)]
pub struct QuestPlayerProgress {
    player: String,
    progress: i32,
    complete: bool,
    claimed: bool,
}

pub async fn get_quest(State(db): State<App>, Path(id): Path<i32>) -> ApiResult<QuestDetail> {
    let q = db.quest_get(id)?.ok_or_else(|| not_found(format!("no quest {id}")))?;
    let players = db.quest_players(id)?;
    let stats = (players.len() as i64, players.iter().filter(|p| p.2).count() as i64);
    Ok(Json(QuestDetail {
        quest: quest_out(&db, q, Some(stats)),
        progress: players.into_iter().map(|(player, progress, complete, claimed)| QuestPlayerProgress { player, progress, complete, claimed }).collect(),
    }))
}

pub async fn update_quest(State(db): State<App>, Path(id): Path<i32>, Json(body): Json<QuestPatch>) -> ApiResult<QuestDetail> {
    if let Some(items) = &body.reward_items {
        for &i in items.iter().filter(|&&i| i > 0) {
            if db.item_name(i)?.is_none() {
                return Err(not_found(format!("no catalogue item {i}")));
            }
        }
    }
    db.quest_update(id, &body)?.ok_or_else(|| not_found(format!("no quest {id}")))?;
    get_quest(State(db), Path(id)).await
}

pub async fn create_quest(State(db): State<App>, Json(body): Json<QuestRow>) -> ApiResult<QuestDetail> {
    if body.id <= 0 {
        return Err(anyhow::anyhow!("quest id must be positive").into());
    }
    db.quest_create(&body)?;
    get_quest(State(db), Path(body.id)).await
}

pub async fn delete_quest(State(db): State<App>, Path(id): Path<i32>) -> ApiResult<serde_json::Value> {
    if !db.quest_delete(id)? {
        return Err(not_found(format!("no quest {id}")));
    }
    Ok(ok())
}

// ---------------------------------------------------------------------------
// per-player quests

pub async fn player_quests(State(db): State<App>, Path(name): Path<String>) -> ApiResult<Vec<PlayerQuestRow>> {
    if db.player(&name)?.is_none() {
        return Err(not_found(format!("no player {name}")));
    }
    Ok(Json(db.player_quests(&name)?))
}

#[derive(Deserialize)]
pub struct AssignQuest {
    quest_id: i32,
}

pub async fn assign_player_quest(State(db): State<App>, Path(name): Path<String>, Json(body): Json<AssignQuest>) -> ApiResult<Vec<PlayerQuestRow>> {
    if db.player(&name)?.is_none() {
        return Err(not_found(format!("no player {name}")));
    }
    if db.quest_get(body.quest_id)?.is_none() {
        return Err(not_found(format!("no quest {}", body.quest_id)));
    }
    db.quest_assign(&name, &[body.quest_id])?;
    Ok(Json(db.player_quests(&name)?))
}

pub async fn patch_player_quest(
    State(db): State<App>,
    Path((name, quest_id)): Path<(String, i32)>,
    Json(body): Json<PlayerQuestPatch>,
) -> ApiResult<Vec<PlayerQuestRow>> {
    if db.player(&name)?.is_none() {
        return Err(not_found(format!("no player {name}")));
    }
    db.player_quest_patch(&name, quest_id, &body)?;
    Ok(Json(db.player_quests(&name)?))
}

pub async fn remove_player_quest(State(db): State<App>, Path((name, quest_id)): Path<(String, i32)>) -> ApiResult<Vec<PlayerQuestRow>> {
    db.quest_remove(&name, &[quest_id])?;
    Ok(Json(db.player_quests(&name)?))
}
