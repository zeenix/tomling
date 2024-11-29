use alloc::vec::Vec;
use serde::Deserialize;

use super::LibraryType;

/// An example target.
#[derive(Debug, Deserialize)]
pub struct Example<'b> {
    name: &'b str,
    path: Option<&'b str>,
    test: Option<bool>,
    bench: Option<bool>,
    doc: Option<bool>,
    harness: Option<bool>,
    edition: Option<&'b str>,
    #[serde(rename = "crate-type")]
    library_type: Option<Vec<LibraryType>>,
    #[serde(rename = "required-features")]
    required_features: Option<Vec<&'b str>>,
}

impl Example<'_> {
    /// The name of the example.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The path to the source of the example.
    pub fn path(&self) -> Option<&str> {
        self.path
    }

    /// Whether or not the example is tested by default by `cargo test`.
    pub fn test(&self) -> Option<bool> {
        self.test
    }

    /// Whether or not the example is benchmarked by default by `cargo bench`.
    pub fn bench(&self) -> Option<bool> {
        self.bench
    }

    /// Whether or not the documentation is built by default by `cargo doc` for this example.
    pub fn doc(&self) -> Option<bool> {
        self.doc
    }

    /// Indicates that the example is a test harness.
    pub fn harness(&self) -> Option<bool> {
        self.harness
    }

    /// The Rust edition this example requires.
    pub fn edition(&self) -> Option<&str> {
        self.edition
    }

    /// The required features of the example.
    pub fn required_features(&self) -> Option<&[&str]> {
        self.required_features.as_deref()
    }

    /// The library type of the example.
    pub fn library_type(&self) -> Option<&[LibraryType]> {
        self.library_type.as_deref()
    }
}
