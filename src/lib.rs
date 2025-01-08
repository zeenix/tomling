#![cfg_attr(all(not(feature = "std"), not(test)), no_std)]
#![forbid(unsafe_code)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs
)]
#![warn(unreachable_pub, clippy::std_instead_of_core)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/zeenix/tomling/fc40ab049d833cb79ee3ab9c441b0eebf05494ef/logo.svg"
)]
#![doc = include_str!("../README.md")]

extern crate alloc;

mod value;
pub use value::Value;
pub mod table;
pub use table::Table;
pub mod array;
pub use array::Array;
pub mod datetime;
pub use datetime::{Date, Datetime, Time};
mod parse;
pub use parse::parse;
#[cfg(feature = "serde")]
mod serde;
#[cfg(feature = "serde")]
pub use crate::serde::from_str;
#[cfg(feature = "cargo-toml")]
pub mod cargo;
mod error;
pub use error::{Error, ParseError};
