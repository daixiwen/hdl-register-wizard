//! building blocks for GUI
//!
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use dioxus::prelude::*;
use crate::file_formats::mdf;
use crate::gui_types;
use crate::page::PageType;
use crate::utils;
use std::cell::RefCell;

/// wraps a closure into a box and a refcell. Used to make widget instantiations a bit simpler
pub fn callback<F>(f: F) -> RefCell<Box<F>> {
    RefCell::new(Box::new(f))
}

/// wraps a closure into an EventHandler that can be trnasmitted to Dioxus
/// the closures work directly on the model or a part of it. It is safe to unwrap() here because apply_function() below already checks
/// that the indexes are valid 
pub fn callback_model<F : 'static>(mut app_data: Signal<HdlWizardApp>, updatefn : impl Fn(&mut mdf::Mdf, F) + 'static) -> EventHandler<F> {
    EventHandler::new(move |f| 
        app_data.with_mut(|data| updatefn(data.get_mut_model(), f)))
}

pub fn callback_interface<F : 'static>(mut app_data: Signal<HdlWizardApp>, updatefn : impl Fn(&mut mdf::Interface, F) + 'static) -> EventHandler<(usize, F)> {
    EventHandler::new(move |(interface_num, f)| 
        app_data.with_mut(|data| updatefn(data.get_mut_model().interfaces.get_mut(interface_num).unwrap(), f)))
}

pub fn callback_register<F : 'static>(mut app_data: Signal<HdlWizardApp>, updatefn : impl Fn(&mut mdf::Register, F) + 'static) -> EventHandler<(usize, usize, F)> {
    EventHandler::new(move |(interface_num, register_num, f)| 
        app_data.with_mut(|data| {
            let interface: &mut mdf::Interface = data.get_mut_model().interfaces.get_mut(interface_num).unwrap();
            updatefn(interface.registers.get_mut(register_num).unwrap(), f)
        }))
}

pub fn callback_field<F : 'static>(mut app_data: Signal<HdlWizardApp>, updatefn : impl Fn(&mut mdf::Field, F) + 'static) -> EventHandler<(usize, usize, usize, F)> {
    EventHandler::new(move |(interface_num, register_num, field_num, f)| 
        app_data.with_mut(|data| {
            let interface: &mut mdf::Interface = data.get_mut_model().interfaces.get_mut(interface_num).unwrap();
            let register: &mut mdf::Register = interface.registers.get_mut(register_num).unwrap();
            updatefn(register.fields.get_mut(field_num).unwrap(), f)
        }))
}

/// calls one of the update functions applying on one part of the model depending on the page type
pub fn apply_function<F : 'static>(
    mut app_data: Signal<HdlWizardApp>,
    value: F,
    undo_description: &str,
    update_model: Option<EventHandler<F>>,
    update_int: Option<EventHandler<(usize,F)>>,
    update_reg: Option<EventHandler<(usize,usize,F)>>,
    update_field: Option<EventHandler<(usize,usize,usize,F)>>,
) {
    let page_type = &app_data.read().page_type.clone();
    match page_type {
        // for a Project page type, call update_model
        PageType::Project => {
            if let Some(updatefn_ref) = update_model {
                updatefn_ref(value);
                app_data.with_mut(|app_data| { app_data.register_undo(undo_description);});
            }
        }

        // for an Interface page type, find the interface and call update_int
        PageType::Interface(interface_number) => {
            if let Some(updatefn_ref) = &update_int {
                if *interface_number < app_data.peek().data.model.interfaces.len() {
                    updatefn_ref((*interface_number,value));
                    app_data.with_mut(|app_data| { app_data.register_undo(undo_description);});
                }
            }
        }

        // for a Register page type, find the interface, the register and call update_reg or find also the field and call update_field
        PageType::Register(interface_number, register_number, field_number) => {
            if let Some(updatefn_ref) = &update_reg {
                let mut valid = false;
                if let Some(interface) =
                    app_data.peek().data.model.interfaces.get(*interface_number)
                {
                    if *register_number < interface.registers.len() {
                        valid = true;
                    }
                }
                if valid {
                    updatefn_ref((*interface_number, *register_number, value));
                    app_data.with_mut(|app_data| { app_data.register_undo(undo_description); });
                }
            } else {
                if let Some(updatefn_ref) = &update_field {
                    let mut valid = false;
                    if let Some(interface) =
                        app_data.peek().data.model.interfaces.get(*interface_number)
                    {
                        if let Some(register) = interface.registers.get(*register_number) {
                            if let Some(field_num) = field_number {
                                if *field_num < register.fields.len() {
                                    valid = true;
                                }
                            }
                        }
                    }
                    if valid {
                        updatefn_ref((*interface_number, *register_number, field_number.unwrap(),value));
                        app_data.with_mut(|app_data| { app_data.register_undo(undo_description); });
                    }
                }
            }
        },
        // preview... should never happen
        PageType::Preview => {

        }
    }
}

