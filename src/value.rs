use alloc::vec::Vec;

use crate::Table;

/// A TOML value.
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value<'a>>),
    Table(Table<'a>),
}
