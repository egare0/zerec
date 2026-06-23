//! Encode + Decode for Rust's primitive types.
//!
//! All integers are little-endian. Signed integers are cast through their
//! unsigned equivalents — the bit pattern is identical, no sign extension
//! needed. Floats are stored as their raw IEEE 754 bit representation.

use crate::{
    codec::{Encode, Decode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError
};

// ── u8 / i8 ───────────────────────────────────────────────────────────────

impl Encode for u8 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u8(*self);
    }
}

impl Decode for u8 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u8()
    }
}

impl Encode for i8 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u8(*self as u8);
    }
}

impl Decode for i8 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u8().map(|b| b as i8)
    }
}

// ── u16 / i16 ─────────────────────────────────────────────────────────────

impl Encode for u16 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u16(*self);
    }
}

impl Decode for u16 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u16()
    }
}

impl Encode for i16 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u16(*self as u16);
    }
}

impl Decode for i16 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u16().map(|v| v as i16)    }
}

// ── u32 / i32 ─────────────────────────────────────────────────────────────

impl Encode for u32 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u32(*self);
    }
}

impl Decode for u32 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u32()
    }
}

impl Encode for i32 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u32(*self as u32);
    }
}

impl Decode for i32 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u32().map(|v| v as i32)
    }
}

// ── u64 / i64 ─────────────────────────────────────────────────────────────

impl Encode for u64 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u64(*self);
    }
}

impl Decode for u64 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u64()
    }
}

impl Encode for i64 {
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u64(*self as u64);
    }
}

impl Decode for i64 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u64().map(|v| v as i64)
    }
}

// ── f32 / f64 ─────────────────────────────────────────────────────────────

impl Encode for f32 {
    /// Stores the IEEE 754 bit pattern as a `u32`.
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u32(self.to_bits());
    }
}

impl Decode for f32 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u32().map(f32::from_bits)
    }
}

impl Encode for f64 {
    /// Stores the IEEE 754 bit pattern as a `u64`.
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u64(self.to_bits());
    }
}

impl Decode for f64 {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        dec.read_u64().map(f64::from_bits)
    }
}

// ── bool ──────────────────────────────────────────────────────────────────

impl Encode for bool {
    /// `false` -> `0x00`, `true` -> `0x01`
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u8(*self as u8)
    }
}

impl Decode for bool {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        match dec.read_u8()? {
            0 => Ok(false),
            1 => Ok(true),
            b => Err(DecodeError::InvalidBool(b))
        }
    }
}

// ── char ──────────────────────────────────────────────────────────────────

impl Encode for char {
    /// Stored as the Unicode scalar value cast to `u32` (4 bytes, LE).
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        enc.write_u32(*self as u32);
    }
}

impl Decode for char {
    #[inline]
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        let n = dec.read_u32()?;
        char::from_u32(n).ok_or(DecodeError::InvalidChar(n))
    }
}

// ── [T; N] ────────────────────────────────────────────────────────────────

impl<T: Encode, const N: usize> Encode for [T; N] {
    /// Encodes each element in order. No length prefix; N is part of the type.
    #[inline]
    fn encode(&self, enc: &mut BufEncoder) {
        for item in self {
            item.encode(enc);
        }
    }
}

impl<T: Decode, const N: usize> Decode for [T; N] {
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
        // Build element-by-element through MaybeUninit to avoid
        // requiring T: Default and to keep the stack frame predictable.
        let mut arr: [core::mem::MaybeUninit<T>; N] =
            // SAFETY: an array of MaybeUninit is always safe to uninit.
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        for slot in &mut arr {
            slot.write(T::decode(dec)?);
        }

        // SAFETY: every element was written above.
        Ok(arr.map(|s| unsafe { s.assume_init() }))
    }
}

// ── tuples (1-8) ──────────────────────────────────────────────────────────

macro_rules! impl_tuple {
    ($($T:ident : $idx:tt),+) => {
        impl<$($T: Encode),+> Encode for ($($T,)+) {
            #[inline]
            fn encode(&self, enc: &mut BufEncoder) {
                $(self.$idx.encode(enc);)+
            }
        }
        impl<$($T: Decode),+> Decode for ($($T,)+) {
            #[inline]
            fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError> {
                Ok(($($T::decode(dec)?,)+))
            }
        }
    };
}

impl_tuple!(A:0);
impl_tuple!(A:0, B:1);
impl_tuple!(A:0, B:1, C:2);
impl_tuple!(A:0, B:1, C:2, D:3);
impl_tuple!(A:0, B:1, C:2, D:3, E:4);
impl_tuple!(A:0, B:1, C:2, D:3, E:4, F:5);
impl_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
impl_tuple!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);