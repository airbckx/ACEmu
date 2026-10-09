//! A tiny HTTP/1.1 endpoint for the launcher, next to the game port.
//!
//! Bound to `ACEMU_LOGIN_HTTP` (default `0.0.0.0:55001`, `off` disables it).
//! Hand-rolled on std: the launcher needs three small JSON calls and the
//! login server has no async runtime to spare.
//!
//! ```text
//! GET  /ping                                  -> {"ok":true,"service":"acemu-login"}
//! POST /verify  {"username","password"}       -> {"result": ok|wrong|no_password|no_such_account}
//! POST /ensure  {"username","password"}       -> {"result": ok|created|password_set|wrong|no_password}
//! GET  /extras                                -> the pack manifest (client/extras/manifest.json) or 404
//! GET  /extras/<name>                         -> one file of the pack (Range: bytes=N- honoured for resume)
//! ```
//!
//! A pack is a folder under `client/` holding the files plus a
//! `manifest.json` `{"version": "...", "files": [{"name", "size", "sha256"},
//! ...]}`; only files listed there are served. `extras` is what the
//! launcher's "Modern renderer" box installs - the dgVoodoo2 D3D9 wrapper
//! and its config, which the server operator puts there (see
//! docs/CLIENT-TWEAKS.md, `ACEMU_EXTRAS_DIR`). Nothing of the game itself is
//! served: players bring their own install.
//!
//! `/ensure` is what the launcher calls before starting the game: an unknown
//! account is created with that password, a password-less account takes a
//! non-empty password, and a wrong password is reported instead of letting
//! the game fail with "no ID".

use acemu_db::model::PasswordCheck;
use anyhow::{anyhow, Result};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

const MAX_HEADERS: usize = 64 * 1024;
const MAX_BODY: usize = 64 * 1024;

pub fn serve(bind: String) {
    let listener = match TcpListener::bind(&bind) {
        Ok(l) => l,
        Err(e) => {
            tracing::warn!(bind, error = %e, "launcher HTTP endpoint not started");
            return;
        }
    };
    tracing::info!("launcher HTTP endpoint listening on http://{bind} (GET /ping, POST /verify, POST /ensure)");
    for stream in listener.incoming().flatten() {
        std::thread::spawn(move || {
            if let Err(e) = handle(stream) {
                tracing::debug!(error = %e, "http request failed");
            }
        });
    }
}

struct Request {
    method: String,
    path: String,
    body: Vec<u8>,
    /// `Range: bytes=N-` start offset, the one form the launcher uses to resume.
    range_from: Option<u64>,
}

fn read_request(s: &mut TcpStream) -> Result<Request> {
    s.set_read_timeout(Some(Duration::from_secs(5)))?;
    let mut buf = Vec::new();
    let mut tmp = [0u8; 4096];
    let header_end = loop {
        let n = s.read(&mut tmp)?;
        if n == 0 {
            return Err(anyhow!("connection closed before the headers ended"));
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(p) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
            break p + 4;
        }
        if buf.len() > MAX_HEADERS {
            return Err(anyhow!("headers too large"));
        }
    };
    let head = String::from_utf8_lossy(&buf[..header_end]).to_string();
    let mut lines = head.lines();
    let mut parts = lines.next().unwrap_or("").split_whitespace();
    let method = parts.next().unwrap_or("").to_ascii_uppercase();
    let path = parts.next().unwrap_or("/").split('?').next().unwrap_or("/").to_string();
    let mut content_length = 0usize;
    let mut range_from = None;
    for l in lines {
        if let Some((k, v)) = l.split_once(':') {
            if k.trim().eq_ignore_ascii_case("content-length") {
                content_length = v.trim().parse().unwrap_or(0);
            } else if k.trim().eq_ignore_ascii_case("range") {
                // "bytes=N-" only; anything else is served whole (200)
                if let Some(spec) = v.trim().strip_prefix("bytes=") {
                    if let Some(start) = spec.strip_suffix('-') {
                        range_from = start.trim().parse::<u64>().ok();
                    }
                }
            }
        }
    }
    if content_length > MAX_BODY {
        return Err(anyhow!("body too large"));
    }
    let mut body = buf[header_end..].to_vec();
    while body.len() < content_length {
        let n = s.read(&mut tmp)?;
        if n == 0 {
            break;
        }
        body.extend_from_slice(&tmp[..n]);
    }
    body.truncate(content_length);
    Ok(Request { method, path, body, range_from })
}

fn respond(s: &mut TcpStream, status: &str, body: &serde_json::Value) -> Result<()> {
    let b = body.to_string();
    write!(
        s,
        "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\
         Access-Control-Allow-Origin: *\r\nAccess-Control-Allow-Methods: GET, POST, OPTIONS\r\n\
         Access-Control-Allow-Headers: Content-Type\r\nConnection: close\r\n\r\n{b}",
        b.len()
    )?;
    s.flush()?;
    Ok(())
}

#[derive(serde::Deserialize)]
struct Credentials {
    username: String,
    #[serde(default)]
    password: String,
}

/// The packs the launcher can fetch, and where each lives:
/// `ACEMU_<PACK>_DIR` or `client/<pack>/`.
const PACKS: [&str; 1] = ["extras"];

fn pack_dir(pack: &str) -> std::path::PathBuf {
    match std::env::var(format!("ACEMU_{}_DIR", pack.to_ascii_uppercase())) {
        Ok(d) if !d.trim().is_empty() => std::path::PathBuf::from(d),
        _ => acemu_paths::home().join("client").join(pack),
    }
}

