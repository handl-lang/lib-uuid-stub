//! `uuid-stub` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const UUID_STUB_HNL: &str = include_str!("uuid-stub.hnl");
