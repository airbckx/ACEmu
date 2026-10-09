//! `GET /api/servers`: is each service operational right now? Probed live on
//! every request (each probe is bounded to a second):
//!
//! - login server: TCP connect to its game port (55000) and a `GET /ping` on
//!   its launcher HTTP endpoint (55001);
//! - channel server: TCP connect to 55999, its heartbeat row in the database
//!   (written every few seconds while it runs), and the NAT-diagnosis UDP
//!   responders on 55997/55998 (they answer any datagram);
//! - admin: this process, plus whether the web app is built.

use crate::App;
use axum::extract::State;
use axum::Json;
use serde::Serialize;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, UdpSocket};
use tokio::time::timeout;

const PROBE: Duration = Duration::from_millis(1000);
const HOST: &str = "127.0.0.1";

#[derive(Serialize)]
pub struct ServerStatus {
    /// `login` | `channel` | `admin`
    pub id: String,
    pub name: String,
    pub description: String,
    /// Every check passed.
    pub operational: bool,
    pub checks: Vec<Check>,
    /// Unix seconds, when known.
    pub started_at: Option<i64>,
}

#[derive(Serialize)]
pub struct Check {
    pub label: String,
    pub target: String,
    pub ok: bool,
    pub detail: String,
}

async fn tcp_open(port: u16) -> Result<(), String> {
    match timeout(PROBE, TcpStream::connect((HOST, port))).await {
        Ok(Ok(_)) => Ok(()),
        Ok(Err(e)) => Err(e.to_string()),
        Err(_) => Err("timeout".into()),
    }
}

/// Minimal HTTP GET over a raw socket (no client library in this crate).
async fn http_get(port: u16, path: &str) -> Result<String, String> {
    let fut = async {
        let mut s = TcpStream::connect((HOST, port)).await.map_err(|e| e.to_string())?;
        let req = format!("GET {path} HTTP/1.1\r\nHost: {HOST}\r\nConnection: close\r\n\r\n");
        s.write_all(req.as_bytes()).await.map_err(|e| e.to_string())?;
        let mut buf = Vec::new();
        s.read_to_end(&mut buf).await.map_err(|e| e.to_string())?;
        Ok::<String, String>(String::from_utf8_lossy(&buf).to_string())
    };
    match timeout(PROBE, fut).await {
        Ok(r) => r,
        Err(_) => Err("timeout".into()),
    }
}

async fn udp_echo(port: u16) -> Result<usize, String> {
    let fut = async {
        let sock = UdpSocket::bind((HOST, 0)).await.map_err(|e| e.to_string())?;
        sock.send_to(&[0u8; 8], (HOST, port)).await.map_err(|e| e.to_string())?;
        let mut buf = [0u8; 64];
        let (n, _) = sock.recv_from(&mut buf).await.map_err(|e| e.to_string())?;
        Ok::<usize, String>(n)
    };
    match timeout(PROBE, fut).await {
        Ok(r) => r,
        Err(_) => Err("no reply".into()),
    }
}

fn check(label: &str, target: String, r: Result<String, String>) -> Check {
    match r {
        Ok(detail) => Check { label: label.into(), target, ok: true, detail },
        Err(detail) => Check { label: label.into(), target, ok: false, detail },
    }
}

pub async fn servers(State(db): State<App>) -> Json<Vec<ServerStatus>> {
    // --- login
    let login_tcp = tcp_open(55000).await.map(|_| "accepting connections".to_string());
    let login_http = match http_get(55001, "/ping").await {
        Ok(body) if body.contains("\"ok\":true") => Ok("launcher endpoint answers /ping".to_string()),
        Ok(body) => Err(format!("unexpected reply: {}", body.lines().next().unwrap_or(""))),
        Err(e) => Err(e),
    };
    let login_checks = vec![check("Game port", format!("tcp {HOST}:55000"), login_tcp), check("Launcher HTTP", format!("http://{HOST}:55001/ping"), login_http)];

    // --- channel
    let channel_tcp = tcp_open(55999).await.map(|_| "accepting connections".to_string());
    let heartbeat = db.get_live("channel_heartbeat").ok().flatten().map(|(_, _, age)| age);
    let started = db.get_live("channel_started_at").ok().flatten().and_then(|(v, _, _)| v.parse::<i64>().ok());
    let sessions = db
        .get_live("channel")
        .ok()
        .flatten()
        .and_then(|(v, _, _)| serde_json::from_str::<serde_json::Value>(&v).ok())
        .and_then(|v| v["sessions"].as_array().map(|a| a.len()))
        .unwrap_or(0);
    let heartbeat_check = match heartbeat {
        Some(age) if age <= 30 => Ok(format!("heartbeat {age}s ago, {sessions} session{}", if sessions == 1 { "" } else { "s" })),
        Some(age) => Err(format!("last heartbeat {age}s ago")),
        None => Err("never published a heartbeat".into()),
    };
    let nat1 = udp_echo(55997).await.map(|n| format!("replied {n} bytes"));
    let nat2 = udp_echo(55998).await.map(|n| format!("replied {n} bytes"));
    let channel_checks = vec![
        check("Game port", format!("tcp {HOST}:55999"), channel_tcp),
        check("Heartbeat", "save/acemu.db live".into(), heartbeat_check),
        check("NAT responder 1", format!("udp {HOST}:55997"), nat1),
        check("NAT responder 2", format!("udp {HOST}:55998"), nat2),
    ];

    // --- admin (this process)
    let webapp = acemu_paths::home().join("webapp").join("out").join("index.html");
    let db_path = acemu_paths::save_file(acemu_db::DB_FILE);
    let db_size = std::fs::metadata(&db_path).map(|m| m.len()).unwrap_or(0);
    let admin_checks = vec![
        check("API", "this service".into(), Ok("answering".into())),
        check(
            "Web app",
            "webapp/out".into(),
            if webapp.is_file() { Ok("built".into()) } else { Err("not built (npm run build in server/webapp)".into()) },
        ),
        check("Database", db_path.display().to_string(), Ok(format!("{} KB", db_size / 1024))),
    ];

    let mk = |id: &str, name: &str, description: &str, checks: Vec<Check>, started_at: Option<i64>| ServerStatus {
        id: id.into(),
        name: name.into(),
        description: description.into(),
        operational: checks.iter().all(|c| c.ok),
        checks,
        started_at,
    };
    Json(vec![
        mk("login", "Login server", "Authenticates the launcher account and hands the client to the channel server (acemu-login).", login_checks, None),
        mk(
            "channel",
            "Channel server",
            "Lobby, rooms, shop, inventory, quests, mail and the match hand-off (acemu-channel).",
            channel_checks,
            started,
        ),
        mk(
            "admin",
            "Admin service",
            "This API and web app (acemu-admin). Not needed to play.",
            admin_checks,
            Some(*crate::STARTED_AT),
        ),
    ])
}
