//! Cargo package information.

use alloc::vec::Vec;
use serde::Deserialize;

use super::{Author, ResolverVersion};
use crate::{Table, Value};

/// The package information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Package<'p> {
    name: &'p str,
    #[serde(borrow)]
    version: WorkspaceInheritable<&'p str>,
    edition: Option<WorkspaceInheritable<RustEdition>>,
    #[serde(rename = "rust-version")]
    rust_version: Option<WorkspaceInheritable<&'p str>>,
    authors: Option<WorkspaceInheritable<Vec<Author<'p>>>>,
    description: Option<WorkspaceInheritable<&'p str>>,
    documentation: Option<WorkspaceInheritable<&'p str>>,
    readme: Option<WorkspaceInheritable<&'p str>>,
    homepage: Option<WorkspaceInheritable<&'p str>>,
    repository: Option<WorkspaceInheritable<&'p str>>,
    license: Option<WorkspaceInheritable<&'p str>>,
    license_file: Option<WorkspaceInheritable<&'p str>>,
    keywords: Option<WorkspaceInheritable<Vec<&'p str>>>,
    categories: Option<WorkspaceInheritable<Vec<&'p str>>>,
    workspace: Option<&'p str>,
    build: Option<&'p str>,
    links: Option<&'p str>,
    publish: Option<WorkspaceInheritable<bool>>,
    metadata: Option<Table<'p>>,
    include: Option<WorkspaceInheritable<Vec<&'p str>>>,
    exclude: Option<WorkspaceInheritable<Vec<&'p str>>>,
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
    pub fn version(&self) -> &WorkspaceInheritable<&'p str> {
        &self.version
    }

    /// The Rust edition.
    pub fn edition(&self) -> Option<&WorkspaceInheritable<RustEdition>> {
        self.edition.as_ref()
    }

    /// The required Rust version.
    pub fn rust_version(&self) -> Option<&WorkspaceInheritable<&'p str>> {
        self.rust_version.as_ref()
    }

    /// The list of authors.
    pub fn authors(&self) -> Option<WorkspaceInheritable<&[Author<'p>]>> {
        self.authors.as_ref().map(WorkspaceInheritable::as_slice)
    }

    /// The package description.
    pub fn description(&self) -> Option<WorkspaceInheritable<&str>> {
        self.description.clone()
    }

    /// The package documentation URL.
    pub fn documentation(&self) -> Option<WorkspaceInheritable<&str>> {
        self.documentation.clone()
    }

    /// The path to the README file.
    pub fn readme(&self) -> Option<WorkspaceInheritable<&str>> {
        self.readme.clone()
    }

    /// The package homepage URL.
    pub fn homepage(&self) -> Option<WorkspaceInheritable<&str>> {
        self.homepage.clone()
    }

    /// The package repository URL.
    pub fn repository(&self) -> Option<WorkspaceInheritable<&str>> {
        self.repository.clone()
    }

    /// The package license.
    pub fn license(&self) -> Option<WorkspaceInheritable<&str>> {
        self.license.clone()
    }

    /// The path to the license file.
    pub fn license_file(&self) -> Option<WorkspaceInheritable<&str>> {
        self.license_file.clone()
    }

    /// The package keywords.
    pub fn keywords(&self) -> Option<WorkspaceInheritable<&[&str]>> {
        self.keywords.as_ref().map(WorkspaceInheritable::as_slice)
    }

    /// The package categories.
    pub fn categories(&self) -> Option<WorkspaceInheritable<&[&str]>> {
        self.categories.as_ref().map(WorkspaceInheritable::as_slice)
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
    pub fn publish(&self) -> Option<WorkspaceInheritable<bool>> {
        self.publish.clone()
    }

    /// The package metadata.
    pub fn metadata(&self) -> Option<&Table<'p>> {
        self.metadata.as_ref()
    }

    /// The paths to include.
    pub fn include(&self) -> Option<WorkspaceInheritable<&[&str]>> {
        self.include.as_ref().map(WorkspaceInheritable::as_slice)
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<WorkspaceInheritable<&[&str]>> {
        self.exclude.as_ref().map(WorkspaceInheritable::as_slice)
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

/// The Rust edition.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RustEdition {
    /// Edition 2015.
    E2015,
    /// Edition 2018.
    E2018,
    /// Edition 2021.
    E2021,
    /// Edition 2024.
    E2024,
}

impl TryFrom<Value<'_>> for RustEdition {
    type Error = crate::Error;

    fn try_from(value: Value<'_>) -> Result<Self, Self::Error> {
        match value {
            Value::String(value) => match value.as_ref() {
                "2015" => Ok(Self::E2015),
                "2018" => Ok(Self::E2018),
                "2021" => Ok(Self::E2021),
                "2024" => Ok(Self::E2024),
                _ => Err(crate::Error::Convert {
                    from: "tomling::Value",
                    to: "tomling::cargo::RustEdition",
                }),
            },
            _ => Err(crate::Error::Convert {
                from: "tomling::Value",
                to: "tomling::cargo::RustEdition",
            }),
        }
    }
}
