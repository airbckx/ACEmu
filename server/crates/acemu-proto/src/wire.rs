//! Little-endian readers/writers for BatteryNet payload values.
//!
//! Scalar encodings are confirmed against live capture (see docs/PROTOCOL.md).
//! Strings are null-terminated UTF-16 (confirmed from `rfReqClientChecksum`).
//! Arrays are opaque sub-buffers, confirmed against the client's
//! `BTBuffer::ReadBuffer` and `BTSocket_Login::rfAckServerList` - see
//! [`Writer::array`] and `docs/PROTOCOL.md`.

use anyhow::{bail, Result};

/// How a `string` is laid out inside a payload.
///
/// The protocol uses [`StringEncoding::NullTerminatedUtf16`]; the alternatives are
/// kept only so a wrong guess can be re-tested from the command line
/// (`ACEMU_STRINGS`) without touching generated code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringEncoding {
    /// `int32` count of UTF-16 code units (excluding terminator), then the units.
    LenPrefixedUtf16,
    /// `int32` count of *bytes*, then UTF-16 units.
    ByteLenPrefixedUtf16,
    /// UTF-16 units terminated by a `0x0000`. This is the real one.
    NullTerminatedUtf16,
    /// `int16` count of UTF-16 code units, then the units, no terminator.
    Len16PrefixedUtf16,
}

/// The encoding used for every string in this process.
///
/// A global rather than a parameter because it is a property of the protocol we are
/// still determining, not of any individual message.
static STRING_ENCODING: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

pub fn set_string_encoding(e: StringEncoding) {
    let v = match e {
        StringEncoding::LenPrefixedUtf16 => 0,
        StringEncoding::ByteLenPrefixedUtf16 => 1,
        StringEncoding::NullTerminatedUtf16 => 2,
        StringEncoding::Len16PrefixedUtf16 => 3,
    };
    STRING_ENCODING.store(v, std::sync::atomic::Ordering::Relaxed);
}

pub fn string_encoding() -> StringEncoding {
    match STRING_ENCODING.load(std::sync::atomic::Ordering::Relaxed) {
        1 => StringEncoding::ByteLenPrefixedUtf16,
        2 => StringEncoding::NullTerminatedUtf16,
        3 => StringEncoding::Len16PrefixedUtf16,
        _ => StringEncoding::LenPrefixedUtf16,
    }
}

// ---------------------------------------------------------------------------

pub struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn position(&self) -> usize {
        self.pos
    }

    pub fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Bytes not consumed by the decoder. A non-empty tail after decoding a packet
    /// means our signature for that opcode is wrong — worth logging loudly.
    pub fn rest(&self) -> &'a [u8] {
        &self.buf[self.pos..]
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            bail!(
                "short read: wanted {} bytes at offset {}, {} remain",
                n,
                self.pos,
                self.remaining()
            );
        }
        let s = &self.buf[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }

    pub fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    /// `ushort` in the spec: the native signature narrows some UnrealScript `int`
    /// parameters (room and channel numbers, option flags) to `unsigned short`.
    pub fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.take(2)?.try_into().unwrap()))
    }

    pub fn i32(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    pub fn u32(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    pub fn f32(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.take(4)?.try_into().unwrap()))
    }

    /// `BtrDouble` — UnrealScript has no 64-bit int, so `Core.u` declares this as
    /// two dummy ints and the native side treats the 8 bytes as a double.
    pub fn f64(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.take(8)?.try_into().unwrap()))
    }

    pub fn string(&mut self) -> Result<String> {
        let units: Vec<u16> = match string_encoding() {
            StringEncoding::LenPrefixedUtf16 => {
                let n = self.i32()?;
                if n < 0 {
                    bail!("negative string length {n}");
                }
                self.utf16_units(n as usize)?
            }
            StringEncoding::ByteLenPrefixedUtf16 => {
                let n = self.i32()?;
                if n < 0 || n % 2 != 0 {
                    bail!("bad string byte-length {n}");
                }
                self.utf16_units(n as usize / 2)?
            }
            StringEncoding::NullTerminatedUtf16 => {
                let mut v = Vec::new();
                loop {
                    let u = u16::from_le_bytes(self.take(2)?.try_into().unwrap());
                    if u == 0 {
                        break;
                    }
                    v.push(u);
                }
                v
            }
            StringEncoding::Len16PrefixedUtf16 => {
                let n = u16::from_le_bytes(self.take(2)?.try_into().unwrap());
                self.utf16_units(n as usize)?
            }
        };
        Ok(String::from_utf16_lossy(&units))
    }

    fn utf16_units(&mut self, count: usize) -> Result<Vec<u16>> {
        let bytes = self.take(count * 2)?;
        Ok(bytes
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect())
    }

    /// An array is a sub-buffer: `int32` byte length, then the elements packed
    /// back-to-back with no element count. The count is whatever fits - see
    /// [`Writer::array`] for where that comes from.
    pub fn array<T, F>(&mut self, mut f: F) -> Result<Vec<T>>
    where
        F: FnMut(&mut Reader<'a>) -> Result<T>,
    {
        let n = self.i32()?;
        if n < 0 {
            bail!("negative array byte-length {n}");
        }
        let n = n as usize;
        if n > self.remaining() {
            bail!("array of {n} bytes cannot fit in {} remaining bytes", self.remaining());
        }
        let mut sub = Reader::new(&self.buf[self.pos..self.pos + n]);
        let mut v = Vec::new();
        while sub.remaining() > 0 {
            let before = sub.pos;
            v.push(f(&mut sub)?);
            if sub.pos == before {
                bail!("array element consumed no bytes");
            }
        }
        self.pos += n;
        Ok(v)
    }
}

// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct Writer {
    buf: Vec<u8>,
}

