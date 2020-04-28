//! Implementation of the WAST text format for wasmer.

#![deny(missing_docs, trivial_numeric_casts, unused_extern_crates)]
#![warn(unused_import_braces)]
#![deny(unstable_features)]
#![cfg_attr(feature = "clippy", plugin(clippy(conf_file = "../../clippy.toml")))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(clippy::new_without_default, clippy::new_without_default_derive)
)]
#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        clippy::float_arithmetic,
        clippy::mut_mut,
        clippy::nonminimal_bool,
        clippy::option_map_unwrap_or,
        clippy::option_map_unwrap_or_else,
        clippy::print_stdout,
        clippy::unicode_not_nfc,
        clippy::use_self
    )
)]

mod errors;
mod spectest;
mod wast;

pub use crate::errors::{DirectiveError, DirectiveErrors};
pub use crate::spectest::spectest_importobject;
pub use crate::wast::Wast;

/// Version number of this crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
