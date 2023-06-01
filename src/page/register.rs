//! page to edit a register
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::HdlWizardApp;
use crate::gui_blocks;
use crate::gui_blocks::callback;
use crate::file_formats::mdf;
use std::cell::RefCell;
use crate::gui_types::Validable;
use crate::utils;
use std::str::FromStr;
use std::default::Default;
use crate::page::PageType;

//fn absdiff(a: u32, b: u32) -> u32 {
//    if a > b {
//        a - b
//    } else {
//        b - a
//    }
//}

// default values for some fields when changing the signal type
fn default_fields(interface_width : u32, register: &mut mdf::Register) {
    let default_width = match register.signal {
        Some(utils::SignalType::Boolean) | Some(utils::SignalType::StdLogic) => 1,
        Some(utils::SignalType::Signed) | Some(utils::SignalType::Unsigned) | Some(utils::SignalType::StdLogicVector) =>
            interface_width,
        None => 0 
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
    value : Option<mdf::AddressStride>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &Option<mdf::AddressStride>) -> () + 'a>>>,
}

fn AddressStride<'a>(
    cx: Scope<'a, GuiAddressStrideProps<'a>>) -> Element<'a>
{
    let validate_pattern = utils::VectorValue::validate_pattern();
    let value = &cx.props.value;
    let is_stride = value.is_some();
    let has_increment = match value {
        Some(addrstr) => addrstr.increment.is_some(),
        _ => false
    };
    let count = match value {
        Some(addrstr) => addrstr.count,
        _ => Default::default()
    };
    let count_string = if is_stride {
        count.to_string()
    } else {
        Default::default()
    };
    let increment_field = match value {
        Some(addrstr) => addrstr.increment,
        _ => Default::default()
    };
    let increment = increment_field.unwrap_or_default();
    let increment_string = if has_increment {
        increment.to_string()
    } else {
        Default::default()
    };
    let label_class = if is_stride {
        ""
    } else {
        "has-text-grey-light"
    };

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", " " }
            }
            div { class:"field-body",
                div { class:"field is-grouped is-align-items-center",
                    div { class:"control",
                        label { class:"checkbox",
                            input { 
                                r#type: "checkbox", 
                                onclick : move | _ | {
                                    let new_value : Option<mdf::AddressStride> = if is_stride {
                                        None
                                    } else {
                                        Some(mdf::AddressStride {
                                            count: utils::VectorValue {
                                                value: 1,
                                                radix: utils::RadixType::Decimal    
                                            },
                                            increment : None
                                        })
                                    };
                                    gui_blocks::apply_function(&cx.props.app_data, new_value, "change address stride status", &None, &None, &cx.props.update_reg, &None);
                                },
                                checked: "{is_stride}"
                            },
                            " Stride: "
                        },
                    },
                    div { class:"control",
                        label {
                            class: "{label_class}",
                            "Count: "
                        }
                    },
                    div { class:"control",    
                        input { class:"input ext-vector-field", r#type:"text", placeholder:"count", pattern:"{validate_pattern}",
                            onchange: move | evt | {
                                if let Ok(new_value) = utils::VectorValue::from_str(&evt.value) {
                                    let new_stride = mdf::AddressStride {
                                        count : new_value,
                                        increment : increment_field
                                    };
                                    gui_blocks::apply_function(&cx.props.app_data, Some(new_stride), "change stride count", &None, &None, &cx.props.update_reg, &None);
                                }
                            },
                            value: "{count_string}",
                            disabled: "{!is_stride}"
                        }
                    },
                    div { class:"control",
                        label { class:"checkbox",
                            class : "{label_class}",
                            input { 
                                r#type: "checkbox", 
                                onclick : move | _ | {
                                    if is_stride {
                                        let new_value : Option<mdf::AddressStride> = if has_increment {
                                            Some(mdf::AddressStride{
                                                count : count,
                                                increment : None
                                            })
                                        } else {
                                            Some(mdf::AddressStride{
                                                count : count,
                                                increment : Some(Default::default())
                                            })
                                        };
                                        gui_blocks::apply_function(&cx.props.app_data, new_value, "change address stride increment option", &None, &None, &cx.props.update_reg, &None);    
                                    }
                                },
                                checked: "{has_increment}",
                                disabled: "{!is_stride}"
                            },
                            " Increment: "
                        },
                    },
                    div { class:"control",
                        label {
                            input { class:"input ext-vector-field", r#type:"text", placeholder:"auto", pattern:"{validate_pattern}",
                                onchange: move | evt | {
                                    if let Ok(new_value) = utils::VectorValue::from_str(&evt.value) {
                                        let new_stride = mdf::AddressStride {
                                            count : count,
                                            increment : Some(new_value)
                                        };
                                        gui_blocks::apply_function(&cx.props.app_data, Some(new_stride), "change stride increment value", &None, &None, &cx.props.update_reg, &None);
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
    value : mdf::CoreSignalProperties,
    is_register : bool
}

fn CoreProperties<'a>(
    cx: Scope<'a, GuiCoreProps<'a>>) -> Element<'a>
{
    let value = &cx.props.value;
    let use_read_enable = value.use_read_enable.unwrap_or(false);
    let use_write_enable = value.use_write_enable.unwrap_or(false);

    let read_update_function_reg : Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &bool) -> () + 'a>>> = 
        if cx.props.is_register { 
            Some(callback(| register, value | register.core_signal_properties.use_read_enable = Some(*value)))
        } else {
            None
        };
    let write_update_function_reg : Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &bool) -> () + 'a>>> = 
        if cx.props.is_register { 
            Some(callback(| register, value | register.core_signal_properties.use_write_enable = Some(*value)))
        } else {
            None
        };
    let read_update_function_field : Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &bool) -> () + 'a>>> = 
        if !cx.props.is_register { 
            Some(callback(| field, value | field.core_signal_properties.use_read_enable = Some(*value)))
        } else {
            None
        };
    let write_update_function_field : Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &bool) -> () + 'a>>> = 
        if !cx.props.is_register { 
            Some(callback(| field, value | field.core_signal_properties.use_write_enable = Some(*value)))
        } else {
            None
        };

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "Core Properties" }
            }
            div { class:"field-body",
                div { class:"field is-grouped is-align-items-center",
                    div { class:"control",
                        label { class:"checkbox",
                            input { 
                                r#type: "checkbox", 
                                onclick : move | _ | {
                                    gui_blocks::apply_function(&cx.props.app_data, !use_read_enable, "change read enable core property", 
                                        &None, &None, &read_update_function_reg, &read_update_function_field)
                                    },
                                checked: "{use_read_enable}"
                            },
                            " Use read enable "
                        },
                    },
                    div { class:"control",
                        label { class:"checkbox",
                            input { 
                                r#type: "checkbox", 
                                onclick : move | _ | {
                                    gui_blocks::apply_function(&cx.props.app_data, !use_write_enable, "change write enable core property", 
                                        &None, &None, &write_update_function_reg, &write_update_function_field)
                                    },
                                checked: "{use_write_enable}"
                            },
                            " Use write enable "
                        },
                    },
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
    field_type: utils::SignalType
) -> Element<'a> {
    if let PageType::Register(interface_number, register_number, _) = app_data.read().page_type {
        let num_of_fields = app_data.read().data.model.interfaces[interface_number].registers[register_number].fields.len();
        let up_disabled = *field_number == 0;
        let down_disabled = *field_number == num_of_fields-1;
    
        let display_name = if field_name.is_empty() {"(empty)" } else  {field_name};
    
        cx.render(rsx! {
            tr {
                td { 
                    a {
                        onclick: move | _ | app_data.with_mut(|data| data.page_type = PageType::Register(interface_number, register_number, Some(*field_number))),
                        "{display_name}",
                    }
                },
                td { "{field_position.to_string()}"},
                td { "{field_access.to_string()}"}
                td { "{field_type.to_string()}"},
                td { 
                    div { class:"buttons are-small ext-buttons-in-table",
                        button { class:"button is-primary", disabled:"{up_disabled}",
                            onclick: move | _ | if !up_disabled {
                                app_data.with_mut(|data| {
                                    data.data.model.interfaces[interface_number].registers[register_number].fields.swap(*field_number-1, *field_number);
                                    data.register_undo("move field up")
                                })
                            },
                            span {
                                class:"icon is_small",
                                i { class:"fa-solid fa-caret-up"}
                            }
                        }
                        button { class:"button is-primary", disabled:"{down_disabled}",
                            onclick: move | _ | if !down_disabled {
                                app_data.with_mut(|data| {
                                    data.data.model.interfaces[interface_number].registers[register_number].fields.swap(*field_number, *field_number+1);
                                    data.register_undo("move field down")
                                })
                            },
                        span {
                                class:"icon is_small",
                                i { class:"fa-solid fa-caret-down"}
                            }
                        }
                        button { class:"button is-link",
                            onclick: move | _ | app_data.with_mut(|data| data.page_type = PageType::Register(interface_number, register_number, Some(*field_number))),
                            span {
                                class:"icon is_small",
                                i { class:"fa-solid fa-pen"}
                            }
                        }
                        button { class:"button is-danger",
                            onclick: move | _ | app_data.with_mut(|data| {
                                data.data.model.interfaces[interface_number].registers[register_number].fields.remove(*field_number);
                                data.register_undo("remove field")
                            }),
                            span {
                                    class:"icon is_small",
                                    i { class:"fa-solid fa-trash"}
                                }
                            }
                    }
                }
            }
        })    
    } else {
        cx.render(rsx!(p { "error.... not in a interface page"}))
    }
}

