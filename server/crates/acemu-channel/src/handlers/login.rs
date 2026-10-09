//! Connection handshake and session lifetime: hash, login (which also starts
//! the NAT diagnosis), client version, keepalive, client-side log lines,
//! server time/type and logout.

use super::Reply;
use crate::nat::{self, NAT_PORTS};
use crate::player::{char_info, default_character};
use crate::session::{self, Ctx};
use crate::{room, unix_now, local_ip};
use acemu_proto::channel;
use acemu_proto::frame::Packet;

pub fn handle(pkt: &Packet, ctx: &mut Ctx) -> Reply {
    let uid = ctx.uid;
    Ok(Some(match pkt.opcode {
        channel::ReqHash::OPCODE => {
            let req = channel::ReqHash::decode(&pkt.payload)?;
            tracing::info!(
                h0 = format_args!("{:08X}", req.hash_field0),
                h1 = format_args!("{:08X}", req.hash_field1),
                "ReqHash"
            );
            vec![channel::AckHash { result: 0, err_msg: String::new() }.packet()]
        }

        channel::ReqLogin::OPCODE => {
            let req = channel::ReqLogin::decode(&pkt.payload)?;
            tracing::info!(id = %req.id, "ReqLogin");
            if ctx.name.is_empty() {
                ctx.name = default_character();
                session::set_name(uid, &ctx.name);
            }
            vec![channel::AckLogin {
                result: 0,
                err_msg: String::new(),
                user_id: uid,
                char_name: ctx.name.clone(),
                clan_name: String::new(),
                bt_char_info: char_info(&ctx.name),
            }
            .packet()]
        }

        channel::ReqLogin2::OPCODE => {
            let req = channel::ReqLogin2::decode(&pkt.payload)?;
            tracing::info!(
                uid,
                char_name = %req.char_name,
                time_stamp = req.time_stamp,
                guid = %req.guid,
                user_type = req.user_type,
                "ReqLogin2"
            );
            // The character comes from the login server via the client; each
            // character has its own inventory file.
            ctx.name = if req.char_name.trim().is_empty() { default_character() } else { req.char_name.trim().to_string() };
            session::set_name(uid, &ctx.name);
            // A new character gets its player row now, so the admin app sees it.
            crate::inventory::ensure_exists(&ctx.name);
            vec![
                channel::AckLogin2 {
                    result: 0,
                    err_msg: String::new(),
                    user_id: uid,
                    char_name: ctx.name.clone(),
                    cur_server_time: 0.0,
                    char_dbid: uid,
                    is_gm: 0,
                }
                .packet(),
                // server-initiated. The host-loading step ("UNatChecker::sendToNatServer")
                // actually sends its 8-byte hello to the *relay server* address the
                // engine holds (GameEngine+0x11BC/+0x11CC/+0x11D8, set by
                // wMatchMaker.LowRelayServerOn from BTTcpHandler::rfReqRelayServerOn), and
                // waits for a 6-byte public-address reply. Without it: sendto 0.0.0.0:0,
                // WSAEAFNOSUPPORT, "Network setup failed (5001)".
                channel::ReqRelayServerOn {
                    relay_server_ip: local_ip(),
                    relay_server_port_num: NAT_PORTS[0] as i32,
                }
                .packet(),
                channel::ReqStartNatDiag {
                    server1addr: local_ip(),
                    server1port: NAT_PORTS[0] as i32,
                    server2addr: local_ip(),
                    server2port: NAT_PORTS[1] as i32,
                }
                .packet(),
            ]
            .into_iter()
            // Forcing a NAT type up front makes the client skip the diagnosis, and
            // the host-loading step then has no server address. Opt-in only.
            .chain(std::env::var("ACEMU_FORCE_NAT").ok().map(|v| {
                channel::ReqForceNatTypeSetting { nat_type: v.parse().unwrap_or(nat::FORCED_NAT_TYPE) }.packet()
            }))
            .collect()
        }

        channel::AckStartNatDiag::OPCODE => {
            let ack = channel::AckStartNatDiag::decode(&pkt.payload)?;
            tracing::info!(uid, nat_type = ack.nat_type, "AckStartNatDiag (client's diagnosis)");
            if ack.nat_type >= 0 {
                session::set_nat(uid, ack.nat_type as u8);
            }
            vec![]
        }

        // The client reports its own internal errors here - worth surfacing loudly.
        channel::ReqWriteServerLog::OPCODE => {
            let req = channel::ReqWriteServerLog::decode(&pkt.payload)?;
            tracing::warn!(uid, dest = req.destination, msg = %req.log_msg, "client log");
            vec![]
        }

        // First packet after connect. No ack in the protocol; just note the value.
        channel::ReqSetClientVersion::OPCODE => {
            let req = channel::ReqSetClientVersion::decode(&pkt.payload)?;
            tracing::info!(client_version = req.client_version, "ReqSetClientVersion");
            vec![]
        }

        // Keepalive: sent every 10 s with the tick limit (30) once connected.
        channel::ReqDummy::OPCODE => {
            let _ = channel::ReqDummy::decode(&pkt.payload)?;
            vec![channel::AckDummy { result: 0 }.packet()]
        }

        channel::ReqServerType::OPCODE => vec![channel::AckServerType { type_: 0 }.packet()],

        channel::ReqServerTime::OPCODE => vec![channel::AckServerTime { time: unix_now() }.packet()],

        // OLE date (days since 1899-12-30); the client keeps it as its clock for
        // rental expiry and sale-end checks.
        channel::ReqGetServerTime::OPCODE => vec![channel::AckGetServerTime {
            server_time: crate::inventory::ole_now(),
            client_ip: local_ip(),
        }
        .packet()],

        channel::ReqLogOut::OPCODE => {
            let req = channel::ReqLogOut::decode(&pkt.payload)?;
            tracing::info!(uid, reason = req.reason, "ReqLogOut");
            let left = room::leave(uid, &ctx.name);
            session::set_in_channel(uid, false);
            super::lobby::push_room_change(left);
            vec![channel::AckLogOut { result: 0, reason: req.reason }.packet()]
        }

        _ => return Ok(None),
    }))
}
