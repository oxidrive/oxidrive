use dioxus::prelude::*;

#[component]
pub fn Pane(children: Element) -> Element {
    rsx! {
        div { class: "bg-primary-100 py-16 px-4 gap-8 flex flex-col items-center justify-evenly gap-4 w-full md:w-1/2 lg:w-1/3 rounded-3xl ",
            { children }
        }
    }
}
