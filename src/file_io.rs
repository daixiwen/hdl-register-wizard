//! File load and save, for both web and desktop
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use crate::keys::KeyAction;
use crate::gui_blocks;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

/// return the file name and its parent path as strings from the handle returned by rfd
#[cfg(not(target_arch = "wasm32"))]
fn file_name(handle : &rfd::FileHandle) -> (String, String) {
    (handle.path().to_str().unwrap_or_default().to_owned(), handle.path().parent().unwrap_or(Path::new("/")).to_str().unwrap_or_default().to_owned())
}

/// return the file name and its parent path (ignored on web) as strings from the handle returned by rfd
#[cfg(target_arch = "wasm32")]
fn file_name(handle : &rfd::FileHandle) -> (String, String) {
    (handle.file_name(), Default::default())
}

/// Open/Load file menu item
#[component]
pub fn Open(app_data: Signal<HdlWizardApp>, key_action : Signal<Option<KeyAction>>) -> Element {
    // the load operation itself is done in a future, so we share the result through this state, holding:
    // - the file name (String)
    // - the file parent path (String)
    // - the result of the load operation (either a Mdf or a serde error)
    let mut open_status: Signal<Option<(String, String, Result<mdf::Mdf, String>)>> = use_signal(|| None);

    // read back the result of the future, if any
    match open_status() {
        // load operation successful. we change the model in the application state and register the change
        // for undo operation
        Some((file_name, file_folder, Ok(model))) => {
            app_data.with_mut(|data| {
                data.data.model = std::sync::Arc::new(model);
                data.data.current_file_name = Some(file_name);
                data.data.current_path = file_folder;
                data.page_type = crate::page::PageType::Project;
                data.register_undo("load file");
            });
            // clear the open status state so that we don't rerun this
            open_status.set(None);
        }
        
        // load error
        Some((_, _, Err(message))) => {
            app_data.with_mut(|data| {
                data.error_message = Some(format!("Error while loading file: {}", message));
            });
            // clear the open status state so that we don't rerun this
            open_status.set(None);
        }

        // future not running, or not completed yet
        None => (),
    }

    let current_path = app_data.read().data.current_path.clone();

    // spawn a future when the open menu item is activated
    let open_file = move || {
        spawn({
            let mut open_status = open_status.to_owned();
            let current_path = current_path.to_owned();

            async move {
                // use a file chooser to get the file to load
                let file = AsyncFileDialog::new()
                    .add_filter("hdl wizard", &["regwiz", "json"])
                    .add_filter("any", &["*"])
                    .set_directory(&current_path)
                    .pick_file()
                    .await;

                if let Some(file) = file {
                    let (file_name, file_folder) = file_name(&file);

                    // load the file
                    open_status.set(Some((file_name, file_folder, serde_json::from_slice::<mdf::Mdf>(&file.read().await).map_err(|e| e.to_string()))));
                }
            }
        });
    };

    rsx! {
        gui_blocks::MenuEntry {
            key_action : key_action,
            binding : KeyAction::OpenFile,
            action : move |_| open_file(),
            icon: "fa-folder-open",
            label : "Open...",
            key_name: "O",
            key_modifiers : Modifiers::CONTROL
        }
    }
}

/// Save menu item
#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn Save(app_data: Signal<HdlWizardApp>, key_action : Signal<Option<KeyAction>>) -> Element {
    // the save operation itself is done in a future, so we share the result through this state, holding
    // just the result. Either an OK or an error message as a string
    let mut save_status: Signal<Option<Result<(), String>>> = use_signal(|| None);

    // read back the result of the future, if any
    match save_status() {
        // save operation completed. send a notification
        Some(Ok(_)) => {
            let file_name = app_data.read().data.current_file_name.clone().unwrap_or_default();
            app_data.with_mut(|data| {
                data.notification = Some(format!("file saved as {}", file_name));
            });
            // clear the open status state so that we don't rerun this
            save_status.set(None);
        }

        // error while saving. Display the error message
        Some(Err(message)) => {
            app_data.with_mut(|data| {
                data.error_message = Some(message);
            });
            // clear the open status state so that we don't rerun this
            save_status.set(None);
        }

        // future not running, or not completed yet
        None => (),
    }

    let app_data = app_data.read();
    let model_to_save = app_data.data.model.clone();
    if let Some(current_file_name) = app_data.data.current_file_name.clone() {

        // spawn a future when the save menu item is selected
        let save_file = move || {
            spawn({
                let mut save_status = save_status.to_owned();
                let model_to_save = model_to_save.to_owned();
                let current_file_name = current_file_name.to_owned();
    
                async move {
                    // save file and record result
                    match std::fs::File::create(Path::new(&current_file_name)) {
                        Ok(file) => {
                            save_status.set(Some(
                                serde_json::to_writer_pretty(
                                    std::io::BufWriter::new(file),
                                    &model_to_save,
                                )
                                .map_err(|e| format!("Error while writing file: {}", e)),
                            ));
                        }
                        Err(errormsg) => {
                            save_status.set(Some(Err(format!(
                                "Error while creating file: {}",
                                errormsg
                            ))));
                        }
                    }
                }
            });
        };
    
        // render the save menu item
        rsx! {
            gui_blocks::MenuEntry {
                key_action : key_action,
                binding : KeyAction::SaveFile,
                action : move |_| save_file(),
                icon: "fa-file-export",
                label : "Save",
                key_name: "S",
                key_modifiers : Modifiers::CONTROL
            }
        }
    } else {
        // we don't have a file name, so we don't even need to display the Save menu item
        None
    }
}

