//! Sign-in for the admin API: admin users with salted SHA-256 passwords in
//! `admin_users`, and HS256 JSON Web Tokens that every `/api` request except
//! the login must carry as `Authorization: Bearer <token>`.
//!
//! Bootstrap: on the first start with no admin user, one is created from
//! `ACEMU_ADMIN_USER` (default `admin`) and `ACEMU_ADMIN_PASSWORD`; without a
//! password a random one is generated, logged, and written to
//! `save/admin-password.txt`. The signing secret is `ACEMU_JWT_SECRET` or a
//! random one kept in `admin_settings` (so tokens survive restarts). Tokens
//! last `ACEMU_JWT_HOURS` (default 24). `acemu-admin --set-admin-password
//! <user> <password>` resets a password from the shell.

use crate::{ApiError, ApiResult, App};
use axum::extract::{Request, State};
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use axum::Json;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::sync::OnceLock;

struct AuthState {
    encoding: EncodingKey,
    decoding: DecodingKey,
    hours: i64,
}

static AUTH: OnceLock<AuthState> = OnceLock::new();

fn auth() -> &'static AuthState {
    AUTH.get().expect("auth::init() runs at startup")
}

fn random_hex(bytes: usize) -> String {
    let mut buf = vec![0u8; bytes];
    rand::thread_rng().fill_bytes(&mut buf);
    buf.iter().map(|b| format!("{b:02x}")).collect()
}

/// A password people can read off a screen: 14 chars, no look-alikes.
fn random_password() -> String {
    const ALPHABET: &[u8] = b"abcdefghjkmnpqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ23456789";
    let mut buf = [0u8; 14];
    rand::thread_rng().fill_bytes(&mut buf);
    buf.iter().map(|b| ALPHABET[(*b as usize) % ALPHABET.len()] as char).collect()
}

fn hash(salt: &str, password: &str) -> String {
    let digest = Sha256::digest(format!("{salt}:{password}").as_bytes());
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

pub fn set_password(db: &acemu_db::Db, username: &str, password: &str) -> anyhow::Result<()> {
    let username = username.trim();
    if username.is_empty() || password.is_empty() {
        anyhow::bail!("username and password must not be empty");
    }
    let salt = random_hex(16);
    db.admin_user_set(username, &salt, &hash(&salt, password))
}

/// Loads the signing secret and bootstraps the first admin user.
pub fn init(db: &acemu_db::Db) -> anyhow::Result<()> {
    let secret = match std::env::var("ACEMU_JWT_SECRET").ok().filter(|s| !s.trim().is_empty()) {
        Some(s) => s,
        None => match db.admin_setting("jwt_secret")? {
            Some(s) => s,
            None => {
                let s = random_hex(32);
                db.admin_setting_set("jwt_secret", &s)?;
                s
            }
        },
    };
    let hours: i64 = std::env::var("ACEMU_JWT_HOURS").ok().and_then(|v| v.parse().ok()).unwrap_or(24);
    let _ = AUTH.set(AuthState {
        encoding: EncodingKey::from_secret(secret.as_bytes()),
        decoding: DecodingKey::from_secret(secret.as_bytes()),
        hours,
    });

    if db.admin_users_count()? == 0 {
        let user = std::env::var("ACEMU_ADMIN_USER").ok().filter(|s| !s.trim().is_empty()).unwrap_or_else(|| "admin".into());
        let (password, generated) = match std::env::var("ACEMU_ADMIN_PASSWORD").ok().filter(|s| !s.is_empty()) {
            Some(p) => (p, false),
            None => (random_password(), true),
        };
        set_password(db, &user, &password)?;
        if generated {
            let path = acemu_paths::save_file("admin-password.txt");
            let _ = std::fs::write(&path, format!("{user}\n{password}\n"));
            tracing::warn!(user, password, file = %path.display(), "no ACEMU_ADMIN_PASSWORD set: admin password generated (change it in the web app)");
        } else {
            tracing::info!(user, "admin user created from ACEMU_ADMIN_PASSWORD");
        }
    }
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: i64,
    iat: i64,
}

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0)
}

