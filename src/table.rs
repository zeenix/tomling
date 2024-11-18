use crate::Value;
use alloc::collections::BTreeMap;

/// A TOML table.
#[derive(Debug, PartialEq, Default)]
pub struct Table<'a>(BTreeMap<&'a str, Value<'a>>);

impl<'a> Table<'a> {
    /// Create a new table.
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Insert a key-value pair into the table.
    pub fn insert(&mut self, key: &'a str, value: Value<'a>) {
        self.0.insert(key, value);
    }

    /// Get the value for the given key.
    pub fn get(&self, key: &str) -> Option<&Value<'a>> {
        self.0.get(key)
    }

    /// Get the length of the table.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// If the table is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get an iterator over the key-value pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'a str, &Value<'a>)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }

    pub(crate) fn entry(
        &mut self,
        key: &'a str,
    ) -> crate::alloc::collections::btree_map::Entry<'_, &'a str, Value<'a>> {
        self.0.entry(key)
    }

    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut Value<'a>> {
        self.0.get_mut(key)
    }
}

impl<'a> FromIterator<(&'a str, Value<'a>)> for Table<'a> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (&'a str, Value<'a>)>,
    {
        Self(iter.into_iter().collect())
    }
}
