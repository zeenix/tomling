//! A TOML table.

use crate::Value;
use alloc::collections::BTreeMap;

/// A TOML table.
#[derive(Debug, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Table<'a>(#[cfg_attr(feature = "serde", serde(borrow))] BTreeMap<&'a str, Value<'a>>);

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
    pub fn iter(&self) -> Iter<'_, 'a> {
        Iter::new(self)
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

/// An iterator over the key-value pairs of a table.
#[derive(Debug)]
pub struct Iter<'i, 'a> {
    iter: alloc::collections::btree_map::Iter<'i, &'a str, Value<'a>>,
}

impl<'t, 'a> Iter<'t, 'a> {
    fn new(table: &'t Table<'a>) -> Iter<'t, 'a> {
        Iter {
            iter: table.0.iter(),
        }
    }
}

impl<'i, 'a> Iterator for Iter<'i, 'a> {
    type Item = (&'a str, &'i Value<'a>);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, v)| (*k, v))
    }
}
