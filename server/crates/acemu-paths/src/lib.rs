//! Where the ACEmu servers keep their files, so a copy of the `server/` folder
//! is self-contained.
//!
//! [`home`] is the server folder. `data/` under it holds the tables extracted
//! from the client (items, shop, boxes, levels, weapon parts; see
//! `tools/ac-items.ps1`) and `save/` holds runtime state (accounts and
//! per-character inventories). The home is resolved once, in this order:
//!
//! 1. `ACEMU_HOME`, an explicit override;
//! 2. the directory the executable is in, if it has a `data/` folder (a shipped
//!    copy of `server/` with the binaries next to the tables);
//! 3. the current directory, then `server/` under it, if either has `data/`
//!    (`cargo run` from `server/` or from the repo root);
//! 4. the executable's directory as a last resort.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static HOME: OnceLock<PathBuf> = OnceLock::new();

/// The server folder (see the module docs for how it is found).
pub fn home() -> &'static Path {
    HOME.get_or_init(resolve)
}

fn resolve() -> PathBuf {
    if let Some(h) = std::env::var_os("ACEMU_HOME") {
        return PathBuf::from(h);
    }
    let exe_dir = std::env::current_exe().ok().and_then(|p| p.parent().map(Path::to_path_buf));
    let cwd = std::env::current_dir().ok();
    let candidates = [exe_dir.clone(), cwd.clone(), cwd.map(|c| c.join("server"))];
    for c in candidates.into_iter().flatten() {
        if c.join("data").is_dir() {
            return c;
        }
    }
    exe_dir.unwrap_or_else(|| PathBuf::from("."))
}

/// The address clients are told to reach this machine at: the channel server
/// in the login server's list, the NAT/relay responders, room owner IPs.
/// `ACEMU_PUBLIC_IP` sets it (the LAN or public IP of the host running the
/// servers); the default only works with clients on the same machine.
pub fn public_ip() -> String {
    std::env::var("ACEMU_PUBLIC_IP").ok().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).unwrap_or_else(|| "127.0.0.1".into())
}

/// A table under `<home>/data/`, e.g. `data_file("shop.json")`.
pub fn data_file(name: &str) -> PathBuf {
    home().join("data").join(name)
}

/// A state file under `<home>/save/` (the folder is created on first use).
pub fn save_file(name: &str) -> PathBuf {
    let dir = home().join("save");
    let _ = std::fs::create_dir_all(&dir);
    dir.join(name)
}
