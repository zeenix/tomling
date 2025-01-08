use alloc::{borrow::Cow, collections::BTreeMap};
use serde::Deserialize;

use super::Dependencies;

/// The set of target-specific options.
#[derive(Debug, Deserialize)]
pub struct Targets<'t>(#[serde(borrow)] BTreeMap<Cow<'t, str>, Target<'t>>);

impl<'t> Targets<'t> {
    /// Get a target by name.
    pub fn by_name(&self, name: &str) -> Option<&Target<'t>> {
        self.0.get(name)
    }

    /// Iterate over the targets.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Target<'t>)> {
        self.0.iter().map(|(k, v)| (&**k, v))
    }
}

/// The target-specific options, e.g depdenencies.
#[derive(Debug, Deserialize)]
pub struct Target<'t> {
    #[serde(borrow)]
    dependencies: Option<Dependencies<'t>>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<Dependencies<'t>>,
    #[serde(rename = "build-dependencies")]
    build_dependencies: Option<Dependencies<'t>>,
}

impl<'t> Target<'t> {
    /// The dependencies.
    pub fn dependencies(&self) -> Option<&Dependencies<'t>> {
        self.dependencies.as_ref()
    }

    /// The dev dependencies.
    pub fn dev_dependencies(&self) -> Option<&Dependencies<'t>> {
        self.dev_dependencies.as_ref()
    }

    /// The build dependencies.
    pub fn build_dependencies(&self) -> Option<&Dependencies<'t>> {
        self.build_dependencies.as_ref()
    }
}