impl Writer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buf
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.buf
    }

    pub fn len(&self) -> usize {
        self.buf.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn u8(&mut self, v: u8) {
        self.buf.push(v);
    }

    pub fn u16(&mut self, v: u16) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn i32(&mut self, v: i32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn u32(&mut self, v: u32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn f32(&mut self, v: f32) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn f64(&mut self, v: f64) {
        self.buf.extend_from_slice(&v.to_le_bytes());
    }

    pub fn string(&mut self, s: &str) {
        let units: Vec<u16> = s.encode_utf16().collect();
        match string_encoding() {
            StringEncoding::LenPrefixedUtf16 => self.i32(units.len() as i32),
            StringEncoding::ByteLenPrefixedUtf16 => self.i32((units.len() * 2) as i32),
            StringEncoding::Len16PrefixedUtf16 => {
                self.buf.extend_from_slice(&(units.len() as u16).to_le_bytes())
            }
            StringEncoding::NullTerminatedUtf16 => {}
        }
        for u in &units {
            self.buf.extend_from_slice(&u.to_le_bytes());
        }
        if string_encoding() == StringEncoding::NullTerminatedUtf16 {
            self.buf.extend_from_slice(&0u16.to_le_bytes());
        }
    }

    /// An `array<T>` parameter is an opaque sub-buffer (`BTBuffer` in the client):
    /// `int32` byte length, then the elements packed back-to-back. There is **no
    /// element count** - the client's `BTBuffer::ReadBuffer` memcpys `length` bytes
    /// and the typed conversion loops "while pos < size", so the count is whatever
    /// fits: `size/4` ints, `size` bytes, and for strings however many null-
    /// terminated UTF-16 runs the buffer holds. Elements use the same scalar
    /// encodings as top-level parameters.
    ///
    /// Sending an element count here instead was read as a 1-byte sub-buffer, and
    /// the next "length" was pulled from the middle of the first string - hence the
    /// client's gigabyte memcpy.
    pub fn array<T, F>(&mut self, items: &[T], mut f: F)
    where
        F: FnMut(&mut Self, &T),
    {
        let mut sub = Writer::new();
        for it in items {
            f(&mut sub, it);
        }
        self.i32(sub.len() as i32);
        self.buf.extend_from_slice(&sub.buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The string encoding is process-global and cargo runs tests in parallel, so
    /// every test that sets it must hold this.
    static ENCODING_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn scalars_round_trip() {
        let mut w = Writer::new();
        w.i32(-7);
        w.u8(0xAB);
        w.f32(1.5);
        w.f64(-2.25);
        let buf = w.into_inner();
        let mut r = Reader::new(&buf);
        assert_eq!(r.i32().unwrap(), -7);
        assert_eq!(r.u8().unwrap(), 0xAB);
        assert_eq!(r.f32().unwrap(), 1.5);
        assert_eq!(r.f64().unwrap(), -2.25);
        assert_eq!(r.remaining(), 0);
    }

    #[test]
    fn strings_round_trip_in_every_encoding() {
        let _guard = ENCODING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        for enc in [
            StringEncoding::LenPrefixedUtf16,
            StringEncoding::ByteLenPrefixedUtf16,
            StringEncoding::NullTerminatedUtf16,
            StringEncoding::Len16PrefixedUtf16,
        ] {
            set_string_encoding(enc);
            let mut w = Writer::new();
            w.string("acemu");
            w.string("");
            let buf = w.into_inner();
            let mut r = Reader::new(&buf);
            assert_eq!(r.string().unwrap(), "acemu", "{enc:?}");
            assert_eq!(r.string().unwrap(), "", "{enc:?}");
            assert_eq!(r.remaining(), 0, "{enc:?}");
        }
        set_string_encoding(StringEncoding::LenPrefixedUtf16);
    }

    #[test]
    fn short_read_is_an_error_not_a_panic() {
        let buf = [1u8, 2, 3];
        let mut r = Reader::new(&buf);
        assert!(r.i32().is_err());
    }

    #[test]
    fn absurd_array_length_is_rejected() {
        let mut buf = Vec::new();
        buf.extend_from_slice(&0x7FFF_FFFFi32.to_le_bytes());
        let mut r = Reader::new(&buf);
        assert!(r.array(|r| r.i32()).is_err());
    }

    /// The exact bytes the client's `BTBuffer` reader expects: byte length, then
    /// packed elements, no count. Derived, not documented, so pin it.
    #[test]
    fn arrays_are_length_prefixed_sub_buffers() {
        let _guard = ENCODING_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_string_encoding(StringEncoding::NullTerminatedUtf16);
        let mut w = Writer::new();
        w.array(&["A", ""], |w, s| w.string(s));
        w.array(&[55999i32, 7], |w, v| w.i32(*v));
        w.array::<u8, _>(&[], |w, v| w.u8(*v));
        let buf = w.into_inner();
        assert_eq!(
            buf,
            [
                6, 0, 0, 0, 0x41, 0, 0, 0, 0, 0, // "A\0" "\0"  -> 6 bytes
                8, 0, 0, 0, 0xBF, 0xDA, 0, 0, 7, 0, 0, 0, // two ints -> 8 bytes
                0, 0, 0, 0, // empty
            ]
        );
        let mut r = Reader::new(&buf);
        assert_eq!(r.array(|r| r.string()).unwrap(), vec!["A".to_string(), String::new()]);
        assert_eq!(r.array(|r| r.i32()).unwrap(), vec![55999, 7]);
        assert_eq!(r.array(|r| r.u8()).unwrap(), Vec::<u8>::new());
        assert_eq!(r.remaining(), 0);
        set_string_encoding(StringEncoding::LenPrefixedUtf16);
    }
}