fn issue(username: &str) -> Result<(String, i64), jsonwebtoken::errors::Error> {
    let exp = now() + auth().hours * 3600;
    let token = jsonwebtoken::encode(&Header::default(), &Claims { sub: username.to_string(), exp, iat: now() }, &auth().encoding)?;
    Ok((token, exp))
}

fn verify(token: &str) -> Option<Claims> {
    jsonwebtoken::decode::<Claims>(token, &auth().decoding, &Validation::default()).ok().map(|d| d.claims)
}

fn unauthorized(msg: &str) -> ApiError {
    ApiError(StatusCode::UNAUTHORIZED, msg.into())
}

/// Every `/api` route except the login needs a valid bearer token.
pub async fn require_token(req: Request, next: Next) -> Result<Response, ApiError> {
    let path = req.uri().path();
    if path == "/auth/login" || path.ends_with("/auth/login") {
        return Ok(next.run(req).await);
    }
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|t| !t.is_empty());
    match token.and_then(verify) {
        Some(_) => Ok(next.run(req).await),
        None => Err(unauthorized("sign in required")),
    }
}

#[derive(Deserialize)]
pub struct LoginBody {
    username: String,
    #[serde(default)]
    password: String,
}

#[derive(Serialize)]
pub struct Session {
    token: String,
    username: String,
    /// Unix seconds.
    expires_at: i64,
}

pub async fn login(State(db): State<App>, Json(body): Json<LoginBody>) -> ApiResult<Session> {
    let username = body.username.trim();
    let user = db.admin_user(username)?;
    let ok = user.as_ref().map(|u| u.password_hash == hash(&u.salt, &body.password)).unwrap_or(false);
    if !ok {
        tracing::warn!(user = username, "admin sign-in failed");
        return Err(unauthorized("wrong username or password"));
    }
    let _ = db.admin_user_touch(username);
    let (token, expires_at) = issue(username).map_err(|e| ApiError(StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    tracing::info!(user = username, "admin signed in");
    Ok(Json(Session { token, username: username.to_string(), expires_at }))
}

#[derive(Serialize)]
pub struct Me {
    username: String,
    expires_at: i64,
}

fn claims_of(req_token: Option<&str>) -> Result<Claims, ApiError> {
    req_token.and_then(verify).ok_or_else(|| unauthorized("sign in required"))
}

fn bearer(headers: &axum::http::HeaderMap) -> Option<&str> {
    headers.get(header::AUTHORIZATION).and_then(|v| v.to_str().ok()).and_then(|v| v.strip_prefix("Bearer ")).map(str::trim)
}

pub async fn me(headers: axum::http::HeaderMap) -> ApiResult<Me> {
    let c = claims_of(bearer(&headers))?;
    Ok(Json(Me { username: c.sub, expires_at: c.exp }))
}

#[derive(Deserialize)]
pub struct ChangePassword {
    #[serde(default)]
    current: String,
    new: String,
}

pub async fn change_password(State(db): State<App>, headers: axum::http::HeaderMap, Json(body): Json<ChangePassword>) -> ApiResult<serde_json::Value> {
    let c = claims_of(bearer(&headers))?;
    let user = db.admin_user(&c.sub)?.ok_or_else(|| unauthorized("unknown admin user"))?;
    if user.password_hash != hash(&user.salt, &body.current) {
        return Err(ApiError(StatusCode::BAD_REQUEST, "current password is wrong".into()));
    }
    if body.new.trim().len() < 6 {
        return Err(ApiError(StatusCode::BAD_REQUEST, "the new password needs at least 6 characters".into()));
    }
    set_password(&db, &c.sub, &body.new)?;
    // the generated-password file is stale once the password changed
    let _ = std::fs::remove_file(acemu_paths::save_file("admin-password.txt"));
    Ok(crate::ok())
}
