#![allow(non_snake_case)]

use crate::{
    component::Loading,
    page::{Home, Setup},
};
use dioxus::prelude::*;
use layout::Centered;

mod api;
mod component;
mod layout;
mod page;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route {
    #[layout(Centered)]
    #[route("/")]
    Home {},
    #[layout(Centered)]
    #[route("/setup")]
    Setup {},
}

pub fn App() -> Element {
    let init = use_resource(api::init);

    if init.read().is_none() {
        return rsx! { Loading {} };
    }

    rsx! { Router::<Route> {} }
}
