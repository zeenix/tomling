use winnow::error::ContextError;

/// The error type of this library.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// An error occurred while parsing the TOML.
    Parse(ParseError),
    #[cfg(feature = "serde")]
    /// An error occurred while deserializing the TOML.
    ///
    /// This variant is only available when the `serde` feature is enabled.
    Deserialize(DeserializeError),
}

// TODO: Uncomment this when we can bump the MSRV to 1.81:
// impl core::error::Error for Error {}

impl alloc::fmt::Display for Error {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            Error::Parse(p) => write!(f, "{p}"),
            #[cfg(feature = "serde")]
            Error::Deserialize(s) => write!(f, "{s}"),
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

#[derive(Debug, Clone, PartialEq)]
pub struct DeserializeError {
    pub(crate) de: serde::de::value::Error,
}

#[cfg(feature = "serde")]
impl serde::de::Error for Error {
    fn custom<T: alloc::fmt::Display>(msg: T) -> Self {
        Self::Deserialize(DeserializeError {
            de: serde::de::value::Error::custom(msg),
        })
    }
}

#[cfg(feature = "serde")]
impl From<serde::de::value::Error> for Error {
    fn from(e: serde::de::value::Error) -> Self {
        Self::Deserialize(DeserializeError { de: e })
    }
}

impl alloc::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "{}", self.de)
    }
}
