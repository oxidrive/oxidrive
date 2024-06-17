#![allow(non_snake_case)]

use std::fmt::Debug;

use crate::{route::Route, toast::Toasts};
use dioxus::{dioxus_core::CapturedError, prelude::*};
use oxidrive_api::ApiError;

mod api;
mod auth;
mod component;
mod i18n;
mod layout;
mod page;
mod route;
mod storage;
mod toast;

pub fn App() -> Element {
    api::init();
    i18n::init();
    auth::init();

    rsx! {
        ErrorBoundary { handle_error: ErrorMessage,
            Toasts {}
            Router::<Route> {}
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct GenericError {
    error: String,
    message: String,
}

impl From<ApiError> for GenericError {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Api(err) => Self {
                error: err.error,
                message: err.message,
            },
            ApiError::Network(err) => Self {
                error: "network".into(),
                message: err.to_string(),
            },
        }
    }
}

fn ErrorMessage(err: CapturedError) -> Element {
    log::error!("{:?}", err);

    rsx! {
        div { p { "{err}" } }
    }
}
