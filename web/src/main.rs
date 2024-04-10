#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    console_error_panic_hook::set_once();

    launch(App);
}

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn Home() -> Element {
    let mut title = use_signal(|| "Oxidrive");

    rsx! {
        div {
            main { role: "main",
                h1 { "{title}" }
                button { onclick: move |_| title.set("Changed!"), "Click me!" }
            }
        }
    }
}
