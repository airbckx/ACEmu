//! NAT diagnosis: the STUN-like UDP responder the client is pointed at after
//! login, and the NAT type handed to room guests.

use crate::hex;

/// UDP ports for the client's NAT diagnosis. The retail game pointed the client at
/// two STUN-like servers; without any address the host-loading step sends to
/// 0.0.0.0:0, fails with WSAEAFNOSUPPORT and the start is aborted (error 5001).
pub const NAT_PORTS: [u16; 2] = [55997, 55998];
/// Value pushed with `rfReqForceNatTypeSetting`. -1 is "not determined" in the
/// client's logs; 1 is assumed to be the most permissive type.
pub const FORCED_NAT_TYPE: i32 = 1;

/// NAT type handed to room guests (`ACEMU_GUEST_NAT` overrides it).
pub fn guest_nat_type() -> i32 {
    std::env::var("ACEMU_GUEST_NAT").ok().and_then(|v| v.parse().ok()).unwrap_or(FORCED_NAT_TYPE)
}

/// STUN-style NAT diagnosis server. The client's `UNatChecker::sendToNatServer`
/// sends 8 zero bytes; `receivedPacketFromNatServer` accepts exactly 6 bytes and
/// reads them as the sender's public address - 4 IP octets then a 16-bit port
/// (IpDrv.dll, `cmp [len],6` / `movzx word [buf+4]`). Reply with the address we
/// saw the datagram come from.
pub fn udp_nat_server(port: u16) {
    let sock = match std::net::UdpSocket::bind(("0.0.0.0", port)) {
        Ok(s) => s,
        Err(e) => {
            tracing::warn!(port, error = %e, "udp bind failed");
            return;
        }
    };
    tracing::info!(port, "nat diag server listening");
    let mut buf = [0u8; 2048];
    loop {
        if let Ok((n, from)) = sock.recv_from(&mut buf) {
            tracing::info!(port, %from, len = n, payload = %hex(&buf[..n]), "nat diag recv");
            let ip = match from.ip() {
                std::net::IpAddr::V4(v4) => v4.octets(),
                _ => [127, 0, 0, 1],
            };
            let mut reply = [0u8; 6];
            reply[..4].copy_from_slice(&ip);
            reply[4..].copy_from_slice(&from.port().to_le_bytes());
            let _ = sock.send_to(&reply, from);
        }
    }
}
