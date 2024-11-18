use crate::{Array, Table};

/// A TOML value.
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Array<'a>),
    Table(Table<'a>),
}
