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
    /// Type conversion error.
    Convert {
        /// The type from which the conversion was attempted.
        from: &'static str,
        /// The type to which the conversion was attempted.
        to: &'static str,
    },
    /// Invalid date and time encoding.
    Datetime,
}

// TODO: Implement core::error::Error instead when we can bump the MSRV to 1.81.
#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Parse(p) => Some(p),
            #[cfg(feature = "serde")]
            Error::Deserialize(d) => Some(d),
            Error::Convert { .. } => None,
            Error::Datetime => None,
        }
    }
}

impl alloc::fmt::Display for Error {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            Error::Parse(p) => write!(f, "{p}"),
            #[cfg(feature = "serde")]
            Error::Deserialize(s) => write!(f, "{s}"),
            Error::Convert { from, to } => write!(f, "cannot convert from {from} to {to}"),
            Error::Datetime => write!(f, "invalid date and time encoding"),
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

#[cfg(feature = "std")]
impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        // For some reason `winnow::error::ContextError` doesn't implement `std::error::Error`.
        None
    }
}

#[cfg(feature = "serde")]
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

#[cfg(feature = "serde")]
impl alloc::fmt::Display for DeserializeError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        write!(f, "{}", self.de)
    }
}

#[cfg(all(feature = "std", feature = "serde"))]
impl std::error::Error for DeserializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.de)
    }
}
