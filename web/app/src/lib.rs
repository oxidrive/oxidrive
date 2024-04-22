#![allow(non_snake_case)]

use crate::page::{Home, Setup};
use dioxus::prelude::*;

pub mod component;
pub mod layout;
pub mod page;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/setup")]
    Setup {},
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
