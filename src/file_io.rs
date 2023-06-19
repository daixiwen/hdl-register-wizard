//! GUI navigation: both the menu bar and the left sidebar
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;
#[cfg(not(target_arch = "wasm32"))]
use std::path::Path;

#[cfg(not(target_arch = "wasm32"))]
fn file_name(handle : &rfd::FileHandle) -> (String, String) {
    (handle.path().to_str().unwrap_or_default().to_owned(), handle.path().parent().unwrap_or(Path::new("/")).to_str().unwrap_or_default().to_owned())
}

#[cfg(target_arch = "wasm32")]
fn file_name(handle : &rfd::FileHandle) -> (String, String) {
    (handle.file_name(), Default::default())
}

#[inline_props]
pub fn Open<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let open_status: &UseState<Option<(String, String, Result<mdf::Mdf, serde_json::Error>)>> =
        use_state(cx, || None);

    match open_status.get() {
        Some((file_name, file_folder, Ok(model))) => {
            app_data.with_mut(|data| {
                data.data.model = model.to_owned();
                data.data.current_file_name = Some(file_name.to_owned());
                data.data.current_path = file_folder.to_owned();
                data.register_undo("load file");
            });
            open_status.set(None);
        }
        Some((_, _, Err(message))) => {
            app_data.with_mut(|data| {
                data.error_message = Some(format!("Error while loading file: {}", message));
            });
            open_status.set(None);
        }
        None => (),
    }

    let current_path = app_data.read().data.current_path.clone();

    let open_file = move |_| {
        cx.spawn({
            let open_status = open_status.to_owned();
            let current_path = current_path.to_owned();

            async move {
                let file = AsyncFileDialog::new()
                    .add_filter("hdl wizard", &["regwiz", "json"])
                    .add_filter("any", &["*"])
                    .set_directory(&current_path)
                    .pick_file()
                    .await;

                if let Some(file) = file {
                    let (file_name, file_folder) = file_name(&file);

                    open_status.set(Some((file_name, file_folder, serde_json::from_slice::<mdf::Mdf>(&file.read().await))));
                }
            }
        })
    };

    cx.render(rsx! {
        a { class: "navbar-item", onclick: open_file,
            i { class: "fa-solid fa-folder-open mr-1" }
            "Open..."
        }
    })
}

#[cfg(not(target_arch = "wasm32"))]
#[inline_props]
pub fn Save<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let save_status: &UseState<Option<Result<(), String>>> = use_state(cx, || None);

    match save_status.get() {
        Some(Ok(_)) => {
            let file_name = app_data.read().data.current_file_name.clone().unwrap_or_default();
            app_data.with_mut(|data| {
                data.notification = Some(format!("file saved as {}", file_name));
            });
            save_status.set(None);
        }
        Some(Err(message)) => {
            app_data.with_mut(|data| {
                data.error_message = Some(message.clone());
            });
            save_status.set(None);
        }
        None => (),
    }

    let model_to_save = app_data.read().data.model.clone();
    if let Some(current_file_name) = app_data.read().data.current_file_name.clone() {


        let save_file = move |_| {
            cx.spawn({
                let save_status = save_status.to_owned();
                let model_to_save = model_to_save.to_owned();
                let current_file_name = current_file_name.to_owned();
    
                async move {
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
            })
        };
    
        cx.render(rsx! {
            a { class: "navbar-item", onclick: save_file,
                i { class: "fa-solid fa-file-export mr-1" }
                "Save"
            }
        })    
    } else {
        cx.render(rsx! { "" })
    }
}

#[cfg(target_arch = "wasm32")]
#[allow(unused)]
#[inline_props]
pub fn Save<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    cx.render(rsx! { "" })
}

#[cfg(not(target_arch = "wasm32"))]
#[inline_props]
pub fn SaveAs<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {

    let save_status: &UseState<Option<Result<String, String>>> = use_state(cx, || None);

    match save_status.get() {
        Some(Ok(file_name)) => {
            app_data.with_mut(|data| {
                data.notification = Some("file saved".to_owned());
                data.data.current_file_name = Some(file_name.to_owned());
                if let Some(file_folder) = Path::new(file_name).parent() {
                    data.data.current_path = file_folder.to_str().unwrap_or_default().to_owned();
                }
            });
            save_status.set(None);
        }
        Some(Err(message)) => {
            app_data.with_mut(|data| {
                data.error_message = Some(message.clone());
            });
            save_status.set(None);
        }
        None => (),
    }

    let model_to_save = app_data.read().data.model.clone();
    let current_path = app_data.read().data.current_path.clone();


    let save_file = move |_| {
        cx.spawn({
            let save_status = save_status.to_owned();
            let model_to_save = model_to_save.to_owned();
            let current_path = current_path.to_owned();

            async move {
                let file = AsyncFileDialog::new()
                    .add_filter("hdl wizard", &["regwiz", "json"])
                    .add_filter("any", &["*"])
                    .set_directory(&current_path)
                    .save_file()
                    .await;

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
        })
    };

    cx.render(rsx! {
        a { class: "navbar-item", onclick: save_file,
            i { class: "fa-solid fa-file-export mr-1" }
            "Save as..."
        }
    })
}

#[cfg(target_arch = "wasm32")]
#[inline_props]
pub fn SaveAs<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let save_file = move |_| {
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

    cx.render(rsx! {
        a { class: "navbar-item", onclick: save_file,
            i { class: "fa-solid fa-file-export mr-1" }
            "Save as..."
        }
    })
}
