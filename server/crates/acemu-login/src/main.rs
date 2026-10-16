//! ACEmu login server.
//!
//! Answers the cold-boot path the client walks before it reaches the lobby:
//!
//!   ReqHash -> AckHash
//!   ReqClientChallenge -> AckClientChallenge
//!   ReqClientChecksum -> AckClientChecksum
//!   ReqErrorReportServerAddr -> AckErrorReportServerAddr
//!   ReqLoginWithGlobal -> AckLoginWithGlobal
//!   ReqServerList -> AckServerList
//!   ReqConnectChannelServer -> AckConnectChannelServer
//!
//! The client must be patched (`the launcher -Name no-packet-crypto`) and
//! pointed here via `ArcticCombat/System/Test.ini`.
//!
//! Semantics are still being learned, so every handler logs what it received and
//! unknown opcodes are logged by name rather than dropped silently.

mod http;

use acemu_db::model::PasswordCheck;
use acemu_proto::frame::{Decoder, Packet};
use acemu_proto::login;
use acemu_proto::wire::{self, StringEncoding};
use anyhow::Result;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::OnceLock;

static DB: OnceLock<acemu_db::Db> = OnceLock::new();

fn db() -> &'static acemu_db::Db {
    DB.get_or_init(|| acemu_db::Db::open_default().expect("cannot open save/acemu.db"))
}

/// Accounts live in `save/acemu.db` (`acemu_db`): one character per account.
///
/// The account id is what the launcher blob carries (`ac-launch.ps1 -AccountID`),
/// and the client sends it as `UserName` in `rfReqLoginInHouseTest`. Unknown
/// accounts are created on first sight without a password, so the old flow
/// keeps working; an account the admin gave a password to must present it as
/// the launcher's `-AuthKey` (`G_AuthKey` in `rfReqLoginWithGlobal`).
///
/// The account's character, creating the account if it is new.
fn character(account: &str) -> Option<String> {
    match db().ensure_account(account) {
        Ok(a) => {
            if a.character.is_none() {
                tracing::info!(account, "no character yet - the client will ask to create one");
            }
            a.character
        }
        Err(e) => {
            tracing::warn!(account, error = %e, "could not read the account");
            None
        }
    }
}

fn set_character(account: &str, name: String) {
    if let Err(e) = db().set_character(account, &name) {
        tracing::warn!(account, character = %name, error = %e, "could not persist the character");
    }
}

/// Per-connection state: which account this socket authenticated as, and
/// whether its password check failed.
#[derive(Default)]
struct Ctx {
    account: String,
    denied: bool,
}

const BIND: &str = "0.0.0.0:55000";

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "acemu_login=debug,acemu_proto=debug".into()),
        )
        .with_target(false)
        .init();

    // Null-terminated UTF-16, determined from a live ReqClientChecksum: its payload
    // was 74 bytes for (string Key1, BtrDouble Key2), i.e. 66 for the string - 32
    // UTF-16 units plus a terminator, with no length prefix. The first four bytes
    // were 20 00 70 00, which is text (" p"), not a count.
    // Override with ACEMU_STRINGS=len|bytelen|nul to re-test.
    let enc = match std::env::var("ACEMU_STRINGS").as_deref() {
        Ok("len") => StringEncoding::LenPrefixedUtf16,
        Ok("bytelen") => StringEncoding::ByteLenPrefixedUtf16,
        _ => StringEncoding::NullTerminatedUtf16,
    };
    wire::set_string_encoding(enc);
    tracing::info!(?enc, "string encoding");
    tracing::info!(home = %acemu_paths::home().display(), "server folder (save/acemu.db)");
    match db().seed_if_empty() {
        Ok(r) if r.accounts > 0 || r.players > 0 => tracing::info!(accounts = r.accounts, players = r.players, "database seeded from save/"),
        Ok(_) => {}
        Err(e) if acemu_db::is_concurrent_seed(&e) => tracing::info!("another service is seeding the database; nothing to do"),
        Err(e) => tracing::warn!(error = %e, "database seeding failed"),
    }

    // The launcher's account check (launcher/AcLauncher.exe) goes over HTTP,
    // so the admin service is not needed to play.
    let http_bind = std::env::var("ACEMU_LOGIN_HTTP").unwrap_or_else(|_| "0.0.0.0:55001".into());
    if http_bind != "off" {
        std::thread::spawn(move || http::serve(http_bind));
    }

    let listener = TcpListener::bind(BIND)?;
    tracing::info!("login server listening on {BIND}");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                std::thread::spawn(move || {
                    let peer = s.peer_addr().map(|a| a.to_string()).unwrap_or_default();
                    tracing::info!(%peer, "client connected");
                    if let Err(e) = serve(s) {
                        tracing::warn!(%peer, error = %e, "session ended");
                    } else {
                        tracing::info!(%peer, "client disconnected");
                    }
                });
            }
            Err(e) => tracing::warn!(error = %e, "accept failed"),
        }
    }
    Ok(())
}

