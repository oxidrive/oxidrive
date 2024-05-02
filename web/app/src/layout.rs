use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Centered() -> Element {
    rsx! {
        main {
            role: "main",
            class: "bg-primary-500 min-h-dvh p-12 flex flex-col items-center justify-center",
            Outlet::<Route> {}
        }
    }
}