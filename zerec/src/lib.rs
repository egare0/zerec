//! # zerec
//!
//! Minimal zero-copy binary codec for Rust.
//!
//! - **No schema files.** Types are the schema.
//! - **No runtime reflection.** Everything is resolved at compile time.
//! - **No type tags on the wire.** You know what you are decoding into.
//! - **Little-endian, tightly packed.** No padding, no alignment waste.
//!
//! ## Quick start
//!
//! ```toml
//! [dependencies]
//! zerec = { version = "0.3", features = ["derive"] }
//! ```
//!
//! ```rust,ignore
//! use zerec::{Encode, Decode, codec::{to_bytes, from_bytes}};
//!
//! #[derive(Encode, Decode, Debug, PartialEq)]
//! struct Bullet {
//!     origin: [f32; 3],
//!     speed:  f32,
//!     damage: u32,
//! }
//!
//! let b = Bullet { origin: [1.0, 0.0, -3.5], speed: 900.0, damage: 42 };
//! let bytes = to_bytes(&b);
//! assert_eq!(from_bytes::<Bullet>(&bytes).unwrap(), b);
//! ```
//!
//! ## Wire format
//!
//! ZRC is tag-free and little-endian. See the crate-level docs or README
//! for the full encoding table.
//!
//! ## Features
//!
//! | Feature | What it enables |
//! |---------|----------------|
//! | `derive` | `#[derive(Encode, Decode)]` via `zerec-derive` |
//! | `glam`   | Encode/Decode for `glam::Vec2/3/4`, `Quat`, `Mat4` |

#![no_std]
extern crate alloc;

pub mod error;
pub mod encoder;
pub mod decoder;
pub mod codec;
pub mod impls;
pub mod zero_copy;
pub mod adapter;
pub mod integrations;

pub use adapter::Adapter;
pub use codec::{Decode, Encode};
pub use error::DecodeError;
pub use zero_copy::ZeroBuf;

// Re-export derive macros when the feature is active.
#[cfg(feature = "derive")]
pub use zerec_derive::{Decode, Encode};