/// properties for a generic GUI widget
#[derive(Props, Clone, PartialEq)]
pub struct GuiGenericProps<F : Clone + PartialEq + 'static> {
    app_data: Signal<HdlWizardApp>,
    gui_label: &'static str,
    value: F,
    field_class: Option<&'static str>,
    undo_label: Option<&'static str>,
    rows: Option<u32>,
    update_model: Option<EventHandler<F>>,
    update_int: Option<EventHandler<(usize,F)>>,
    update_reg: Option<EventHandler<(usize,usize,F)>>,
    update_field: Option<EventHandler<(usize,usize,usize,F)>>,
}

/// generic text widget component, using any type that can be converted to and from a string
pub fn TextGeneric<F: gui_types::Validable + std::string::ToString + std::str::FromStr + Clone + PartialEq>(
    props: GuiGenericProps<F>,
) -> Element {
    let gui_label = props.gui_label;
    let value = props.value.to_string();
    let undo_description = props.undo_label.unwrap_or_default();
    let validate_pattern = F::validate_pattern();
    let field_class = props.field_class.unwrap_or_default();
    let app_data = props.app_data;
    let update_model = props.update_model;
    let update_int = props.update_int;
    let update_reg = props.update_reg;
    let update_field = props.update_field;

    rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "{gui_label}" } }
            div { class: "field-body",
                div { class: "field",
                    div { class: "control",
                        input {
                            class: "input {field_class}",
                            r#type: "text",
                            placeholder: "{gui_label}",
                            pattern: "{validate_pattern}",
                            onchange: move |evt| {
                                if let Ok(value) = F::from_str(&evt.value()) {
                                    apply_function(
                                        app_data,
                                        value,
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                }
                            },
                            value: "{value}"
                        }
                    }
                }
            }
        }
    }
}

/// textarea widget component, using an optional vector of strings for the value type
pub fn TextArea(props: GuiGenericProps<Option<Vec<String>>>) -> Element {
    let gui_label = props.gui_label;
    let value = utils::opt_vec_str_to_textarea(&props.value);
    let undo_description = props.undo_label.unwrap_or_default();
    let rows = props.rows.unwrap_or(4);
    let app_data = props.app_data;
    let update_model = props.update_model;
    let update_int = props.update_int;
    let update_reg = props.update_reg;
    let update_field = props.update_field;

    rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "{gui_label}" } }
            div { class: "field-body",
                div { class: "field",
                    div { class: "control",
                        textarea {
                            class: "textarea",
                            placeholder: "{gui_label}",
                            rows: "{rows}",
                            onchange: move |evt| {
                                let value = utils::textarea_to_opt_vec_str(&evt.value());
                                apply_function(
                                    app_data,
                                    value,
                                    undo_description,
                                    update_model,
                                    update_int,
                                    update_reg,
                                    update_field,
                                );
                            },
                            "{value}"
                        }
                    }
                }
            }
        }
    }
}

