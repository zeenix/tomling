//! The dependencies of a package.

use alloc::{borrow::Cow, collections::BTreeMap, vec::Vec};
use serde::{de, Deserialize};

use crate::{Table, Value};

/// The dependencies.
#[derive(Debug, Clone, Deserialize)]
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
#[derive(Debug, Clone, PartialEq)]
pub struct Dependency<'d> {
    version: Option<&'d str>,
    optional: Option<bool>,
    features: Option<Vec<&'d str>>,
    workspace: Option<bool>,
    package: Option<&'d str>,
    source: Option<Source<'d>>,
}

impl Dependency<'_> {
    /// The version of the dependency.
    pub fn version(&self) -> Option<&str> {
        self.version
    }

    /// Whether the dependency is optional.
    ///
    /// N/A if the it's a dev dependency.
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

    /// The package name.
    pub fn package(&self) -> Option<&str> {
        self.package
    }

    /// The source.
    pub fn source(&self) -> Option<&Source<'_>> {
        self.source.as_ref()
    }
}

impl<'d, 'de: 'd> Deserialize<'de> for Dependency<'d> {
    fn deserialize<D>(deserializer: D) -> Result<Dependency<'d>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(Cow::Owned(_)) => Err(de::Error::invalid_type(
                de::Unexpected::Other("not a borrowed string"),
                &"a borrowed string",
            )),
            Value::String(Cow::Borrowed(version)) => Ok(Dependency {
                version: Some(version),
                optional: None,
                features: None,
                workspace: None,
                package: None,
                source: None,
            }),
            Value::Table(table) => {
                let version = get_string(&table, "version")?;
                let optional = table.get("optional").and_then(|v| v.as_bool());
                let features = table
                    .get("features")
                    .map(|v| match v {
                        Value::Array(a) => a
                            .clone()
                            .into_iter()
                            .map(|v| v.try_into().map_err(de::Error::custom))
                            .collect(),
                        _ => Err(de::Error::invalid_type(
                            de::Unexpected::Other("not an array"),
                            &"an array",
                        )),
                    })
                    .transpose()?;
                let workspace = table.get("workspace").map(|v| v.as_bool().unwrap_or(false));
                let package = get_string(&table, "package")?;
                let source = Source::new(&table)?;

                Ok(Dependency {
                    version,
                    optional,
                    features,
                    workspace,
                    package,
                    source,
                })
            }
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("not a string or table"),
                &"a string or table",
            )),
        }
    }
}

/// A git repository or a local path.
#[derive(Debug, Clone, PartialEq)]
pub enum Source<'r> {
    /// A git repository.
    Git(Git<'r>),
    /// The local file path to a crate.
    Path(&'r str),
}

impl<'r> Source<'r> {
    fn new<E>(table: &Table<'r>) -> Result<Option<Self>, E>
    where
        E: de::Error,
    {
        let git = Git::new(table)?;
        let path = get_string(table, "path")?;

        match (git, path) {
            (Some(git), None) => Ok(Some(Source::Git(git))),
            (None, Some(path)) => Ok(Some(Source::Path(path))),
            (None, None) => Ok(None),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Other("both `git` and `path` specified"),
                &"either `git` or `path`",
            )),
        }
    }

    /// The git repository.
    pub fn git(&self) -> Option<&Git<'r>> {
        match self {
            Source::Git(git) => Some(git),
            _ => None,
        }
    }

    /// The local file path to a crate.
    pub fn path(&self) -> Option<&str> {
        match self {
            Source::Path(path) => Some(path),
            _ => None,
        }
    }
}

/// The git properties.
#[derive(Debug, Clone, PartialEq)]
pub struct Git<'g> {
    repo: &'g str,
    commit: Option<GitCommit<'g>>,
}

impl<'c> Git<'c> {
    fn new<E>(table: &Table<'c>) -> Result<Option<Self>, E>
    where
        E: de::Error,
    {
        let git_repo = get_string(table, "git")?;
        match git_repo {
            Some(git_repo) => Ok(Some(Git {
                repo: git_repo,
                commit: GitCommit::new(table)?,
            })),
            None => Ok(None),
        }
    }

    /// The git repository.
    pub fn repository(&self) -> &str {
        self.repo
    }

    /// The commit of the git dependency.
    pub fn commit(&self) -> Option<&GitCommit<'_>> {
        self.commit.as_ref()
    }
}

/// The commit of a git dependency.
#[derive(Debug, Clone, PartialEq)]
pub enum GitCommit<'c> {
    /// A branch name.
    Branch(&'c str),
    /// A tag.
    Tag(&'c str),
    /// A revision.
    Rev(&'c str),
}

impl<'c> GitCommit<'c> {
    fn new<E>(table: &Table<'c>) -> Result<Option<Self>, E>
    where
        E: de::Error,
    {
        let branch = get_string(table, "branch")?;
        let tag = get_string(table, "tag")?;
        let rev = get_string(table, "rev")?;

        match (branch, tag, rev) {
            (Some(branch), None, None) => Ok(Some(GitCommit::Branch(branch))),
            (None, Some(tag), None) => Ok(Some(GitCommit::Tag(tag))),
            (None, None, Some(rev)) => Ok(Some(GitCommit::Rev(rev))),
            (None, None, None) => Ok(None),
            _ => Err(de::Error::invalid_value(
                de::Unexpected::Other("invalid commit specification"),
                &"either a branch, tag, or rev",
            )),
        }
    }

    /// The branch name.
    pub fn branch(&self) -> Option<&str> {
        match self {
            GitCommit::Branch(branch) => Some(branch),
            _ => None,
        }
    }

    /// The tag.
    pub fn tag(&self) -> Option<&str> {
        match self {
            GitCommit::Tag(tag) => Some(tag),
            _ => None,
        }
    }

    /// The revision.
    pub fn revision(&self) -> Option<&str> {
        match self {
            GitCommit::Rev(rev) => Some(rev),
            _ => None,
        }
    }
}

fn get_string<'t, E>(table: &Table<'t>, key: &str) -> Result<Option<&'t str>, E>
where
    E: de::Error,
{
    table
        .get(key)
        .map(|v| match v {
            Value::String(Cow::Borrowed(s)) => Ok(s),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("not a borrowed string"),
                &"a borrowed string",
            )),
        })
        .transpose()
        .map(|s| s.cloned())
}
