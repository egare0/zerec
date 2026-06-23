//! Adapter pattern for foreign types.
//!
//! Use [`Adapter`] when you want to encode/decode a type from another crate
//! but cannot implement [`crate::Encode`] / [`crate::Decode`] for it directly
//! (the orphan rule prevents that). An `Adapter` declares an intermediate
//! representation (`Repr`) that zerec already knows how to handle, and two
//! conversion functions.
//!
//! You rarely call these methods yourself — the `#[zerec(via = "...")]`
//! attribute on a derived field does it for you.
//!
//! # Example
//!
//! ```rust,ignore
//! use zerec::Adapter;
//!
//! // A foreign physics body we cannot touch.
//! struct RigidBody { mass: f32, vel: [f32; 3] }
//!
//! struct RigidBodyAdapter;
//!
//! impl Adapter<RigidBody> for RigidBodyAdapter {
//!     type Repr = (f32, [f32; 3]);
//!     fn to_repr(v: &RigidBody) -> Self::Repr { (v.mass, v.vel) }
//!     fn from_repr(r: Self::Repr) -> RigidBody { RigidBody { mass: r.0, vel: r.1 } }
//! }
//!
//! // In your own struct:
//! #[derive(Encode, Decode)]
//! struct Scene {
//!     #[zerec(via = "RigidBodyAdapter")]
//!     body: RigidBody,
//! }
//! ```

use crate::{
    codec::{Decode, Encode},
    decoder::BufDecoder,
    encoder::BufEncoder,
    error::DecodeError,
};

/// A bridge between a foreign type `T` and zerec's codec.
pub trait Adapter<T> {
    /// The intermediate type that implements [`Encode`] + [`Decode`].
    type Repr: Encode + Decode;

    /// Converts a reference to `T` into its wire representation.
    fn to_repr(val: &T) -> Self::Repr;

    /// Reconstructs `T` from the decoded representation.
    fn from_repr(repr: Self::Repr) -> T;

    /// Encodes `val` through this adapter.
    ///
    /// Called by derived code. Override only for specialized layouts.
    #[inline]
    fn encode_via(val: &T, enc: &mut BufEncoder) {
        Self::to_repr(val).encode(enc);
    }

    /// Decodes a `T` through this adapter.
    ///
    /// Called by derived code. Override only for specialized layouts.
    #[inline]
    fn decode_via(dec: &mut BufDecoder<'_>) -> Result<T, DecodeError> {
        Self::Repr::decode(dec).map(Self::from_repr)
    }
}