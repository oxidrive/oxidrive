use dioxus::prelude::*;
use oxidrive_api::sessions::{self, Credentials, Session, SessionRequest};
use oxidrive_api::{ApiError, ErrorResponse, Oxidrive};
use serde::Deserialize;

use crate::auth::{store_token, use_token_storage, CurrentUser};
use crate::storage::UseLocalStorage;
use crate::toast::ToastColor;
use crate::{
    api::use_oxidrive_api, auth::use_current_user, component::*, i18n::use_localizer, Route,
};
use crate::{toast, GenericError};

#[component]
pub fn Login() -> Element {
    let current_user = use_current_user();
    let i18n = use_localizer();
    let api = use_oxidrive_api();
    let navigator = use_navigator();
    let token_storage = use_token_storage();

    let mut auth_failed = use_signal(|| false);

    if current_user.read().is_some() {
        navigator.replace(Route::Home {});
    }

    rsx! {
        Pane {
            Logo { with_name: true }
            Title { h: Heading::H1, color: TitleColor::Primary, {i18n.localize("login-title")} }
            if auth_failed() {
                p { class: "text-danger-500", {i18n.localize("login-auth-failed")} }
            }
            form {
                class: "flex flex-col justify-space-evenly gap-6 w-full items-center content-stretch justify-center",
                oninput: move |_| auth_failed.set(false),
                onsubmit: move |event| {
                    let i18n = i18n.clone();
                    async move {
                        if let Some(result) = submit(
                                api,
                                token_storage,
                                current_user,
                                event.parsed_values().unwrap(),
                            )
                            .await
                            .throw()
                        {
                            match result {
                                AuthResult::Success => {
                                    toast::add(
                                        ToastColor::Success,
                                        i18n.localize("login-auth-succeeded"),
                                        i18n.localize("login-auth-succeeded.message"),
                                    );
                                    navigator.replace(Route::Home {});
                                }
                                AuthResult::Failed => {
                                    auth_failed.set(true);
                                }
                            }
                        }
                    }
                },
                div { class: "flex flex-col gap-4 items-center content-stretch justify-center w-full",
                    TextField {
                        name: "username",
                        placeholder: i18n.localize("login-form-username"),
                        error: auth_failed()
                    }
                    TextField {
                        name: "password",
                        placeholder: i18n.localize("login-form-password"),
                        kind: FieldKind::Password,
                        error: auth_failed()
                    }
                }
                div { class: "flex flex-col gap-2 content-stretch items-center justify-center",
                    Button { {i18n.localize("login-form-submit-cta")} }
                }
            }
        }
    }
}

#[derive(Debug, Deserialize)]
struct LoginFormData {
    username: Vec<String>,
    password: Vec<String>,
}

enum AuthResult {
    Success,
    Failed,
}

async fn submit(
    api: Signal<Oxidrive>,
    mut token_storage: UseLocalStorage<Option<String>>,
    current_user: Signal<Option<CurrentUser>>,
    LoginFormData { username, password }: LoginFormData,
) -> Result<AuthResult, GenericError> {
    let username = username.into_iter().next().unwrap();
    let password = password.into_iter().next().unwrap();

    let token = match api()
        .sessions()
        .create(SessionRequest {
            credentials: Credentials::Password { username, password },
        })
        .await
    {
        Ok(Session { token, .. }) => token,
        Err(ApiError::Api(ErrorResponse { error, message })) => {
            return match error {
                sessions::ErrorKind::AuthenticationFailed => Ok(AuthResult::Failed),
                sessions::ErrorKind::UnknownError => Err(GenericError {
                    error: error.to_string(),
                    message,
                }),
            };
        }
        Err(err) => Err(err)?,
    };

    store_token(&mut token_storage, current_user, token);
    Ok(AuthResult::Success)
}
