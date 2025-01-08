//! Cargo package information.

use core::borrow::Borrow;

use alloc::{borrow::Cow, vec::Vec};
use serde::Deserialize;

use super::{Author, ResolverVersion, RustEdition};
use crate::{Table, Value};

/// The package information.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Package<'p> {
    name: Cow<'p, str>,
    #[serde(borrow)]
    version: Option<WorkspaceInheritable<Cow<'p, str>>>,
    edition: Option<WorkspaceInheritable<RustEdition>>,
    #[serde(rename = "rust-version")]
    rust_version: Option<WorkspaceInheritable<Cow<'p, str>>>,
    authors: Option<WorkspaceInheritable<Vec<Author<'p>>>>,
    description: Option<WorkspaceInheritable<Cow<'p, str>>>,
    documentation: Option<WorkspaceInheritable<Cow<'p, str>>>,
    readme: Option<WorkspaceInheritable<Cow<'p, str>>>,
    homepage: Option<WorkspaceInheritable<Cow<'p, str>>>,
    repository: Option<WorkspaceInheritable<Cow<'p, str>>>,
    license: Option<WorkspaceInheritable<Cow<'p, str>>>,
    license_file: Option<WorkspaceInheritable<Cow<'p, str>>>,
    keywords: Option<WorkspaceInheritable<Vec<Cow<'p, str>>>>,
    categories: Option<WorkspaceInheritable<Vec<Cow<'p, str>>>>,
    workspace: Option<Cow<'p, str>>,
    build: Option<Cow<'p, str>>,
    links: Option<Cow<'p, str>>,
    publish: Option<WorkspaceInheritable<bool>>,
    metadata: Option<Table<'p>>,
    include: Option<WorkspaceInheritable<Vec<Cow<'p, str>>>>,
    exclude: Option<WorkspaceInheritable<Vec<Cow<'p, str>>>>,
    #[serde(rename = "default-run")]
    default_run: Option<Cow<'p, str>>,
    autobins: Option<bool>,
    autoexamples: Option<bool>,
    autotests: Option<bool>,
    autobenches: Option<bool>,
    resolver: Option<ResolverVersion>,
}

impl<'p> Package<'p> {
    /// The package name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The package version.
    pub fn version(&self) -> Option<WorkspaceInheritable<&str>> {
        self.version.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The Rust edition.
    pub fn edition(&self) -> Option<&WorkspaceInheritable<RustEdition>> {
        self.edition.as_ref()
    }

    /// The required Rust version.
    pub fn rust_version(&self) -> Option<WorkspaceInheritable<&str>> {
        self.rust_version.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The list of authors.
    pub fn authors(&self) -> Option<WorkspaceInheritable<impl Iterator<Item = &Author<'_>>>> {
        self.authors
            .as_ref()
            .map(WorkspaceInheritable::borrow_iteratable)
    }

    /// The package description.
    pub fn description(&self) -> Option<WorkspaceInheritable<&str>> {
        self.description.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The package documentation URL.
    pub fn documentation(&self) -> Option<WorkspaceInheritable<&str>> {
        self.documentation
            .as_ref()
            .map(WorkspaceInheritable::borrow)
    }

    /// The path to the README file.
    pub fn readme(&self) -> Option<WorkspaceInheritable<&str>> {
        self.readme.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The package homepage URL.
    pub fn homepage(&self) -> Option<WorkspaceInheritable<&str>> {
        self.homepage.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The package repository URL.
    pub fn repository(&self) -> Option<WorkspaceInheritable<&str>> {
        self.repository.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The package license.
    pub fn license(&self) -> Option<WorkspaceInheritable<&str>> {
        self.license.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The path to the license file.
    pub fn license_file(&self) -> Option<WorkspaceInheritable<&str>> {
        self.license_file.as_ref().map(WorkspaceInheritable::borrow)
    }

    /// The package keywords.
    pub fn keywords(&self) -> Option<WorkspaceInheritable<impl Iterator<Item = &str>>> {
        self.keywords
            .as_ref()
            .map(WorkspaceInheritable::borrow_iteratable)
    }

    /// The package categories.
    pub fn categories(&self) -> Option<WorkspaceInheritable<impl Iterator<Item = &str>>> {
        self.categories
            .as_ref()
            .map(WorkspaceInheritable::borrow_iteratable)
    }

    /// The workspace path.
    pub fn workspace(&self) -> Option<&str> {
        self.workspace.as_deref()
    }

    /// The build script path.
    pub fn build(&self) -> Option<&str> {
        self.build.as_deref()
    }

    /// The package links.
    pub fn links(&self) -> Option<&str> {
        self.links.as_deref()
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
    pub fn include(&self) -> Option<WorkspaceInheritable<impl Iterator<Item = &str>>> {
        self.include
            .as_ref()
            .map(WorkspaceInheritable::borrow_iteratable)
    }

    /// The paths to exclude.
    pub fn exclude(&self) -> Option<WorkspaceInheritable<impl Iterator<Item = &str>>> {
        self.exclude
            .as_ref()
            .map(WorkspaceInheritable::borrow_iteratable)
    }

    /// The default run command.
    pub fn default_run(&self) -> Option<&str> {
        self.default_run.as_deref()
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
    pub fn uninherited(self) -> Option<W> {
        match self {
            Self::Uninherited(value) => Some(value),
            Self::Inherited => None,
        }
    }

    /// Get a reference to the value if it is uninherited.
    pub fn uninherited_ref(&self) -> Option<&W> {
        match self {
            Self::Uninherited(value) => Some(value),
            Self::Inherited => None,
        }
    }

    /// If it is inherited from the workspace.
    pub fn inherited(&self) -> bool {
        matches!(self, Self::Inherited)
    }

    fn borrow<Borrowed: ?Sized>(&self) -> WorkspaceInheritable<&Borrowed>
    where
        W: AsRef<Borrowed>,
    {
        match self {
            WorkspaceInheritable::Uninherited(d) => WorkspaceInheritable::Uninherited(d.as_ref()),
            WorkspaceInheritable::Inherited => WorkspaceInheritable::Inherited,
        }
    }
}

impl<T> WorkspaceInheritable<Vec<T>> {
    fn borrow_iteratable<'w, U>(&'w self) -> WorkspaceInheritable<impl Iterator<Item = &'w U>>
    where
        T: Borrow<U>,
        U: 'w + ?Sized,
    {
        match self {
            WorkspaceInheritable::Uninherited(d) => {
                WorkspaceInheritable::Uninherited(d.iter().map(Borrow::borrow))
            }
            WorkspaceInheritable::Inherited => WorkspaceInheritable::Inherited,
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
