//! Main functions for generate
#![allow(non_snake_case)]

use crate::app::HdlWizardApp;
use dioxus::prelude::*;
use crate::file_formats::mdf::Mdf;
use std::sync::Arc;
use crate::settings::Settings;
use crate::page::PageType;

#[cfg(not(target_arch = "wasm32"))]
use rfd::AsyncFileDialog;
#[cfg(not(target_arch = "wasm32"))]
use super::genmodel::GenModel;
#[cfg(not(target_arch = "wasm32"))]
use super::documentation;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

#[cfg(not(target_arch = "wasm32"))]
/// Called from the menu to generate the files
async fn gen_all(model: Arc<Mdf>, settings: Settings, mut status: Signal<Option<Result<(), String>>>, _gen_doc : bool, _gen_code : bool) {
    // open file dialog to choose file name
    let file = AsyncFileDialog::new()
        .add_filter("word document", &["html"])
        .add_filter("any", &["*"])
//        .set_directory(&current_path)
        .save_file()
        .await;

    // save operation itself
    if let Some(file) = file {
        let file_path = file.path();
        match std::fs::File::create(file_path) {
            Ok(mut file) => {

                // create the generation model and write it directly for now
                match GenModel::from_model(&model, &settings) {
                    Ok(model) => {
                        match documentation::generate_doc(&model) {
                            Ok(result_doc) => match file.write_all(result_doc.as_bytes()) {
                                Ok(_) => status.set(Some(Ok(()))),
                                Err(error) => status.set(Some(Err(error.to_string())))
                            },
                            Err(error) => status.set(Some(Err(error.to_string())))         
                        }},
                    Err(error) => {
                        status.set(Some(Err(error.to_string())));
                    }
                }
            }
            Err(errormsg) => {
                status.set(Some(Err(format!(
                    "Error while creating file: {}",
                    errormsg
                ))));
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
/// Called from the menu to generate the files
async fn gen_all(_model: Arc<Mdf>, _settings: Settings, mut status: Signal<Option<Result<(), String>>>, _gen_doc : bool, _gen_code : bool) {
    status.set(Some(Ok(())));
}

/// Generate menu
#[component]
pub fn Menu(app_data: Signal<HdlWizardApp>) -> Element {
    // the save operation itself is done in a future, so we share the result through this state, holding
    // just the result. Either an OK or an error message as a string
    let mut save_status: Signal<Option<Result<(), String>>> = use_signal(|| None);

    // read back the result of the future, if any
    match save_status() {
        // save operation completed. send a notification
        Some(Ok(_)) => {
            app_data.with_mut(|data| {
                data.notification = Some("output generation complete".to_owned());
            });
            // clear the open status state so that we don't rerun this
            save_status.set(None);
        }

        // error while saving. Display the error message
        Some(Err(message)) => {
            app_data.with_mut(|data| {
                data.error_message = Some(message.clone());
            });
            // clear the open status state so that we don't rerun this
            save_status.set(None);
        }

        // future not running, or not completed yet
        None => (),
    }
    
    rsx! {
        div { class: "navbar-item has-dropdown is-hoverable",
            a { class: "navbar-link", "Generate" }
            div { class: "navbar-dropdown",
                a {
                    class: "navbar-item",
                    onclick: move |_| {
                        app_data
                            .with_mut(|app| {
                                app.generate_preview = true;
                                app.page_type = PageType::Preview;
                            })
                    },
                    i { class: "fa-solid fa-book mr-1" }
                    "Preview"
                }
                a {
                    class: "navbar-item",
                    onclick: move |_| {
                        let save_status = save_status.to_owned();
                        let model = app_data.read().data.model.clone();
                        let settings = app_data.read().data.settings.clone();

                        spawn({
                            gen_all(model, settings, save_status, true, false)
                        });
                    },
                    i { class: "fa-solid fa-industry mr-1" }
                    "Documentation"
                }
                a {
                    class: "navbar-item",
                    onclick: move |_| {
                        let save_status = save_status.to_owned();
                        let model = app_data.read().data.model.clone();
                        let settings = app_data.read().data.settings.clone();

                        spawn({
                            gen_all(model, settings, save_status, false, true)
                        });
                    },
                    i { class: "fa-solid fa-industry mr-1" }
                    "Code"
                }
                a {
                    class: "navbar-item",
                    onclick: move |_| {
                        let save_status = save_status.to_owned();
                        let model = app_data.read().data.model.clone();
                        let settings = app_data.read().data.settings.clone();

                        spawn({
                            gen_all(model, settings, save_status, true, true)
                        });
                    },
                    i { class: "fa-solid fa-industry mr-1" }
                    "All"
                }
            }
        }
    }
}
