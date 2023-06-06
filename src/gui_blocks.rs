//! building blocks for GUI
//!
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use dioxus::prelude::*;

use crate::gui_types;
use std::cell::RefCell;
use crate::file_formats::mdf;
use crate::page::PageType;
use crate::utils;

// wraps a closure into a box and a refcell. Used to make widget instantiations a bit simpler
pub fn callback<F> (f: F) -> RefCell<Box<F>> {
    RefCell::new(Box::new(f))
}

// call one of the update functions applying on one part of the model depending on the page type
pub fn apply_function<'a, F>(
    app_data: &'a UseRef<HdlWizardApp>,
    value : F,
    undo_description : &str,
    update_model: &Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &F) -> () + 'a>>>,
    update_int: &Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &F) -> () + 'a>>>,
    update_reg: &Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &F) -> () + 'a>>>,
    update_field : &Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &F) -> () + 'a>>>) {

    let page_type = &app_data.read().page_type.clone();
    match page_type {
        PageType::Project => {
            if let Some(updatefn_ref) = &update_model {
                let mut updatefn = updatefn_ref.borrow_mut();
                app_data.with_mut(|app_data| {
                    updatefn(&mut app_data.data.model, &value);
                    app_data.register_undo(undo_description);
                })
            }        
        },
        PageType::Interface(interface_number) => {
            if let Some(updatefn_ref) = &update_int {
                let mut updatefn = updatefn_ref.borrow_mut();
                app_data.with_mut(|app_data| {
                    if let Some(mut interface) = app_data.data.model.interfaces.get_mut(*interface_number) {
                        updatefn(&mut interface, &value);
                        app_data.register_undo(undo_description);
                    }
                })
            }      
        },
        PageType::Register(interface_number, register_number, field_number) => {
            if let Some(updatefn_ref) = &update_reg {
                let mut updatefn = updatefn_ref.borrow_mut();
                app_data.with_mut(|app_data| {
                    if let Some(interface) = app_data.data.model.interfaces.get_mut(*interface_number) {
                        if let Some(mut register) = interface.registers.get_mut(*register_number) {
                            updatefn(&mut register, &value);
                            app_data.register_undo(undo_description);    
                        } 
                    }
                })
            } else {
                if let Some(updatefn_ref) = &update_field {
                    let mut updatefn = updatefn_ref.borrow_mut();
                    app_data.with_mut(|app_data| {
                        if let Some(interface) = app_data.data.model.interfaces.get_mut(*interface_number) {
                            if let Some(register) = interface.registers.get_mut(*register_number) {
                                if let Some(field_num) = field_number {
                                    if let Some(mut field) = register.fields.get_mut(*field_num) {
                                        updatefn(&mut field, &value);
                                        app_data.register_undo(undo_description);            
                                    }
                                }
                            } 
                        }
                    })
                }
            }   
        },
    }
}

// properties for a generic GUI widget
#[derive(Props)]
pub struct GuiGenericProps<'a, F> {
    app_data: &'a UseRef<HdlWizardApp>,
    gui_label : &'a str,
    value : F,
    field_class : Option<&'a str>,
    undo_label : Option<&'a str>,
    rows : Option<u32>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &F) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &F) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &F) -> () + 'a>>>,
    update_field: Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &F) -> () + 'a>>>,
}

// generic text widget component, using any type that can be converted to and from a string
pub fn TextGeneric<'a, F: gui_types::Validable + std::string::ToString + std::str::FromStr>(
    cx: Scope<'a, GuiGenericProps<'a, F>>) -> Element<'a>
{
    let gui_label = cx.props.gui_label;
    let value = cx.props.value.to_string();
    let undo_description = cx.props.undo_label.unwrap_or_default();
    let validate_pattern = F::validate_pattern();

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "{gui_label}" }
            }
            div { class:"field-body",
                div { class:"field",
                    div { class:"control",
                        input { class:"input {cx.props.field_class.unwrap_or_default()}", r#type:"text", placeholder:"{gui_label}", pattern:"{validate_pattern}",
                            onchange: move | evt | {
                                if let Ok(value) = F::from_str(&evt.value) {
                                    apply_function(&cx.props.app_data, value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                }
                            },
                            value: "{value}"
                        }
                    }
                }
            }
        }      
    })
}

