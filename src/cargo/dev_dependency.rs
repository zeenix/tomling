use alloc::{collections::BTreeMap, vec::Vec};
use serde::Deserialize;

/// The dev dependencies.
#[derive(Debug, Clone, Deserialize)]
pub struct DevDependencies<'d>(#[serde(borrow)] BTreeMap<&'d str, DevDependency<'d>>);

impl<'d> DevDependencies<'d> {
    /// Get a dev dependency by name.
    pub fn by_name(&self, name: &str) -> Option<&DevDependency<'d>> {
        self.0.get(name)
    }

    /// Iterate over the dev dependencies.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &DevDependency<'d>)> {
        self.0.iter().map(|(k, v)| (*k, v))
    }
}

/// A dev dependency.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum DevDependency<'d> {
    /// A dependency defined only by required version.
    VersionOnly(&'d str),
    /// A full dependency definition.
    Full(FullDevDependency<'d>),
}

/// A full dev dependency definition.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct FullDevDependency<'f> {
    version: &'f str,
    features: Option<Vec<&'f str>>,
    workspace: Option<bool>,
}

impl FullDevDependency<'_> {
    /// The version of the dev dependency.
    pub fn version(&self) -> &str {
        self.version
    }

    /// The features of the dev dependency.
    pub fn features(&self) -> Option<&[&str]> {
        self.features.as_deref()
    }

    /// Inherit from the workspace.
    pub fn workspace(&self) -> Option<bool> {
        self.workspace
    }
}

/// Build dependencies.
///
/// They have the same symantics as dev dependencies and hence they are type aliases.
pub type BuildDependencies<'d> = DevDependencies<'d>;
/// A build dependency.
pub type BuildDependency<'d> = DevDependency<'d>;
/// A full build dependency definition.
pub type FullBuildDependency<'f> = FullDevDependency<'f>;
