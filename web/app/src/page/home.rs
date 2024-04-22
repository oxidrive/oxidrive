use crate::{component::*, layout};
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        layout::BoxedCentered {
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
