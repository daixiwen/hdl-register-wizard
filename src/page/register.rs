//! page to edit a register
#![allow(non_snake_case)]

use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use crate::gui_blocks;
use crate::gui_blocks::callback;
use crate::gui_types::Validable;
use crate::page::PageType;
use crate::utils;
use dioxus::prelude::*;
use std::cell::RefCell;
use std::default::Default;
use std::str::FromStr;

//fn absdiff(a: u32, b: u32) -> u32 {
//    if a > b {
//        a - b
//    } else {
//        b - a
//    }
//}

// default values for some fields when changing the signal type
fn default_fields(interface_width: u32, register: &mut mdf::Register) {
    let default_width = match register.signal {
        Some(utils::SignalType::Boolean) | Some(utils::SignalType::StdLogic) => 1,
        Some(utils::SignalType::Signed)
        | Some(utils::SignalType::Unsigned)
        | Some(utils::SignalType::StdLogicVector) => interface_width,
        None => 0,
    };

    if register.signal.is_none() {
        // signal is a bitfield. There are a bunch of parameters that aren't available
        register.width = None;
        register.access = None;
        register.reset = None;
        register.core_signal_properties = mdf::CoreSignalProperties::default();
    } else {
        if register.width.is_none() {
            register.width = Some(default_width);
        }
        if register.access.is_none() {
            register.access = Some(mdf::AccessType::RW);
        }
        if register.reset.is_none() {
            register.reset = Some(utils::VectorValue::new());
        }
    }
}

// widget for the address stride
#[derive(Props)]
struct GuiAddressStrideProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    #[props(!optional)]
    value: Option<mdf::AddressStride>,
    update_reg:
        Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &Option<mdf::AddressStride>) -> () + 'a>>>,
}

fn AddressStride<'a>(cx: Scope<'a, GuiAddressStrideProps<'a>>) -> Element<'a> {
    let validate_pattern = utils::VectorValue::validate_pattern();
    let value = &cx.props.value;
    let is_stride = value.is_some();
    let has_increment = match value {
        Some(addrstr) => addrstr.increment.is_some(),
        _ => false,
    };
    let count = match value {
        Some(addrstr) => addrstr.count,
        _ => Default::default(),
    };
    let count_string = if is_stride {
        count.to_string()
    } else {
        Default::default()
    };
    let increment_field = match value {
        Some(addrstr) => addrstr.increment,
        _ => Default::default(),
    };
    let increment = increment_field.unwrap_or_default();
    let increment_string = if has_increment {
        increment.to_string()
    } else {
        Default::default()
    };
    let label_class = if is_stride { "" } else { "has-text-grey-light" };

    cx.render(rsx!{
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", " " } }
            div { class: "field-body",
                div { class: "field is-grouped is-align-items-center",
                    div { class: "control",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                onclick: move |_| {
                                    let new_value: Option<mdf::AddressStride> = if is_stride {
                                        None
                                    } else {
                                        Some(mdf::AddressStride {
                                            count: utils::VectorValue {
                                                value: 1,
                                                radix: utils::RadixType::Decimal,
                                            },
                                            increment: None,
                                        })
                                    };
                                    gui_blocks::apply_function(
                                        &cx.props.app_data,
                                        new_value,
                                        "change address stride status",
                                        &None,
                                        &None,
                                        &cx.props.update_reg,
                                        &None,
                                    );
                                },
                                checked: "{is_stride}"
                            }
                            " Stride: "
                        }
                    }
                    div { class: "control", label { class: "{label_class}", "Count: " } }
                    div { class: "control",
                        input {
                            class: "input ext-vector-field",
                            r#type: "text",
                            placeholder: "count",
                            pattern: "{validate_pattern}",
                            onchange: move |evt| {
                                if let Ok(new_value) = utils::VectorValue::from_str(&evt.value) {
                                    let new_stride = mdf::AddressStride {
                                        count: new_value,
                                        increment: increment_field,
                                    };
                                    gui_blocks::apply_function(
                                        &cx.props.app_data,
                                        Some(new_stride),
                                        "change stride count",
                                        &None,
                                        &None,
                                        &cx.props.update_reg,
                                        &None,
                                    );
                                }
                            },
                            value: "{count_string}",
                            disabled: "{!is_stride}"
                        }
                    }
                    div { class: "control",
                        label { class: "checkbox", class: "{label_class}",
                            input {
                                r#type: "checkbox",
                                onclick: move |_| {
                                    if is_stride {
                                        let new_value: Option<mdf::AddressStride> = if has_increment {
                                            Some(mdf::AddressStride {
                                                count: count,
                                                increment: None,
                                            })
                                        } else {
                                            Some(mdf::AddressStride {
                                                count: count,
                                                increment: Some(Default::default()),
                                            })
                                        };
                                        gui_blocks::apply_function(
                                            &cx.props.app_data,
                                            new_value,
                                            "change address stride increment option",
                                            &None,
                                            &None,
                                            &cx.props.update_reg,
                                            &None,
                                        );
                                    }
                                },
                                checked: "{has_increment}",
                                disabled: "{!is_stride}"
                            }
                            " Increment: "
                        }
                    }
                    div { class: "control",
                        label {
                            input {
                                class: "input ext-vector-field",
                                r#type: "text",
                                placeholder: "auto",
                                pattern: "{validate_pattern}",
                                onchange: move |evt| {
                                    if let Ok(new_value) = utils::VectorValue::from_str(&evt.value) {
                                        let new_stride = mdf::AddressStride {
                                            count: count,
                                            increment: Some(new_value),
                                        };
                                        gui_blocks::apply_function(
                                            &cx.props.app_data,
                                            Some(new_stride),
                                            "change stride increment value",
                                            &None,
                                            &None,
                                            &cx.props.update_reg,
                                            &None,
                                        );
                                    }
                                },
                                value: "{increment_string}",
                                disabled: "{!has_increment}"
                            }
                        }
                    }
                }
            }
        }
    })
}

