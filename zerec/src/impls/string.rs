//! Encode + Decode for string types.

use crate::{
    codec::{Decode, Encode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError
};

/// Maximum byte length for a single string during decode (64 MiB).
const STRING_LIMIT: u32 = 64 * 1024 * 1024;

// ── String ────────────────────────────────────────────────────────────────
impl Encode for String {
    /// Wire layout: `u32` byte length, then raw UTF-8 bytes.
    fn encode(&self, enc: &mut BufEncoder) {
        let b = self.as_bytes();
        enc.write_u32(u32::try_from(b.len()).expect("String byte length exceeds u32::MAX"));
        enc.write_bytes(b);
    }
}

impl Decode for String {
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        let len = dec.read_u32()?;

        if len > STRING_LIMIT {
            return Err(DecodeError::CollectionTooLarge { len, limit: STRING_LIMIT });
        }

        let bytes = dec.read_bytes(len as usize)?;
        core::str::from_utf8(bytes)
            .map(|s| s.to_owned())
            .map_err(|_| DecodeError::InvalidUtf8)
    }
}

// ── &str (encode only) ────────────────────────────────────────────────────

impl Encode for str {
    /// Same wire layout as `String`.
    /// `Decode for &str` is not provided here — use [`crate::ZeroBuf`]
    /// to borrow directly from the source buffer without allocating.
    fn encode(&self, enc: &mut BufEncoder) {
        let b = self.as_bytes();
        enc.write_u32(b.len() as u32);
        enc.write_bytes(b);
    }
}

impl Encode for &str {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        (**self).encode(enc);
    }
}