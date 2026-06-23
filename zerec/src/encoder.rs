//! Write-side of the ZRC wire format.

/// A write-only byte buffer used during encoding.
///
/// All [`crate::Encode`] implementations write through this type.
/// It wraps a `Vec<u8>` and exposes only the primitives the wire format needs:
/// fixed-width little-endian integers and raw byte slices.
///
/// Encoding is always infallible — there are no error paths here.
///
/// # Example
///
/// ```rust
/// use zerec::encoder::BufEncoder;
///
/// let mut enc = BufEncoder::new();
/// enc.write_u32(42);
/// enc.write_bytes(b"hello");
/// let bytes = enc.finish();
/// assert_eq!(&bytes[..4], &42u32.to_le_bytes());
/// ```
pub struct BufEncoder {
    buf: Vec<u8>
}

impl BufEncoder {
    /// Creates an empty encoder.
    #[inline]
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Creates an encoder with preallocated capacity.
    ///
    /// Use this when the encoded size is roughly known in advance to
    /// avoid repeated reallocations on the hot path.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self { buf: Vec::with_capacity(cap) }
    }

    /// Appends one byte.
    #[inline]
    pub fn write_u8(&mut self, value: u8) {
        self.buf.push(value);
    }

    /// Appends two bytes, little-endian.
    #[inline]
    pub fn write_u16(&mut self, value: u16) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends four bytes, little-endian.
    #[inline]
    pub fn write_u32(&mut self, value: u32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends eight bytes, little-endian.
    #[inline]
    pub fn write_u64(&mut self, value: u64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Appends a raw byte slice without any length prefix.
    ///
    /// The caller must write the length separately when the decoder
    /// needs to know it (e.g. `Vec<u8>`, `String`).
    #[inline]
    pub fn write_bytes(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    /// Consumes the encoder and returns the underlying buffer.
    #[inline]
    pub fn finish(self) -> Vec<u8> {
        self.buf
    }

    /// Number of bytes written so far.
    #[inline]
    pub fn len(&self) -> usize {
        self.buf.len()
    }

    /// Returns `true` if nothing has been written yet.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }
}

impl Default for BufEncoder {
    fn default() -> Self {
        Self::new()
    }
}