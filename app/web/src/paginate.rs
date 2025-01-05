use std::{
    fmt::Display,
    ops::{Deref, DerefMut},
};

use axum::{
    extract::{FromRequestParts, Query},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD as ENGINE, Engine};
use oxidrive_paginate::{Paginate, Slice, DEFAULT_LIMIT};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct PageParams(pub Paginate);

#[derive(Clone, Debug, Deserialize)]
struct Params {
    after: Option<Cursor>,
    first: Option<usize>,

    before: Option<Cursor>,
    last: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
struct Backward {}

impl<S> FromRequestParts<S> for PageParams
where
    S: Send + Sync,
{
    type Rejection = InvalidPageParams;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Params {
            after,
            first,
            before,
            last,
        } = Query::<Params>::from_request_parts(parts, state)
            .await
            .ok()
            .map(|q| q.0)
            .unwrap();

        if first.is_some() && last.is_some() {
            return Err(InvalidPageParams::InvalidParams);
        }

        let first = first.unwrap_or(DEFAULT_LIMIT);

        let paginate = match (after, before) {
            (None, None) => match last {
                Some(last) => Paginate::last(last),
                None => Paginate::first(first),
            },
            (None, Some(before)) => {
                Paginate::backward(before.unwrap()?, last.unwrap_or(DEFAULT_LIMIT))
            }
            (Some(after), None) => Paginate::forward(after.unwrap()?, first),
            (Some(_), Some(_)) => return Err(InvalidPageParams::InvalidParams),
        };

        Ok(Self(paginate))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidPageParams {
    #[error(transparent)]
    InvalidCursor(#[from] InvalidCursor),
    #[error(
        "invalid pagination parameters provided. Either pass after and limit, or before and limit"
    )]
    InvalidParams,
}

impl IntoResponse for InvalidPageParams {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub next: Option<Cursor>,
    pub previous: Option<Cursor>,
}

impl<T> Page<T> {
    pub fn encoded(slice: Slice<T>) -> Self {
        Self {
            items: slice.items,
            next: slice.next.map(Cursor::encode),
            previous: slice.previous.map(Cursor::encode),
        }
    }
}

impl<T> From<Slice<T>> for Page<T> {
    fn from(slice: Slice<T>) -> Self {
        Self {
            items: slice.items,
            next: slice.next.map(Cursor::from),
            previous: slice.previous.map(Cursor::from),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cursor {
    Plain(String),
    Encoded(String),
}

impl Cursor {
    pub fn encode<S: Into<String>>(value: S) -> Self {
        Self::Encoded(ENGINE.encode(value.into()))
    }

    pub fn unwrap(self) -> Result<String, InvalidCursor> {
        let s = match self {
            Cursor::Plain(s) => return Ok(s),
            Cursor::Encoded(s) => s,
        };

        let buf = ENGINE.decode(&s)?;

        String::from_utf8(buf).map_err(|_| InvalidCursor::Invalid(s))
    }
}

impl<S: Into<String>> From<S> for Cursor {
    fn from(value: S) -> Self {
        Self::Plain(value.into())
    }
}

impl Display for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cursor::Plain(s) => s,
            Cursor::Encoded(s) => s,
        }
        .fmt(f)
    }
}

impl Deref for Cursor {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        match self {
            Cursor::Plain(s) => s,
            Cursor::Encoded(s) => s,
        }
    }
}

impl DerefMut for Cursor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Cursor::Plain(s) => s,
            Cursor::Encoded(s) => s,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum InvalidCursor {
    #[error("could not unwrap cursor: {0}")]
    Base64(#[from] base64::DecodeError),
    #[error("invalid cursor '{0}'")]
    Invalid(String),
}

impl Serialize for Cursor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Cursor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match Self::Encoded(value.clone()).unwrap() {
            Ok(_) => Ok(Self::Encoded(value)),
            Err(_) => Ok(Self::Plain(value)),
        }
    }
}
