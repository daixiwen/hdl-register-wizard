//! main page for the project
#![allow(non_snake_case)]

use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use crate::gui_blocks;
use crate::gui_blocks::callback;
use crate::page::PageType;
use dioxus::prelude::*;

/// builds a line in the table with all the interfaces
#[inline_props]
fn TableLine<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>,
    interface_number: usize,
    interface_name: String,
    interface_type: mdf::InterfaceType,
) -> Element<'a> {
    let num_of_interfaces = app_data.read().data.model.interfaces.len();
    let up_disabled = *interface_number == 0;
    let down_disabled = *interface_number == num_of_interfaces - 1;

    let display_name = if interface_name.is_empty() {
        "(empty)"
    } else {
        interface_name
    };

    cx.render(rsx! {
        tr {
            td {
                a { onclick: move |_| {
                        app_data.with_mut(|data| data.page_type = PageType::Interface(*interface_number))
                    },
                    "{display_name}"
                }
            }
            td { "{interface_type.to_string()}" }
            td {
                div { class: "buttons are-small ext-buttons-in-table",
                    button {
                        class: "button is-link",
                        disabled: "{up_disabled}",
                        onclick: move |_| {
                            if !up_disabled {
                                app_data
                                    .with_mut(|data| {
                                        data.get_mut_model()
                                            .interfaces
                                            .swap(*interface_number - 1, *interface_number);
                                        data.register_undo("move interface up")
                                    })
                            }
                        },
                        span { class: "icon is_small", i { class: "fa-solid fa-caret-up" } }
                    }
                    button {
                        class: "button is-link",
                        disabled: "{down_disabled}",
                        onclick: move |_| {
                            if !down_disabled {
                                app_data
                                    .with_mut(|data| {
                                        data.get_mut_model()
                                            .interfaces
                                            .swap(*interface_number, *interface_number + 1);
                                        data.register_undo("move interface down")
                                    })
                            }
                        },
                        span { class: "icon is_small", i { class: "fa-solid fa-caret-down" } }
                    }
                    button {
                        class: "button is-link",
                        onclick: move |_| {
                            app_data.with_mut(|data| data.page_type = PageType::Interface(*interface_number))
                        },
                        span { class: "icon is_small", i { class: "fa-solid fa-pen" } }
                    }
                    button {
                        class: "button is-danger",
                        onclick: move |_| {
                            app_data
                                .with_mut(|data| {
                                    data.get_mut_model().interfaces.remove(*interface_number);
                                    data.register_undo("remove interface")
                                })
                        },
                        span { class: "icon is_small", i { class: "fa-solid fa-trash" } }
                    }
                }
            }
        }
    })
}

/// Whole page for the project top level
#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let project_name = app_data.read().data.model.name.clone();

    // extract a list of interfaces and types
    let int_list = app_data
        .read()
        .data
        .model
        .interfaces
        .iter()
        .enumerate()
        .map(|(n, int)| (n, int.name.clone(), int.interface_type.clone()))
        .collect::<Vec<_>>();

    // now build some items from that list
    let int_items = int_list.iter().map(|(n, int_name, int_type)| {
        rsx!(
            TableLine {
                app_data: app_data,
                interface_number: *n,
                interface_name: int_name.clone(),
                interface_type: *int_type,
                key: "{int_name}{n}"
            }
        )
    });

    cx.render(rsx! {
        div { class: "container",
            h1 { class: "title page-title", "HDL Register Wizard Project" }
            div { class: "m-4",
                gui_blocks::TextGeneric {
                    app_data: app_data,
                    update_model: callback(|model, value: &String| model.name = value.clone()),
                    gui_label: "Name",
                    undo_label: "change project name",
                    value: project_name
                }
            }
            h2 { class: "subtitle page-title", "Interfaces" }
            table { class: "table is-striped is-hoverable is-fullwidth",
                thead {
                    tr {
                        th { "Name" }
                        th { "Type" }
                        th {}
                    }
                }
                tbody { int_items }
            }
            div { class: "buttons",
                button {
                    class: "button is-primary",
                    onclick: move |_| {
                        app_data
                            .with_mut(|app| {
                                app.get_mut_model().interfaces.push(mdf::Interface::new());
                                app.page_type = PageType::Interface(app.data.model.interfaces.len() - 1);
                                app.register_undo("create interface")
                            })
                    },
                    "New interface"
                }
            }
        }
    })
}
