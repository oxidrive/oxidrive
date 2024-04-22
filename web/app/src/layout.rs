use dioxus::prelude::*;

#[component]
pub fn BoxedCentered(children: Element) -> Element {
    rsx! {
        main {
            role: "main",
            class: "bg-primary-500 min-h-dvh p-12 flex flex-col items-center justify-center",
            div { class: "bg-primary-100 py-16 px-4 gap-8 flex flex-col items-center justify-evenly gap-4 min-w-fit w-full md:w-1/2 lg:w-1/3 rounded-3xl ",
                {children}
            }
        }
    }
}
