//! A TOML array.

use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use winnow::stream::Accumulate;

use crate::Value;

/// A TOML array.
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Array<'a>(#[cfg_attr(feature = "serde", serde(borrow))] Vec<Value<'a>>);

impl<'a> Array<'a> {
    /// Create a new array.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Push a value to the array.
    pub fn push(&mut self, value: Value<'a>) {
        self.0.push(value);
    }

    /// Get the value at the given index.\
    pub fn get(&self, index: usize) -> Option<&Value<'a>> {
        self.0.get(index)
    }

    /// Get the length of the array.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// If the array is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// The array content as a slice.
    pub fn as_slice(&self) -> &[Value<'a>] {
        self.0.as_slice()
    }

    /// An iterator over the array.
    pub fn iter(&self) -> Iter<'_, 'a> {
        Iter::new(self)
    }
}

impl<'a> Deref for Array<'a> {
    type Target = [Value<'a>];

    fn deref(&self) -> &Self::Target {
        self.0.as_slice()
    }
}

impl DerefMut for Array<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice()
    }
}

impl<'a> FromIterator<Value<'a>> for Array<'a> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Value<'a>>,
    {
        Self(iter.into_iter().collect())
    }
}

/// An iterator over the values of an array.
#[derive(Debug)]
pub struct Iter<'i, 'a> {
    iter: alloc::slice::Iter<'i, Value<'a>>,
}

impl<'i, 'a> Iter<'i, 'a> {
    fn new(array: &'i Array<'a>) -> Iter<'i, 'a> {
        Iter {
            iter: array.0.iter(),
        }
    }
}

impl<'i, 'a> Iterator for Iter<'i, 'a> {
    type Item = &'i Value<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> Accumulate<Value<'a>> for Array<'a> {
    fn initial(capacity: Option<usize>) -> Self {
        Self(capacity.map(Vec::with_capacity).unwrap_or_default())
    }

    fn accumulate(&mut self, acc: Value<'a>) {
        self.0.push(acc);
    }
}
