//! Core `Encode` and `Decode` traits, plus top-level convenience functions.

use crate::{decoder::BufDecoder, encoder::BufEncoder, error::DecodeError};

/// Writes a value into a [`BufEncoder`].
///
/// # Contract
///
/// - Write fields in declaration order.
/// - No type flags, field names, or padding — just raw bytes.
/// - The byte count produced must be deterministic for a given value.
///
/// Encoding is always infallible. If a value can be constructed, it can be encoded.
///
/// # Deriving
///
/// Enable the `derive` feature and use `#[derive(Encode)]`:
///
/// ```rust,ignore
/// use zerec::Encode;
///
/// #[derive(Encode)]
/// struct Hit {
///     position: [f32; 3],
///     damage: u32,
/// }
/// ```
pub trait Encode {
    /// Encodes `self` into `enc`.
    fn encode(&self, enc: &mut BufEncoder);
}

/// Reads a value from [`BufDecoder`].
///
/// # Contract
///
/// - Read fields in the exact order and sizes that [`Encode`] wrote them.
/// - Return [`DecodeError`] on malformed input; never panic.
/// - Consume exactly as many bytes as [`Encode`] produced.
///
/// # Deriving
///
/// Enable the `derive` feature and use `#[derive(Decode)]`:
///
/// ```rust,ignore
/// use zerec::Decode;
///
/// #[derive(Decode)]
/// struct Hit {
///     position: [f32; 3],
///     damage: u32,
/// }
/// ```
pub trait Decode: Sized {
    /// Decodes a value by consuming bytes from `dec`.
    fn decode(dec: &mut BufDecoder<'_>) -> Result<Self, DecodeError>;
}

/// Encodes `value` and returns the result as a freshly allocated `Vec<u8>`.
///
/// ```rust
/// use zerec::{Encode, codec::to_bytes};
/// use zerec::encoder::BufEncoder;
///
/// struct Score(u32);
///
/// impl Encode for Score {
///     fn encode(&self, enc: &mut BufEncoder) { self.0.encode(enc); }
/// }
///
/// let bytes = to_bytes(&Score(999));
/// assert_eq!(bytes, 999u32.to_le_bytes());
/// ```
pub fn to_bytes<T: Encode>(value: &T) -> Vec<u8> {
    let mut enc = BufEncoder::new();
    value.encode(&mut enc);
    enc.finish()
}

/// Decodes a value from a byte slice.
///
/// Does not require the entire slice to be consumed — use [`BufDecoder`]
/// directly if you need to check for trailing bytes.
///
/// ```rust
/// use zerec::codec::{to_bytes, from_bytes};
///
/// let n: u64 = 0xDEAD_BEEF_CAFE_0001;
/// assert_eq!(from_bytes::<u64>(&to_bytes(&n)).unwrap(), n);
/// ```
pub fn from_bytes<T: Decode>(bytes: &[u8]) -> Result<T, DecodeError> {
    let mut dec = BufDecoder::new(bytes);
    T::decode(&mut dec)
}