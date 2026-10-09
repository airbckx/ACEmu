//! Admin endpoints for the social graph (`acemu_db::friends`): a player's
//! friends and block list, and a repair-all shortcut for their inventory.

use crate::{not_found, ok, ApiResult, App};
use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct FriendOut {
    name: String,
    level: i32,
    /// Whether the friend has blocked this player (one-directional).
    blocked_you: bool,
}

#[derive(Serialize)]
pub struct SocialOut {
    friends: Vec<FriendOut>,
    blocks: Vec<String>,
    /// Players who have blocked this one.
    blocked_by: Vec<String>,
}

fn social_out(db: &acemu_db::Db, name: &str) -> anyhow::Result<SocialOut> {
    let friends = db
        .friends_of(name)?
        .into_iter()
        .map(|f| {
            let level = db.player(&f).ok().flatten().map(|p| p.stats.level).unwrap_or(0);
            let blocked_you = db.is_blocked(&f, name).unwrap_or(false);
            FriendOut { name: f, level, blocked_you }
        })
        .collect();
    let blocks = db.blocks_of(name)?;
    let blocked_by = db.player_names()?.into_iter().filter(|p| p != name && db.is_blocked(p, name).unwrap_or(false)).collect();
    Ok(SocialOut { friends, blocks, blocked_by })
}

fn require_player(db: &acemu_db::Db, name: &str) -> Result<(), crate::ApiError> {
    if db.player(name)?.is_none() {
        return Err(not_found(format!("no player named {name}")));
    }
    Ok(())
}

pub async fn get_social(State(db): State<App>, Path(name): Path<String>) -> ApiResult<SocialOut> {
    require_player(&db, &name)?;
    Ok(Json(social_out(&db, &name)?))
}

#[derive(Deserialize)]
pub struct OtherPlayer {
    name: String,
}

pub async fn add_friend(State(db): State<App>, Path(name): Path<String>, Json(body): Json<OtherPlayer>) -> ApiResult<SocialOut> {
    let other = body.name.trim().to_string();
    require_player(&db, &name)?;
    require_player(&db, &other)?;
    if other == name {
        return Err(anyhow::anyhow!("a player cannot befriend themselves").into());
    }
    db.friend_add(&name, &other)?;
    Ok(Json(social_out(&db, &name)?))
}

pub async fn remove_friend(State(db): State<App>, Path((name, other)): Path<(String, String)>) -> ApiResult<SocialOut> {
    require_player(&db, &name)?;
    if !db.friend_remove(&name, &other)? {
        return Err(not_found(format!("{name} and {other} are not friends")));
    }
    Ok(Json(social_out(&db, &name)?))
}

pub async fn add_block(State(db): State<App>, Path(name): Path<String>, Json(body): Json<OtherPlayer>) -> ApiResult<SocialOut> {
    let other = body.name.trim().to_string();
    require_player(&db, &name)?;
    require_player(&db, &other)?;
    if other == name {
        return Err(anyhow::anyhow!("a player cannot block themselves").into());
    }
    db.block_add(&name, &other)?;
    Ok(Json(social_out(&db, &name)?))
}

pub async fn remove_block(State(db): State<App>, Path((name, other)): Path<(String, String)>) -> ApiResult<SocialOut> {
    require_player(&db, &name)?;
    if !db.block_remove(&name, &other)? {
        return Err(not_found(format!("{name} has not blocked {other}")));
    }
    Ok(Json(social_out(&db, &name)?))
}

/// Sets every item of the player back to full durability (what the in-game
/// repair does, without charging points).
pub async fn repair_all(State(db): State<App>, Path(name): Path<String>) -> ApiResult<serde_json::Value> {
    let rec = db.player(&name)?.ok_or_else(|| not_found(format!("no player named {name}")))?;
    let full = crate::full_durability();
    let mut repaired = 0;
    for it in rec.items.iter().filter(|i| i.durability < full) {
        if db.update_item(&name, it.unique_id, None, Some(full), None)?.is_some() {
            repaired += 1;
        }
    }
    let _ = ok();
    Ok(Json(serde_json::json!({ "ok": true, "repaired": repaired })))
}
