use alloc::vec::Vec;
use serde::Deserialize;

/// A library target.
#[derive(Debug, Deserialize)]
pub struct Library<'l> {
    name: Option<&'l str>,
    path: Option<&'l str>,
    test: Option<bool>,
    bench: Option<bool>,
    doc: Option<bool>,
    doctest: Option<bool>,
    #[serde(rename = "proc-macro")]
    proc_macro: Option<bool>,
    harness: Option<bool>,
    edition: Option<&'l str>,
    #[serde(rename = "crate-type")]
    library_type: Option<Vec<LibraryType>>,
}

impl Library<'_> {
    /// The name of the library.
    pub fn name(&self) -> Option<&str> {
        self.name
    }

    /// The path to the source of the library.
    pub fn path(&self) -> Option<&str> {
        self.path
    }

    /// Whether or not the library is tested by default by `cargo test`.
    pub fn test(&self) -> Option<bool> {
        self.test
    }

    /// Whether or not the library is benchmarked by default by `cargo bench`.
    pub fn bench(&self) -> Option<bool> {
        self.bench
    }

    /// Whether or not the documentation is built by default by `cargo doc` for this library.
    pub fn doc(&self) -> Option<bool> {
        self.doc
    }

    /// Whether or not the documentation is tested by default by `cargo test`.
    pub fn doctest(&self) -> Option<bool> {
        self.doctest
    }

    /// Whether the library is a procedural macro.
    pub fn proc_macro(&self) -> Option<bool> {
        self.proc_macro
    }

    /// Indicates that the library is a test harness.
    pub fn harness(&self) -> Option<bool> {
        self.harness
    }

    /// The Rust edition this library requires.
    pub fn edition(&self) -> Option<&str> {
        self.edition
    }

    /// The crate type of the library.
    pub fn library_type(&self) -> Option<&[LibraryType]> {
        self.library_type.as_deref()
    }
}

/// The crate type.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum LibraryType {
    /// A normal library
    #[serde(rename = "lib")]
    Lib,
    /// A proc-macro crate.
    #[serde(rename = "proc-macro")]
    ProcMacro,
    /// A rlib library.
    #[serde(rename = "rlib")]
    Rlib,
    /// A dylib library.
    #[serde(rename = "dylib")]
    Dylib,
    /// A static library.
    #[serde(rename = "staticlib")]
    Staticlib,
    /// A cdylib library.
    #[serde(rename = "cdylib")]
    Cdylib,
}
