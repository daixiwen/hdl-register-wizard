//! page to edit names settings
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::HdlWizardApp;
use crate::generate::user_strings;
use core::slice::Iter;

// table line with one string
fn TableLine(mut app_data: Signal<HdlWizardApp>, template: &user_strings::UserStringSpec, pattern: &str) -> Element {
    let value = if let Some(setting_value) = app_data.read().data.settings.user_templates.get(template.template_name) {
        setting_value.clone()
    } else { "??".to_owned()};

    let default_value = template.default_value.to_owned();
    let template_name = template.template_name.to_owned();

    rsx! {
        tr {
            td {
                div { class: "field-label is-normal", label { class: "label", "{template.label}" } }
            }
            td {
                div { class: "field-body",
                    div { class: "field",
                        div { class: "control",
                            input {
                                class: "input",
                                r#type: "text",
                                placeholder: "{template.default_value}",
                                pattern: "{pattern}",
                                size: "40",
//                                pattern: "{validate_pattern}",
                                onchange: move |evt| {
                                    app_data.with_mut(|appdata| { 
                                        appdata.data.settings.user_templates.insert(
                                            template_name.clone(), 
                                            if evt.value().is_empty() {
                                                default_value.clone()
                                            } else {
                                                evt.value()
                                            }
                                        ); 
                                    })
                                },
                                value: "{value}"
                            }
                        }
                    }
                }
            }
            td {
                "{template.description}"
            }
        }
    }
}

fn Table(app_data: Signal<HdlWizardApp>, templates: Iter<'_, user_strings::UserStringSpec>, pattern: &str) -> Element {
    rsx! {
        table {
            class:"table is-striped is-hoverable is-fullwidth",
            thead {
                tr {
                    th { "Name" }
                    th { "value" }
                    th { "Description" }
                }
            }
            tbody {
                {templates.map(|template| TableLine(app_data, template, pattern))}
            }
        }
    }
}

// main page
#[component]
pub fn Content(app_data: Signal<HdlWizardApp>) -> Element {
    // this string needs to be made here or else the rsx macro will try to format it, and escaping { and } seems to work
    // in different ways between the web and desktop platforms
    let names_description = r#"Each string must have a * for digits to prevent duplicates, and can also use "{{ project }}", "{{ interface }}", "{{ register }}" and "{{ field }}", when applicable. The resulting string must be a valid VHDL identifier."#;
    let description_description = r#"These strings are used in comments or the documentation. They can contain "{{ full_name }}", which will be replaced by the name of the object described."#;

    rsx! {
        h1 { class: "title page-title", "Settings: Strings" },
        p { "Configure here how the different strings in the generated code and documentation are generated."}
        h1 { class: "subtitle page-title", "Names" },
        p { {names_description } }

        { Table(app_data, user_strings::USER_NAMES_SPECS.iter(),r"^(\{\{ *(project|interface|register|field) *\}\}|[A-Za-z])(\{\{ *(project|interface|register|field) *\}\}|[0-9A-Za-z_])*\*(\{\{ *(project|interface|register|field) *\}\}|[0-9A-Za-z_])*$") }

        h1 { class: "subtitle page-title", "Descriptions" },
        p { {description_description} }

        { Table(app_data, user_strings::USER_COMMENTS_SPECS.iter(), r"^(\{\{ *full_name *\}\}|[^\{\}])*$") }
    }
}