fn serve(mut stream: TcpStream) -> Result<()> {
    stream.set_nodelay(true)?;
    let mut dec = Decoder::new();
    let mut buf = [0u8; 8192];
    let mut ctx = Ctx::default();

    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        dec.feed(&buf[..n]);

        while let Some(pkt) = dec.next_packet()? {
            // A decode failure means our signature for that opcode is wrong. That is
            // information, not a reason to drop the client - log the payload so the
            // real layout can be worked out, and keep the session alive.
            let replies = match handle(&pkt, &mut ctx) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!(
                        opcode = pkt.opcode,
                        name = login::opcode_name(pkt.opcode).unwrap_or("?"),
                        len = pkt.payload.len(),
                        error = %e,
                        payload = %hex(&pkt.payload),
                        "decode failed"
                    );
                    continue;
                }
            };
            for reply in replies {
                tracing::debug!(
                    opcode = reply.opcode,
                    name = login::opcode_name(reply.opcode).unwrap_or("?"),
                    len = reply.payload.len(),
                    "-> send"
                );
                stream.write_all(&reply.encode())?;
            }
        }
    }
}

fn unix_now() -> i32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i32)
        .unwrap_or(0)
}

/// Hex + printable rendering, for payloads whose layout we do not yet know.
fn hex(b: &[u8]) -> String {
    let h: Vec<String> = b.iter().map(|x| format!("{x:02X}")).collect();
    let a: String = b
        .iter()
        .map(|&c| if (32..127).contains(&c) { c as char } else { '.' })
        .collect();
    format!("{} | {a}", h.join(" "))
}

