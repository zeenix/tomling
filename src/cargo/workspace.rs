//! Cargo package information.

use alloc::vec::Vec;
use serde::Deserialize;

use super::{Author, Dependencies, ResolverVersion, RustEdition};
use crate::Table;

/// The package information.
#[derive(Debug, Deserialize, Clone)]
pub struct Workspace<'p> {
    #[serde(borrow)]
    package: Option<Package<'p>>,
    resolver: Option<ResolverVersion>,
    dependencies: Option<Dependencies<'p>>,
    members: Option<Vec<&'p str>>,
    #[serde(rename = "default-members")]
    default_members: Option<Vec<&'p str>>,
    exclude: Option<Vec<&'p str>>,
    metadata: Option<Table<'p>>,
    lints: Option<Table<'p>>,
}

impl<'p> Workspace<'p> {
    /// The package information.
    pub fn package(&self) -> Option<&Package<'p>> {
        self.package.as_ref()
    }

    /// The resolver version.
    pub fn resolver(&self) -> Option<ResolverVersion> {
        self.resolver
    }

    /// The dependencies.
    pub fn dependencies(&self) -> Option<&Dependencies<'p>> {
        self.dependencies.as_ref()
    }

    /// The workspace members.
    pub fn members(&self) -> Option<&[&str]> {
        self.members.as_deref()
    }

    /// The default workspace members.
    pub fn default_members(&self) -> Option<&[&str]> {
        self.default_members.as_deref()
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<&[&str]> {
        self.exclude.as_deref()
    }

    /// The workspace metadata.
    pub fn metadata(&self) -> Option<&Table<'p>> {
        self.metadata.as_ref()
    }

    /// The workspace lints.
    pub fn lints(&self) -> Option<&Table<'p>> {
        self.lints.as_ref()
    }
}

/// The package information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Package<'p> {
    #[serde(borrow)]
    version: Option<&'p str>,
    edition: Option<RustEdition>,
    #[serde(rename = "rust-version")]
    rust_version: Option<&'p str>,
    authors: Option<Vec<Author<'p>>>,
    description: Option<&'p str>,
    documentation: Option<&'p str>,
    readme: Option<&'p str>,
    homepage: Option<&'p str>,
    repository: Option<&'p str>,
    license: Option<&'p str>,
    license_file: Option<&'p str>,
    keywords: Option<Vec<&'p str>>,
    categories: Option<Vec<&'p str>>,
    publish: Option<bool>,
    include: Option<Vec<&'p str>>,
    exclude: Option<Vec<&'p str>>,
}

impl<'p> Package<'p> {
    /// The package version.
    pub fn version(&self) -> Option<&str> {
        self.version
    }

    /// The Rust edition.
    pub fn edition(&self) -> Option<RustEdition> {
        self.edition
    }

    /// The required Rust version.
    pub fn rust_version(&self) -> Option<&str> {
        self.rust_version
    }

    /// The list of authors.
    pub fn authors(&self) -> Option<&[Author<'p>]> {
        self.authors.as_deref()
    }

    /// The package description.
    pub fn description(&self) -> Option<&str> {
        self.description
    }

    /// The package documentation URL.
    pub fn documentation(&self) -> Option<&str> {
        self.documentation
    }

    /// The path to the README file.
    pub fn readme(&self) -> Option<&str> {
        self.readme
    }

    /// The package homepage URL.
    pub fn homepage(&self) -> Option<&str> {
        self.homepage
    }

    /// The package repository URL.
    pub fn repository(&self) -> Option<&str> {
        self.repository
    }

    /// The package license.
    pub fn license(&self) -> Option<&str> {
        self.license
    }

    /// The path to the license file.
    pub fn license_file(&self) -> Option<&str> {
        self.license_file
    }

    /// The package keywords.
    pub fn keywords(&self) -> Option<&[&str]> {
        self.keywords.as_deref()
    }

    /// The package categories.
    pub fn categories(&self) -> Option<&[&str]> {
        self.categories.as_deref()
    }

    /// Whether the package should be published.
    pub fn publish(&self) -> Option<bool> {
        self.publish
    }

    /// The paths to include.
    pub fn include(&self) -> Option<&[&str]> {
        self.include.as_deref()
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<&[&str]> {
        self.exclude.as_deref()
    }
}