// widget for the bitfield position
#[derive(Props)]
struct GuiBitFieldPositionProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    value : mdf::FieldPosition,
    update_field: Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &mdf::FieldPosition) -> () + 'a>>>,
}

fn FieldPosition<'a>(
    cx: Scope<'a, GuiBitFieldPositionProps<'a>>) -> Element<'a>
{
    let validate_pattern = u32::validate_pattern();
    let value = &cx.props.value;
    let pos_high = match value {
        mdf::FieldPosition::Single(pos) => pos.to_string(),
        mdf::FieldPosition::Field(high, _) => high.to_string()
    };
    let pos_low = match value {
        mdf::FieldPosition::Single(_) => Default::default(),
        mdf::FieldPosition::Field(_, low) => low.to_string()
    };


    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "Position" }
            }
            div { class:"field-body",
                div { class:"field is-grouped is-align-items-center",
                    div { class:"control",    
                        input { class:"input ext-vector-field", r#type:"text", placeholder:"MSB", pattern:"{validate_pattern}",
                            onchange: move | evt | {
                                if let Ok(new_value) = u32::from_str(&evt.value) {
                                    let new_pos = match &value {
                                        mdf::FieldPosition::Single(_) => mdf::FieldPosition::Single(new_value),
                                        mdf::FieldPosition::Field(_, low) => mdf::FieldPosition::Field(new_value, *low),

                                    };
                                    gui_blocks::apply_function(&cx.props.app_data, new_pos, "change bitfield position msb", &None, &None,  &None, &cx.props.update_field);
                                }
                            },
                            value: "{pos_high}",
                        }
                    },
                    div { class:"control",
                        label { 
                            " downto "
                        },
                    },
                    div { class:"control",
                        label {
                            input { class:"input ext-vector-field", r#type:"text", placeholder:"single", pattern:"{validate_pattern}",
                                onchange: move | evt | {
                                    if let Ok(new_value) = u32::from_str(&evt.value) {
                                        let new_pos = match &value {
                                            mdf::FieldPosition::Single(high) => mdf::FieldPosition::Field(*high, new_value),
                                            mdf::FieldPosition::Field(high, _) => mdf::FieldPosition::Field(*high, new_value),
                                        };
                                        gui_blocks::apply_function(&cx.props.app_data, new_pos, "change bitfield position lsb", &None, &None,  &None, &cx.props.update_field);
                                    } else {
                                        // field is empty or invalid
                                        let new_pos = match &value {
                                            mdf::FieldPosition::Single(high) => mdf::FieldPosition::Single(*high),
                                            mdf::FieldPosition::Field(high, _) => mdf::FieldPosition::Single(*high),
                                        };
                                        gui_blocks::apply_function(&cx.props.app_data, new_pos, "change bitfield position lsb", &None, &None,  &None, &cx.props.update_field);
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

// page contents
#[derive(Props)]
pub struct ContentProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    interface_num: usize,
    register_num: usize,
    #[props(!optional)]
    field_num: Option<usize>
}

pub fn Content<'a>(
    cx: Scope<'a, ContentProps<'a>>) -> Element<'a> {
    let app_data = &cx.props.app_data;
    if let Some(interface) = app_data.read().data.model.interfaces.get(cx.props.interface_num) {
        if let Some(register) = interface.registers.get(cx.props.register_num) {

            let interface_data_width = interface.data_width.unwrap_or(32);

            // extract a list of fields, positions, access and types
            let fld_list = register.fields.iter().enumerate().map(
                |(n, field)| (n, field.name.clone(), field.position.clone(), field.access, field.signal)).collect::<Vec<_>>();

            // now build some items from that list
            let fld_items = fld_list.iter().map( |(n, fld_name, fld_pos, fld_access, fld_signal) | {
                        rsx!(
                            TableLine {
                                app_data: app_data,
                                field_number: *n,
                                field_name: fld_name.clone(),
                                field_position: fld_pos.clone(),
                                field_access: fld_access.clone(),
                                field_type: fld_signal.clone(),
                                key: "{fld_name}{n}"
                            }
                        )
                    }
                );

            cx.render(rsx! {
                h1 { class:"title page-title", "Register" },
                div { class:"m-4",
                    gui_blocks::TextGeneric {
                        app_data: app_data,
                        update_reg: callback( |register, value : &String| register.name = value.clone()),
                        gui_label: "Name",
                        undo_label: "change register name",
                        value: register.name.clone()              
                    },
                    gui_blocks::TextArea {
                        app_data: app_data,
                        update_reg: callback( |register, value | register.summary = value.clone()),
                        gui_label: "Summary",
                        undo_label: "change register summary",
                        rows: 1,
                        value: register.summary.clone()
                    },
                    gui_blocks::TextArea {
                        app_data: app_data,
                        update_reg: callback( |register, value | register.description = value.clone()),
                        gui_label: "Description",
                        undo_label: "change register description",
                        value: register.description.clone()
                    },
                    gui_blocks::AutoManuText {
                        app_data: app_data,
                        update_reg: callback( |register, value | register.address.value = *value),
                        gui_label: "Address",
                        field_class: "ext-vector-field",
                        undo_label: "change register base address",
                        value: register.address.value,
                    },
                    AddressStride {
                        app_data: app_data,
                        update_reg: callback( |register, value | register.address.stride = value.clone()),
                        value : register.address.stride.clone()
                    },
                    gui_blocks::OptionEnumWidget {
                        app_data: app_data,
                        update_reg: callback( move |register, value| {
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
                                value: register.width.unwrap_or_default()             
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
                                value: register.reset.unwrap_or_default()             
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
                                }
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
            cx.render(rsx! { p { "bad register number"} })
        }
    } else {
        cx.render(rsx! { p { "bad interface number"} })
    }
}

/*

    undo_label : Option<&'a str>,
    field_for_none : Option<&'a str>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &Option<F>) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &Option<F>) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &Option<F>) -> () + 'a>>>,
 */
/*


        ui.horizontal(|mut ui| {
            gui_blocks::widget_auto_manual_u32_inline(
                &mut register.width,
                &mut ui,
                "Width",
                register.fields.is_empty(),
                undo,
            );

            gui_blocks::widget_combobox_inline(
                &mut register.access,
                &mut ui,
                "Access",
                None,
                match register.fields.is_empty() {
                    true => Some(model_gui::AccessType::PerField),
                    false => None,
                },
                undo,
            );
            gui_blocks::widget_combobox_inline(
                &mut register.location,
                &mut ui,
                "Location",
                None,
                match register.fields.is_empty() {
                    true => Some(model_gui::LocationType::PerField),
                    false => None,
                },
                undo,
            );
        });

        if register.location == model_gui::LocationType::Core {
            ui.horizontal(|mut ui| {
                let disabled_option = match register.fields.is_empty() {
                    true => Some(model_gui::CoreSignalProperty::PerField),
                    false => None,
                };
                gui_blocks::widget_combobox_inline(
                    &mut register.core_use_read_enable,
                    &mut ui,
                    "use read enable",
                    None,
                    disabled_option,
                    undo,
                );
                gui_blocks::widget_combobox_inline(
                    &mut register.core_use_write_enable,
                    &mut ui,
                    "use write enable",
                    None,
                    disabled_option,
                    undo,
                );
            });
        }

        if register.fields.is_empty() {
            ui.horizontal(|mut ui| {
                gui_blocks::widget_combobox_inline(
                    &mut register.signal_type,
                    &mut ui,
                    "Signal type",
                    None,
                    None,
                    undo,
                );
                gui_blocks::widget_vectorvalue_inline(
                    &mut register.reset,
                    &mut ui,
                    "reset value",
                    None,
                    undo,
                );
            });
            ui.separator();
        } else {
            ui.separator();
            register.update_bitfield(interface_data_width);
            gui_blocks::widget_bitfield(ui, &register.bitfield);
        }

        ui.horizontal(|ui| {
            ui.heading("Fields:");
            if ui.button("New").clicked() {
                // if this is the first field and the register width is not valid, switch it to auto
                if register.fields.is_empty() && (!register.width.manual.str_valid) {
                    register.width.is_auto = true;
                }
                // find highest bit to put the new field over it
                let new_bit = register.fields.iter().fold(0, |maxbit, field| {
                    u32::max(
                        maxbit,
                        u32::max(field.position_start.value_int, field.position_end.value_int) + 1,
                    )
                });

                let position = gui_types::GuiU32 {
                    value_str: new_bit.to_string(),
                    str_valid: true,
                    value_int: new_bit,
                };

                register.fields.push(model_gui::Field {
                    position_start: position.clone(),
                    position_end: position,
                    ..Default::default()
                });
                undo.register_modification("create new field", undo::ModificationType::Finished);
            }
        });

        if !register.fields.is_empty() {
            let mut action: Option<FieldsModification> = None;
            let mut hovered_field: Option<usize> = None;
            let num_fields = register.fields.len();
            let can_access_as_register = register.access != model_gui::AccessType::PerField;
            let can_location_as_register = register.location != model_gui::LocationType::PerField;
            let register_location_core = register.location == model_gui::LocationType::Core;
            let can_use_re_as_register = register_location_core
                && register.core_use_read_enable != model_gui::CoreSignalProperty::PerField;
            let can_use_we_as_register = register_location_core
                && register.core_use_write_enable != model_gui::CoreSignalProperty::PerField;

            ui.add_space(5.0);
            egui::ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for (n, field) in register.fields.iter_mut().enumerate() {
                        let field_inner_response = ui.vertical(|mut ui| {
                            ui.separator();

                            ui.horizontal(|ui| {
                                ui.label("from bit ");
                                gui_blocks::widget_u32_inline_nolabel(
                                    &mut field.position_start,
                                    ui,
                                    &format!("bit start {}", n),
                                    "field bit start",
                                    undo,
                                );
                                ui.label("to bit ");
                                gui_blocks::widget_u32_inline_nolabel(
                                    &mut field.position_end,
                                    ui,
                                    &format!("bit stop {}", n),
                                    "field bit start",
                                    undo,
                                );

                                let size_text = match absdiff(
                                    field.position_end.value_int,
                                    field.position_start.value_int,
                                ) {
                                    0 => "(1 bit)".to_string(),
                                    n => format!("({} bits)", n + 1),
                                };
                                ui.label(size_text);

                                if ui.button("🗑").clicked() {
                                    action = Some(FieldsModification::Delete(n));
                                    undo.register_modification(
                                        "delete field",
                                        undo::ModificationType::Finished,
                                    );
                                }
                                ui.add_enabled_ui(n > 0, |ui| {
                                    if ui.button("⬆").clicked() {
                                        action = Some(FieldsModification::Swap(n - 1, n));
                                        undo.register_modification(
                                            "move field",
                                            undo::ModificationType::Finished,
                                        );
                                    }
                                });
                                ui.add_enabled_ui(n < (num_fields - 1), |ui| {
                                    if ui.button("⬇").clicked() {
                                        action = Some(FieldsModification::Swap(n, n + 1));
                                        undo.register_modification(
                                            "move field",
                                            undo::ModificationType::Finished,
                                        );
                                    }
                                });
                            });

                            gui_blocks::widget_text(
                                &mut field.name,
                                &mut ui,
                                "Name",
                                gui_blocks::TextWidgetType::SingleLine,
                                undo,
                            );

                            ui.horizontal(|mut ui| {
                                gui_blocks::widget_combobox_inline(
                                    &mut field.access,
                                    &mut ui,
                                    "Access",
                                    Some(&format!("field access{}", n)),
                                    match can_access_as_register {
                                        false => Some(model_gui::AccessTypeField::AsRegister),
                                        true => None,
                                    },
                                    undo,
                                );
                                gui_blocks::widget_combobox_inline(
                                    &mut field.location,
                                    &mut ui,
                                    "Location",
                                    Some(&format!("field location{}", n)),
                                    match can_location_as_register {
                                        false => Some(model_gui::LocationTypeField::AsRegister),
                                        true => None,
                                    },
                                    undo,
                                );

                                if (register_location_core
                                    && field.location == model_gui::LocationTypeField::AsRegister)
                                    || field.location == model_gui::LocationTypeField::Core
                                {
                                    gui_blocks::widget_combobox_inline(
                                        &mut field.core_use_read_enable,
                                        &mut ui,
                                        "use read enable",
                                        Some(&format!("core use re field {}", n)),
                                        match can_use_re_as_register {
                                            false => {
                                                Some(model_gui::CoreSignalPropertyField::AsRegister)
                                            }
                                            true => None,
                                        },
                                        undo,
                                    );
                                    gui_blocks::widget_combobox_inline(
                                        &mut field.core_use_write_enable,
                                        &mut ui,
                                        "use write enable",
                                        Some(&format!("core use we field {}", n)),
                                        match can_use_we_as_register {
                                            false => {
                                                Some(model_gui::CoreSignalPropertyField::AsRegister)
                                            }
                                            true => None,
                                        },
                                        undo,
                                    );
                                }
                            });
                            ui.horizontal(|mut ui| {
                                gui_blocks::widget_combobox_inline(
                                    &mut field.signal_type,
                                    &mut ui,
                                    "Signal type",
                                    Some(&format!("field signal type {}", n)),
                                    None,
                                    undo,
                                );
                                gui_blocks::widget_vectorvalue_inline(
                                    &mut field.reset,
                                    &mut ui,
                                    "reset value",
                                    Some(&format!("field signal reset value {}", n)),
                                    undo,
                                );
                            });
                            gui_blocks::widget_text(
                                &mut field.description,
                                &mut ui,
                                "Description",
                                gui_blocks::TextWidgetType::MultiLine,
                                undo,
                            );
                        });

                        if field_inner_response.response.hovered() {
                            hovered_field = Some(n);
                        }
                    }

                    register.hovered_field = hovered_field;
                });

            match action {
                Some(FieldsModification::Delete(n)) => {
                    register.fields.remove(n);
                }
                Some(FieldsModification::Swap(a, b)) => {
                    register.fields.swap(a, b);
                }
                None => (),
            }
        }
    });
}
*/