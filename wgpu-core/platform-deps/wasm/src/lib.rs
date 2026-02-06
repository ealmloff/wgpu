//! This crate exists to allow platform and feature specific features work correctly. The features
//! enabled on this crate are only enabled on `any(target_arch = "wasm32", feature = "wasm-bindgen")` platforms. See wgpu-hal's `Cargo.toml`
//! for more information.
