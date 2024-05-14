use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::FaXmark, Icon};
use strum::Display;

static TOASTS: GlobalSignal<Vec<ToastData>> = Signal::global(Vec::default);

#[derive(Debug, Clone, PartialEq)]
struct ToastData {
    level: ToastLevel,
    title: String,
    message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ToastLevel {
    Success,
    Error,
}

pub fn add(color: ToastLevel, title: impl ToString, message: impl ToString) {
    TOASTS.write().push(ToastData {
        level: color,
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
            "data-testid": "toast-{idx}",
            "data-toast-level": "{toast.level}",
            class: "p-2 rounded min-w-[12rem] max-w-[75vw] md:max-w-[25vw] border-2",
            class: match toast.level {
                ToastLevel::Success => "bg-success-200 border-success-500",
                ToastLevel::Error => "bg-danger-200 border-danger-500",
            },
            span { class: "flex flex-row items-center justify-between",
                h3 { class: "font-bold text-l mb-1 break-all", "{toast.title}" }
                button {
                    onclick: move |_| {
                        TOASTS.write().remove(idx);
                    },
                    Icon {
                        width: 20,
                        height: 20,
                        class: match toast.level {
                            ToastLevel::Success => "fill-success-800",
                            ToastLevel::Error => "fill-danger-800",
                        },
                        icon: FaXmark
                    }
                }
            }
            p { "{toast.message}" }
        }
    }
}