fn handle(pkt: &Packet, ctx: &mut Ctx) -> Result<Vec<Packet>> {
    let name = login::opcode_name(pkt.opcode).unwrap_or("<unknown>");
    tracing::debug!(opcode = pkt.opcode, name, len = pkt.payload.len(), "<- recv");
    tracing::trace!(payload = %hex(&pkt.payload), "raw");

    Ok(match pkt.opcode {
        login::ReqHash::OPCODE => {
            let req = login::ReqHash::decode(&pkt.payload)?;
            tracing::info!(
                h0 = format_args!("{:08X}", req.hash_field0),
                h1 = format_args!("{:08X}", req.hash_field1),
                h2 = format_args!("{:08X}", req.hash_field2),
                h3 = format_args!("{:08X}", req.hash_field3),
                "ReqHash"
            );
            // Result semantics unknown; 0 is the usual "ok" in this protocol
            // (rfAckCreateChar/rfAckLoginWithGlobal both use 0 for success).
            vec![login::AckHash { result: 0, err_msg: String::new() }.packet()]
        }

        login::ReqClientChallenge::OPCODE => {
            let _ = login::ReqClientChallenge::decode(&pkt.payload)?;
            // The client answers with ReqClientChecksum(Key1, Key2); echoing a
            // known pair lets us see how it transforms them.
            vec![login::AckClientChallenge {
                key1: "ACEMU-CHALLENGE".into(),
                key2: 0.0,
            }
            .packet()]
        }

        login::ReqClientChecksum::OPCODE => {
            let req = login::ReqClientChecksum::decode(&pkt.payload)?;
            tracing::info!(key1 = %req.key1, key2 = req.key2, "ReqClientChecksum");
            vec![login::AckClientChecksum { result: 0 }.packet()]
        }

        login::ReqErrorReportServerAddr::OPCODE => {
            vec![login::AckErrorReportServerAddr {
                addr: acemu_paths::public_ip(),
                port: 0,
            }
            .packet()]
        }

        login::ReqServerTime::OPCODE => vec![login::AckServerTime { time: unix_now() }.packet()],

        login::ReqLoginWithGlobal::OPCODE => {
            let req = login::ReqLoginWithGlobal::decode(&pkt.payload)?;
            // This is the packet that settles the string encoding: we already know
            // the plaintext, because ac-launch.ps1 put it there.
            tracing::info!(
                account = %req.g_account_id,
                guid = %req.g_guid,
                authkey = %req.g_auth_key,
                user_type = req.user_type,
                "ReqLoginWithGlobal - if these read back correctly the string encoding is right"
            );
            ctx.account = req.g_account_id.trim().to_string();
            // Password check: the launcher's AuthKey against the account's
            // password (accounts without one accept anything).
            let check = db().verify_password(&ctx.account, req.g_auth_key.trim()).unwrap_or_else(|e| {
                tracing::warn!(error = %e, "password check failed - letting the login through");
                PasswordCheck::NoPassword
            });
            ctx.denied = check == PasswordCheck::Wrong;
            if ctx.denied {
                tracing::warn!(account = %ctx.account, "wrong password (AuthKey) - login refused");
            }
            vec![login::AckLoginWithGlobal { result: if ctx.denied { 1 } else { 0 } }.packet()]
        }

        // The path the client actually takes. It sends UserName/Password/Guid with
        // the password hardcoded to "battery" - no credential check to satisfy.
        login::ReqLoginInHouseTest::OPCODE => {
            let req = login::ReqLoginInHouseTest::decode(&pkt.payload)?;
            tracing::info!(
                user = %req.user_name, password = %req.password, guid = %req.guid,
                "ReqLoginInHouseTest"
            );
            // The client re-sends this after creating a character, to refresh its
            // view of the account - so it must reflect the character we stored.
            if !req.user_name.trim().is_empty() {
                ctx.account = req.user_name.trim().to_string();
            }
            let existing = if ctx.denied { None } else { character(&ctx.account) };
            vec![login::AckLoginInHouseTest {
                // Despite the name, `Sucess` is a result code, not a boolean. The
                // developers left the enum in the localization file:
                //   GUIWarfareControls.gpt strClientDefine[17] =
                //     "(0) successful, (1) DB Error, (2,3) no ID, (50) already in use"
                // Sending 1 makes the client show "Login DB Error"; a refused
                // password answers 2 ("no ID").
                sucess: if ctx.denied { 2 } else { 0 },
                char_name: existing.clone().unwrap_or_default(),
                time_stamp: unix_now(),
                login_hash1: 0,
                login_hash2: 0,
                login_hash3: 0,
                login_hash4: 0,
                login_hash5: 0,
                // Inverted relative to its name: we sent 0 both before and after
                // creating a character and the client went to EPage_CharacterCreation
                // both times, so 0 means "no character yet".
                need_char: if existing.is_some() { 1 } else { 0 },
                err_msg: String::new(),
                b_day: "1990-01-01".into(),
                sex: 0,
                server_time: unix_now() as f64,
            }
            .packet()]
        }

        login::ReqProcessPreLoginUser::OPCODE => {
            let req = login::ReqProcessPreLoginUser::decode(&pkt.payload)?;
            tracing::info!(char_name = %req.char_name, ty = req.type_, "ReqProcessPreLoginUser");
            vec![login::AckProcessPreLoginUser { result: 0, err_msg: String::new() }.packet()]
        }

        // Character creation. Result codes come from the same developer note as
        // AckLoginInHouseTest's: 0 succeeds, and the client has strings for
        // "This nickname already exists" (15) and "Character Creation Error" (16).
        login::ReqCreateChar::OPCODE => {
            let req = login::ReqCreateChar::decode(&pkt.payload)?;
            tracing::info!(
                char_name = %req.char_name,
                head = req.head_item_id,
                helmet = req.helmet_item_id,
                acceso = req.acceso_item_id,
                backpack = req.backpack_item_id,
                pouch = req.pouch_item_id,
                camo = req.camouflage_item_id,
                "ReqCreateChar"
            );
            set_character(&ctx.account, req.char_name.clone());
            vec![login::AckCreateChar { success: 0, err_msg: String::new() }.packet()]
        }

        login::ReqHeadItemTypeList::OPCODE => {
            vec![login::AckHeadItemTypeList { head_item_id: vec![0] }.packet()]
        }

        login::ReqHelmetItemTypeList::OPCODE => vec![login::AckHelmetItemTypeList {
            helmet_item_name: vec!["Default".into()],
            helmet_item_id: vec![0],
        }
        .packet()],

        login::ReqAccesItemTypeList::OPCODE => vec![login::AckAccesItemTypeList {
            acceso_item_name: vec!["Default".into()],
            acceso_item_id: vec![0],
        }
        .packet()],

        // Level and EXP must be the real ones: the client's "Today's Results"
        // window (Quit Game from the server-selection page) draws its EXP bar
        // and next-rank rewards from this answer, not from the channel's.
        login::ReqCharInfo::OPCODE => {
            let req = login::ReqCharInfo::decode(&pkt.payload)?;
            let (level, exp) = db()
                .player(&req.char_name)
                .ok()
                .flatten()
                .map(|p| (p.stats.level, p.stats.exp))
                .unwrap_or((1, 0));
            tracing::info!(name = %req.char_name, level, exp, "ReqCharInfo");
            vec![login::AckCharInfo {
                result: 0,
                char_name: req.char_name,
                level,
                exp,
                clan_name: String::new(),
            }
            .packet()]
        }

        login::ReqAdvertisement::OPCODE => {
            vec![login::AckAdvertisement { result: 0, type_: 0, address: String::new() }.packet()]
        }

        login::ReqEventNotify::OPCODE => vec![login::AckEventNotify { type_: 0 }.packet()],

        login::ReqFriendsList::OPCODE => vec![
            login::AckFriendsListStart { result: 0, total_count: 0 }.packet(),
            login::AckFriendsListEnd.packet(),
        ],

        login::ReqServerList::OPCODE => vec![server_list().packet()],

        login::ReqConnectChannelServer::OPCODE => {
            let req = login::ReqConnectChannelServer::decode(&pkt.payload)?;
            tracing::info!(ip = %req.server_ip, port = req.server_port, "ReqConnectChannelServer");
            vec![login::AckConnectChannelServer {
                session_key: "ACEMU-SESSION".into(),
                time_stamp: 0,
            }
            .packet()]
        }

        login::ReqKeepAlive::OPCODE => vec![],

        // Sent when the player enters a channel: the client leaves the login
        // server and only then connects to the channel server, so without this
        // ack it sits on "Connecting to the channel" forever.
        login::ReqLogOut::OPCODE => {
            let req = login::ReqLogOut::decode(&pkt.payload)?;
            tracing::info!(reason = req.reason, "ReqLogOut");
            vec![login::AckLogOut { result: 0, reason: req.reason }.packet()]
        }

        _ => {
            tracing::warn!(opcode = pkt.opcode, name, "no handler");
            vec![]
        }
    })
}

