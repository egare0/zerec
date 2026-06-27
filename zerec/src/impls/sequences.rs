//! Encode + Decode for sequential collection types.

use crate::{
    codec::{Decode, Encode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError
};

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
        dec.enter()?;

        let len = dec.read_u32()?;
        let limit = dec.collection_limit();

        if len > limit {
            return Err(DecodeError::CollectionTooLarge { len, limit })
        }

        // Start small and grow as elements arrive rather than allocating
        // the full requested capacity upfront. Limits the damage from
        // large but otherwise valid length prefixes.
        let mut out = Vec::with_capacity((len as usize).min(1024));

        for _ in 0..len {
            out.push(T::decode(dec)?);
        }

        dec.leave();
        Ok(out)
    }
}

