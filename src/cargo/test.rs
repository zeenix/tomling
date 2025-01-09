use alloc::{borrow::Cow, vec::Vec};
use serde::Deserialize;

/// A test target.
#[derive(Debug, Deserialize)]
pub struct Test<'b> {
    name: Cow<'b, str>,
    path: Option<Cow<'b, str>>,
    test: Option<bool>,
    bench: Option<bool>,
    doc: Option<bool>,
    harness: Option<bool>,
    edition: Option<Cow<'b, str>>,
    #[serde(rename = "required-features")]
    required_features: Option<Vec<Cow<'b, str>>>,
}

impl Test<'_> {
    /// The name of the test.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The path to the source of the test.
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Whether or not the test is tested by default by `cargo test`.
    pub fn test(&self) -> Option<bool> {
        self.test
    }

    /// Whether or not the test is benchmarked by default by `cargo bench`.
    pub fn bench(&self) -> Option<bool> {
        self.bench
    }

    /// Whether or not the documentation is built by default by `cargo doc` for this test.
    pub fn doc(&self) -> Option<bool> {
        self.doc
    }

    /// Indicates that the test is a test harness.
    pub fn harness(&self) -> Option<bool> {
        self.harness
    }

    /// The Rust edition this test requires.
    pub fn edition(&self) -> Option<&str> {
        self.edition.as_deref()
    }

    /// The required features of the test.
    pub fn required_features(&self) -> Option<impl Iterator<Item = &str>> {
        self.required_features
            .as_ref()
            .map(|v| v.iter().map(|s| &**s))
    }
}
