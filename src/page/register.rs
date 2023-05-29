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
                                    gui_blocks::apply_function(&cx.props.app_data, new_value, "change address stride status", &None, &None, &cx.props.update_reg);
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
                                    gui_blocks::apply_function(&cx.props.app_data, Some(new_stride), "change stride count", &None, &None, &cx.props.update_reg);
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
                                        gui_blocks::apply_function(&cx.props.app_data, new_value, "change address stride increment option", &None, &None, &cx.props.update_reg);    
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
                                        gui_blocks::apply_function(&cx.props.app_data, Some(new_stride), "change stride increment value", &None, &None, &cx.props.update_reg);
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

#[inline_props]
pub fn Content<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>,
    interface_num: usize,
    register_num: usize
) -> Element<'a> {
    if let Some(interface) = app_data.read().data.model.interfaces.get(*interface_num) {
        if let Some(register) = interface.registers.get(*register_num) {

            let interface_data_width = interface.data_width.unwrap_or(32);

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

                        cx.render(rsx! {
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
                            gui_blocks::OptionEnumWidget {
                                app_data: app_data,
                                gui_label: "Location",
                                value: register.location,
                                undo_label: "change register locatione",
                                update_reg : callback( |register, value | register.location = *value)
                            },
                            gui_blocks::CheckBox {
                                app_data: app_data,
                                gui_label: "Core Properties",
                                checkbox_label: "use read enable",
                                value: register.core_signal_properties.use_read_enable.unwrap_or(false),
                                undo_label: "change use read enable core property",
                                update_reg : callback( |register, value | register.core_signal_properties.use_read_enable = Some(*value))
                            },
                            gui_blocks::CheckBox {
                                app_data: app_data,
                                gui_label: "",
                                checkbox_label: "use write enable",
                                value: register.core_signal_properties.use_write_enable.unwrap_or(false),
                                undo_label: "change use write enable core property",
                                update_reg : callback( |register, value | register.core_signal_properties.use_write_enable = Some(*value))
                            },
                        })
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