use winnow::error::ContextError;

/// The error type of this library.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An error occurred while parsing the TOML.
    Parse(ParseError),
}

impl core::error::Error for Error {}

impl alloc::fmt::Display for Error {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            Error::Parse(p) => write!(f, "{p}"),
        }
    }
}

/// The context of the `Error::Parse`.
#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    pub(crate) context: ContextError,
}

impl ParseError {
    /// Create a new parse error.
    pub(crate) fn new(context: ContextError) -> Self {
        Self { context }
    }
}

impl alloc::fmt::Display for ParseError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "{}", self.context)
    }
}
