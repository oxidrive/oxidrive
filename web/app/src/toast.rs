use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaXmark, Icon};

static TOASTS: GlobalSignal<Vec<ToastData>> = Signal::global(Vec::default);

#[derive(Debug, Clone, PartialEq)]
struct ToastData {
    color: ToastColor,
    title: String,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastColor {
    Success,
}

pub fn add(color: ToastColor, title: impl ToString, message: impl ToString) {
    TOASTS.write().push(ToastData {
        color,
        title: title.to_string(),
        message: message.to_string(),
    });
}

pub fn Toasts() -> Element {
    if TOASTS().is_empty() {
        return None;
    }

    rsx! {
        div { class: "fixed top-0 right-0 z-50 m-4 flex flex-col gap-2",
            for (idx , toast) in TOASTS.read().iter().enumerate() {
                Toast { idx: idx, toast: toast.clone() }
            }
        }
    }
}

#[component]
fn Toast(idx: usize, toast: ToastData) -> Element {
    spawn(async move {
        gloo_timers::future::TimeoutFuture::new(5000).await;
        TOASTS.write().remove(idx);
    });

    rsx! {
        span {
            class: "p-2 rounded w-[12rem] max-w-[calc(100vw-2rem)] border-2",
            class: match toast.color {
                ToastColor::Success => "bg-success-200 border-success-500",
            },
            span { class: "flex flex-row items-center justify-between",
                h3 { class: "font-bold text-l mb-1", "{toast.title}" }
                button {
                    onclick: move |_| {
                        TOASTS.write().remove(idx);
                    },
                    Icon {
                        width: 20,
                        height: 20,
                        class: match toast.color {
                            ToastColor::Success => "fill-success-800",
                        },
                        icon: FaXmark
                    }
                }
            }
            p { "{toast.message}" }
        }
    }
}
