use crate::{Array, Table};
use alloc::borrow::Cow;

/// A TOML value.
#[derive(Debug, Clone, PartialEq)]
// FIXME: Make this more efficient, by manually implementing `Deserialize``.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Value<'a> {
    /// A string.
    #[cfg_attr(feature = "serde", serde(borrow))]
    String(Cow<'a, str>),
    /// An integer.
    Integer(i64),
    /// A floating-point number.
    Float(f64),
    /// A boolean.
    Boolean(bool),
    /// An array.
    Array(Array<'a>),
    /// A table.
    Table(Table<'a>),
}
