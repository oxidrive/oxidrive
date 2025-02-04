use std::fmt::Display;

pub mod reserved {
    pub const ALL: &[&str] = &[NAME, CONTENT_TYPE, SIZE];

    pub const NAME: &str = "name";
    pub const CONTENT_TYPE: &str = "content_type";
    pub const SIZE: &str = "size";
    pub const FILE_EXT: &str = "ext";
}

const RESERVED_KEYWORDS: &[&str] = &["AND", "OR"];
const INVALID_CHARACTERS: &[char] = &[';', '(', ')'];

#[macro_export]
macro_rules! tag {
    ($($arg:tt)*) => {
        $crate::Tag::parse(format!($($arg)*)).unwrap()
    };
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tag {
    pub key: String,
    pub value: Option<String>,
}

impl Tag {
    pub fn full<K: Into<String>, V: Into<String>>(key: K, value: V) -> Self {
        Self {
            key: key.into(),
            value: Some(value.into()),
        }
    }

    pub fn key<K: Into<String>>(key: K) -> Self {
        Self {
            key: key.into(),
            value: None,
        }
    }

    pub fn parse<S: AsRef<str>>(expr: S) -> Result<Self, ParseError> {
        let expr = expr.as_ref().to_string();

        if expr.chars().any(invalid_character) {
            return Err(ParseError::Invalid(expr));
        }

        for keyword in RESERVED_KEYWORDS {
            if expr.contains(keyword) {
                return Err(ParseError::Invalid(expr));
            }
        }

        let (key, value) = if expr.contains(':') {
            let (key, value) = expr.split_once(':').unwrap();
            (key.to_string(), Some(value.to_string()))
        } else {
            (expr.clone(), None)
        };

        if key.is_empty() {
            return Err(ParseError::Invalid(expr));
        }

        Ok(Self { key, value })
    }

    pub fn parse_public<S: AsRef<str>>(expr: S) -> Result<Self, ParseError> {
        let tag = Self::parse(expr)?;
        if tag.is_reserved() {
            return Err(ParseError::Reserved(tag.key));
        }

        Ok(tag)
    }

    pub fn is_reserved(&self) -> bool {
        reserved::ALL.contains(&self.key.as_str())
    }

    #[inline]
    pub fn is_public(&self) -> bool {
        !self.is_reserved()
    }
}

impl From<(String, Option<String>)> for Tag {
    fn from((key, value): (String, Option<String>)) -> Self {
        Self { key, value }
    }
}

impl From<Tag> for (String, Option<String>) {
    fn from(tag: Tag) -> Self {
        (tag.key, tag.value)
    }
}

impl From<Tag> for (String, Tag) {
    fn from(tag: Tag) -> Self {
        (tag.key.clone(), tag)
    }
}

fn invalid_character(c: char) -> bool {
    INVALID_CHARACTERS.contains(&c)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("parse failed: key '{0}' is reserved")]
    Reserved(String),
    #[error("parse failed: '{0}' is not a valid tag")]
    Invalid(String),
}

impl<S: AsRef<str>> PartialEq<S> for Tag {
    fn eq(&self, other: &S) -> bool {
        // do not inline, otherwise clippy will simplify it to *self == other.as_ref(), which recurses infinitely
        let s = self.to_string();
        s == other.as_ref()
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
    use assert2::{check, let_assert};
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("example", "example", None)]
    #[case("hello:world", "hello", Some("world"))]
    #[case("hello:world:what", "hello", Some("world:what"))]
    #[case("food:🥐", "food", Some("🥐"))]
    #[case("hello:oxidrive tests", "hello", Some("oxidrive tests"))]
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
    #[case(":world")]
    #[case("hello;world:what")]
    #[case("hello:w(orld:what)")]
    #[case("hello:worldANDtest")]
    fn it_fails_to_parse_an_invalid_tag_string(#[case] s: &str) {
        let_assert!(Err(ParseError::Invalid(_)) = Tag::parse(s));
    }

    #[rstest]
    fn it_fails_to_parse_a_reserved_key() {
        for tag in reserved::ALL {
            let_assert!(Err(ParseError::Reserved(_)) = Tag::parse_public(tag));
        }
    }
}
