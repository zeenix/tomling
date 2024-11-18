use alloc::vec::Vec;

use crate::Map;

/// A TOML value.
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value<'a>>),
    Table(Map<'a>),
}