// widget for the core properties
#[derive(Props)]
struct GuiCoreProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    #[props(!optional)]
    value: mdf::CoreSignalProperties,
    is_register: bool,
}

fn CoreProperties<'a>(cx: Scope<'a, GuiCoreProps<'a>>) -> Element<'a> {
    let value = &cx.props.value;
    let use_read_enable = value.use_read_enable.unwrap_or(false);
    let use_write_enable = value.use_write_enable.unwrap_or(false);

    let read_update_function_reg: Option<
        RefCell<Box<dyn FnMut(&mut mdf::Register, &bool) -> () + 'a>>,
    > = if cx.props.is_register {
        Some(callback(|register, value| {
            register.core_signal_properties.use_read_enable = Some(*value)
        }))
    } else {
        None
    };
    let write_update_function_reg: Option<
        RefCell<Box<dyn FnMut(&mut mdf::Register, &bool) -> () + 'a>>,
    > = if cx.props.is_register {
        Some(callback(|register, value| {
            register.core_signal_properties.use_write_enable = Some(*value)
        }))
    } else {
        None
    };
    let read_update_function_field: Option<
        RefCell<Box<dyn FnMut(&mut mdf::Field, &bool) -> () + 'a>>,
    > = if !cx.props.is_register {
        Some(callback(|field, value| {
            field.core_signal_properties.use_read_enable = Some(*value)
        }))
    } else {
        None
    };
    let write_update_function_field: Option<
        RefCell<Box<dyn FnMut(&mut mdf::Field, &bool) -> () + 'a>>,
    > = if !cx.props.is_register {
        Some(callback(|field, value| {
            field.core_signal_properties.use_write_enable = Some(*value)
        }))
    } else {
        None
    };

    cx.render(rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "Core Properties" } }
            div { class: "field-body",
                div { class: "field is-grouped is-align-items-center",
                    div { class: "control",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                onclick: move |_| {
                                    gui_blocks::apply_function(
                                        &cx.props.app_data,
                                        !use_read_enable,
                                        "change read enable core property",
                                        &None,
                                        &None,
                                        &read_update_function_reg,
                                        &read_update_function_field,
                                    )
                                },
                                checked: "{use_read_enable}"
                            }
                            " Use read enable "
                        }
                    }
                    div { class: "control",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                onclick: move |_| {
                                    gui_blocks::apply_function(
                                        &cx.props.app_data,
                                        !use_write_enable,
                                        "change write enable core property",
                                        &None,
                                        &None,
                                        &write_update_function_reg,
                                        &write_update_function_field,
                                    )
                                },
                                checked: "{use_write_enable}"
                            }
                            " Use write enable "
                        }
                    }
                }
            }
        }
    })
}