/// combobox widget using an enum type that uses the strum derives for conversion to and from a string
pub fn EnumWidget<F: PartialEq + Clone + strum::IntoEnumIterator + std::string::ToString + std::str::FromStr>(
    props: GuiGenericProps<F>) -> Element {
    let gui_label = props.gui_label;
    let value = props.value;
    let undo_description = props.undo_label.unwrap_or_default();

    let options = F::iter().map(|enum_value| {
        rsx!( option { selected: "{enum_value == value}", "{enum_value.to_string()}" } )
    });

    let app_data = props.app_data;
    let update_model = props.update_model;
    let update_int = props.update_int;
    let update_reg = props.update_reg;
    let update_field = props.update_field;

    rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "{gui_label}" } }
            div { class: "field-body",
                div { class: "field",
                    div { class: "control select",
                        select { onchange: move |evt| {
                                if let Ok(value) = F::from_str(&evt.value()) {
                                    apply_function(
                                        app_data,
                                        value,
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                }
                            },
                            {options}
                        }
                    }
                }
            }
        }
    }
}

/// properties for a GUI widget with an optional value (Auto/Manual)
#[derive(Props, Clone, PartialEq)]
pub struct GuiAutoManuProps<F : Clone + PartialEq + 'static> {
    app_data: Signal<HdlWizardApp>,
    gui_label: &'static str,
    field_class: Option<&'static str>,
    #[props(!optional)]
    value: Option<F>,
    placeholder: Option<String>,
    default: Option<F>,
    undo_label: Option<&'static str>,
    update_model: Option<EventHandler<Option<F>>>,
    update_int: Option<EventHandler<(usize,Option<F>)>>,
    update_reg: Option<EventHandler<(usize,usize,Option<F>)>>,
    update_field: Option<EventHandler<(usize,usize,usize,Option<F>)>>,
}

/// text widget component with an Auto option
pub fn AutoManuText<F: Default + Clone + PartialEq + gui_types::Validable + std::string::ToString + std::str::FromStr + Clone>(
    props: GuiAutoManuProps<F>) -> Element {
    let gui_label = props.gui_label;
    let default = match props.default {
        Some(default) => default.clone(),
        None => F::default(),
    };
    let value = props.value;
    let is_auto = value.is_none();
    let value = match value {
        None => String::default(),
        Some(val) => val.to_string(),
    };
    let undo_description = props.undo_label.unwrap_or_default();
    let validate_pattern = F::validate_pattern();

    let placeholder = match props.placeholder {
        None => gui_label.to_owned(),
        Some(placeholder) => placeholder,
    };

    let field_class = props.field_class.unwrap_or_default();
    let app_data = props.app_data;
    let update_model = props.update_model;
    let update_int = props.update_int;
    let update_reg = props.update_reg;
    let update_field = props.update_field;

    rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "{gui_label}" } }
            div { class: "field-body",
                div { class: "field is-grouped is-align-items-center",
                    div { class: "control",
                        label { class: "radio",
                            input {
                                r#type: "radio",
                                name: "{gui_label}",
                                onclick: move |_| {
                                    apply_function(
                                        app_data,
                                        None,
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                },
                                checked: "{is_auto}"
                            }
                            " Auto "
                        }
                        label { class: "radio",
                            input {
                                r#type: "radio",
                                name: "{gui_label}",
                                onclick: move |_| {
                                    if is_auto {
                                        apply_function(
                                            app_data,
                                            Some(default.clone()),
                                            undo_description,
                                            update_model,
                                            update_int,
                                            update_reg,
                                            update_field,
                                        );
                                    }
                                },
                                checked: "{!is_auto}"
                            }
                            " Manual: "
                        }
                    }
                    div { class: "control",
                        input {
                            class: "input {field_class}",
                            r#type: "text",
                            placeholder: "{placeholder}",
                            pattern: "{validate_pattern}",
                            onchange: move |evt| {
                                if let Ok(value) = F::from_str(&evt.value()) {
                                    apply_function(
                                        app_data,
                                        Some(value),
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                }
                            },
                            value: "{value}",
                            disabled: "{is_auto}"
                        }
                    }
                }
            }
        }
    }
}

