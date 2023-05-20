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
fn apply_function<'a, F>(
    app_data: &'a UseRef<HdlWizardApp>,
    value : F,
    undo_description : &str,
    update_model: &Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &F) -> () + 'a>>>,
    update_int: &Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &F) -> () + 'a>>>,
    _update_reg: &Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &F) -> () + 'a>>>) {

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
        _ => {}
    }

}

// properties for a generic GUI widget
#[derive(Props)]
pub struct GuiGenericProps<'a, F> {
    app_data: &'a UseRef<HdlWizardApp>,
    gui_label : &'a str,
    value : F,
    undo_label : Option<&'a str>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &F) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &F) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &F) -> () + 'a>>>,
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
                        input { class:"input", r#type:"text", placeholder:"{gui_label}", pattern:"{validate_pattern}",
                            onchange: move | evt | {
                                if let Ok(value) = F::from_str(&evt.value) {
                                    apply_function(&cx.props.app_data, value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg);
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
                            onchange: move | evt | {
                                let value = utils::textarea_to_opt_vec_str(&evt.value);
                                apply_function(&cx.props.app_data, value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg);
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
                                    apply_function(&cx.props.app_data, value, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg);
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
pub struct GuiAutoManuProps<'a, F :'a> {
    app_data: &'a UseRef<HdlWizardApp>,
    gui_label : &'a str,
    #[props(!optional)]
    value : Option<F>,
    default : Option<F>,
    undo_label : Option<&'a str>,
    update_model: Option<RefCell<Box<dyn FnMut(&mut mdf::Mdf, &Option<F>) -> () + 'a>>>,
    update_int: Option<RefCell<Box<dyn FnMut(&mut mdf::Interface, &Option<F>) -> () + 'a>>>,
    update_reg: Option<RefCell<Box<dyn FnMut(&mut mdf::Register, &Option<F>) -> () + 'a>>>,
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
                                onclick : move | _ | {apply_function(&cx.props.app_data, None, undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg);},
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
                                        apply_function(&cx.props.app_data, Some(default.clone()), undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg);
                                    }
                                },
                                checked: "{!is_auto}",
                            }
                            " Manual: ",
                        }
                    },
                    div { class:"control",
                        input { class:"input", r#type:"text", placeholder:"{gui_label}", pattern:"{validate_pattern}",
                            onchange: move | evt | {
                                if let Ok(value) = F::from_str(&evt.value) {
                                    apply_function(&cx.props.app_data, Some(value), undo_description, &cx.props.update_model, &cx.props.update_int, &cx.props.update_reg);
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

/*
pub fn widget_u32_inline_nolabel(
    value: &mut gui_types::GuiU32,
    ui: &mut egui::Ui,
    id: &str,
    label: &str,
    undo: &mut undo::Undo,
) {
    let mut textedit = egui::TextEdit::singleline(&mut value.value_str).id_source(&id);
    if !value.str_valid {
        textedit = textedit.text_color(egui::Color32::RED);
    }
    let response = ui.add_sized([30.0, ui.available_size()[1]], textedit);
    if response.changed() {
        value.validate();
        undo.register_modification(
            &format!("{} change", label).to_lowercase(),
            undo::ModificationType::OnGoing(response.id),
        );
    }
    if undo.lost_focus(response.id) && !value.str_valid {
        value.str_valid = true;
        value.value_str = value.value_int.to_string();
    }
}

pub fn widget_auto_manual_u32_inline(
    value: &mut gui_types::AutoManualU32,
    ui: &mut egui::Ui,
    label: &str,
    forcemanual: bool,
    undo: &mut undo::Undo,
) {
    ui.label(format!("{}:", label));
    if forcemanual {
        value.is_auto = false;
    } else {
        if ui.checkbox(&mut value.is_auto, "automatic").changed() {
            undo.register_modification(
                &format!(
                    "{} {}",
                    label,
                    match value.is_auto {
                        true => "set to automatic",
                        false => "set to manual",
                    }
                ),
                undo::ModificationType::Finished,
            );
        }
        ui.label(" or manual:");
    }

    ui.add_enabled_ui(!value.is_auto, |ui| {
        widget_u32_inline_nolabel(&mut value.manual, ui, label, label, undo);
    });
}

pub fn widget_auto_manual_u32(
    value: &mut gui_types::AutoManualU32,
    ui: &mut egui::Ui,
    label: &str,
    forcemanual: bool,
    undo: &mut undo::Undo,
) {
    ui.horizontal(|ui| {
        widget_auto_manual_u32_inline(value, ui, label, forcemanual, undo);
    });
}

pub enum TextWidgetType {
    SingleLine,
    MultiLine,
}

pub fn widget_text(
    value: &mut String,
    ui: &mut egui::Ui,
    label: &str,
    widget_type: TextWidgetType,
    undo: &mut undo::Undo,
) {
    ui.horizontal(|ui| {
        ui.label(format!("{}:", label));
        let response = ui.add_sized(
            ui.available_size(),
            match widget_type {
                TextWidgetType::SingleLine => egui::TextEdit::singleline(value),
                TextWidgetType::MultiLine => egui::TextEdit::multiline(value),
            },
        );

        if response.changed() {
            undo.register_modification(
                &format!("{} change", label).to_lowercase(),
                undo::ModificationType::OnGoing(response.id),
            );
        }
    });
}

pub fn widget_combobox_inline<S: strum::IntoEnumIterator + ToString + PartialEq + Copy>(
    value: &mut S,
    ui: &mut egui::Ui,
    label: &str,
    override_id: Option<&str>,
    disabled_option: Option<S>,
    undo: &mut undo::Undo,
) {
    let previous_value = *value;
    let id = match override_id {
        Some(new_id) => new_id,
        None => label,
    };

    ui.label(format!("{}:", label));
    egui::ComboBox::from_id_source(id)
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            for int_type in S::iter() {
                ui.add_enabled_ui(Some(int_type) != disabled_option, |ui| {
                    ui.selectable_value(value, int_type, int_type.to_string());
                });
            }
        });

    // egui doesn't signal a change of the combobox in the response object, so we
    // detect a change manually
    if *value != previous_value {
        undo.register_modification(
            &format!("changed {}", label.to_lowercase()),
            undo::ModificationType::Finished,
        );
    }
}

pub fn widget_combobox<S: strum::IntoEnumIterator + ToString + PartialEq + Copy>(
    value: &mut S,
    ui: &mut egui::Ui,
    label: &str,
    disabled_option: Option<S>,
    undo: &mut undo::Undo,
) {
    ui.horizontal(|ui| {
        widget_combobox_inline(value, ui, label, None, disabled_option, undo);
    });
}

pub fn widget_vectorvalue_inline(
    entry: &mut gui_types::VectorValue,
    ui: &mut egui::Ui,
    label: &str,
    override_id: Option<&str>,
    undo: &mut undo::Undo,
) {
    let id = match override_id {
        Some(new_id) => new_id,
        None => label,
    };

    ui.label(format!("{}:", label));
    let mut textedit = egui::TextEdit::singleline(&mut entry.value_str).id_source(&id);
    if !entry.str_valid {
        textedit = textedit.text_color(egui::Color32::RED);
    }
    let response = ui.add_sized([100.0, ui.available_size()[1]], textedit);
    if response.changed() {
        match utils::VectorValue::from_str(&entry.value_str) {
            Ok(new_value) => {
                entry.value = new_value;
                entry.str_valid = true;
            }
            Err(_) => {
                entry.str_valid = false;
            }
        }
        undo.register_modification(
            &format!("{} change", label).to_lowercase(),
            undo::ModificationType::OnGoing(response.id),
        );
    }
    if undo.lost_focus(response.id) && !entry.str_valid {
        entry.str_valid = true;
        entry.value_str = entry.value.to_string();
    }
}

pub fn widget_vectorvalue(
    entry: &mut gui_types::VectorValue,
    ui: &mut egui::Ui,
    label: &str,
    undo: &mut undo::Undo,
) {
    ui.horizontal(|ui| {
        widget_vectorvalue_inline(entry, ui, label, None, undo);
    });
}

pub fn widget_auto_manual_vectorvalue_inline(
    value: &mut gui_types::AutoManualVectorValue,
    ui: &mut egui::Ui,
    label: &str,
    undo: &mut undo::Undo,
) {
    ui.label(format!("{}:", label));
    if ui.checkbox(&mut value.is_auto, "auto").changed() {
        undo.register_modification(
            &format!(
                "{} {}",
                label,
                match value.is_auto {
                    true => "set to automatic",
                    false => "set to manual",
                }
            ),
            undo::ModificationType::Finished,
        );
    }

    ui.add_enabled_ui(!value.is_auto, |ui| {
        widget_vectorvalue_inline(&mut value.manual, ui, " or manual:", None, undo);
    });
}

// custom widget representing a register with bitfields
pub fn widget_bitfield(ui: &mut egui::Ui, bitfield: &str) {
    // Deciding widget size:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(bitfield.len() as f32, 1.0);

    // Allocating space:
    let (rect, mut _response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    //   let (_id, rect) = ui.allocate_space(desired_size);

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {
        //        let inactive_color = ui.style().visuals.widgets.inactive.bg_fill;
        let empty_color = ui.style().visuals.extreme_bg_color;
        let used_color = ui.style().visuals.widgets.inactive.bg_fill;
        let hovered_color = ui.style().visuals.widgets.inactive.fg_stroke.color;
        let error_color = egui::Color32::RED;
        let grid_stroke = ui.style().visuals.widgets.noninteractive.fg_stroke;

        let mut single_bit_rec = rect;
        single_bit_rec.set_width(single_bit_rec.height());
        for (i, c) in bitfield.chars().enumerate() {
            ui.painter().rect(
                single_bit_rec.shrink(2.0),
                2.0,
                match c {
                    'e' => empty_color,
                    'u' => used_color,
                    'h' => hovered_color,
                    _ => error_color,
                },
                ui.style().visuals.widgets.inactive.bg_stroke,
            );

            if i != 0 {
                let bitnum = bitfield.len() - i;
                if bitnum != 0 {
                    if bitnum % 8 == 0 {
                        ui.painter().line_segment(
                            [single_bit_rec.left_center(), single_bit_rec.left_bottom()],
                            grid_stroke,
                        );
                    } else if bitnum % 4 == 0 {
                        ui.painter().line_segment(
                            [
                                single_bit_rec.left_bottom() + egui::vec2(0.0, -4.0),
                                single_bit_rec.left_bottom(),
                            ],
                            grid_stroke,
                        );
                    }
                }
            }
            *single_bit_rec.right_mut() += rect.height();
            *single_bit_rec.left_mut() += rect.height();
        }
    }
}

pub fn dark_light_mode_switch(ui: &mut egui::Ui, ctx: &egui::CtxRef, setting: &mut Option<bool>) {
    if ctx.style().visuals.dark_mode {
        if ui
            .add(egui::Button::new("☀").frame(false))
            .on_hover_text("Switch to light mode")
            .clicked()
        {
            *setting = Some(false);
            utils::set_theme(ctx, setting);
        }
    } else if ui
        .add(egui::Button::new("🌙").frame(false))
        .on_hover_text("Switch to dark mode")
        .clicked()
    {
        *setting = Some(true);
        utils::set_theme(ctx, setting);
    }
}
*/