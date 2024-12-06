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

macro_rules! impl_from {
    ($ty:ty => $variant:ident) => {
        impl<'a> From<$ty> for Value<'a> {
            fn from(value: $ty) -> Self {
                Value::$variant(value.into())
            }
        }
    };
}

impl_from!(Cow<'a, str> => String);
impl_from!(&'a str => String);
impl_from!(i64 => Integer);
impl_from!(f64 => Float);
impl_from!(bool => Boolean);
impl_from!(Array<'a> => Array);
impl_from!(Table<'a> => Table);

macro_rules! impl_try_from {
    ($variant:ident => $ty:ty) => {
        impl<'a> TryFrom<Value<'a>> for $ty {
            type Error = crate::Error;

            fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(value) => Ok(value),
                    _ => Err(crate::Error::Convert {
                        from: "tomling::Value",
                        to: stringify!($ty),
                    }),
                }
            }
        }
    };
}

impl_try_from!(String => Cow<'a, str>);
impl_try_from!(Integer => i64);
impl_try_from!(Float => f64);
impl_try_from!(Boolean => bool);
impl_try_from!(Array => Array<'a>);
impl_try_from!(Table => Table<'a>);

macro_rules! impl_try_from_ref {
    ($variant:ident => $ty:ty) => {
        impl<'a, 'b> TryFrom<&'a Value<'b>> for &'a $ty {
            type Error = crate::Error;

            fn try_from(value: &'a Value<'b>) -> Result<Self, Self::Error> {
                match value {
                    Value::$variant(value) => Ok(&*value),
                    _ => Err(crate::Error::Convert {
                        from: "tomling::Value",
                        to: stringify!($ty),
                    }),
                }
            }
        }
    };
}

impl_try_from_ref!(String => Cow<'b, str>);
impl_try_from_ref!(String => str);
impl_try_from_ref!(Integer => i64);
impl_try_from_ref!(Float => f64);
impl_try_from_ref!(Boolean => bool);
impl_try_from_ref!(Array => Array<'b>);
impl_try_from_ref!(Table => Table<'b>);