/// properties for a combobox widget with an optional value
#[derive(Props, Clone, PartialEq)]
pub struct OptionEnumWidgetProps<F : Clone + PartialEq + 'static> {
    app_data: Signal<HdlWizardApp>,
    gui_label: &'static str,
    #[props(!optional)]
    value: Option<F>,
    undo_label: Option<&'static str>,
    field_for_none: Option<&'static str>,
    disabled: Option<bool>,
    update_model: Option<EventHandler<Option<F>>>,
    update_int: Option<EventHandler<(usize,Option<F>)>>,
    update_reg: Option<EventHandler<(usize,usize,Option<F>)>>,
    update_field: Option<EventHandler<(usize,usize,usize,Option<F>)>>,
}

/// combobox widget using an option of an enum type that uses the strum derives for conversion to and from a string.
/// field_for_none can be used to indicate which label should be used for None
pub fn OptionEnumWidget<F: PartialEq + Clone + strum::IntoEnumIterator + std::string::ToString + std::str::FromStr>(
    props: OptionEnumWidgetProps<F>) -> Element {
    let gui_label = props.gui_label;
    let value = props.value;
    let undo_description = props.undo_label.unwrap_or_default();

    let options = F::iter().map(|enum_value| {
        rsx!( option { selected: "{value == Some(enum_value)}", "{enum_value.to_string()}" } )
    });

    let disabled = props.disabled.unwrap_or(false);
    let field_for_none = props.field_for_none;
    let app_data = props.app_data;
    let update_model = props.update_model;
    let update_int = props.update_int;
    let update_reg = props.update_reg;
    let update_field = props.update_field;

    rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "{gui_label}" } }
            div { class: "field-body",
                div { class: "field",
                    div { class: "control select",
                        select {
                            onchange: move |evt| {
                                if let Ok(value) = F::from_str(&evt.value()) {
                                    apply_function(
                                        app_data,
                                        Some(value),
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                } else {
                                    apply_function(
                                        app_data,
                                        None,
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                }
                            },
                            disabled: "{disabled}",
                            {
                                if field_for_none.is_none() && value.is_none() {
                                    rsx!{
                                        option {
                                            selected: "true",
                                            "(select)"
                                        },
                                    }
                                } else {
                                    rsx!{}
                                }
                            },
                            {options},
                            {
                                if let Some(field) = field_for_none {
                                    rsx!{
                                        option {
                                            selected: "{value == None}",
                                            "{field}"
                                        }
                                    }
                                } else {
                                    rsx!{}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// properties for a checkbox
#[derive(Props, Clone, PartialEq)]
pub struct CheckBoxProps {
    app_data: Signal<HdlWizardApp>,
    gui_label: &'static str,
    checkbox_label: &'static str,
    value: bool,
    undo_label: Option<&'static str>,
    update_model: Option<EventHandler<bool>>,
    update_int: Option<EventHandler<(usize,bool)>>,
    update_reg: Option<EventHandler<(usize,usize,bool)>>,
    update_field: Option<EventHandler<(usize,usize,usize,bool)>>,
}

/// checkbox widget component, using a boolean for the value type
pub fn CheckBox(props: CheckBoxProps) -> Element {
    let gui_label = props.gui_label;
    let checkbox_label = props.checkbox_label;
    let value = props.value;
    let undo_description = props.undo_label.unwrap_or_default();

    let app_data = props.app_data;
    let update_model = props.update_model;
    let update_int = props.update_int;
    let update_reg = props.update_reg;
    let update_field = props.update_field;

    rsx! {
        div { class: "field is-horizontal",
            div { class: "field-label is-normal", label { class: "label", "{gui_label}" } }
            div { class: "field-body",
                div { class: "field",
                    div { class: "control",
                        label { class: "checkbox",
                            input {
                                r#type: "checkbox",
                                onclick: move |_| {
                                    apply_function(
                                        app_data,
                                        !value,
                                        undo_description,
                                        update_model,
                                        update_int,
                                        update_reg,
                                        update_field,
                                    );
                                },
                                checked: "{value}"
                            }
                            " {checkbox_label} "
                        }
                    }
                }
            }
        }
    }
}
