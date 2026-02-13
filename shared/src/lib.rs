//! Lmoa
//!
//!
//!

/// App code
pub mod app;
pub use app::*;

#[cfg(any(feature = "wasm_bindgen", feature = "uniffi"))]
/// FFI bindings for the crate
mod ffi;

/// Server sent events, will be removed
pub mod sse;

/// Data structures
pub mod types;

/// Errors
mod error;
pub use error::*;

pub use crux_core::Core;
pub use crux_http as http;

#[cfg(any(feature = "wasm_bindgen", feature = "uniffi"))]
pub use ffi::CoreFFI;

#[cfg(feature = "uniffi")]
const _: () = assert!(
    uniffi::check_compatible_version("0.29.4"),
    "please use uniffi v0.29.4"
);
#[cfg(feature = "uniffi")]
uniffi::setup_scaffolding!();
