#![allow(non_snake_case)]

use std::fmt::{Debug, Display};

use crate::{
    page::{Home, Login, NotFound, Setup},
    toast::Toasts,
};
use dioxus::{dioxus_core::CapturedError, prelude::*};
use layout::{AppShell, Centered};
use oxidrive_api::ApiError;

mod api;
mod auth;
mod component;
mod i18n;
mod instance;
mod layout;
mod page;
mod storage;
mod toast;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(AppShell)]
    #[layout(Centered)]
    #[route("/setup")]
    Setup {},
    #[route("/login")]
    Login {},
    #[route("/")]
    Home {},
    #[route("/:..path")]
    NotFound { path: Vec<String> },
}

impl Route {
    pub fn requires_setup(&self) -> bool {
        !matches!(self, Self::Setup {})
    }

    pub fn requires_authentication(&self) -> bool {
        matches!(self, Self::Home {})
    }
}

pub fn App() -> Element {
    auth::init();
    i18n::init();
    api::init();
    instance::init();

    rsx! {
        ErrorBoundary { handle_error: ErrorMessage,
            div {
                Toasts {}
                Router::<Route> {}
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct GenericError {
    error: String,
    message: String,
}

impl<T: Display + Debug> From<ApiError<T>> for GenericError {
    fn from(err: ApiError<T>) -> Self {
        match err {
            ApiError::Api(err) => Self {
                error: err.error.to_string(),
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
