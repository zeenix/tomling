use crate::Value;
use alloc::collections::BTreeMap;

/// A TOML table.
pub type Map<'a> = BTreeMap<&'a str, Value<'a>>;
