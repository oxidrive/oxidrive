use dioxus::prelude::*;
use oxidrive_api::instance;

use crate::{
    component::{
        Button, ButtonLink, ButtonVariant, FontWeight, Heading, Logo, Size, TextField, Title,
        TitleColor,
    },
    layout,
};

pub fn Setup() -> Element {
    let status = instance::Status {
        database: "sqlite".into(),
        file_storage: "filesystem".into(),
        public_url: "http://localhost:8080".into(),
        setup_completed: false,
    };

    let mut username = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut password_confirmation = use_signal(String::new);

    rsx! {
        layout::BoxedCentered {
            h1 { Logo { with_name: true } }
            Title { color: TitleColor::Primary, "Create an admin account for" }
            form { class: "flex flex-col justify-space-evenly gap-6 w-full items-center content-stretch justify-center",
                div { class: "flex flex-col gap-4 items-center content-stretch justify-center w-full",
                    TextField {
                        name: "username",
                        value: "{username}",
                        placeholder: "Username",
                        oninput: move |evt: FormEvent| username.set(evt.value())
                    }
                    TextField {
                        name: "password",
                        value: "{password}",
                        placeholder: "Password",
                        oninput: move |evt: FormEvent| password.set(evt.value())
                    }
                    TextField {
                        name: "password_confirmation",
                        value: "{password_confirmation}",
                        placeholder: "Confirm Password",
                        oninput: move |evt: FormEvent| password_confirmation.set(evt.value())
                    }
                }
                div { class: "bg-primary-200 rounded-3xl inline-block p-2",
                    Title {
                        h: Heading::H3,
                        size: Size::Medium,
                        color: TitleColor::Primary,
                        weight: FontWeight::Bold,
                        class: "text-center",
                        "Configuration Recap"
                    }
                      ButtonLink {variant: ButtonVariant::Ghost, to: &status.public_url, "{status.public_url}" }
                      div { class: "pt-1",
                            // RecapEntry { name: "Public URL", value: &status.public_url, size: Size::XS }
                            RecapEntry { name: "Database", value: &status.database }
                            RecapEntry { name: "File Storage", value: &status.file_storage }
                      }
                }
                div { class: "flex flex-col gap-2 content-stretch items-center justify-center",
                    Button { "Complete Setup" }
                    ButtonLink {
                        variant: ButtonVariant::Ghost,
                        to: "https://github.com/oxidrive/oxidrive/discussions",
                        new_tab: true,
                        "Find help"
                    }
                }
            }
        }
    }
}

#[component]
fn RecapEntry(
    name: String,
    #[props(into)] value: String,
    #[props(default = Size::Medium)] size: Size,
) -> Element {
    let size = format!("text-{}", size.class_suffix());
    rsx! {
        span { class: "flex flex-row flex-nowrap gap-4 content-space-evenly items-start justify-between text-primary-500",
            p { class: "whitespace-nowrap font-bold", "{name}:" }
            p { class: "truncate {size}", title: value, "{value}" }
        }
    }
}
