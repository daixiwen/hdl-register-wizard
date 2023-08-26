//! Main functions for generate
#![allow(non_snake_case)]

use crate::app::HdlWizardApp;
use dioxus::prelude::*;
use crate::file_formats::mdf::Mdf;
use std::sync::Arc;
use crate::settings::Settings;
use rfd::AsyncFileDialog;

use super::genmodel::GenModel;

#[cfg(not(target_arch = "wasm32"))]
/// Called from the menu to generate the files
async fn gen_all(model: Arc<Mdf>, settings: Settings, status: UseState<Option<Result<(), String>>>, _gen_doc : bool, _gen_code : bool) {
    // open file dialog to choose file name
    let file = AsyncFileDialog::new()
        .add_filter("word document", &["docx"])
        .add_filter("any", &["*"])
//        .set_directory(&current_path)
        .save_file()
        .await;

    // save operation itself
    if let Some(file) = file {
        let file_path = file.path();
        match std::fs::File::create(file_path) {
            Ok(file) => {

                // create the generation model and write it directly for now
                match GenModel::from_model(&model, &settings) {
                    Ok(model) => {
                        match serde_json::to_writer_pretty(file, &model) {
                            Ok(_) => status.set(Some(Ok(()))),
                            Err(error) => status.set(Some(Err(error.to_string())))
                        }         
                    },
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
async fn gen_all(_model: Arc<Mdf>, _settings: Settings, status: UseState<Option<Result<(), String>>>, _gen_doc : bool, _gen_code : bool) {
    status.set(Some(Ok(())));
}

/// Generate menu
#[inline_props]
pub fn Menu<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    // the save operation itself is done in a future, so we share the result through this state, holding
    // just the result. Either an OK or an error message as a string
    let save_status: &UseState<Option<Result<(), String>>> = use_state(cx, || None);

    // read back the result of the future, if any
    match save_status.get() {
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
    
    cx.render(rsx! {
        div { class: "navbar-item has-dropdown is-hoverable",
            a { class: "navbar-link", "Generate" }
            div { class: "navbar-dropdown",
                a {
                    class: "navbar-item",
                    onclick: move |_| {
                        let save_status = save_status.to_owned();
                        let model = app_data.read().data.model.clone();
                        let settings = app_data.read().data.settings.clone();

                        cx.spawn({
                            gen_all(model, settings, save_status, true, false)
                        })
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

                        cx.spawn({
                            gen_all(model, settings, save_status, false, true)
                        })
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

                        cx.spawn({
                            gen_all(model, settings, save_status, true, true)
                        })
                    },
                    i { class: "fa-solid fa-industry mr-1" }
                    "All"
                }
            }
        }
    })
}
