use std::collections::HashSet;

use crate::{
    api::use_oxidrive_api,
    component::*,
    i18n::use_localizer,
    loc_args,
    route::Route,
    toast::{self, ToastLevel},
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::fa_solid_icons::*, Icon, IconShape};
use oxidrive_api::{
    files::{File, FileKind, FileUpload, ListFilesParams},
    ApiError, List,
};
use web_sys::{Document, HtmlInputElement};

#[derive(Clone, Copy, PartialEq)]
pub enum ViewMode {
    Grid,
    List,
}

#[component]
pub fn Files(path: Vec<String>) -> Element {
    let prefix = format!("/{}", path.join("/"));

    let api = use_oxidrive_api();
    let i18n = use_localizer();
    let view_mode = use_signal(|| ViewMode::Grid);

    let document = web_sys::window()?.document()?;

    let future = use_resource(use_reactive((&prefix,), move |(prefix,)| async move {
        api()
            .files()
            .list(ListFilesParams {
                prefix: Some(prefix),
                ..Default::default()
            })
            .await
    }));

    let files = match future.read().as_ref() {
        Some(Ok(files)) => files.clone(),
        Some(Err(err)) => {
            return Err(err.to_string()).throw();
        }
        None => {
            return rsx! { Loading {} };
        }
    };

    let selected = use_signal(|| HashSet::with_capacity(files.count));

    rsx! {
        main { class: "bg-primary-500 w-full min-h-dvh",
            h1 { class: "sr-only", {i18n.localize("files-title")} }
            Navbar {}
            div { class: "bg-white w-full min-h-[calc(100dvh-66px)] rounded-t-2xl",
                ActionBar { view_mode: view_mode }
                FilesView {
                    files: files,
                    view_mode: view_mode,
                    selected: selected,
                    selected_label: i18n.localize("files-selected"),
                    empty_message: i18n.localize("files-empty")
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

#[component]
fn FilesView(
    files: List<File>,
    view_mode: Signal<ViewMode>,
    selected: Signal<HashSet<String>>,
    selected_label: String,
    empty_message: String,
) -> Element {
    if files.items.is_empty() {
        return rsx! {
            div { class: "w-full h-96 flex flex-col justify-center items-center",
                Icon { class: "fill-primary-200", height: 80, width: 80, icon: FaFolder }
                p { {empty_message} }
            }
        };
    }

    match view_mode() {
        ViewMode::Grid => FilesGrid(files, selected, selected_label),
        ViewMode::List => FilesList(files, selected, selected_label),
    }
}

fn FilesGrid(
    files: List<File>,
    mut selected: Signal<HashSet<String>>,
    selected_label: String,
) -> Element {
    rsx! {
        div { class: "p-4 grid gap-6 grid-cols-[repeat(auto-fill,minmax(160px,1fr))]",
            for file in files.items {
                FileBox {
                    file: file.clone(),
                    selected: selected().contains(&file.path),
                    selected_label: &selected_label,
                    onselected: move |is_selected| {
                        let mut selected = selected.write();
                        if is_selected {
                            selected.insert(file.path.clone());
                        } else {
                            selected.remove(&file.path);
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FileBox(
    file: File,
    selected: bool,
    selected_label: String,
    onselected: EventHandler<bool>,
) -> Element {
    rsx! {
        div {
            title: "{file.name}",
            class: "flex flex-col items-center w-full h-full hover:bg-primary-50 p-2",
            div { class: "flex flex-row justify-between items-center w-full",
                Checkbox {
                    label: selected_label,
                    name: "selected",
                    value: selected,
                    oninput: move |ev: Event<FormData>| {
                        onselected.call(ev.data().parsed::<bool>().throw().unwrap_or_default())
                    }
                }
            }
            FileLink { kind: file.kind, to: &file.path,
                match file.kind {
                    FileKind::File => file_icon(FaFile, "", 80, 80),
                    FileKind::Folder => file_icon(FaFolder, "fill-primary-500", 80, 80),
                }
            }
            div { class: "flex flex-row justify-between items-center gap-4 p-2 w-full",
                FileLink { kind: file.kind, to: file.path, p { class: "text-primary-500 truncate", "{file.name}" } }
                Icon {
                    class: "fill-primary-500 grow-0 shrink-0",
                    height: 15,
                    width: 15,
                    icon: FaEllipsis
                }
            }
        }
    }
}

fn file_icon<I: IconShape + Clone + PartialEq + 'static>(
    icon: I,
    class: &'static str,
    height: u32,
    width: u32,
) -> Element {
    rsx! { Icon { class: format!("m-2 {class}"), height: height, width: width, icon: icon } }
}

fn FilesList(
    files: List<File>,
    mut selected: Signal<HashSet<String>>,
    selected_label: String,
) -> Element {
    rsx! {
        div { class: "p-4 flex flex-col",
            for file in files.items {
                FileRow {
                    file: file.clone(),
                    selected: selected().contains(&file.path),
                    selected_label: &selected_label,
                    onselected: move |is_selected| {
                        let mut selected = selected.write();
                        if is_selected {
                            selected.insert(file.path.clone());
                        } else {
                            selected.remove(&file.path);
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn FileRow(
    file: File,
    selected: bool,
    selected_label: String,
    onselected: EventHandler<bool>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-2 items-center",
            div { class: "flex flex-row flex-nowrap items-center justify-between w-full",
                span { class: "flex flex-row flex-nowrap items-center justify-start",
                    Checkbox {
                        label: selected_label,
                        name: "selected",
                        value: selected,
                        oninput: move |ev: Event<FormData>| {
                            onselected.call(ev.data().parsed::<bool>().throw().unwrap_or_default())
                        }
                    }
                    FileLink { kind: file.kind, to: &file.path,
                        match file.kind {
                            FileKind::File => file_icon(FaFile, "", 40, 40),
                            FileKind::Folder => file_icon(FaFolder, "fill-primary-500", 40, 40),
                        }
                    }
                    FileLink { kind: file.kind, to: file.path, p { "{file.name}" } }
                }
                Icon {
                    class: "fill-primary-500 grow-0 shrink-0",
                    height: 15,
                    width: 15,
                    icon: FaEllipsis
                }
            }
            hr { class: "w-full h-[1px] bg-primary-300" }
        }
    }
}

#[component]
fn FileLink(kind: FileKind, to: String, children: Element) -> Element {
    match kind {
        FileKind::File => children,
        FileKind::Folder => rsx! {
            Link { to: Route::files(to), {children} }
        },
    }
}

fn get_upload_input(document: &Document) -> Option<HtmlInputElement> {
    use wasm_bindgen::JsCast as _;
    let el = document.get_element_by_id("fab-upload")?;
    el.dyn_into().throw()
}