// textarea widget component, using an optional vector of strings for the value type
pub fn TextArea<'a>(
    cx: Scope<'a, GuiGenericProps<'a, Option<Vec<String>>>>) -> Element<'a>
{
    let gui_label = cx.props.gui_label;
    let value = utils::opt_vec_str_to_textarea(&cx.props.value);
    let undo_description = cx.props.undo_label.unwrap_or_default();

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "{gui_label}" }
            }
            div { class:"field-body",
                div { class:"field",
                    div { class:"control",
                        textarea { class:"textarea", placeholder:"{gui_label}",
                            rows: "{cx.props.rows.unwrap_or(4)}",
                            onchange: move | evt | {
                                let value = utils::textarea_to_opt_vec_str(&evt.value);
                                apply_function(&cx.props.app_data, value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                            },
                            "{value}"
                        }
                    }
                }
            }
        }      
    })
}

// combobox widget using an enum type that uses the strum derives for conversion to and from a string
pub fn EnumWidget<'a, F: PartialEq + strum::IntoEnumIterator + std::string::ToString + std::str::FromStr>(
    cx: Scope<'a, GuiGenericProps<'a, F>>) -> Element<'a>
{
    let gui_label = cx.props.gui_label;
    let value = &cx.props.value;
    let undo_description = cx.props.undo_label.unwrap_or_default();

    let options = F::iter().map( | enum_value | {
        rsx!(
            option {
                selected: "{enum_value == *value}",
                "{enum_value.to_string()}"
            }
        )
    });

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "{gui_label}" }
            }
            div { class:"field-body",
                div { class: "field",
                    div { class: "control select",
                        select {
                            onchange: move | evt | {
                                if let Ok(value) = F::from_str(&evt.value) {
                                    apply_function(&cx.props.app_data, value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                }
                            },
                            options
                        }
                    }
                }
            }
        }      
    })
}

// properties for a GUI widget with an optional value (Auto/Manual)
#[derive(Props)]
pub struct GuiAutoManuProps<'a, F : 'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    gui_label : &'a str,
    field_class : Option<&'a str>,
    #[props(!optional)]
    value : Option<F>,
    placeholder : Option<String>,
    default : Option<F>,
    undo_label : Option<&'a str>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &Option<F>) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &Option<F>) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &Option<F>) -> () + 'a>>>,
    update_field: Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &Option<F>) -> () + 'a>>>,
}

// text widget component with an Auto option
pub fn AutoManuText<'a, F: Default + gui_types::Validable + std::string::ToString + std::str::FromStr + Clone>(
    cx: Scope<'a, GuiAutoManuProps<'a, F>>) -> Element<'a>
{
    let gui_label = cx.props.gui_label;
    let default = match &cx.props.default {
        Some(default) => default.clone(),
        None => F::default()
    };
    let value = match &cx.props.value {
        None => String::default(),
        Some(val) => val.to_string()
    };
    let undo_description = cx.props.undo_label.unwrap_or_default();
    let is_auto = cx.props.value.is_none();
    let validate_pattern = F::validate_pattern();

    let placeholder = match &cx.props.placeholder {
        None => gui_label,
        Some(placeholder) => placeholder
    };

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "{gui_label}" }
            }
            div { class:"field-body",
                div { class:"field is-grouped is-align-items-center",
                    div { class:"control",
                        label { class:"radio",
                            input { 
                                r#type: "radio", 
                                name: "{gui_label}",
                                onclick : move | _ | {apply_function(&cx.props.app_data, None, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);},
                                checked: "{is_auto}"
                            },
                            " Auto "
                        }
                        label { class:"radio",
                            input { 
                                r#type: "radio", 
                                name: "{gui_label}",
                                onclick : move | _ | {
                                    if is_auto {
                                        apply_function(&cx.props.app_data, Some(default.clone()), undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                    }
                                },
                                checked: "{!is_auto}",
                            }
                            " Manual: ",
                        }
                    },
                    div { class:"control",
                        input { class:"input {cx.props.field_class.unwrap_or_default()}", r#type:"text", placeholder:"{placeholder}", pattern:"{validate_pattern}",
                            onchange: move | evt | {
                                if let Ok(value) = F::from_str(&evt.value) {
                                    apply_function(&cx.props.app_data, Some(value), undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                }
                            },
                            value: "{value}",
                            disabled: "{is_auto}"
                        }
                    }
                }
            }
        }      
    })
}

