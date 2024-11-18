#![no_std]

extern crate alloc;

mod value;
pub use value::Value;
mod table;
pub use table::Table;
mod parse;
pub use parse::parse;