// builds a line in the table with all the fields
#[inline_props]
fn TableLine<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>,
    field_number: usize,
    field_name: String,
    field_position: mdf::FieldPosition,
    field_access: mdf::AccessType,
    field_type: utils::SignalType,
    is_selected: bool,
) -> Element<'a> {
    if let PageType::Register(interface_number, register_number, _) = app_data.read().page_type {
        let num_of_fields = app_data.read().data.model.interfaces[interface_number].registers
            [register_number]
            .fields
            .len();
        let up_disabled = *field_number == 0;
        let down_disabled = *field_number == num_of_fields - 1;

        let display_name = if field_name.is_empty() {
            "(empty)"
        } else {
            field_name
        };

        let tr_class = if *is_selected { "is-selected" } else { "" };

        cx.render(rsx! {
            tr { class: "{tr_class}",
                td {
                    a { onclick: move |_| {
                            app_data
                                .with_mut(|data| {
                                    data
                                        .page_type = PageType::Register(
                                        interface_number,
                                        register_number,
                                        Some(*field_number),
                                    );
                                })
                        },
                        "{display_name}"
                    }
                }
                td { "{field_position.to_string()}" }
                td { "{field_access.to_string()}" }
                td { "{field_type.to_string()}" }
                td {
                    div { class: "buttons are-small ext-buttons-in-table",
                        button {
                            class: "button is-link",
                            disabled: "{up_disabled}",
                            onclick: move |_| {
                                if !up_disabled {
                                    app_data
                                        .with_mut(|data| {
                                            data.data
                                                .model
                                                .interfaces[interface_number]
                                                .registers[register_number]
                                                .fields
                                                .swap(*field_number - 1, *field_number);
                                            data.register_undo("move field up")
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
                                            data.data
                                                .model
                                                .interfaces[interface_number]
                                                .registers[register_number]
                                                .fields
                                                .swap(*field_number, *field_number + 1);
                                            data.register_undo("move field down")
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
                                            Some(*field_number),
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
                                        data.data
                                            .model
                                            .interfaces[interface_number]
                                            .registers[register_number]
                                            .fields
                                            .remove(*field_number);
                                        data.register_undo("remove field")
                                    })
                            },
                            span { class: "icon is_small", i { class: "fa-solid fa-trash" } }
                        }
                    }
                }
            }
        })
    } else {
        cx.render(rsx!( p { "error.... not in a interface page" } ))
    }
}

// widget for the bitfield position
#[derive(Props)]
struct GuiBitFieldPositionProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    value: mdf::FieldPosition,
    update_field: Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &mdf::FieldPosition) -> () + 'a>>>,
}

fn FieldPosition<'a>(cx: Scope<'a, GuiBitFieldPositionProps<'a>>) -> Element<'a> {
    let validate_pattern = u32::validate_pattern();
    let value = &cx.props.value;
    let pos_high = match value {
        mdf::FieldPosition::Single(pos) => pos.to_string(),
        mdf::FieldPosition::Field(high, _) => high.to_string(),
    };
    let pos_low = match value {
        mdf::FieldPosition::Single(_) => Default::default(),
        mdf::FieldPosition::Field(_, low) => low.to_string(),
    };

    cx.render(rsx!{
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "Position" } }
            div { class: "field-body",
                div { class: "field is-grouped is-align-items-center",
                    div { class: "control",
                        input {
                            class: "input ext-vector-field",
                            r#type: "text",
                            placeholder: "MSB",
                            pattern: "{validate_pattern}",
                            onchange: move |evt| {
                                if let Ok(new_value) = u32::from_str(&evt.value) {
                                    let new_pos = match &value {
                                        mdf::FieldPosition::Single(_) => mdf::FieldPosition::Single(new_value),
                                        mdf::FieldPosition::Field(_, low) => {
                                            mdf::FieldPosition::Field(new_value, *low)
                                        }
                                    };
                                    gui_blocks::apply_function(
                                        &cx.props.app_data,
                                        new_pos,
                                        "change bitfield position msb",
                                        &None,
                                        &None,
                                        &None,
                                        &cx.props.update_field,
                                    );
                                }
                            },
                            value: "{pos_high}"
                        }
                    }
                    div { class: "control", label { " downto " } }
                    div { class: "control",
                        label {
                            input {
                                class: "input ext-vector-field",
                                r#type: "text",
                                placeholder: "single",
                                pattern: "{validate_pattern}",
                                onchange: move |evt| {
                                    if let Ok(new_value) = u32::from_str(&evt.value) {
                                        let new_pos = match &value {
                                            mdf::FieldPosition::Single(high) => {
                                                mdf::FieldPosition::Field(*high, new_value)
                                            }
                                            mdf::FieldPosition::Field(high, _) => {
                                                mdf::FieldPosition::Field(*high, new_value)
                                            }
                                        };
                                        gui_blocks::apply_function(
                                            &cx.props.app_data,
                                            new_pos,
                                            "change bitfield position lsb",
                                            &None,
                                            &None,
                                            &None,
                                            &cx.props.update_field,
                                        );
                                    } else {
                                        let new_pos = match &value {
                                            mdf::FieldPosition::Single(high) => mdf::FieldPosition::Single(*high),
                                            mdf::FieldPosition::Field(high, _) => mdf::FieldPosition::Single(*high),
                                        };
                                        gui_blocks::apply_function(
                                            &cx.props.app_data,
                                            new_pos,
                                            "change bitfield position lsb",
                                            &None,
                                            &None,
                                            &None,
                                            &cx.props.update_field,
                                        );
                                    }
                                },
                                value: "{pos_low}"
                            }
                        }
                    }
                }
            }
        }
    })
}

