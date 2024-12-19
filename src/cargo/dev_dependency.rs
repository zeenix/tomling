use std::borrow::Cow;

use alloc::{collections::BTreeMap, vec::Vec};
use serde::{de, Deserialize};

use crate::Value;

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
#[derive(Debug, Clone, PartialEq)]
pub struct DevDependency<'d> {
    version: Option<&'d str>,
    features: Option<Vec<&'d str>>,
    workspace: Option<bool>,
    package: Option<&'d str>,
}

impl DevDependency<'_> {
    /// The version of the dependency.
    pub fn version(&self) -> Option<&str> {
        self.version
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
}

impl<'d, 'de: 'd> Deserialize<'de> for DevDependency<'d> {
    fn deserialize<D>(deserializer: D) -> Result<DevDependency<'d>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(Cow::Owned(_)) => Err(de::Error::invalid_type(
                de::Unexpected::Other("not a borrowed string"),
                &"a borrowed string",
            )),
            Value::String(Cow::Borrowed(version)) => Ok(DevDependency {
                version: Some(version),
                features: None,
                workspace: None,
                package: None,
            }),
            Value::Table(table) => {
                let version = table
                    .get("version")
                    .map(|v| match v {
                        Value::String(Cow::Borrowed(s)) => Ok(s),
                        _ => Err(de::Error::invalid_type(
                            de::Unexpected::Other("not a borrowed string"),
                            &"a borrowed string",
                        )),
                    })
                    .transpose()?
                    .cloned();
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
                let package = table
                    .get("package")
                    .map(|v| match v {
                        Value::String(Cow::Borrowed(s)) => Ok(s),
                        _ => Err(de::Error::invalid_type(
                            de::Unexpected::Other("not a borrowed string"),
                            &"a borrowed string",
                        )),
                    })
                    .transpose()?
                    .cloned();
                Ok(DevDependency {
                    version,
                    features,
                    workspace,
                    package,
                })
            }
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("not a string or table"),
                &"a string or table",
            )),
        }
    }
}

/// Build dependencies.
///
/// They have the same symantics as dev dependencies and hence they are type aliases.
pub type BuildDependencies<'d> = DevDependencies<'d>;
/// A build dependency.
pub type BuildDependency<'d> = DevDependency<'d>;
