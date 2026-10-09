//! BatteryNet packet framing and stream reassembly.
//!
//! Confirmed against live capture (a live capture):
//!
//! ```text
//! offset  size  field
//!      0     4  int32  Length   total packet size, including this field
//!      4     4  int32  Magic    always 0x00030D41 (200001)
//!      8     4  int32  Opcode
//!     12     n  params
//! ```
//!
//! Note this is the *plaintext* framing. A stock client wraps it in a further
//! `int32 length | ciphertext padded to 16 | 0x08` envelope; ACEmu expects the
//! client to be patched with `no-packet-crypto` (see `the launcher`).

use anyhow::{bail, Result};

/// The constant in every header. Recovered independently by the earlier ArcticEmu
/// project and since confirmed on the wire.
pub const MAGIC: u32 = 200_001;

pub const HEADER_LEN: usize = 12;

/// Refuse anything larger rather than trusting a length field from the network.
pub const MAX_PACKET: usize = 1 << 20;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packet {
    pub magic: u32,
    pub opcode: u32,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(opcode: u32, payload: Vec<u8>) -> Self {
        Self { magic: MAGIC, opcode, payload }
    }

    pub fn encode(&self) -> Vec<u8> {
        let len = HEADER_LEN + self.payload.len();
        let mut out = Vec::with_capacity(len);
        out.extend_from_slice(&(len as u32).to_le_bytes());
        out.extend_from_slice(&self.magic.to_le_bytes());
        out.extend_from_slice(&self.opcode.to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }
}

/// Accumulates bytes from a socket and yields whole packets.
///
/// The earlier C# implementation got this wrong in two ways that are easy to repeat:
/// it discarded the tail of each read, and it handled at most one packet per read.
/// Our very first capture had two packets coalesced into one 58-byte read, so both
/// bugs would have shown up immediately.
#[derive(Default)]
pub struct Decoder {
    buf: Vec<u8>,
}

impl Decoder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn feed(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    pub fn buffered(&self) -> usize {
        self.buf.len()
    }

    /// Pops the next complete packet, or `Ok(None)` if more bytes are needed.
    pub fn next_packet(&mut self) -> Result<Option<Packet>> {
        if self.buf.len() < 4 {
            return Ok(None);
        }
        let len = u32::from_le_bytes(self.buf[0..4].try_into().unwrap()) as usize;

        if len < HEADER_LEN {
            bail!("packet length {len} is smaller than the {HEADER_LEN}-byte header");
        }
        if len > MAX_PACKET {
            bail!("packet length {len} exceeds MAX_PACKET ({MAX_PACKET})");
        }
        if self.buf.len() < len {
            return Ok(None);
        }

        let magic = u32::from_le_bytes(self.buf[4..8].try_into().unwrap());
        let opcode = u32::from_le_bytes(self.buf[8..12].try_into().unwrap());
        let payload = self.buf[HEADER_LEN..len].to_vec();
        self.buf.drain(..len);

        if magic != MAGIC {
            // Almost always means the client is unpatched and this is ciphertext.
            bail!(
                "bad magic 0x{magic:08X} (expected 0x{MAGIC:08X}) - is the client \
                 patched with no-packet-crypto?"
            );
        }
        Ok(Some(Packet { magic, opcode, payload }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw(opcode: u32, payload: &[u8]) -> Vec<u8> {
        Packet::new(opcode, payload.to_vec()).encode()
    }

    #[test]
    fn decodes_the_real_captured_bytes() {
        // straight from a live capture
        let bytes = [
            0x0C, 0x00, 0x00, 0x00, 0x41, 0x0D, 0x03, 0x00, 0x38, 0x00, 0x00, 0x00, // op 56
            0x1C, 0x00, 0x00, 0x00, 0x41, 0x0D, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, // op 0
            0x7C, 0xEE, 0x23, 0x52, 0x3E, 0x15, 0x54, 0x17, 0x4D, 0x46, 0x6A, 0x00,
            0x72, 0x24, 0x94, 0xAE,
        ];
        let mut d = Decoder::new();
        d.feed(&bytes);

        let p1 = d.next_packet().unwrap().unwrap();
        assert_eq!(p1.opcode, 56); // rfReqErrorReportServerAddr()
        assert!(p1.payload.is_empty());

        let p2 = d.next_packet().unwrap().unwrap();
        assert_eq!(p2.opcode, 0); // rfReqHash(int,int,int,int)
        assert_eq!(p2.payload.len(), 16);

        assert!(d.next_packet().unwrap().is_none());
        assert_eq!(d.buffered(), 0);
    }

    #[test]
    fn reassembles_across_arbitrary_splits() {
        let mut stream = raw(1, &[1, 2, 3, 4]);
        stream.extend_from_slice(&raw(2, &[]));
        stream.extend_from_slice(&raw(3, &[9; 40]));

        for chunk in 1..=stream.len() {
            let mut d = Decoder::new();
            let mut got = Vec::new();
            for part in stream.chunks(chunk) {
                d.feed(part);
                while let Some(p) = d.next_packet().unwrap() {
                    got.push(p.opcode);
                }
            }
            assert_eq!(got, vec![1, 2, 3], "failed at chunk size {chunk}");
            assert_eq!(d.buffered(), 0, "leftover at chunk size {chunk}");
        }
    }

    #[test]
    fn round_trip() {
        let p = Packet::new(67, vec![7; 12]);
        let mut d = Decoder::new();
        d.feed(&p.encode());
        assert_eq!(d.next_packet().unwrap().unwrap(), p);
    }

    #[test]
    fn rejects_nonsense_lengths() {
        let mut d = Decoder::new();
        d.feed(&[3, 0, 0, 0]);
        assert!(d.next_packet().is_err());

        let mut d = Decoder::new();
        d.feed(&0xFFFF_FFFFu32.to_le_bytes());
        assert!(d.next_packet().is_err());
    }

    #[test]
    fn bad_magic_is_reported() {
        let mut bytes = raw(5, &[]);
        bytes[4] = 0xFF; // corrupt the magic
        let mut d = Decoder::new();
        d.feed(&bytes);
        let err = d.next_packet().unwrap_err().to_string();
        assert!(err.contains("bad magic"), "{err}");
    }
}