/// One server, one channel, pointing back at us.
///
/// 24 parallel arrays that the client indexes together, so they must stay the same
/// length. Built as a single entry to keep that obvious.
fn server_list() -> login::AckServerList {
    login::AckServerList {
        c_servername: vec!["ACEmu".into()],
        c_server_short_name: vec!["AC".into()],
        server_ip: vec![acemu_paths::public_ip()],
        server_port: vec![55999],
        channel_name: vec!["Channel 1".into()],
        channel_nick_name: vec!["CH1".into()],
        channel_num: vec![0],
        ch_max_user: vec![100],
        ch_cur_user: vec![0],
        server_group_name: vec!["ACEmu".into()],
        server_group_id: vec![0],
        server_disp_order: vec![0],
        data_port: vec![55999],
        option_count_byte: vec![0],
        option_key_byte: vec![0],
        option_value_byte: vec![0],
        option_count_int: vec![0],
        option_key_int: vec![0],
        option_value_int: vec![0],
        option_count_float: vec![0],
        option_key_float: vec![0],
        option_value_float: vec![0.0],
        is_clan_channel: vec![0],
        is_use_dedicate_host: vec![0],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn server_list_arrays_stay_aligned() {
        let s = server_list();
        // the per-channel arrays are indexed together by the client
        let n = s.c_servername.len();
        for (label, len) in [
            ("c_server_short_name", s.c_server_short_name.len()),
            ("server_ip", s.server_ip.len()),
            ("server_port", s.server_port.len()),
            ("channel_name", s.channel_name.len()),
            ("channel_nick_name", s.channel_nick_name.len()),
            ("channel_num", s.channel_num.len()),
            ("ch_max_user", s.ch_max_user.len()),
            ("ch_cur_user", s.ch_cur_user.len()),
            ("server_group_name", s.server_group_name.len()),
            ("server_group_id", s.server_group_id.len()),
            ("server_disp_order", s.server_disp_order.len()),
            ("data_port", s.data_port.len()),
            ("is_clan_channel", s.is_clan_channel.len()),
            ("is_use_dedicate_host", s.is_use_dedicate_host.len()),
        ] {
            assert_eq!(len, n, "{label} has {len} entries, expected {n}");
        }
    }

    #[test]
    fn replies_to_the_real_captured_packets() {
        // the three packets the client actually sent, from the live capture
        for (opcode, payload) in [
            (56u32, vec![]),
            (
                0u32,
                vec![
                    0x7C, 0xEE, 0x23, 0x52, 0x3E, 0x15, 0x54, 0x17, 0x4D, 0x46, 0x6A, 0x00,
                    0x72, 0x24, 0x94, 0xAE,
                ],
            ),
            (52u32, vec![]),
        ] {
            let pkt = Packet::new(opcode, payload);
            let mut ctx = Ctx::default();
            let replies = handle(&pkt, &mut ctx).expect("handler should not error");
            assert_eq!(replies.len(), 1, "opcode {opcode} should produce one reply");
            assert_eq!(replies[0].opcode, opcode + 1, "Ack should be Req + 1");
        }
    }
}

