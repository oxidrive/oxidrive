use dioxus::prelude::*;

use oxidrive_api::{
    instance::{InitialAdminData, SetupRequest, Status, StatusResponse},
    Oxidrive,
};
use serde::Deserialize;

use crate::{
    api::use_oxidrive_api,
    component::{
        Button, ButtonColor, ButtonLink, ButtonVariant, FieldKind, FontWeight, Heading, Loading,
        Logo, Pane, Size, TextField, Title, TitleColor,
    },
    Route,
};

pub fn Setup() -> Element {
    let api = use_oxidrive_api();
    let navigator = use_navigator();

    let future = use_resource(move || async move { api().instance().status().await });

    let Status {
        database,
        file_storage,
        public_url,
        setup_completed,
    } = match future.read().as_ref() {
        Some(Ok(StatusResponse { status })) => status.clone(),
        Some(Err(err)) => return rsx! {"Error: {err:?}"},
        None => return rsx! { Loading {} },
    };

    if setup_completed {
        log::info!("setup has already been completed, redirecting to home page...");
        if let Some(err) = navigator.replace(Route::Home {}) {
            return rsx! {"Error: {err:?}"};
        }
    }

    rsx! {
        Pane {
            h1 { Logo { with_name: true } }
            Title { h: Heading::H2, color: TitleColor::Primary, "Create an admin account" }
            form {
                class: "flex flex-col justify-space-evenly gap-6 w-full items-center content-stretch justify-center",
                onsubmit: move |event| async move {
                    submit(api, event.parsed_values().unwrap()).await.unwrap();
                    if let Some(err) = navigator.replace(Route::Home {}) {
                        panic!("{err:?}");
                    }
                },
                div { class: "flex flex-col gap-4 items-center content-stretch justify-center w-full",
                    TextField { name: "username", placeholder: "Username" }
                    TextField { name: "password", placeholder: "Password", kind: FieldKind::Password }
                    TextField {
                        name: "password_confirmation",
                        placeholder: "Confirm Password",
                        kind: FieldKind::Password
                    }
                }
                div { class: "bg-primary-200 rounded-3xl inline-block p-2",
                    Title {
                        h: Heading::H3,
                        size: Size::Medium,
                        color: TitleColor::PrimaryDark,
                        weight: FontWeight::Bold,
                        class: "text-center",
                        "Configuration Recap"
                    }
                    ButtonLink {
                        variant: ButtonVariant::Ghost,
                        color: ButtonColor::PrimaryDark,
                        to: public_url.clone(),
                        {public_url}
                    }
                    div { class: "pt-1",
                        RecapEntry { name: "Database", value: database }
                        RecapEntry { name: "File Storage", value: file_storage }
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
fn RecapEntry(name: String, #[props(into)] value: String) -> Element {
    rsx! {
        span { class: "flex flex-row flex-nowrap gap-4 content-space-evenly items-start justify-between text-primary-600",
            p { class: "whitespace-nowrap font-bold", "{name}:" }
            p { class: "truncate", title: value, "{value}" }
        }
    }
}

#[derive(Debug, Deserialize)]
struct SetupFormData {
    username: Vec<String>,
    password: Vec<String>,
    password_confirmation: Vec<String>,
}

async fn submit(
    api: Signal<Oxidrive>,
    SetupFormData {
        username,
        password,
        password_confirmation,
    }: SetupFormData,
) -> Result<(), String> {
    let username = username.into_iter().next().unwrap();
    let password = password.into_iter().next().unwrap();
    let password_confirmation = password_confirmation.into_iter().next().unwrap();

    if password != password_confirmation {
        return Err("passwords must match".into());
    }

    api()
        .instance()
        .setup(SetupRequest {
            admin: InitialAdminData { username, password },
        })
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}
