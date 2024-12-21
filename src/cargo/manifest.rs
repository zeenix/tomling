use alloc::vec::Vec;
use serde::Deserialize;

use super::{Bench, Binary, Dependencies, Features, Library, Package, Targets, Test, Workspace};

/// A parsed `Cargo.toml` file.
#[derive(Debug, Deserialize)]
pub struct Manifest<'c> {
    #[serde(borrow)]
    package: Option<Package<'c>>,
    workspace: Option<Workspace<'c>>,
    dependencies: Option<Dependencies<'c>>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<Dependencies<'c>>,
    #[serde(rename = "build-dependencies")]
    build_dependencies: Option<Dependencies<'c>>,
    #[serde(rename = "target")]
    targets: Option<Targets<'c>>,
    features: Option<Features<'c>>,
    #[serde(rename = "lib")]
    library: Option<Library<'c>>,
    #[serde(rename = "bin")]
    binaries: Option<Vec<Binary<'c>>>,
    #[serde(rename = "example")]
    examples: Option<Vec<Binary<'c>>>,
    #[serde(rename = "test")]
    tests: Option<Vec<Test<'c>>>,
    #[serde(rename = "bench")]
    benches: Option<Vec<Bench<'c>>>,
}

impl<'c> Manifest<'c> {
    /// The package name.
    pub fn package(&self) -> Option<&Package<'c>> {
        self.package.as_ref()
    }

    /// The workspace.
    pub fn workspace(&self) -> Option<&Workspace<'c>> {
        self.workspace.as_ref()
    }

    /// The dependencies.
    pub fn dependencies(&self) -> Option<&Dependencies<'c>> {
        self.dependencies.as_ref()
    }

    /// The dev dependencies.
    pub fn dev_dependencies(&self) -> Option<&Dependencies<'c>> {
        self.dev_dependencies.as_ref()
    }

    /// The build dependencies.
    pub fn build_dependencies(&self) -> Option<&Dependencies<'c>> {
        self.build_dependencies.as_ref()
    }

    /// The targets.
    pub fn targets(&self) -> Option<&Targets<'c>> {
        self.targets.as_ref()
    }

    /// The features.
    pub fn features(&self) -> Option<&Features<'c>> {
        self.features.as_ref()
    }

    /// The library section.
    pub fn library(&self) -> Option<&Library<'c>> {
        self.library.as_ref()
    }

    /// The binaries.
    pub fn binaries(&self) -> Option<&[Binary<'c>]> {
        self.binaries.as_deref()
    }

    /// The examples.
    pub fn examples(&self) -> Option<&[Binary<'c>]> {
        self.examples.as_deref()
    }

    /// The tests.
    pub fn tests(&self) -> Option<&[Test<'c>]> {
        self.tests.as_deref()
    }

    /// The benches.
    pub fn benches(&self) -> Option<&[Bench<'c>]> {
        self.benches.as_deref()
    }
}
