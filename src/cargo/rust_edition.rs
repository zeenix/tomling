use crate::Value;
use serde::Deserialize;

/// The Rust edition.
#[derive(Debug, Deserialize, Copy, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RustEdition {
    /// Edition 2015.
    #[serde(rename = "2015")]
    E2015,
    /// Edition 2018.
    #[serde(rename = "2018")]
    E2018,
    /// Edition 2021.
    #[serde(rename = "2021")]
    E2021,
    /// Edition 2024.
    #[serde(rename = "2024")]
    E2024,
}

impl TryFrom<Value<'_>> for RustEdition {
    type Error = crate::Error;

    fn try_from(value: Value<'_>) -> Result<Self, Self::Error> {
        match value {
            Value::String(value) => match value.as_ref() {
                "2015" => Ok(Self::E2015),
                "2018" => Ok(Self::E2018),
                "2021" => Ok(Self::E2021),
                "2024" => Ok(Self::E2024),
                _ => Err(crate::Error::Convert {
                    from: "tomling::Value",
                    to: "tomling::cargo::RustEdition",
                }),
            },
            _ => Err(crate::Error::Convert {
                from: "tomling::Value",
                to: "tomling::cargo::RustEdition",
            }),
        }
    }
}
