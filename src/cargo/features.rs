use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};
use serde::Deserialize;

/// A Cargo features section.
#[derive(Debug, Deserialize)]
pub struct Features<'f>(#[serde(borrow)] BTreeMap<Cow<'f, str>, Vec<&'f str>>);

impl Features<'_> {
    /// Get the features by name.
    pub fn by_name(&self, name: &str) -> Option<&[&str]> {
        self.0.get(name).map(|v| v.as_slice())
    }

    /// Iterate over the features.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &[&str])> {
        self.0.iter().map(|(k, v)| (&**k, v.as_slice()))
    }
}
