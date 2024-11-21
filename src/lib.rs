//! A simple TOML parser crate.

#![no_std]
#![forbid(unsafe_code)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
#![warn(unreachable_pub)]

extern crate alloc;

mod value;
pub use value::Value;
pub mod table;
pub use table::Table;
pub mod array;
pub use array::Array;
mod parse;
pub use parse::parse;
#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use serde::from_str;
