//! Zero-copy borrowing from the source buffer.
//!
//! [`ZeroBuf`] is a companion to [`crate::Decode`]. Where `Decode` always
//! produces owned values, `ZeroBuf` implementations borrow directly from
//! the byte slice the [`crate::decoder::BufDecoder`] was created over —
//! no allocation, no copying.
//!
//! # When to use
//!
//! Implement `ZeroBuf` for types that are a straight reinterpretation of
//! a contiguous region of the source buffer: `&str`, `&[u8]`, and any
//! fixed-layout struct on little-endian targets where byte alignment does
//! not matter.
//!
//! For types that require reconstruction (e.g. `Vec3`, enums), use
//! [`crate::Decode`] instead.

use crate::{decoder::BufDecoder, error::DecodeError};

/// Decodes a value by borrowing bytes directly from the source buffer.
///
/// The lifetime `'buf` is tied to the buffer passed to
/// [`BufDecoder::new`], so the returned value cannot outlive the buffer.
///
/// # Example
///
/// ```rust
/// use zerec::{ZeroBuf, decoder::BufDecoder};
///
/// let data = {
///     let s = b"hello";
///     let mut v = (s.len() as u32).to_le_bytes().to_vec();
///     v.extend_from_slice(s);
///     v
/// };
///
/// let mut dec = BufDecoder::new(&data);
/// let s: &str = ZeroBuf::decode_borrowed(&mut dec).unwrap();
/// assert_eq!(s, "hello");
/// ```
pub trait ZeroBuf<'buf>: Sized {
    /// Decodes by borrowing from the underlying buffer in `dec`.
    fn decode_borrowed(dec: &mut BufDecoder<'buf>) -> Result<Self, DecodeError>;
}

// ── &[u8] ─────────────────────────────────────────────────────────────────

impl<'buf> ZeroBuf<'buf> for &'buf [u8] {
    /// Reads a `u32` length prefix, then returns a slice of that many
    /// bytes pointing into the original buffer.
    fn decode_borrowed(dec: &mut BufDecoder<'buf>) -> Result<Self, DecodeError> {
        let len = dec.read_u32()? as usize;
        dec.read_bytes(len)
    }
}

// ── &str ──────────────────────────────────────────────────────────────────

impl<'buf> ZeroBuf<'buf> for &'buf str {
    /// Reads a `u32` byte-length prefix, then borrows the UTF-8 bytes
    /// directly from the buffer without copying.
    fn decode_borrowed(dec: &mut BufDecoder<'buf>) -> Result<Self, DecodeError> {
        let bytes = <&'buf [u8]>::decode_borrowed(dec)?;
        core::str::from_utf8(bytes).map_err(|_| DecodeError::InvalidUtf8)
    }
}