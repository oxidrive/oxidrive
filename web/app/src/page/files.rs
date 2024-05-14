use crate::{
    api::use_oxidrive_api,
    component::*,
    i18n::use_localizer,
    loc_args,
    toast::{self, ToastLevel},
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::*, Icon};
use oxidrive_api::{files::FileUpload, ApiError};
use web_sys::{Document, HtmlInputElement};

pub enum ViewMode {
    Grid,
    List,
}

#[component]
pub fn Files(path: Vec<String>) -> Element {
    let api = use_oxidrive_api();
    let i18n = use_localizer();
    let view_mode = use_signal(|| ViewMode::Grid);

    let document = web_sys::window()?.document()?;

    rsx! {
        main { class: "bg-primary-500 w-full min-h-dvh",
            h1 { class: "sr-only", {i18n.localize("files-title")} }
            Navbar {}
            div { class: "bg-white w-full min-h-[calc(100dvh-66px)] rounded-t-2xl",
                ActionBar { view_mode: view_mode }
                div { class: "w-full h-96 flex flex-col items-center justify-center",
                    Icon { class: "fill-primary-200", height: 80, width: 80, icon: FaFolder }
                    p { {i18n.localize("files-empty")} }
                }
                Fab {
                    label: "Upload",
                    onclick: move |_| {
                        let Some(input) = get_upload_input(&document) else {
                            log::error!("could not find input with id 'fab-upload'");
                            return;
                        };
                        input.click();
                    },
                    icon: FaPlus,
                    input {
                        id: "fab-upload",
                        "data-testid": "upload-files",
                        r#type: "file",
                        multiple: true,
                        hidden: true,
                        onclick: move |evt| evt.stop_propagation(),
                        onchange: move |evt| {
                            let path = path.clone();
                            let i18n = i18n.clone();
                            async move {
                                if let Some(file_engine) = &evt.files() {
                                    let files = file_engine.files();
                                    for filename in files {
                                        let mut path = path.clone();
                                        path.push(filename.clone());
                                        let Some(content) = file_engine.read_file(&filename).await else {
                                            continue;
                                        };
                                        let path = path.join("/");
                                        let args = loc_args!["file" => filename.clone()];
                                        let file = FileUpload { filename, content };
                                        match api().files().upload(path, file).await {
                                            Ok(_) => {
                                                toast::add(
                                                    ToastLevel::Success,
                                                    i18n.localize_with("files-upload-succeeded", &args),
                                                    i18n.localize("files-upload-succeeded.message"),
                                                )
                                            }
                                            Err(ApiError::Api(err)) => {
                                                toast::add(
                                                    ToastLevel::Error,
                                                    i18n.localize_with("files-upload-failed", &args),
                                                    err,
                                                )
                                            }
                                            Err(err) => {
                                                Result::<
                                                    (),
                                                    ApiError<oxidrive_api::files::ErrorKind>,
                                                >::Err(err)
                                                    .throw();
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ActionBar(mut view_mode: Signal<ViewMode>) -> Element {
    rsx! {
        div { class: "p-4 flex flex-row items-center justify-between",
            Icon { class: "fill-primary-900", height: 20, width: 20, icon: FaHouse }
            {match *view_mode.read() {
                ViewMode::Grid => rsx! {
                    button {
                        "aria-label": "Switch to list view",
                        onclick: move |_| {*view_mode.write() = ViewMode::List;},
                        Icon {
                            class: "fill-primary-900",
                            height: 20,
                            width: 20,
                            icon: FaListUl,
                        }
                    }
                },
                ViewMode::List => rsx! {
                    button {
                        "aria-label": "Switch to grid view",
                        onclick: move |_| {*view_mode.write() = ViewMode::Grid;},
                        Icon {
                            class: "fill-primary-900",
                            height: 20,
                            width: 20,
                            icon: FaBorderAll,
                        }
                    }
                },
            }}
        }
    }
}

fn get_upload_input(document: &Document) -> Option<HtmlInputElement> {
    use wasm_bindgen::JsCast as _;
    let el = document.get_element_by_id("fab-upload")?;
    el.dyn_into().throw()
}
