use alloc::{collections::BTreeMap, vec::Vec};
use serde::Deserialize;

/// The dependencies.
#[derive(Debug, Deserialize)]
pub struct Dependencies<'d>(#[serde(borrow)] BTreeMap<&'d str, Dependency<'d>>);

impl<'d> Dependencies<'d> {
    /// Get a dependency by name.
    pub fn by_name(&self, name: &str) -> Option<&Dependency<'d>> {
        self.0.get(name)
    }

    /// Iterate over the dependencies.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Dependency<'d>)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }
}

/// A dependency.
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Dependency<'d> {
    /// A dependency defined only by required version.
    VersionOnly(&'d str),
    /// A full dependency definition.
    Full(FullDependency<'d>),
}

/// A full dependency definition.
#[derive(Debug, Deserialize, PartialEq)]
pub struct FullDependency<'f> {
    version: &'f str,
    optional: Option<bool>,
    features: Option<Vec<&'f str>>,
    workspace: Option<bool>,
}

impl FullDependency<'_> {
    /// The version of the dependency.
    pub fn version(&self) -> &str {
        self.version
    }

    /// Whether the dependency is optional.
    pub fn optional(&self) -> Option<bool> {
        self.optional
    }

    /// The features of the dependency.
    pub fn features(&self) -> Option<&[&str]> {
        self.features.as_deref()
    }

    /// Inherit from the workspace.
    pub fn workspace(&self) -> Option<bool> {
        self.workspace
    }
}
