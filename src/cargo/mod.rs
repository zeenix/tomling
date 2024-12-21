//! This module provides API for Cargo manifest (`Cargo.toml` files) parsing.
//!
//! This module is only available when `cargo-toml` feature is enabled.

mod author;
mod bench;
mod binary;
pub mod dependency;
mod example;
mod features;
mod library;
mod manifest;
pub mod package;
mod resolver_version;
mod rust_edition;
mod target;
mod test;
pub mod workspace;

pub use author::*;
pub use bench::*;
pub use binary::*;
pub use dependency::{Dependencies, Dependency};
pub use example::*;
pub use features::*;
pub use library::*;
pub use manifest::*;
pub use package::Package;
pub use resolver_version::*;
pub use rust_edition::*;
pub use target::*;
pub use test::*;
pub use workspace::Workspace;
