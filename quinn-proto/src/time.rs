//! Time utilities

#[cfg(feature = "wasm-web")]
pub use web_time::*;
#[cfg(not(feature = "wasm-web"))]
pub use std::time::*;
