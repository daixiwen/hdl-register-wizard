//! documentation preview page
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::HdlWizardApp;
use std::sync::Arc;
use std::error::Error;
use crate::generate::genmodel;
use crate::generate::documentation;
use crate::file_formats::mdf;
use crate::settings;

// generate the documentation as a string from the given model
fn generate_html(model : Arc<mdf::Mdf>, settings: &settings::Settings) -> Result<String, Box<dyn Error>> {
    let model = genmodel::GenModel::from_model(model.as_ref(), settings)?;
    documentation::generate_doc(&model)
}

// Whole page for the project top level
#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {

    // the preview generation itself is done in a future, so we share the result through this state, holding
    // a result with the generated html or an error message as a string
    let preview_status: &UseState<Option<Result<String, String>>> = use_state(cx, || None);
    let model_to_save = app_data.read().data.model.clone();
    let settings = app_data.read().data.settings.clone();

    // check if we should send a new request to generate the preview
    if app_data.read().generate_preview {
        app_data.with_mut(|data| data.generate_preview = false);
        cx.spawn({
            let preview_status = preview_status.to_owned();
            preview_status.set(None);

            async move {
                preview_status.set(Some(generate_html(model_to_save, &settings).map_err(|err| err.to_string())));
            }
        });
    }
    cx.render(rsx! {
        div { class: "container",
            h1 { class: "title page-title", "Documentation preview" }
            // see if we have any result
            match preview_status.get() {
                None => {
                    rsx! {
                        div { "generating documentation...."
                        }        
                }},
                Some(Ok(document)) => {
                    rsx! {
                        div { class: "content",
                            article {
                                dangerous_inner_html : "{document}"
                            }
                        }
                    }
                },
                Some(Err(message)) => {
                    app_data.with_mut(|data| data.error_message = Some(format!("error while generating preview: {}", message)));
                    preview_status.set(None);
                    rsx! {
                        div {
                            "error message"
                        }
                    }
                }
            }
        }
    })
}