// properties for a combobox widget with an optional value
#[derive(Props)]
pub struct OptionEnumWidgetProps<'a, F> {
    app_data: &'a UseRef<HdlWizardApp>,
    gui_label : &'a str,
    #[props(!optional)]
    value : Option<F>,
    undo_label : Option<&'a str>,
    field_for_none : Option<&'a str>,
    disabled : Option<bool>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &Option<F>) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &Option<F>) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &Option<F>) -> () + 'a>>>,
    update_field: Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &Option<F>) -> () + 'a>>>,
}

// combobox widget using an option of an enum type that uses the strum derives for conversion to and from a string
pub fn OptionEnumWidget<'a, F: PartialEq + strum::IntoEnumIterator + std::string::ToString + std::str::FromStr>(
    cx: Scope<'a, OptionEnumWidgetProps<'a, F>>) -> Element<'a>
{
    let gui_label = cx.props.gui_label;
    let value = &cx.props.value;
    let undo_description = cx.props.undo_label.unwrap_or_default();

    let options = F::iter().map( | enum_value | {
        rsx!(
            option {
                selected: "{*value == Some(enum_value)}",
                "{enum_value.to_string()}"
            }
        )
    });

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "{gui_label}" }
            }
            div { class:"field-body",
                div { class: "field",
                    div { class: "control select",
                        select {
                            onchange: move | evt | {
                                if let Ok(value) = F::from_str(&evt.value) {
                                    apply_function(&cx.props.app_data, Some(value), undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                } else {
                                    apply_function(&cx.props.app_data, None, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                }
                            },
                            disabled: "{cx.props.disabled.unwrap_or(false)}",
                            if cx.props.field_for_none.is_none() && value.is_none() {
                                cx.render(rsx!{
                                    option {
                                        selected: "true",
                                        "(select)"
                                    },    
                                })
                            },
                            options,
                            if let Some(field) = cx.props.field_for_none {
                                cx.render(rsx!{
                                    option {
                                        selected: "{*value == None}",
                                        "{field}"
                                    }
                                })
                            }
                        }
                    }
                }
            }
        }      
    })
}

// checkbox widget component, using a boolean for the value type
#[derive(Props)]
pub struct CheckBoxProps<'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    gui_label : &'a str,
    checkbox_label : &'a str,
    value : bool,
    undo_label : Option<&'a str>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &bool) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &bool) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &bool) -> () + 'a>>>,
    update_field: Option<RefCell<Box<dyn FnMut(&mut mdf::Field, &bool) -> () + 'a>>>,
}

pub fn CheckBox<'a>(
    cx: Scope<'a, CheckBoxProps<'a>>) -> Element<'a>
{
    let gui_label = cx.props.gui_label;
    let checkbox_label = cx.props.checkbox_label;
    let value = cx.props.value;
    let undo_description = cx.props.undo_label.unwrap_or_default();

    cx.render(rsx!{
        div { class:"field is-horizontal",
            div { class:"field-label is-normal",
                label { class:"label", "{gui_label}" }
            }
            div { class:"field-body",
                div { class:"field",
                    div { class:"control",
                        label { class:"checkbox",
                            input { 
                                r#type: "checkbox", 
                                onclick : move | _ | {
                                    apply_function(&cx.props.app_data, !value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg, &cx.props.update_field);
                                },
                                checked: "{value}"
                            },
                            " {checkbox_label} "
                        },
                    }
                }
            }
        }      
    })
}
