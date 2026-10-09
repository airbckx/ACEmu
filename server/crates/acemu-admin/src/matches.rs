//! Match history and leaderboard endpoints.
//!
//! ```text
//! GET    /api/matches?limit=&offset=&player=   newest first; player= filters to a participant
//! GET    /api/matches/{id}                      scoreboard, kill feed and the objective events
//!                                               (`events`: round ends, bomb plants/defuses, points taken, ...)
//! DELETE /api/matches/{id}
//! GET    /api/leaderboard                       career stats + history aggregates, unsorted
//! GET    /api/players/{name}/matches?limit=&offset=
//! ```

use super::{not_found, ok, ApiResult, App};
use acemu_db::matches::{LeaderRow, MatchDetail, MatchSummary};
use axum::extract::{Path, Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct MatchQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    player: Option<String>,
}

#[derive(Serialize)]
pub struct MatchPage {
    total: i64,
    matches: Vec<MatchSummary>,
}

pub async fn list_matches(State(db): State<App>, Query(q): Query<MatchQuery>) -> ApiResult<MatchPage> {
    let (total, matches) = db.matches(q.limit.unwrap_or(50), q.offset.unwrap_or(0), q.player.as_deref().filter(|p| !p.is_empty()))?;
    Ok(Json(MatchPage { total, matches }))
}

pub async fn get_match(State(db): State<App>, Path(id): Path<i64>) -> ApiResult<MatchDetail> {
    Ok(Json(db.match_detail(id)?.ok_or_else(|| not_found(format!("no match with id {id}")))?))
}

pub async fn delete_match(State(db): State<App>, Path(id): Path<i64>) -> ApiResult<serde_json::Value> {
    if !db.delete_match(id)? {
        return Err(not_found(format!("no match with id {id}")));
    }
    Ok(ok())
}

pub async fn leaderboard(State(db): State<App>) -> ApiResult<Vec<LeaderRow>> {
    Ok(Json(db.leaderboard()?))
}

pub async fn player_matches(State(db): State<App>, Path(name): Path<String>, Query(q): Query<MatchQuery>) -> ApiResult<MatchPage> {
    let (total, matches) = db.matches(q.limit.unwrap_or(20), q.offset.unwrap_or(0), Some(&name))?;
    Ok(Json(MatchPage { total, matches }))
}
