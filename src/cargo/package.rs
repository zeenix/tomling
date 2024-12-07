use alloc::vec::Vec;
use serde::Deserialize;

use super::Author;
use crate::{Table, Value};

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

/// The property inheritable from the workspace.
#[derive(Debug, Clone, PartialEq)]
pub enum WorkspaceInheritable<W> {
    /// The value.
    Uninherited(W),
    /// Inherit from the workspace.
    Inherited,
}

impl<W> WorkspaceInheritable<W> {
    /// Get the value if it is uninherited.
    pub fn uninherited(&self) -> Option<&W> {
        match self {
            Self::Uninherited(value) => Some(value),
            Self::Inherited => None,
        }
    }

    /// If it is inherited from the workspace.
    pub fn inherited(&self) -> bool {
        matches!(self, Self::Inherited)
    }
}

impl<T> WorkspaceInheritable<Vec<T>> {
    /// Get the value as a slice if it is uninherited.
    pub fn as_slice(&self) -> WorkspaceInheritable<&[T]> {
        match self {
            Self::Uninherited(value) => WorkspaceInheritable::Uninherited(value.as_slice()),
            Self::Inherited => WorkspaceInheritable::Inherited,
        }
    }
}

impl<W> From<W> for WorkspaceInheritable<W> {
    fn from(value: W) -> Self {
        Self::Uninherited(value)
    }
}

impl<'value, 'de: 'value, W> Deserialize<'de> for WorkspaceInheritable<W>
where
    W: TryFrom<Value<'value>, Error = crate::Error>,
{
    fn deserialize<D>(deserializer: D) -> Result<WorkspaceInheritable<W>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match <Value<'value>>::deserialize(deserializer)? {
            Value::Table(table) => {
                table
                    .get("workspace")
                    .and_then(|v| (v == &Value::Boolean(true)).then_some(()))
                    .ok_or_else(|| serde::de::Error::missing_field("workspace"))?;
                Ok(Self::Inherited)
            }
            value => value
                .try_into()
                .map(Self::Uninherited)
                .map_err(serde::de::Error::custom),
        }
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
