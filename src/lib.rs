#![no_std]

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
