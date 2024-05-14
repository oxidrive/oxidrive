use crate::{
    api::use_oxidrive_api,
    component::{
        Button, ButtonColor, ButtonLink, ButtonVariant, FieldKind, FontWeight, Heading, Loading,
        Logo, Pane, Size, TextField, Title, TitleColor,
    },
    i18n::use_localizer,
    toast::{self, ToastLevel},
};
use dioxus::prelude::*;
use oxidrive_api::{
    instance::{InitialAdminData, SetupRequest, Status, StatusResponse},
    Oxidrive,
};
use serde::Deserialize;

pub fn Setup() -> Element {
    let i18n = use_localizer();
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
        Some(Err(err)) => {
            return Err(err.to_string()).throw();
        }
        None => {
            return rsx! { Loading {} };
        }
    };

    if setup_completed {
        navigator.replace("/files");
    }

    rsx! {
        Pane {
            Logo { with_name: true }
            Title { h: Heading::H1, color: TitleColor::Primary, {i18n.localize("setup-title")} }
            form {
                class: "flex flex-col justify-space-evenly gap-6 w-full items-center content-stretch justify-center",
                onsubmit: move |event| {
                    let i18n = i18n.clone();
                    async move {
                        let _ = submit(api, event.parsed_values().unwrap()).await.throw();
                        toast::add(
                            ToastLevel::Success,
                            i18n.localize("setup-succeeded"),
                            i18n.localize("setup-succeeded.message"),
                        );
                        navigator.replace("/");
                    }
                },
                div { class: "flex flex-col gap-4 items-center content-stretch justify-center w-full",
                    TextField { name: "username", placeholder: i18n.localize("setup-form-username") }
                    TextField {
                        name: "password",
                        placeholder: i18n.localize("setup-form-password"),
                        kind: FieldKind::Password
                    }
                    TextField {
                        name: "password_confirmation",
                        placeholder: i18n.localize("setup-form-confirm-password"),
                        kind: FieldKind::Password
                    }
                }
                div { class: "bg-primary-200 rounded-3xl inline-block p-2",
                    Title {
                        h: Heading::H2,
                        size: Size::Medium,
                        color: TitleColor::PrimaryDark,
                        weight: FontWeight::Bold,
                        class: "text-center",
                        {i18n.localize("setup-form-configuration-recap")}
                    }
                    ButtonLink {
                        variant: ButtonVariant::Ghost,
                        color: ButtonColor::PrimaryDark,
                        to: public_url.clone(),
                        { public_url }
                    }
                    div { class: "pt-1",
                        RecapEntry {
                            name: i18n.localize("setup-form-configuration-recap.database"),
                            value: database
                        }
                        RecapEntry {
                            name: i18n.localize("setup-form-configuration-recap.file-storage"),
                            value: file_storage
                        }
                    }
                }
                div { class: "flex flex-col gap-2 content-stretch items-center justify-center",
                    Button { {i18n.localize("setup-form-submit-cta")} }
                    ButtonLink {
                        variant: ButtonVariant::Ghost,
                        to: "https://github.com/oxidrive/oxidrive/discussions",
                        new_tab: true,
                        {i18n.localize("setup-form-help-cta")}
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
