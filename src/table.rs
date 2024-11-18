use crate::Value;
use alloc::collections::BTreeMap;

/// A TOML table.
pub type Table<'a> = BTreeMap<&'a str, Value<'a>>;
