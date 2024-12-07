use alloc::borrow::Cow;

use crate::Value;

/// Author information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author<'a> {
    name: Cow<'a, str>,
    email: Option<Cow<'a, str>>,
}

impl Author<'_> {
    /// The name of the author.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The email address of the author (if provided).
    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

impl<'value> TryFrom<Value<'value>> for Author<'value> {
    type Error = crate::Error;

    fn try_from(value: Value<'value>) -> Result<Author<'value>, Self::Error> {
        match value {
            Value::String(Cow::Borrowed(s)) => {
                use winnow::{
                    combinator::{separated_pair, terminated},
                    token::take_until,
                    Parser,
                };

                let parse_author = |s| {
                    separated_pair(
                        take_until::<_, _, ()>(1.., " <"),
                        " <",
                        terminated(take_until(1.., '>'), '>'),
                    )
                    .map(|(name, email)| (name, Some(email)))
                    .parse(s)
                };

                let (name, email) = parse_author(s).unwrap_or((s, None));
                Ok(Author {
                    name: name.into(),
                    email: email.map(Into::into),
                })
            }
            _ => Err(crate::Error::Convert {
                from: "tomling::Value",
                to: "tomling::cargo::Author",
            }),
        }
    }
}
