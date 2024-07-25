//! page to edit an interface
#![allow(non_snake_case)]

use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use crate::gui_blocks;
use crate::gui_blocks::callback_interface;
use crate::page::PageType;
use crate::utils;
use dioxus::prelude::*;

/// builds a single line in the table with all the registers
#[component]
fn TableLine(
    app_data: Signal<HdlWizardApp>,
    register_number: usize,
    register_name: String,
    #[props(!optional)] register_type: Option<utils::SignalType>,
    register_address: mdf::Address,
) -> Element {
    let page_type = app_data.read().page_type.clone();
    if let PageType::Interface(interface_number) = page_type {
        // utility variables used when generating the html
        let num_of_registers = app_data.read().data.model.interfaces[interface_number]
            .registers
            .len();
        let up_disabled = register_number == 0;
        let down_disabled = register_number == num_of_registers - 1;

        let display_name = if register_name.is_empty() {
            "(empty)".to_owned()
        } else {
            register_name
        };
        let display_type = match register_type {
            Some(signal_type) => signal_type.to_string(),
            None => "bitfield".to_owned(),
        };

        // render the line
        rsx! {
            tr {
                td {
                    a { onclick: move |_| {
                            app_data
                                .with_mut(|data| {
                                    data
                                        .page_type = PageType::Register(
                                        interface_number,
                                        register_number,
                                        None,
                                    );
                                })
                        },
                        "{display_name}"
                    }
                }
                td { "{register_address.nice_str()}" }
                td { "{display_type}" }
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
                                                .interfaces[interface_number]
                                                .registers
                                                .swap(register_number - 1, register_number);
                                            data.register_undo("move register up")
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
                                                .interfaces[interface_number]
                                                .registers
                                                .swap(register_number, register_number + 1);
                                            data.register_undo("move register down")
                                        })
                                }
                            },
                            span { class: "icon is_small", i { class: "fa-solid fa-caret-down" } }
                        }
                        button {
                            class: "button is-link",
                            onclick: move |_| {
                                app_data
                                    .with_mut(|data| {
                                        data
                                            .page_type = PageType::Register(
                                            interface_number,
                                            register_number,
                                            None,
                                        );
                                    })
                            },
                            span { class: "icon is_small", i { class: "fa-solid fa-pen" } }
                        }
                        button {
                            class: "button is-danger",
                            onclick: move |_| {
                                app_data
                                    .with_mut(|data| {
                                        data.get_mut_model()
                                            .interfaces[interface_number]
                                            .registers
                                            .remove(register_number);
                                        data.register_undo("remove register")
                                    })
                            },
                            span { class: "icon is_small", i { class: "fa-solid fa-trash" } }
                        }
                    }
                }
            }
        }
    } else {
        rsx!{ p { "error.... not in a interface page" } }
    }
}

/// Whole page for an interface
#[component]
pub fn Content(app_data: Signal<HdlWizardApp>, interface_num: usize) -> Element {
    let data = app_data.read();
    let get_interface = data.data.model.interfaces.get(interface_num);
    if let Some(interface) = get_interface {
        // extract a list of registers, addresses and types
        let int_list = interface
            .registers
            .iter()
            .enumerate()
            .map(|(n, reg)| (n, reg.name.clone(), reg.address.clone(), reg.signal.clone()))
            .collect::<Vec<_>>();

        // now build some items from that list
        let int_items = int_list.iter().map(|(n, int_name, int_address, int_type)| {
            rsx!(
                TableLine {
                    app_data: app_data,
                    register_number: *n,
                    register_name: int_name.clone(),
                    register_type: *int_type,
                    register_address: int_address.clone(),
                    key: "{int_name}{n}"
                }
            )
        });

        let interface_width = interface.get_data_width();

        // render the page
        rsx! {
            div {
                a {
                    class: "button is-link is-outlined ext-return-button",
                    onclick: move |_| {
                        app_data
                            .with_mut(|app| {
                                app.page_type = PageType::Project;
                            })
                    },
                    span { class: "icon ", i { class: "fa-solid fa-caret-left" } }
                }
                h1 { class: "title page-title", "Interface" }
            }
            div { class: "m-4",
                gui_blocks::TextGeneric {
                    app_data: app_data,
                    update_int: callback_interface(app_data, |interface, value| interface.name = value),
                    gui_label: "Name",
                    undo_label: "change interface name",
                    value: interface.name.clone()
                }
                gui_blocks::EnumWidget {
                    app_data: app_data,
                    update_int: callback_interface(app_data, |interface, value| interface.interface_type = value),
                    gui_label: "Type",
                    undo_label: "change interface type",
                    value: interface.interface_type
                }
                gui_blocks::TextArea {
                    app_data: app_data,
                    update_int: callback_interface(app_data, |interface, value| interface.description = value),
                    gui_label: "Description",
                    undo_label: "change interface description",
                    value: interface.description.clone()
                }
                gui_blocks::AutoManuText {
                    app_data: app_data,
                    update_int: callback_interface(app_data, |interface, value| interface.address_width = value),
                    gui_label: "Address width",
                    undo_label: "change interface address width",
                    value: interface.address_width,
                    default: 32
                }
                gui_blocks::AutoManuText {
                    app_data: app_data,
                    update_int: callback_interface(app_data, |interface, value| interface.data_width = value),
                    gui_label: "Data width",
                    undo_label: "change interface data width",
                    value: interface.data_width,
                    placeholder: match interface_width {
                        None => "Data witdh".to_owned(),
                        Some(width) => width.to_string(),
                    },
                    default: interface_width.unwrap_or(32)
                }
            }
            h2 { class: "subtitle page-title", "Registers" }
            table { class: "table is-striped is-hoverable is-fullwidth",
                thead {
                    tr {
                        th { "Name" }
                        th { "Address" }
                        th { "Type" }
                        th {}
                    }
                }
                tbody { {int_items} }
            }
            div { class: "buttons",
                button {
                    class: "button is-primary",
                    onclick: move |_| {
                        app_data
                            .with_mut(|app| {
                                app.get_mut_model()
                                    .interfaces[interface_num]
                                    .registers
                                    .push(mdf::Register::new());
                                app
                                    .page_type = PageType::Register(
                                    interface_num,
                                    app.data.model.interfaces[interface_num].registers.len() - 1,
                                    None,
                                );
                                app.register_undo("create register")
                            })
                    },
                    "New register"
                }
                button {
                    class: "button is-primary",
                    onclick: move |_| {
                        app_data
                            .with_mut(|app| {
                                let result = app.get_mut_model().interfaces[interface_num].assign_addresses();
                                app.test_result(result);
                                app.register_undo("assign addresses")
                            })
                    },
                    "Assign addresses"
                }
                button {
                    class: "button is-danger",
                    onclick: move |_| {
                        app_data
                            .with_mut(|app| {
                                let result = app.get_mut_model().interfaces[interface_num].deassign_addresses();
                                app.test_result(result);
                                app.register_undo("unassign addresses")
                            })
                    },
                    "Unassign addresses"
                }
            }
        }
    } else {
        rsx! { p { "Wrong interface" } }
    }
}
