//! Entrypoints for the standard C API

#[macro_use]
pub mod macros;

pub mod engine;

/// cbindgen:ignore
pub mod externals;

/// cbindgen:ignore
pub mod instance;

/// cbindgen:ignore
pub mod module;

/// cbindgen:ignore
mod own;

/// cbindgen:ignore
pub mod store;

/// cbindgen:ignore
pub mod trap;

/// cbindgen:ignore
pub mod types;

/// cbindgen:ignore
pub mod value;

#[cfg(feature = "wasi")]
pub mod wasi;

pub mod wasmer;

#[cfg(feature = "wat")]
pub mod wat;
