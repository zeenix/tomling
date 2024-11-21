use serde::Deserialize;

/// Author information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Author<'a> {
    name: &'a str,
    email: Option<&'a str>,
}

impl Author<'_> {
    /// The name of the author.
    pub fn name(&self) -> &str {
        self.name
    }

    /// The email address of the author (if provided).
    pub fn email(&self) -> Option<&str> {
        self.email
    }
}

impl<'a, 'de: 'a> Deserialize<'de> for Author<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Author<'a>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
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

        let s = <&'de str>::deserialize(deserializer)?;
        let (name, email) = parse_author(s).unwrap_or((s, None));

        Ok(Author { name, email })
    }
}