#[derive(Clone)]
enum FieldBitStatus {
    Unused,
    Used,
    Selected,
    Error,
}

fn update_status(
    previous: &FieldBitStatus,
    field_num: usize,
    selected_field: Option<usize>,
) -> FieldBitStatus {
    match previous {
        FieldBitStatus::Unused => {
            if let Some(select) = selected_field {
                if select == field_num {
                    FieldBitStatus::Selected
                } else {
                    FieldBitStatus::Used
                }
            } else {
                FieldBitStatus::Used
            }
        }

        _ => FieldBitStatus::Error,
    }
}

// page contents
#[derive(Props)]
pub struct ContentProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    interface_num: usize,
    register_num: usize,
    #[props(!optional)]
    field_num: Option<usize>,
}

pub fn Content<'a>(cx: Scope<'a, ContentProps<'a>>) -> Element<'a> {
    let app_data = &cx.props.app_data;
    if let Some(interface) = app_data
        .read()
        .data
        .model
        .interfaces
        .get(cx.props.interface_num)
    {
        if let Some(register) = interface.registers.get(cx.props.register_num) {
            let interface_data_width = interface.data_width.unwrap_or(32);

            // extract a list of fields, positions, access and types
            let fld_list = register
                .fields
                .iter()
                .enumerate()
                .map(|(n, field)| {
                    (
                        n,
                        field.name.clone(),
                        field.position.clone(),
                        field.access,
                        field.signal,
                    )
                })
                .collect::<Vec<_>>();

            // now build some items from that list
            let fld_items =
                fld_list
                    .iter()
                    .map(|(n, fld_name, fld_pos, fld_access, fld_signal)| {
                        rsx!(
                            TableLine {
                                app_data: app_data,
                                field_number: *n,
                                field_name: fld_name.clone(),
                                field_position: fld_pos.clone(),
                                field_access: fld_access.clone(),
                                field_type: fld_signal.clone(),
                                is_selected: cx.props.field_num == Some(*n),
                                key: "{fld_name}{n}"
                            }
                        )
                    });

            // build a vector with statuses for each bit in the field to display the bitmap
            let field_width = interface.get_data_width().unwrap_or(32) as usize;
            let mut bit_statuses = vec![FieldBitStatus::Unused; field_width];

            for (i, field) in register.fields.iter().enumerate() {
                match field.position {
                    mdf::FieldPosition::Single(bit) => {
                        if (bit as usize) < field_width {
                            bit_statuses[bit as usize] =
                                update_status(&bit_statuses[bit as usize], i, cx.props.field_num);
                        }
                    }
                    mdf::FieldPosition::Field(msb, lsb) => {
                        for bit in lsb..=msb {
                            if (bit as usize) < field_width {
                                bit_statuses[bit as usize] = update_status(
                                    &bit_statuses[bit as usize],
                                    i,
                                    cx.props.field_num,
                                );
                            }
                        }
                    }
                };
            }

            // build the table header. We'll have only 16 bits with displayed bit number
            let regular_colspan = usize::max((field_width + 8) / 16, 1);
            let num_headers = field_width / regular_colspan;
            let first_colspan = field_width - regular_colspan * (num_headers - 1);
            let table_header = (0..num_headers).map(|i| {
                let colspan = if i == 0 {
                    first_colspan
                } else {
                    regular_colspan
                };
                let display = (num_headers - 1 - i) * regular_colspan;
                rsx! {
                    th { colspan: "{colspan}", div { class: "ext-bitfield-header", "{display.clone()}" } }
                }
            });

            // build the table content. empty cells with different background color depending on the status
            bit_statuses.reverse();
            let table_content = bit_statuses.iter().map(|element| {
                let class = match element {
                    FieldBitStatus::Unused => "has-background-dark",
                    FieldBitStatus::Used => "has-background-link",
                    FieldBitStatus::Selected => "has-background-primary",
                    FieldBitStatus::Error => "has-background-danger",
                };

                rsx! {
                    td { class: "ext-bitfield-cell", div { class: "{class} ext-bitfield-contents", " " } }
                }
            });

            cx.render(rsx! {
                div {
                    a {
                        class: "button is-link is-outlined ext-return-button",
                        onclick: move |_| {
                            app_data
                                .with_mut(|app| {
                                    app.page_type = PageType::Interface(cx.props.interface_num);
                                })
                        },
                        span { class: "icon ", i { class: "fa-solid fa-caret-left" } }
                    }
                    h1 { class: "title page-title", "Register" }
                }
                div { class: "m-4",
                    gui_blocks::TextGeneric {
                        app_data: app_data,
                        update_reg: callback(|register, value: &String| register.name = value.clone()),
                        gui_label: "Name",
                        undo_label: "change register name",
                        value: register.name.clone()
                    }
                    gui_blocks::TextArea {
                        app_data: app_data,
                        update_reg: callback(|register, value| register.summary = value.clone()),
                        gui_label: "Summary",
                        undo_label: "change register summary",
                        rows: 1,
                        value: register.summary.clone()
                    }
                    gui_blocks::TextArea {
                        app_data: app_data,
                        update_reg: callback(|register, value| register.description = value.clone()),
                        gui_label: "Description",
                        undo_label: "change register description",
                        value: register.description.clone()
                    }
                    gui_blocks::AutoManuText {
                        app_data: app_data,
                        update_reg: callback(|register, value| register.address.value = *value),
                        gui_label: "Address",
                        field_class: "ext-vector-field",
                        undo_label: "change register base address",
                        value: register.address.value
                    }
                    AddressStride {
                        app_data: app_data,
                        update_reg: callback(|register, value| register.address.stride = value.clone()),
                        value: register.address.stride.clone()
                    }
                    gui_blocks::OptionEnumWidget {
                        app_data: app_data,
                        update_reg: callback(move |register, value| {
    register.signal = *value;
    default_fields(interface_data_width, register);
}),
                        gui_label: "Signal type",
                        field_for_none: "(bitfield)",
                        undo_label: "change register signal type",
                        value: register.signal
                    }
                    if register.signal.is_some() {

                        rsx! {
                            gui_blocks::OptionEnumWidget {
                                app_data: app_data,
                                gui_label: "Location",
                                value: register.location,
                                undo_label: "change register locatione",
                                update_reg : callback( |register, value | register.location = *value)
                            },
                            gui_blocks::TextGeneric {
                                app_data: app_data,
                                update_reg: callback( |register, value | register.width = Some(*value)),
                                gui_label: "Width",
                                field_class: "ext-vector-field",
                                undo_label: "change register name",
                                value: register.width.unwrap_or_default(),
                            },
                            gui_blocks::OptionEnumWidget {
                                app_data: app_data,
                                gui_label: "Access",
                                value: register.access,
                                undo_label: "change register access mode",
                                update_reg : callback( |register, value | register.access = *value)
                            },
                            gui_blocks::TextGeneric {
                                app_data: app_data,
                                update_reg: callback( |register, value | register.reset = Some(*value)),
                                gui_label: "Reset value",
                                field_class: "ext-vector-field",
                                undo_label: "change reset value",
                                value: register.reset.unwrap_or_default(),
                            },
                            CoreProperties {
                                app_data: app_data,
                                value: register.core_signal_properties.clone(),
                                is_register: true
                            },
                        }
                    } else { // signal is bitfield

                        rsx! {
                            gui_blocks::OptionEnumWidget {
                                app_data: app_data,
                                gui_label: "Location",
                                field_for_none : "define per field"
                                value: register.location,
                                undo_label: "change register locatione",
                                update_reg : callback( |register, value | {
                                    register.location = *value;
                                    if register.location.is_some() {
                                        for field in register.fields.iter_mut() {
                                            field.location = None
                                        }
                                    }
                                })
                            },
                            h2 { class:"subtitle page-title", "Fields"},
                            table {
                                class: "ext-bitfield-table",
                                style: "min-width: 100%;",
                                thead {
                                    tr {
                                        table_header
                                    }
                                }
                                tbody {
                                    tr {
                                        table_content
                                    }
                                }
                            }

                            table {
                                class:"table is-striped is-hoverable is-fullwidth",
                                thead {
                                    tr {
                                        th { "Name"},
                                        th { "Bits"},
                                        th { "Access"},
                                        th { "Type"},
                                        th {}
                                    }
                                },
                                tbody {
                                    fld_items
                                }
                            }
                            div { class:"buttons",
                                button { class:"button is-primary",
                                    onclick: move |_| app_data.with_mut(|app| {
                                        app.data.model.interfaces[cx.props.interface_num].registers[cx.props.register_num].fields.push(mdf::Field::new());
                                        app.page_type = PageType::Register(cx.props.interface_num, cx.props.register_num, Some(app.data.model.interfaces[cx.props.interface_num].registers[cx.props.register_num].fields.len()-1));
                                        app.register_undo("create field")
                                        }),
                                    "New field"
                                },
                                button { class:"button is-dark",
                                    onclick: move |_| app_data.with_mut(|app| {
                                        app.page_type = PageType::Register(cx.props.interface_num, cx.props.register_num, None);
                                        }),
                                    "Deselect field"
                                },
                                button { class:"button is-primary",
                                    onclick: move |_| app_data.with_mut(|app| {
                                        let result = app.data.model.interfaces[cx.props.interface_num].registers[cx.props.register_num].assign_fields();
                                        app.test_result(result);
                                        app.register_undo("assign bitfields")
                                    }),
                                    "Assign bits"
                                },
                                button { class:"button is-danger",
                                    onclick: move |_| app_data.with_mut(|app| {
                                        let result = app.data.model.interfaces[cx.props.interface_num].registers[cx.props.register_num].deassign_fields();
                                        app.test_result(result);
                                        app.register_undo("unassign bitfields")
                                    }),
                                    "Unassign bits"
                                },
                            }
                            if let Some(field_num) = cx.props.field_num {
                                if let Some(field) = register.fields.get(field_num) {
                                    let location_disabled = register.location.is_some();
                                    let location_value = if location_disabled {
                                        register.location
                                    } else {
                                        field.location
                                    };
                                    rsx!(
                                        gui_blocks::TextGeneric {
                                            app_data: app_data,
                                            update_field: callback( |field, value : &String| field.name = value.clone()),
                                            gui_label: "Field Name",
                                            undo_label: "change field name",
                                            value: field.name.clone()
                                        },
                                        FieldPosition {
                                            app_data: app_data,
                                            update_field: callback( |field, value : &mdf::FieldPosition | field.position = value.clone()),
                                            value: field.position.clone()
                                        },
                                        gui_blocks::TextArea {
                                            app_data: app_data,
                                            update_field: callback( |field, value | field.description = value.clone()),
                                            gui_label: "Description",
                                            undo_label: "change field description",
                                            value: field.description.clone()
                                        },
                                        gui_blocks::EnumWidget {
                                            app_data: app_data,
                                            gui_label: "Access",
                                            value: field.access,
                                            undo_label: "change field access mode",
                                            update_field : callback( |field, value | field.access = *value)
                                        },
                                        gui_blocks::EnumWidget {
                                            app_data: app_data,
                                            gui_label: "Type",
                                            value: field.signal,
                                            undo_label: "change field signal type",
                                            update_field : callback( |field, value | field.signal = *value)
                                        },
                                        gui_blocks::TextGeneric {
                                            app_data: app_data,
                                            update_field: callback( |field, value | field.reset = *value),
                                            gui_label: "Reset value",
                                            field_class: "ext-vector-field",
                                            undo_label: "change reset value",
                                            value: field.reset
                                        },
                                        gui_blocks::OptionEnumWidget {
                                            app_data: app_data,
                                            gui_label: "Location",
                                            disabled: location_disabled,
                                            value: location_value,
                                            undo_label: "change field location",
                                            update_field : callback( |field, value | field.location = *value)
                                        },
                                        CoreProperties {
                                            app_data: app_data,
                                            value: field.core_signal_properties.clone(),
                                            is_register: false
                                        },
                                    )
                                } else {
                                    rsx!(
                                        p {}
                                    )
                                }
                            }
                        }
                    }
                }
            })
        } else {
            cx.render(rsx! { p { "bad register number" } })
        }
    } else {
        cx.render(rsx! { p { "bad interface number" } })
    }
}
