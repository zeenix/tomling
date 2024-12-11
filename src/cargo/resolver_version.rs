use serde::Deserialize;

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
