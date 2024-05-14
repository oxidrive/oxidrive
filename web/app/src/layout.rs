use crate::{auth::use_current_user, component::Loading, Route};
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

#[component]
pub fn AppShell() -> Element {
    let navigator = use_navigator();
    let route = use_route::<Route>();
    let current_user = use_current_user();

    if route.requires_authentication() && current_user.read().is_none() {
        navigator.replace(Route::Login {
            redirect_to: route.to_string(),
        })?;
        return rsx! { Loading {} };
    }

    rsx! { Outlet::<Route> {} }
}
