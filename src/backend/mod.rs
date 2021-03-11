#[cfg(not(target_arch = "wasm32"))]
pub mod kira;

#[cfg(target_arch = "wasm32")]
pub mod quadsnd;