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
    let key = pairs.next().unwrap().as_str().into();

    let value = if pairs.peek().is_some_and(|p| p.as_rule() == Rule::value) {
        pairs.next().map(|p| p.as_str().into())
    } else {
        None
    };

    Filter::Tag { key, value }
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
        value: Option<String>,
    },
    Op {
        lhs: Box<Filter>,
        op: Op,
        rhs: Box<Filter>,
    },
}

impl FromStr for Filter {
    type Err = QueryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_query(s)
    }
}

impl Filter {
    #[cfg(test)]
    fn tag<S: Into<String>>(key: S, value: Option<S>) -> Self {
        Self::Tag {
            key: key.into(),
            value: value.map(Into::into),
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => "*".fmt(f)?,
            Self::Tag { key, value } => {
                write!(f, "{key}")?;
                if let Some(value) = value {
                    write!(f, ":{value}")?;
                }
            }
            Self::Op { lhs, op, rhs } => {
                write!(f, "(")?;
                lhs.fmt(f)?;
                write!(f, " {op} ")?;
                rhs.fmt(f)?;
                write!(f, ")")?;
            }
        }

        Ok(())
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

#[cfg(test)]
mod tests {
    use assert2::check;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("*", Filter::All, "*")]
    #[case("", Filter::All, "*")]
    #[case("test hello:world example", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None)),
            op: Op::And,
            rhs: Box::new(Filter::tag("hello", Some("world"))),
        }),
        op: Op::And,
        rhs: Box::new(Filter::tag("example", None)),
    }, "((test AND hello:world) AND example)")]
    #[case("test hello:world OR (a AND b)", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None)),
            op: Op::And,
            rhs: Box::new(Filter::tag("hello", Some("world"))),
        }),
        op: Op::Or,
        rhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("a", None)),
            op: Op::And,
            rhs: Box::new(Filter::tag("b", None)),
        }),
    }, "((test AND hello:world) OR (a AND b))")]
    #[case("test hello:world OR a AND b", Filter::Op {
        lhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("test", None)),
            op: Op::And,
            rhs: Box::new(Filter::tag("hello", Some("world"))),
        }),
        op: Op::Or,
        rhs: Box::new(Filter::Op {
            lhs: Box::new(Filter::tag("a", None)),
            op: Op::And,
            rhs: Box::new(Filter::tag("b", None)),
        }),
    }, "((test AND hello:world) OR (a AND b))")]
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
