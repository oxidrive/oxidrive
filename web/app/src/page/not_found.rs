use dioxus::prelude::*;

use crate::{component::*, i18n::use_localizer};

#[component]
pub fn NotFound(path: Vec<String>) -> Element {
    let i18n = use_localizer();

    rsx! {
        Pane {
            Title { h: Heading::H1, color: TitleColor::Primary, {i18n.localize("not-found-title")} }
        }
    }
}
