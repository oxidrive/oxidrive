use std::{fmt::Display, str::FromStr};

use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "querylang.pest"]
struct QueryParser;

pub fn parse_query(q: impl AsRef<str>) -> Result<Filter, QueryParseError> {
    let q = q.as_ref();
    if q.is_empty() {
        return Ok(Filter::All);
    }

    let mut query = QueryParser::parse(Rule::query, q).map_err(Box::new)?;
    let query = query.next().unwrap();
    match query.as_rule() {
        Rule::all => Ok(Filter::All),
        Rule::filter => Ok(parse_filter(query.into_inner())),
        unexpected => unreachable!(
            "encountered unexpected rule {:?}({}) while parsing expression query",
            unexpected,
            query.as_str()
        ),
    }
}

#[derive(Debug, thiserror::Error)]
pub enum QueryParseError {
    #[error("query parse failed: {0}")]
    Parse(#[from] Box<pest::error::Error<Rule>>),
}

fn parse_filter(mut pairs: Pairs<Rule>) -> Filter {
    let pair = pairs.next().unwrap();

    let lhs = match pair.as_rule() {
        Rule::tag => parse_tag(pair.into_inner()),
        Rule::tags => parse_tags(pair.into_inner()),
        Rule::filter => parse_filter(pair.into_inner()),
        unexpected => unreachable!(
            "encountered unexpected rule {:?}({}) while parsing expression left-hand side",
            unexpected,
            pair.as_str()
        ),
    };

    let Some(pair) = pairs.next() else {
        return lhs;
    };

    let op = match pair.as_rule() {
        Rule::tag => {
            let rhs = parse_tag(pair.into_inner());
            return Filter::Op {
                lhs: Box::new(lhs),
                op: Op::And,
                rhs: Box::new(rhs),
            };
        }
        Rule::and => Op::And,
        Rule::or => Op::Or,
        unexpected => unreachable!(
            "encountered unexpected rule {:?}({}) while parsing expression operator",
            unexpected,
            pair.as_str()
        ),
    };

    let rhs = parse_filter(pairs);

    Filter::Op {
        lhs: Box::new(lhs),
        op,
        rhs: Box::new(rhs),
    }
}

fn parse_tag(mut pairs: Pairs<Rule>) -> Filter {
    let pair = pairs.next().unwrap();

    let key = match pair.as_rule() {
        Rule::not => return Filter::not(parse_tag(pairs)),
        Rule::key => pair.as_str().into(),
        unexpected => unreachable!(
            "encountered unexpected rule {:?}({}) while parsing tag",
            unexpected,
            pair.as_str()
        ),
    };

    let values = if pairs
        .peek()
        .is_some_and(|p| matches!(p.as_rule(), Rule::value | Rule::quoted_value))
    {
        let value = pairs.next().unwrap();

        value
            .into_inner()
            .map(|pair| match pair.as_rule() {
                Rule::text | Rule::r#match => pair.as_str().into(),
                unexpected => unreachable!(
                    "encountered unexpected rule {:?}({}) while parsing tag value",
                    unexpected,
                    pair.as_str()
                ),
            })
            .collect()
    } else {
        Values::default()
    };

    Filter::Tag { key, values }
}

fn parse_tags(mut pairs: Pairs<Rule>) -> Filter {
    let first = parse_tag(pairs.next().unwrap().into_inner());

    pairs.fold(first, |lhs, pair| {
        let rhs = parse_tag(pair.into_inner());
        Filter::Op {
            lhs: Box::new(lhs),
            op: Op::And,
            rhs: Box::new(rhs),
        }
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Filter {
    All,
    Tag {
        key: String,
        values: Values,
    },
    Op {
        lhs: Box<Filter>,
        op: Op,
        rhs: Box<Filter>,
    },
    Mod {
        modifier: Mod,
        inner: Box<Filter>,
    },
}

impl FromStr for Filter {
    type Err = QueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_query(s)
    }
}

impl Filter {
    fn not(inner: Self) -> Self {
        Self::Mod {
            modifier: Mod::Not,
            inner: Box::new(inner),
        }
    }

    #[cfg(test)]
    fn tag<K: Into<String>, S: Into<Value>, V: IntoIterator<Item = S>>(key: K, value: V) -> Self {
        Self::Tag {
            key: key.into(),
            values: value.into_iter().map(Into::into).collect(),
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => "*".fmt(f)?,
            Self::Tag { key, values } => {
                write!(f, "{key}")?;

                if values.is_empty() {
                    return Ok(());
                }

                write!(f, ":")?;

                values.fmt(f)?;
            }
            Self::Op { lhs, op, rhs } => {
                write!(f, "(")?;
                lhs.fmt(f)?;
                write!(f, " {op} ")?;
                rhs.fmt(f)?;
                write!(f, ")")?;
            }
            Self::Mod { modifier, inner } => write!(f, "{modifier}{inner}")?,
        }

        Ok(())
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct Values(Vec<Value>);

impl Values {
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn has_matches(&self) -> bool {
        self.iter().any(|value| value == &Value::Match)
    }
}

impl IntoIterator for Values {
    type Item = Value;

    type IntoIter = std::vec::IntoIter<Value>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<Value> for Values {
    fn from_iter<T: IntoIterator<Item = Value>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.0.iter().fold(String::new(), |mut s, value| {
            match value {
                Value::Text(value) => {
                    if !s.ends_with('*') {
                        s.push(' ')
                    };

                    s.push_str(value);
                }
                Value::Match => s.push('*'),
            }
            s
        });

        let value = value.trim();

        if value.contains(' ') {
            write!(f, r#""{value}""#)?;
        } else {
            write!(f, "{value}")?;
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Text(String),
    Match,
}

impl<S> From<S> for Value
where
    String: From<S>,
{
    fn from(value: S) -> Self {
        let value = String::from(value);

        match value.as_str() {
            "*" => Self::Match,
            _ => Self::Text(value),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Op {
    And,
    Or,
}

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::And => "AND",
            Self::Or => "OR",
        }
        .fmt(f)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mod {
    Not,
}

impl Display for Mod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Not => "-",
        }
        .fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("*", Filter::All, "*")]
    #[case("", Filter::All, "*")]
    #[case("-test", Filter::not(Filter::tag("test", None::<String>)), "-test")]
    #[case("test hello:world example", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::tag("hello", Some("world"))),
        }),
        op: Op::And,
        rhs: Box::new(Filter::tag("example", None::<String>)),
    }, "((test AND hello:world) AND example)")]
    #[case("test hello:world OR (a AND b)", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::tag("hello", Some("world"))),
        }),
        op: Op::Or,
        rhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("a", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::tag("b", None::<String>)),
        }),
    }, "((test AND hello:world) OR (a AND b))")]
    #[case("test hello:world OR a AND b", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::tag("hello", Some("world"))),
        }),
        op: Op::Or,
        rhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("a", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::tag("b", None::<String>)),
        }),
    }, "((test AND hello:world) OR (a AND b))")]
    #[case("test -hello:world OR a AND b", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::not(Filter::tag("hello", Some("world")))),
        }),
        op: Op::Or,
        rhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("a", None::<String>)),
            op: Op::And,
            rhs: Box::new(Filter::tag("b", None::<String>)),
        }),
    }, "((test AND -hello:world) OR (a AND b))")]
    #[case(
        r#"hello:"test with spaces""#,
        Filter::tag("hello", ["test", "with", "spaces"]),
        r#"hello:"test with spaces""#
    )]
    #[case(
        "hello:start*",
        Filter::tag("hello", ["start".into(), Value::Match]),
        "hello:start*"
    )]
    #[case(
        "hello:midd*le",
        Filter::tag("hello", ["midd".into(), Value::Match, "le".into()]),
        "hello:midd*le"
    )]
    #[case(
        "hello:*end",
        Filter::tag("hello", [Value::Match, "end".into()]),
        "hello:*end"
    )]
    fn it_parses_some_queries(
        #[case] q: &str,
        #[case] expected: Filter,
        #[case] to_string: String,
    ) {
        let parsed = parse_query(q).unwrap();
        check!(parsed == expected);
        check!(parsed.to_string() == to_string);
    }
}
