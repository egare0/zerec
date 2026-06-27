//! Read-side of the ZRC wire format.

use crate::error::DecodeError;

/// Maximum nesting depth for collection types during decode.
///
/// Prevents stack overflow from adversarially crafted deeply nested
/// structures. 64 levels is far deeper than any real-world data.
pub const MAX_DECODE_DEPTH: u32 = 64;

/// Default maximum element/byte count allowed in a single collection or string .
///
/// This limit exists to prevent adversarial inputs from triggering large
/// allocations before any data is actually validated. Can be overridden
/// per-decoder with [`BufDecoder::with_collection_limit`].
pub const DEFAULT_COLLECTION_LIMIT: u32 = 64_000_000;

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
    depth: u32,
    collection_limit: u32,
}

impl<'buf> BufDecoder<'buf> {
    /// Creates a decoder starting at byte 0 of `buf`.
    ///
    /// The collection limit defaults to [`DEFAULT_COLLECTION_LIMIT`].
    #[inline]
    pub fn new(buf: &'buf [u8]) -> Self {
        Self { buf, pos: 0, depth: 0, collection_limit: DEFAULT_COLLECTION_LIMIT }
    }

    /// Sets a custom collection limit and returns the decoder.
    ///
    /// The limit applies to `Vec`, `String`, and any other variable-length
    /// type decoded through this decoder instance.
    ///
    /// ```rust
    /// use zerec::decoder::BufDecoder;
    ///
    /// let mut dec = BufDecoder::new(&[]).with_collection_limit(1024);
    /// ```
    #[inline]
    pub fn with_collection_limit(mut self, limit: u32) -> Self {
        self.collection_limit = limit;
        self
    }

    /// Returns the active collection limit for this decoder.
    #[inline]
    pub fn collection_limit(&self) -> u32 {
        self.collection_limit
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
        Ok(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
    }

    /// Increments the nesting depth counter.
    ///
    /// Call this at the start of any [`Decode`] impl that dynamically
    /// decodes a variable number of inner values (e.g. `Vec<T>`).
    /// Always pair with [`Self::leave`] on the success path.
    ///
    /// Returns [`DecodeError::NestingTooDeep`] if the limit is exceeded.
    /// On error paths inside the guarded region, `leave` need not be
    /// called — a failed decode cannot be meaningfully continued.
    #[inline]
    pub fn enter(&mut self) -> Result<(), DecodeError> {
        if self.depth >= MAX_DECODE_DEPTH {
            return Err(DecodeError::NestingTooDeep);
        }

        self.depth += 1;

        Ok(())
    }

    /// Decrements the nesting depth counter. Pair with [`Self::enter`].
    #[inline]
    pub fn leave(&mut self) {
        debug_assert!(self.depth > 0, "leave() called without matching enter()");
        self.depth = self.depth.saturating_sub(1);
    }
}