/// No save function in web application, only Save As
#[cfg(target_arch = "wasm32")]
#[allow(unused)]
#[component]
pub fn Save(app_data: Signal<HdlWizardApp>, key_action : Signal<Option<KeyAction>>) -> Element {
    None
}

/// SaveAs menu item, desktop version
#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn SaveAs(app_data: Signal<HdlWizardApp>, key_action : Signal<Option<KeyAction>>) -> Element {

    // the save as operation itself is done in a future, so we share the result through this state, holding
    // a result with the file name or an error message as a string
    let mut save_status: Signal<Option<Result<String, String>>> = use_signal(|| None);

    // read back the result from the future, if any
    match save_status() {
        // save as operation completed. send a notification and remember the file name
        Some(Ok(file_name)) => {
            app_data.with_mut(|data| {
                data.notification = Some("file saved".to_owned());
                if let Some(file_folder) = Path::new(&file_name).parent() {
                    data.data.current_path = file_folder.to_str().unwrap_or_default().to_owned();
                }
                data.data.current_file_name = Some(file_name);
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

    let app_data = app_data.read();
    let model_to_save = app_data.data.model.clone();
    let current_path = app_data.data.current_path.clone();


    // spawn a future when the save menu item is selected
    let save_file = move || {
        spawn({
            let mut save_status = save_status.to_owned();
            let model_to_save = model_to_save.to_owned();
            let current_path = current_path.to_owned();

            async move {
                // open file dialog to choose file name
                let file = AsyncFileDialog::new()
                    .add_filter("hdl wizard", &["regwiz", "json"])
                    .add_filter("any", &["*"])
                    .set_directory(&current_path)
                    .save_file()
                    .await;

                // save operation itself
                if let Some(file) = file {
                    let file_path = file.path();
                    match std::fs::File::create(file_path) {
                        Ok(file) => {
                            save_status.set(Some(
                                serde_json::to_writer_pretty(
                                    std::io::BufWriter::new(file),
                                    &model_to_save,
                                )
                                .map_err(|e| format!("Error while writing file: {}", e))
                                .map(|_| file_path.to_str().unwrap_or_default().to_owned()),
                            ));
                        }
                        Err(errormsg) => {
                            save_status.set(Some(Err(format!(
                                "Error while creating file: {}",
                                errormsg
                            ))));
                        }
                    }
                }
            }
        });
    };

    // render the Save As menu item
    rsx! {
        gui_blocks::MenuEntry {
            key_action : key_action,
            binding : KeyAction::SaveFileAs,
            action : move |_| save_file(),
            icon: "fa-file-export",
            label : "Save as...",
            key_name: "S",
            key_modifiers : Modifiers::CONTROL | Modifiers::SHIFT
        }
    }
}

/// SaveAs menu item, web version
#[cfg(target_arch = "wasm32")]
#[component]
pub fn SaveAs(app_data: Signal<HdlWizardApp>, key_action : Signal<Option<KeyAction>>) -> Element {

    // function that performs the actual save, as an uri embedded in the html. It will be "displayed" on the next round
    let mut save_file = move || {
        let file_serialize = serde_json::to_string_pretty(&app_data.read().data.model);
        match file_serialize {
            Ok(file_text) => {
                app_data.with_mut(|data| {
                    data.web_file_save = Some(format!(
                        "data:text/plain;charset=utf-8,{}",
                        js_sys::encode_uri_component(&file_text)
                    ));
                });
            }
            Err(message) => {
                app_data.with_mut(|data| {
                    data.error_message = Some(format!("Error generating save file: {}", message));
                });
            }
        }
    };

    // render the Save As menu item
    rsx! {
        gui_blocks::MenuEntry {
            key_action : key_action,
            binding : KeyAction::SaveFile,
            action : move |_| save_file(),
            icon: "fa-file-export",
            label : "Save as...",
            key_name: "S",
            key_modifiers : Modifiers::CONTROL
        }
    }
}
