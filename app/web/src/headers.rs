use std::{fmt::Display, str::FromStr};

use axum::{
    extract::{FromRequestParts, OptionalFromRequestParts},
    http::{HeaderMap, StatusCode},
};
use mime_guess::{mime::*, Mime};

use crate::api::error::ApiError;

#[derive(Debug, Default)]
pub struct Accept {
    types: Vec<Mime>,
}

impl Accept {
    pub fn contains<F>(&self, predicate: F) -> bool
    where
        F: FnMut(&Mime) -> bool,
    {
        self.types.iter().any(predicate)
    }

    pub fn json() -> Self {
        Self {
            types: vec![APPLICATION_JSON],
        }
    }

    pub fn accepts_html(&self) -> bool {
        self.contains(|accept| {
            matches!(
                (accept.type_(), accept.subtype()),
                (_, HTML) | (TEXT, STAR) | (STAR, STAR)
            )
        })
    }
}

impl FromStr for Accept {
    type Err = mime_guess::mime::FromStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.chars().filter(|c| *c == ',').count();
        let mut types = Vec::with_capacity(items);

        for typ in s.split(',') {
            let mime = Mime::from_str(typ.trim())?;
            types.push(mime);
        }

        Ok(Self { types })
    }
}

impl Display for Accept {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.types
            .iter()
            .map(Mime::essence_str)
            .collect::<Vec<_>>()
            .join(",")
            .fmt(f)
    }
}

impl<S> OptionalFromRequestParts<S> for Accept
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let headers = HeaderMap::from_request_parts(parts, state)
            .await
            .unwrap_or_else(|_| unreachable!("HeaderMap::from_request_parts is infallible"));

        let Some(header) = headers.get("accept") else {
            return Ok(None);
        };

        let header = header.to_str().map_err(|err| {
            ApiError::new(err)
                .error("INVALID_HEADER_VALUE")
                .status(StatusCode::BAD_REQUEST)
        })?;

        let accept = Accept::from_str(header).map_err(|err| {
            ApiError::new(err)
                .error("INVALID_HEADER_VALUE")
                .status(StatusCode::BAD_REQUEST)
        })?;

        Ok(Some(accept))
    }
}
