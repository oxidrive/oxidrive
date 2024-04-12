use crate::component::*;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        main {
            role: "main",
            class: "bg-primary-500 h-dvh px-12 flex flex-col items-center justify-center",
            div { class: "bg-primary-100 py-24 px-4 gap-8 flex flex-col items-center justify-evenly gap-4 w-auto rounded-3xl ",
                Logo { with_name: true }
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
}
