//! Integration tests for #[derive(Encode, Decode)].

use zerec::{
    codec::{from_bytes, to_bytes},
    Adapter, Decode, Encode,
};

// ── basic struct ──────────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn derive_basic_struct() {
    let p = Point { x: 1.0, y: -2.5 };
    assert_eq!(from_bytes::<Point>(&to_bytes(&p)).unwrap(), p);
}

#[test]
fn derive_struct_wire_size() {
    let p = Point { x: 0.0, y: 0.0 };
    assert_eq!(to_bytes(&p).len(), 8); // 2 x f32
}

// ── nested struct ─────────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct Bullet {
    origin: Point,
    damage: u32,
    active: bool,
}

#[test]
fn derive_nested_struct() {
    let b = Bullet { origin: Point { x: 1.0, y: 2.0 }, damage: 42, active: true };
    assert_eq!(from_bytes::<Bullet>(&to_bytes(&b)).unwrap(), b);
}

// ── tuple struct ──────────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct Rgb(u8, u8, u8);

#[test]
fn derive_tuple_struct() {
    let c = Rgb(255, 128, 0);
    assert_eq!(from_bytes::<Rgb>(&to_bytes(&c)).unwrap(), c);
}

// ── unit struct ───────────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct Marker;

#[test]
fn derive_unit_struct() {
    let m = Marker;
    let bytes = to_bytes(&m);
    assert_eq!(bytes.len(), 0);
    from_bytes::<Marker>(&bytes).unwrap();
}

// ── enum ─────────────────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[test]
fn derive_unit_enum() {
    for v in [Direction::North, Direction::South, Direction::East, Direction::West] {
        let bytes = to_bytes(&v);
        assert_eq!(bytes.len(), 4); // u32 variant index
        assert_eq!(from_bytes::<Direction>(&bytes).unwrap(), v);
    }
}

#[derive(Encode, Decode, Debug, PartialEq)]
enum Message {
    Ping,
    Move { x: f32, y: f32 },
    Attack(u32),
}

#[test]
fn derive_enum_variants() {
    let cases = [
        Message::Ping,
        Message::Move { x: 1.0, y: 2.0 },
        Message::Attack(999),
    ];
    for msg in cases {
        assert_eq!(from_bytes::<Message>(&to_bytes(&msg)).unwrap(), msg);
    }
}

#[test]
fn derive_enum_unknown_variant() {
    // variant index 99 does not exist
    let bytes = to_bytes(&99u32);
    let result = from_bytes::<Direction>(&bytes);
    assert!(matches!(result, Err(zerec::DecodeError::UnknownVariant(99))));
}

// ── #[zerec(skip)] ───────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct WithSkip {
    written: u32,
    #[zerec(skip)]
    transient: u64,
}

#[test]
fn derive_skip() {
    let v = WithSkip { written: 7, transient: 999 };
    let bytes = to_bytes(&v);
    assert_eq!(bytes.len(), 4); // only `written` on the wire
    let back = from_bytes::<WithSkip>(&bytes).unwrap();
    assert_eq!(back.written, 7);
    assert_eq!(back.transient, 0); // Default::default()
}

// ── #[zerec(via = "...")] ─────────────────────────────────────────────────

// A foreign type we cannot impl Encode/Decode on directly.
struct Velocity { dx: f32, dy: f32 }

struct VelocityAdapter;

impl Adapter<Velocity> for VelocityAdapter {
    type Repr = (f32, f32);
    fn to_repr(v: &Velocity) -> Self::Repr { (v.dx, v.dy) }
    fn from_repr(r: Self::Repr) -> Velocity { Velocity { dx: r.0, dy: r.1 } }
}

#[derive(Encode, Decode)]
struct Projectile {
    #[zerec(via = "VelocityAdapter")]
    vel: Velocity,
    damage: u32,
}

#[test]
fn derive_via_adapter() {
    let p = Projectile { vel: Velocity { dx: 10.0, dy: -3.0 }, damage: 5 };
    let bytes = to_bytes(&p);
    assert_eq!(bytes.len(), 12); // 2 x f32 + u32
    let back = from_bytes::<Projectile>(&bytes).unwrap();
    assert_eq!(back.vel.dx, 10.0);
    assert_eq!(back.vel.dy, -3.0);
    assert_eq!(back.damage, 5);
}

// ── #[zerec(map_enc/map_dec)] ─────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct Packed {
    #[zerec(
        map_enc = "|v: &[f32; 3]| *v",
        map_dec = "|dec| { let a: [f32; 3] = zerec::Decode::decode(dec)?; Ok(a) }"
    )]
    position: [f32; 3],
    hp: u32,
}

#[test]
fn derive_map_enc_dec() {
    let v = Packed { position: [1.0, 2.0, 3.0], hp: 100 };
    assert_eq!(from_bytes::<Packed>(&to_bytes(&v)).unwrap(), v);
}

// ── generics ─────────────────────────────────────────────────────────────

#[derive(Encode, Decode, Debug, PartialEq)]
struct Wrapper<T> {
    value: T,
    tag: u8,
}

#[test]
fn derive_generic_struct() {
    let w = Wrapper { value: 42u32, tag: 1 };
    assert_eq!(from_bytes::<Wrapper<u32>>(&to_bytes(&w)).unwrap(), w);
}