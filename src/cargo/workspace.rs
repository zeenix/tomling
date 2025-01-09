//! Cargo package information.

use alloc::{borrow::Cow, vec::Vec};
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
    members: Option<Vec<Cow<'p, str>>>,
    #[serde(rename = "default-members")]
    default_members: Option<Vec<Cow<'p, str>>>,
    exclude: Option<Vec<Cow<'p, str>>>,
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
    pub fn members(&self) -> Option<impl Iterator<Item = &str>> {
        self.members.as_ref().map(|v| v.iter().map(|s| &**s))
    }

    /// The default workspace members.
    pub fn default_members(&self) -> Option<impl Iterator<Item = &str>> {
        self.default_members
            .as_ref()
            .map(|v| v.iter().map(|s| &**s))
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<impl Iterator<Item = &str>> {
        self.exclude.as_ref().map(|v| v.iter().map(|s| &**s))
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
    version: Option<Cow<'p, str>>,
    edition: Option<RustEdition>,
    #[serde(rename = "rust-version")]
    rust_version: Option<Cow<'p, str>>,
    authors: Option<Vec<Author<'p>>>,
    description: Option<Cow<'p, str>>,
    documentation: Option<Cow<'p, str>>,
    readme: Option<Cow<'p, str>>,
    homepage: Option<Cow<'p, str>>,
    repository: Option<Cow<'p, str>>,
    license: Option<Cow<'p, str>>,
    license_file: Option<Cow<'p, str>>,
    keywords: Option<Vec<Cow<'p, str>>>,
    categories: Option<Vec<Cow<'p, str>>>,
    publish: Option<bool>,
    include: Option<Vec<Cow<'p, str>>>,
    exclude: Option<Vec<Cow<'p, str>>>,
}

impl<'p> Package<'p> {
    /// The package version.
    pub fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    /// The Rust edition.
    pub fn edition(&self) -> Option<RustEdition> {
        self.edition
    }

    /// The required Rust version.
    pub fn rust_version(&self) -> Option<&str> {
        self.rust_version.as_deref()
    }

    /// The list of authors.
    pub fn authors(&self) -> Option<&[Author<'p>]> {
        self.authors.as_deref()
    }

    /// The package description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// The package documentation URL.
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }

    /// The path to the README file.
    pub fn readme(&self) -> Option<&str> {
        self.readme.as_deref()
    }

    /// The package homepage URL.
    pub fn homepage(&self) -> Option<&str> {
        self.homepage.as_deref()
    }

    /// The package repository URL.
    pub fn repository(&self) -> Option<&str> {
        self.repository.as_deref()
    }

    /// The package license.
    pub fn license(&self) -> Option<&str> {
        self.license.as_deref()
    }

    /// The path to the license file.
    pub fn license_file(&self) -> Option<&str> {
        self.license_file.as_deref()
    }

    /// The package keywords.
    pub fn keywords(&self) -> Option<impl Iterator<Item = &str>> {
        self.keywords.as_ref().map(|v| v.iter().map(|s| &**s))
    }

    /// The package categories.
    pub fn categories(&self) -> Option<impl Iterator<Item = &str>> {
        self.categories.as_ref().map(|v| v.iter().map(|s| &**s))
    }

    /// Whether the package should be published.
    pub fn publish(&self) -> Option<bool> {
        self.publish
    }

    /// The paths to include.
    pub fn include(&self) -> Option<impl Iterator<Item = &str>> {
        self.include.as_ref().map(|v| v.iter().map(|s| &**s))
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<impl Iterator<Item = &str>> {
        self.exclude.as_ref().map(|v| v.iter().map(|s| &**s))
    }
}
