use std::borrow::Cow;

use alloc::{collections::BTreeMap, vec::Vec};
use serde::{de, Deserialize};

use crate::Value;

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
}

impl Dependency<'_> {
    /// The version of the dependency.
    pub fn version(&self) -> Option<&str> {
        self.version
    }

    /// Whether the dependency is optional.
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
                Ok(Dependency {
                    version,
                    optional,
                    features,
                    workspace,
                })
            }
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("not a string or table"),
                &"a string or table",
            )),
        }
    }
}
