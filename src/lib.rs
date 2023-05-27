//! Short format for time durations.
// #[cfg(feature = "wasm")]
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(test)]
mod float32; // NB: Included only to document precision issues.
mod float64;

/// Format 64-bit float of seconds into a short string of five characters.
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn present(seconds: f64) -> String {
    float64::present(seconds)
}
