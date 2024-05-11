use crate::{auth::use_current_user, component::Loading, instance::use_instance_status, Route};
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
    let status = use_instance_status();

    if route.requires_setup() && !status.read().setup_completed {
        navigator.replace(Route::Setup {})?;
        return rsx! { Loading {} };
    }

    if route.requires_authentication() && current_user.read().is_none() {
        navigator.replace(Route::Login {})?;
        return rsx! { Loading {} };
    }

    rsx! { Outlet::<Route> {} }
}
