//! Decode-time error types.
//!
//! Encoding is infallible by design: a well-formed Rust value can always be
//! written to a byte buffer. Decoding operates on untrusted input, so every
//! read is checked and surfaces a typed error instead of panicking.

/// Every way a decode operation can fail.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecodeError {
    /// The buffer ran out of bytes before the value was fully decoded.
    UnexpectedEof {
        /// How many bytes the decoder needed.
        needed: usize,
        /// How many bytes were still available.
        remaining: usize,
    },

    /// A `bool` field held a byte other than `0x00` or `0x01`
    InvalidBool(u8),

    /// A `char` field held a value that is not a valid Unicode scalar.
    InvalidChar(u32),

    /// A `String` or `&str` field contained invalid UTF-8.
    InvalidUtf8,

    /// A length prefix exceeded the configured safety limit.
    ///
    /// Prevents adversarial input from triggering multi-gigabyte allocations.
    CollectionTooLarge {
        /// The length read from the wire.
        len: u32,
        /// The limit that was enforced.
        limit: u32
    },

    /// An enum variant index had no matching variant in the type.
    UnknownVariant(u32),
}

impl core::fmt::Display for DecodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnexpectedEof { needed, remaining } => write!(f, "unexpected end of file: need {needed} bytes, {remaining} left"),
            Self::InvalidBool(b) => write!(f, "invalid boolean byte: 0x{b:02X}"),
            Self::InvalidChar(c) => write!(f, "invalid character: U+{c:04X}"),
            Self::InvalidUtf8 => write!(f, "invalid UTF-8 in string field"),
            Self::CollectionTooLarge { len, limit } => write!(f, "collection length {len} exceeds safety limit {limit}"),
            Self::UnknownVariant(idx) => write!(f, "unknown variant index: {idx}")
        }
    }
}

impl core::error::Error for DecodeError {}