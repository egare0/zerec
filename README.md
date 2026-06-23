> `derive` feature is not yet available — coming in v0.2.0.

# zerec

Minimal zero-copy binary codec for Rust.

No schema files. No runtime reflection. No type tags on the wire. Just bytes.

## Quick start

```toml
[dependencies]
zerec = { version = "0.1", features = ["derive"] }
```

```rust
use zerec::{Encode, Decode, codec::{to_bytes, from_bytes}};

#[derive(Encode, Decode, Debug, PartialEq)]
struct Bullet { 
    origin: [f32; 3],
    speed:  f32,
    damage: u32,
}

fn main() {
    let b = Bullet { origin: [1.0, 0.0, -3.5], speed: 900.0, damage: 42 };
    let bytes = to_bytes(&b);
    assert_eq!(from_bytes::<Bullet>(&bytes).unwrap(), b);
}
```

## Features

| Feature  | What it enables                                    |
|----------|----------------------------------------------------|
| `derive` | `#[derive(Encode, Decode)]` proc macros            |
| `glam`   | Encode/Decode for `glam::Vec2/3/4`, `Quat`, `Mat4` |

## Wire format (ZRC)

Tag-free, little-endian, tightly packed. No padding, no alignment waste.

| Type                  | Wire size                           |
|-----------------------|-------------------------------------|
| `u8` / `i8`           | 1 byte                              |
| `u16` / `i16`         | 2 bytes                             |
| `u32` / `i32` / `f32` | 4 bytes                             |
| `u64` / `i64` / `f64` | 8 bytes                             |
| `bool`                | 1 byte (`0x00` / `0x01`)            |
| `char`                | 4 bytes (Unicode scalar as `u32`)   |
| `[T; N]`              | `N * size_of(T)` — no length prefix |
| `Vec<T>`              | `u32` element count + elements      |
| `String` / `&str`     | `u32` byte length + UTF-8 bytes     |
| `Option<T>`           | 1 tag byte + optional payload       |
| `enum`                | `u32` variant index + payload       |
| `struct`              | fields in declaration order         |

Field order is the contract. Adding or reordering fields is a breaking change.

## Field attributes (v0.2.0)

```rust
#[derive(Encode, Decode)]
struct Example {
    // Exclude from the wire; filled with Default::default() on decode.
    #[zerec(skip)]
    cache: u64,

    // Route through an Adapter for foreign types (orphan rule workaround).
    #[zerec(via = "RigidBodyAdapter")]
    body: RigidBody,

    // Inline closures for lightweight conversions.
    #[zerec(
        map_enc = "|v: &glam::Vec3| [v.x, v.y, v.z]",
        map_dec = "|dec| {
            let a: [f32; 3] = zerec::Decode::decode(dec)?;
            Ok(glam::Vec3::from_array(a))
        }"
    )]
    position: glam::Vec3,
}
```

## Adapter pattern

For types, you don't own:

```rust
use zerec::Adapter;

struct RigidBody { mass: f32, vel: [f32; 3] }
struct RigidBodyAdapter;

impl Adapter<RigidBody> for RigidBodyAdapter {
type Repr = (f32, [f32; 3]);
fn to_repr(v: &RigidBody) -> Self::Repr { (v.mass, v.vel) }
fn from_repr(r: Self::Repr) -> RigidBody { RigidBody { mass: r.0, vel: r.1 } }
}
```

## Zero-copy reads

Borrow `&str` and `&[u8]` straight from the source buffer:

```rust
use zerec::{ZeroBuf, decoder::BufDecoder};

fn main() {
    let mut dec = BufDecoder::new(&bytes);
    let name: &str = ZeroBuf::decode_borrowed(&mut dec).unwrap();
}
```

## Workspace

```
zerec/
├── zerec/          # core library
└── zerec-derive/   # proc-macro crate (used via feature = "derive")
```

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or
[MIT License](LICENSE-MIT) at your option.