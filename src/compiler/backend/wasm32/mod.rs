/// WebAssembly backend for the Aura compiler (wasm32-unknown-unknown).
///
/// This module exposes the Aura compiler pipeline to JavaScript via wasm-bindgen.
/// Build with: `wasm-pack build --target web --features wasm`
#[cfg(target_arch = "wasm32")]
pub mod api;
