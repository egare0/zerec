//! Read-side of the ZRC wire format.

use crate::error::DecodeError;

/// A read-only cursor over a borrowed byte slice used during decoding.
///
/// `BufDecoder` advances an internal position as bytes are consumed.
/// Every read is bounds-checked; on underflow it returns
/// [`DecodeError::UnexpectedEof`] rather than panicking.
///
/// The lifetime `'buf` ties the decoder to the source buffer, which
/// enables zero-copy reads via [`crate::ZeroBuf`].
///
/// # Example
///
/// ```rust
/// use zerec::decoder::BufDecoder;
///
/// let data = 42u32.to_le_bytes();
/// let mut dec = BufDecoder::new(&data);
/// assert_eq!(dec.read_u32().unwrap(), 42);
/// assert_eq!(dec.remaining(), 0);
/// ```
pub struct BufDecoder<'buf> {
    buf: &'buf [u8],
    pos: usize,
}

impl<'buf> BufDecoder<'buf> {
    /// Creates a decoder starting at byte 0 of `buf`.
    #[inline]
    pub fn new(buf: &'buf [u8]) -> Self {
        Self { buf, pos: 0}
    }

    /// Number of bytes consumed so far.
    #[inline]
    pub fn position(&self) -> usize {
        self.pos
    }

    /// Number of bytes not yet consumed.
    #[inline]
    pub fn remaining(&self) -> usize {
        self.buf.len().saturating_sub(self.pos)
    }

    /// Reads exactly `n` bytes and advances the cursor.
    ///
    /// Returns a slice into the original buffer - no allocation.
    /// Fails with [`DecodeError::UnexpectedEof`] if fewer than `n`
    /// bytes remain.
    #[inline]
    pub fn read_bytes(&mut self, n: usize) -> Result<&'buf [u8], DecodeError> {
        let end = self.pos.checked_add(n).filter(|&e| e <= self.buf.len()).ok_or(DecodeError::UnexpectedEof {
            needed: n,
            remaining: self.remaining(),
        })?;
        let slice = &self.buf[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    /// Reads one byte.
    #[inline]
    pub fn read_u8(&mut self) -> Result<u8, DecodeError> {
        Ok(self.read_bytes(1)?[0])
    }

    /// Reads two bytes as a little-endian `u16`.
    #[inline]
    pub fn read_u16(&mut self) -> Result<u16, DecodeError> {
        let b = self.read_bytes(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    /// Reads four bytes as a little-endian `u32`.
    #[inline]
    pub fn read_u32(&mut self) -> Result<u32, DecodeError> {
        let b = self.read_bytes(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Reads eight bytes as a little-endian `u64`.
    #[inline]
    pub fn read_u64(&mut self) -> Result<u64, DecodeError> {
        let b = self.read_bytes(8)?;
        Ok(u64::from_le_bytes(b.try_into().unwrap()))
    }
}