/// The pack's manifest, parsed, or an error message for the client.
fn pack_manifest(pack: &str) -> std::result::Result<serde_json::Value, (&'static str, String)> {
    let text = std::fs::read_to_string(pack_dir(pack).join("manifest.json"))
        .map_err(|_| ("404 Not Found", format!("no '{pack}' pack on this server (client/{pack}/manifest.json)")))?;
    let v: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| ("500 Internal Server Error", format!("{pack}/manifest.json unreadable: {e}")))?;
    if !v.get("files").map(|f| f.is_array()).unwrap_or(false) {
        return Err(("500 Internal Server Error", format!("{pack}/manifest.json has no files array")));
    }
    Ok(v)
}

/// Whether `name` is one of the pack's listed files.
fn pack_lists(pack: &str, name: &str) -> bool {
    match pack_manifest(pack) {
        Ok(v) => v["files"].as_array().map(|a| a.iter().any(|f| f["name"].as_str() == Some(name))).unwrap_or(false),
        Err(_) => false,
    }
}

/// Streams a file with a Content-Length (the launcher shows progress from the
/// bytes it has received); `from` serves a tail as 206 for resumed downloads.
fn send_file(s: &mut TcpStream, path: &std::path::Path, content_type: &str, from: Option<u64>, missing: &str) -> Result<()> {
    use std::io::Seek;
    let mut f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return respond(s, "404 Not Found", &serde_json::json!({ "error": missing })),
    };
    let total = f.metadata()?.len();
    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
    let start = from.filter(|&o| o > 0 && o < total);
    match start {
        Some(o) => {
            f.seek(std::io::SeekFrom::Start(o))?;
            write!(
                s,
                "HTTP/1.1 206 Partial Content\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\
                 Content-Range: bytes {o}-{}/{total}\r\nAccept-Ranges: bytes\r\n\
                 Content-Disposition: attachment; filename=\"{name}\"\r\n\
                 Access-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n",
                total - o,
                total - 1
            )?;
        }
        None => write!(
            s,
            "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {total}\r\nAccept-Ranges: bytes\r\n\
             Content-Disposition: attachment; filename=\"{name}\"\r\n\
             Access-Control-Allow-Origin: *\r\nConnection: close\r\n\r\n"
        )?,
    }
    s.set_write_timeout(Some(Duration::from_secs(60)))?;
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        s.write_all(&buf[..n])?;
    }
    s.flush()?;
    Ok(())
}

/// A file name that is exactly one path component, for `/hd/<name>`.
fn plain_name(name: &str) -> Option<&str> {
    if name.is_empty() || name.contains(['/', '\\', ':']) || name.contains("..") || name.starts_with('.') {
        None
    } else {
        Some(name)
    }
}

fn handle(mut s: TcpStream) -> Result<()> {
    let req = read_request(&mut s)?;
    if req.method == "GET" {
        for pack in PACKS {
            if req.path == format!("/{pack}") {
                let (status, body) = match pack_manifest(pack) {
                    Ok(v) => ("200 OK", v),
                    Err((st, e)) => (st, serde_json::json!({ "error": e })),
                };
                return respond(&mut s, status, &body);
            }
            if let Some(name) = req.path.strip_prefix(&format!("/{pack}/")) {
                return match plain_name(name) {
                    Some(n) if pack_lists(pack, n) => {
                        send_file(&mut s, &pack_dir(pack).join(n), "application/octet-stream", req.range_from, "file missing from the pack folder")
                    }
                    _ => respond(&mut s, "404 Not Found", &serde_json::json!({ "error": format!("not a file of the '{pack}' pack") })),
                };
            }
        }
    }
    let (status, body) = match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/ping") => ("200 OK", serde_json::json!({ "ok": true, "service": "acemu-login" })),
        ("OPTIONS", _) => ("204 No Content", serde_json::json!({})),
        ("POST", "/verify") | ("POST", "/ensure") => match serde_json::from_slice::<Credentials>(&req.body) {
            Err(e) => ("400 Bad Request", serde_json::json!({ "error": format!("bad json body: {e}") })),
            Ok(c) => {
                let username = c.username.trim();
                if username.is_empty() {
                    ("400 Bad Request", serde_json::json!({ "error": "username must not be empty" }))
                } else {
                    match account_step(username, &c.password, req.path == "/ensure") {
                        Ok(r) => ("200 OK", serde_json::json!({ "result": r, "username": username })),
                        Err(e) => ("500 Internal Server Error", serde_json::json!({ "error": e.to_string() })),
                    }
                }
            }
        },
        _ => ("404 Not Found", serde_json::json!({ "error": "not found - GET /ping, POST /verify, POST /ensure" })),
    };
    respond(&mut s, status, &body)
}

fn account_step(username: &str, password: &str, ensure: bool) -> Result<&'static str> {
    let db = crate::db();
    Ok(match db.verify_password(username, password)? {
        PasswordCheck::Ok => "ok",
        PasswordCheck::Wrong => "wrong",
        PasswordCheck::NoSuchAccount => {
            if !ensure {
                "no_such_account"
            } else {
                db.create_account(username, password)?;
                tracing::info!(account = username, with_password = !password.is_empty(), "account created by the launcher");
                "created"
            }
        }
        PasswordCheck::NoPassword => {
            if !ensure || password.is_empty() {
                "no_password"
            } else {
                if let Some(a) = db.account_by_username(username)? {
                    db.update_account(a.id, None, Some(password), None)?;
                }
                tracing::info!(account = username, "password set by the launcher");
                "password_set"
            }
        }
    })
}
