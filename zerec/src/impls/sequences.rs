//! Encode + Decode for sequential collection types.

use crate::{
    codec::{Decode, Encode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError
};

/// Maximum element count for collections during decode.
///
/// Prevents adversarial inputs from causing large allocations before
/// any data is actually validated. 64 million elements.
const COLLECTION_LIMIT: u32 = 64_000_000;

// ── Vec<T> ────────────────────────────────────────────────────────────────

impl<T: Encode> Encode for Vec<T> {
    /// Wire layout: `u32` element count, then each element in order.
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u32(u32::try_from(self.len()).expect("Vec length exceeds u32::MAX"));
        for item in self {
            item.encode(enc);
        }
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        let len = dec.read_u32()?;

        if len > COLLECTION_LIMIT {
            return Err(DecodeError::CollectionTooLarge { len, limit: COLLECTION_LIMIT });
        }

        let mut out = Vec::with_capacity(len as usize);

        for _ in 0..len {
            out.push(T::decode(dec)?);
        }

        Ok(out)
    }
}

