use crate::{api::use_oxidrive_api, component::*, i18n::use_localizer};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::*, Icon};
use oxidrive_api::files::FileUpload;

pub enum ViewMode {
    Grid,
    List,
}

#[component]
pub fn Files(path: Vec<String>) -> Element {
    let api = use_oxidrive_api();
    let i18n = use_localizer();
    let view_mode = use_signal(|| ViewMode::Grid);

    // We can't trigger events programmatically with Dioxus yet
    let upload = eval(
        r#"
const id = await dioxus.recv();
const input = document.getElementById(id);
input.click();
"#,
    );

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
                        upload.send("fab-upload".into()).unwrap();
                    },
                    icon: FaPlus,
                    input {
                        id: "fab-upload",
                        "data-testid": "upload-files",
                        r#type: "file",
                        multiple: true,
                        hidden: true,
                        onchange: move |evt| {
                            let path = path.clone();
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
                                        let file = FileUpload { filename, content };
                                        let _ = api().files().upload(path, file).await.throw();
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
