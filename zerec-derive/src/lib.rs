//! Derive macros for [zerec](https://docs.rs/zerec).
//!
//! Pull these in through zerec's `derive` feature:
//!
//! ```toml
//! [dependencies]
//! zerec = { version = "0.3", features = ["derive"] }
//! ```
//!
//! ## Field attributes
//!
//! All attributes live under `#[zerec(...)]`.
//!
//! ### `skip`
//!
//! Exclude a field from the wire. Filled with `Default::default()` on decode.
//!
//! ```rust,ignore
//! #[derive(Encode, Decode)]
//! struct Foo {
//!     written: u32,
//!     #[zerec(skip)]
//!     transient: u64,
//! }
//! ```
//!
//! ### `via = "AdapterType"`
//!
//! Route a field through a type that implements `zerec::Adapter<FieldType>`.
//!
//! ```rust,ignore
//! #[derive(Encode, Decode)]
//! struct Scene {
//!     #[zerec(via = "RigidBodyAdapter")]
//!     body: RigidBody,
//! }
//! ```
//!
//! ### `map_enc` / `map_dec`
//!
//! Inline closures for lightweight conversions without a dedicated adapter.
//!
//! ```rust,ignore
//! #[derive(Encode, Decode)]
//! struct Player {
//!     #[zerec(
//!         map_enc = "|v: &glam::Vec3| [v.x, v.y, v.z]",
//!         map_dec = "|dec| {
//!             let a: [f32; 3] = zerec::Decode::decode(dec)?;
//!             Ok(glam::Vec3::from_array(a))
//!         }"
//!     )]
//!     position: glam::Vec3,
//! }
//! ```

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attr;
mod decode;
mod encode;

/// Derives [`zerec::Encode`] for a struct or enum.
///
/// Fields are encoded in declaration order. No type tags or field names
/// are written to the wire.
#[proc_macro_derive(Encode, attributes(zerec))]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    encode::derive_encode(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// Derives [`zerec::Decode`] for a struct or enum.
///
/// Fields are decoded in the same order [`derive_encode`] writes them.
/// Returns an error on malformed input; never panics.
#[proc_macro_derive(Decode, attributes(zerec))]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    decode::derive_decode(&input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}