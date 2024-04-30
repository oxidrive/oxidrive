use crate::{
    api::use_oxidrive_api,
    component::{ButtonLink, ButtonVariant, Loading, Logo, Pane},
    Route,
};
use dioxus::prelude::*;
use oxidrive_api::instance::{Status, StatusResponse};

#[component]
pub fn Home() -> Element {
    let api = use_oxidrive_api();
    let navigator = use_navigator();

    let future = use_resource(move || async move { api().instance().status().await });
    let setup_completed = match future.read().as_ref() {
        Some(Ok(StatusResponse {
            status: Status {
                setup_completed, ..
            },
        })) => *setup_completed,
        Some(Err(err)) => {
            return rsx! {"{err:?}"};
        }
        None => return Loading(),
    };

    if !setup_completed {
        if let Some(err) = navigator.replace(Route::Setup {}) {
            return rsx! {"Error: {err:?}"};
        }
    }

    rsx! {
        Pane {
            h1 { Logo { with_name: true } }
            div { class: "flex flex-col gap-4 items-center justify-evenly",
                p { "Oxidrive is coming soon" }
                ButtonLink {
                    variant: ButtonVariant::Filled,
                    to: "https://github.com/oxidrive/oxidrive/discussions",
                    new_tab: true,
                    "Join the community!"
                }
            }
        }
    }
}
