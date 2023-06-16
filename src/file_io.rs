//! GUI navigation: both the menu bar and the left sidebar
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use dioxus::prelude::*;
use rfd::AsyncFileDialog;

#[inline_props]
pub fn Open<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let open_status: &UseState<Option<Result<mdf::Mdf, serde_json::Error>>> =
        use_state(cx, || None);

    match open_status.get() {
        Some(Ok(model)) => {
            app_data.with_mut(|data| {
                data.data.model = model.to_owned();
                data.register_undo("load file");
            });
            open_status.set(None);
        }
        Some(Err(message)) => {
            app_data.with_mut(|data| {
                data.error_message = Some(format!("Error while loading file: {}", message));
            });
            open_status.set(None);
        }
        None => (),
    }

    let open_file = move |_| {
        cx.spawn({
            let open_status = open_status.to_owned();

            async move {
                let file = AsyncFileDialog::new()
                    .add_filter("hdl wizard", &["regwiz", "json"])
                    .add_filter("any", &["*"])
                    .set_directory("/")
                    .pick_file()
                    .await;

                if let Some(file) = file {
                    open_status.set(Some(serde_json::from_slice::<mdf::Mdf>(&file.read().await)));
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
pub fn SaveAs<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let save_status: &UseState<Option<Result<(), String>>> = use_state(cx, || None);

    match save_status.get() {
        Some(Ok(_)) => {
            app_data.with_mut(|data| {
                data.notification = Some("file saved".to_owned());
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

    let save_file = move |_| {
        cx.spawn({
            let save_status = save_status.to_owned();
            let model_to_save = model_to_save.to_owned();

            async move {
                let file = AsyncFileDialog::new()
                    .add_filter("hdl wizard", &["regwiz", "json"])
                    .add_filter("any", &["*"])
                    .set_directory("/")
                    .save_file()
                    .await;

                if let Some(file) = file {
                    match std::fs::File::create(file.path()) {
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
