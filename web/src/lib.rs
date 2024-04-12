#![allow(non_snake_case)]

use crate::page::Home;
use dioxus::prelude::*;

pub mod component;
pub mod page;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[route("/")]
    Home {},
}

pub fn App() -> Element {
    rsx! { Router::<Route> {} }
}
