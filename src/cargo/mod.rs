//! This module provides API for Cargo manifest (`Cargo.toml` files) parsing.
//!
//! This module is only available when `cargo-toml` feature is enabled.

mod author;
mod bench;
mod binary;
mod dependency;
mod dev_dependency;
mod example;
mod features;
mod library;
mod manifest;
mod package;
mod resolver_version;
mod target;
mod test;

pub use author::*;
pub use bench::*;
pub use binary::*;
pub use dependency::*;
pub use dev_dependency::*;
pub use example::*;
pub use features::*;
pub use library::*;
pub use manifest::*;
pub use package::*;
pub use resolver_version::*;
pub use target::*;
pub use test::*;
