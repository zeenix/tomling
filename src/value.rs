use crate::{datetime, Array, Date, Datetime, Table, Time};
use alloc::{borrow::Cow, string::String, vec::Vec};

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
    /// A date and time.
    Datetime(Datetime),
}

impl<'a> Value<'a> {
    /// Returns the underlying `&str` if the `Value` is a string
    pub fn as_str(&'a self) -> Option<&'a str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the underlying `i64` if the `Value` is an integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            &Self::Integer(i) => Some(i),
            _ => None,
        }
    }

    /// Returns the underlying `f64` if the `Value` is a float
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            &Self::Float(f) => Some(f),
            _ => None,
        }
    }

    /// Returns the underlying `f64` if the `Value` is a float
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            &Self::Boolean(b) => Some(b),
            _ => None,
        }
    }

    /// Returns the underlying [`Array`] if the `Value` is an array
    pub fn as_array(&'a self) -> Option<&'a Array<'a>> {
        match self {
            Self::Array(a) => Some(a),
            _ => None,
        }
    }

    /// Returns the underlying [`Table`] if the `Value` is a table
    pub fn as_table(&'a self) -> Option<&'a Table<'a>> {
        match self {
            Self::Table(t) => Some(t),
            _ => None,
        }
    }

    /// Returns the underlying [`Datetime`] if the `Value` is a date and time
    /// value
    pub fn as_datetime(&self) -> Option<Datetime> {
        match self {
            &Self::Datetime(dt) => Some(dt),
            _ => None,
        }
    }
}

impl<'a, V> FromIterator<V> for Value<'a>
where
    V: Into<Value<'a>>,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Value::Array(iter.into_iter().map(|v| v.into()).collect())
    }
}

impl<'a, K, V> FromIterator<(K, V)> for Value<'a>
where
    K: Into<Cow<'a, str>>,
    V: Into<Value<'a>>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Table(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
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
impl_from!(String => String);
impl_from!(i64 => Integer);
impl_from!(f64 => Float);
impl_from!(bool => Boolean);
impl_from!(Array<'a> => Array);
impl_from!(Table<'a> => Table);
impl_from!(Datetime => Datetime);
impl_from!(Date => Datetime);
impl_from!(Time => Datetime);
impl_from!(datetime::Offset => Datetime);

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
impl_try_from!(Datetime => Datetime);

impl<'a> TryFrom<Value<'a>> for &'a str {
    type Error = crate::Error;

    fn try_from(value: Value<'a>) -> Result<Self, Self::Error> {
        match value {
            Value::String(Cow::Borrowed(s)) => Ok(s),
            _ => Err(crate::Error::Convert {
                from: "tomling::Value",
                to: "&str",
            }),
        }
    }
}

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
impl_try_from_ref!(Datetime => Datetime);

impl<'value, T> TryFrom<Value<'value>> for Vec<T>
where
    T: TryFrom<Value<'value>, Error = crate::Error>,
{
    type Error = crate::Error;

    fn try_from(value: Value<'value>) -> Result<Self, Self::Error> {
        match value {
            Value::Array(array) => array.into_iter().map(|e| e.try_into()).collect(),
            _ => Err(crate::Error::Convert {
                from: "tomling::Value",
                to: "Vec<T>",
            }),
        }
    }
}
