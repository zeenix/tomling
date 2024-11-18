#![no_std]

extern crate alloc;

mod value;
pub use value::Value;
mod map;
pub use map::Map;
mod parse;
pub use parse::parse;
