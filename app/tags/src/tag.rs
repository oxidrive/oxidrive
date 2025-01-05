use std::fmt::Display;

pub use index::*;

mod index;

const INVALID_CHARACTERS: &[char] = &[';'];

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

impl Tag {
    pub fn parse<S: AsRef<str>>(expr: S) -> Result<Self, ParseError> {
        let expr = expr.as_ref().to_string();

        if expr.chars().any(invalid_character) {
            return Err(ParseError::Invalid(expr));
        }

        if expr.chars().filter(|c| *c == ':').count() > 1 {
            return Err(ParseError::Invalid(expr));
        }

        let mut parts = expr.split(':');

        let key = parts
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| ParseError::Invalid(expr.clone()))?
            .to_string();

        let value = parts
            .next()
            .filter(|s| !s.is_empty())
            .map(ToString::to_string);

        if parts.next().is_some() {
            return Err(ParseError::Invalid(expr));
        }

        Ok(Self { key, value })
    }
}

fn invalid_character(c: char) -> bool {
    INVALID_CHARACTERS.contains(&c)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("parse failed: '{0}' is not a valid tag")]
    Invalid(String),
}

impl<S: AsRef<str>> PartialEq<S> for Tag {
    fn eq(&self, other: &S) -> bool {
        self.to_string() == other.as_ref()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.key.fmt(f)?;
        if let Some(value) = &self.value {
            write!(f, ":{value}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("example", "example", None)]
    #[case("hello:world", "hello", Some("world"))]
    fn it_parses_a_valid_tag_string(
        #[case] s: &str,
        #[case] key: &str,
        #[case] value: Option<&str>,
    ) {
        let tag = Tag::parse(s).unwrap();
        check!(tag.key == key);
        check!(tag.value.as_deref() == value);
        check!(tag == s);
    }

    #[rstest]
    #[case("")]
    #[case("hello:world:what")]
    #[case("hello;world:what")]
    fn it_failes_to_parse_an_invalid_tag_string(#[case] s: &str) {
        Tag::parse(s).unwrap_err();
    }
}
