#[cfg(not(feature = "dynamic_link"))]
pub use blue_engine_core::*;

#[cfg(all(feature = "dynamic_link", not(target_family = "wasm")))]
pub use blue_engine_dynamic::*;
