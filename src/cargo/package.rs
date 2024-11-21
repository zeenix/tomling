use alloc::vec::Vec;
use serde::Deserialize;

use super::Author;
use crate::Table;

/// The package information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Package<'p> {
    name: &'p str,
    version: &'p str,
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
    workspace: Option<&'p str>,
    build: Option<&'p str>,
    links: Option<&'p str>,
    publish: Option<bool>,
    metadata: Option<Table<'p>>,
    include: Option<Vec<&'p str>>,
    exclude: Option<Vec<&'p str>>,
    #[serde(rename = "default-run")]
    default_run: Option<&'p str>,
    autobins: Option<bool>,
    autoexamples: Option<bool>,
    autotests: Option<bool>,
    autobenches: Option<bool>,
    resolver: Option<ResolverVersion>,
}

impl<'p> Package<'p> {
    /// The package name.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The package version.
    pub fn version(&self) -> &str {
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

    /// The workspace path.
    pub fn workspace(&self) -> Option<&str> {
        self.workspace
    }

    /// The build script path.
    pub fn build(&self) -> Option<&str> {
        self.build
    }

    /// The package links.
    pub fn links(&self) -> Option<&str> {
        self.links
    }

    /// Whether the package should be published.
    pub fn publish(&self) -> Option<bool> {
        self.publish
    }

    /// The package metadata.
    pub fn metadata(&self) -> Option<&Table<'p>> {
        self.metadata.as_ref()
    }

    /// The paths to include.
    pub fn include(&self) -> Option<&[&str]> {
        self.include.as_deref()
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<&[&str]> {
        self.exclude.as_deref()
    }

    /// The default run command.
    pub fn default_run(&self) -> Option<&str> {
        self.default_run
    }

    /// Whether to automatically build binaries.
    pub fn autobins(&self) -> Option<bool> {
        self.autobins
    }

    /// Whether to automatically build examples.
    pub fn autoexamples(&self) -> Option<bool> {
        self.autoexamples
    }

    /// Whether to automatically build tests.
    pub fn autotests(&self) -> Option<bool> {
        self.autotests
    }

    /// Whether to automatically build benchmarks.
    pub fn autobenches(&self) -> Option<bool> {
        self.autobenches
    }

    /// The resolver version.
    pub fn resolver(&self) -> Option<ResolverVersion> {
        self.resolver
    }
}

/// The resolver version.
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResolverVersion {
    /// Resolver version 1.
    #[serde(rename = "1")]
    V1,
    /// Resolver version 2.
    #[serde(rename = "2")]
    V2,
}

/// The Rust edition.
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RustEdition {
    /// Edition 2015.
    #[serde(rename = "2015")]
    E2015,
    /// Edition 2018.
    #[serde(rename = "2018")]
    E2018,
    /// Edition 2021.
    #[serde(rename = "2021")]
    E2021,
    /// Edition 2024.
    #[serde(rename = "2024")]
    E2024,
}
