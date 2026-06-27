# Roadmap

Planned work for upcoming zerec releases.

## v0.1.0 (released)

Core codec. `BufEncoder`, `BufDecoder`, `Encode`, `Decode`, `ZeroBuf`, `Adapter`, `glam` feature.

## v0.2.0 (released)

`zerec-derive` crate gets its first real implementation.
`#[derive(Encode, Decode)]` for structs and enums.
Field attributes:
- `#[zerec(skip)]` — exclude a field from the wire
- `#[zerec(via = "...")]` — route through an adapter type
- `#[zerec(map_enc/map_dec)]` — inline closures for quick conversions

Integration test suite.

## v0.3.0 (current)

Drop the `std` requirement, support `no_std + alloc`.
`COLLECTION_LIMIT` becomes configurable instead of a hardcoded constant.

## v0.4.0 — Benchmarks

Benchmarks via `criterion` against bincode and postcard.
Fuzz testing groundwork.