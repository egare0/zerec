//! Encode + Decode for Option<T>.

use crate::{
    codec::{Decode, Encode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError
};

// ── Option<T> ─────────────────────────────────────────────────────────────

impl<T: Encode> Encode for Option<T> {
    /// `None` -> `0x00`, `Some(v)` -> `0x01` then encoded `v`.
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        match self {
            None    => enc.write_u8(0),
            Some(v) => {
                enc.write_u8(1);
                v.encode(enc);
            }
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        match dec.read_u8()? {
            0 => Ok(None),
            1 => Ok(Some(T::decode(dec)?)),
            b => Err(DecodeError::InvalidBool(b)),
        }
    }
}