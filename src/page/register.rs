//! page to edit a register
use eframe::{egui, epi};
use crate::model_gui;
use crate::undo;
use crate::gui_blocks;
use crate::gui_types;

enum FieldsModification {
    Delete(usize),
    Swap(usize, usize),
}

fn absdiff(a: u32, b:u32) -> u32 {
    if a > b {
        a-b
    } else {
        b-a
    }
}

pub fn panel(register : &mut model_gui::Register, interface_data_width: &gui_types::AutoManualU32, ctx: &egui::CtxRef, 
        _frame: &epi::Frame, undo: &mut undo::Undo) {

    egui::CentralPanel::default().show(ctx, |mut ui| {
        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Register");

        gui_blocks::widget_text(&mut register.name, &mut ui, "Name", gui_blocks::TextWidgetType::SingleLine, undo);
        gui_blocks::widget_text(&mut register.summary, &mut ui, "Summary", gui_blocks::TextWidgetType::MultiLine, undo);
        gui_blocks::widget_text(&mut register.description, &mut ui, "Description", gui_blocks::TextWidgetType::MultiLine, undo);

        ui.horizontal(|mut ui| {

            gui_blocks::widget_combobox(&mut register.address_type, &mut ui, "Address", None, undo);

            match register.address_type {
                model_gui::AddressType::Auto => (),

                model_gui::AddressType::Single => {
                    gui_blocks::widget_vectorvalue(&mut register.address_value, &mut ui, "Value", undo);
                }
                model_gui::AddressType::Stride => {
                    gui_blocks::widget_vectorvalue(&mut register.address_value, &mut ui, "First", undo);
                    gui_blocks::widget_vectorvalue(&mut register.address_count, &mut ui, "Count", undo);
                    gui_blocks::widget_vectorvalue(&mut register.address_incr, &mut ui, "Increment", undo);
                }
            }

        });
        ui.horizontal(|mut ui| {

            gui_blocks::widget_auto_manual_u32_inline(&mut register.width, &mut ui, "Width", register.fields.is_empty(), undo);

            gui_blocks::widget_combobox_inline(&mut register.access, &mut ui, "Access", None, match register.fields.is_empty() {
                true => Some(model_gui::AccessType::PerField),
                false => None}, undo);
            gui_blocks::widget_combobox_inline(&mut register.location, &mut ui, "Location", None, match register.fields.is_empty() {
                true => Some(model_gui::LocationType::PerField),
                false => None}, undo);
        });

        if register.location == model_gui::LocationType::Core {
            ui.horizontal(|mut ui| {
                let disabled_option = match register.fields.is_empty() {
                    true => Some(model_gui::CoreSignalProperty::PerField),
                    false => None};
                gui_blocks::widget_combobox_inline(&mut register.core_use_read_enable, &mut ui, "use read enable", None, disabled_option, undo);
                gui_blocks::widget_combobox_inline(&mut register.core_use_write_enable, &mut ui, "use write enable", None, disabled_option, undo);
            });
        }

        if register.fields.is_empty() {
            ui.horizontal(|mut ui| {
                gui_blocks::widget_combobox_inline(&mut register.signal_type, &mut ui, "Signal type", None, None, undo);
                gui_blocks::widget_vectorvalue_inline(&mut register.reset, &mut ui, "reset value", None, undo);
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
                // find highest bit to put the new field over it 
                let mut new_bit = 0;
                for field in &register.fields {
                    new_bit = u32::max(new_bit, u32::max(field.position_start.value_int, field.position_end.value_int) + 1);
                }
                let position = gui_types::GuiU32 {
                    value_str : new_bit.to_string(),
                    str_valid : true,
                    value_int : new_bit
                };

                register.fields.push(model_gui::Field {
                    position_start : position.clone(),
                    position_end : position,
                    ..Default::default()
                });
                undo.register_modification("create new field", undo::ModificationType::Finished);
            }
        });

        if ! register.fields.is_empty() {
            let mut action : Option<FieldsModification> = None;
            let mut hovered_field : Option<usize> = None;
            let num_fields = register.fields.len();
            let can_access_as_register = register.access != model_gui::AccessType::PerField;
            let can_location_as_register = register.location != model_gui::LocationType::PerField;
            let register_location_core = register.location == model_gui::LocationType::Core;
            let can_use_re_as_register = register_location_core && register.core_use_read_enable != model_gui::CoreSignalProperty::PerField;
            let can_use_we_as_register = register_location_core && register.core_use_write_enable != model_gui::CoreSignalProperty::PerField;

            ui.add_space(5.0);
            egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, | ui | {
                for (n, field) in register.fields.iter_mut().enumerate() {
                    let field_inner_response = ui.vertical(|mut ui| {
                        ui.separator();

                        ui.horizontal(|ui| {
                            ui.label("from bit ");
                            gui_blocks::widget_u32_inline_nolabel(&mut field.position_start, ui, &format!("bit start {}", n), "field bit start", undo);
                            ui.label("to bit ");
                            gui_blocks::widget_u32_inline_nolabel(&mut field.position_end, ui, &format!("bit stop {}", n), "field bit start", undo);

                            let size_text = match absdiff(field.position_end.value_int, field.position_start.value_int) {
                                0 => "(1 bit)".to_string(),
                                n => format!("({} bits)", n+1)
                            };
                            ui.label(size_text);

                            if ui.button("🗑").clicked() {
                                action = Some(FieldsModification::Delete(n));
                                undo.register_modification("delete field", undo::ModificationType::Finished);
                            }
                            ui.add_enabled_ui(n > 0, |ui| {
                                if ui.button("⬆").clicked() {
                                    action = Some(FieldsModification::Swap(n-1,n));
                                    undo.register_modification("move field", undo::ModificationType::Finished);
                                }
                            });
                            ui.add_enabled_ui(n < (num_fields - 1), |ui| {
                                if ui.button("⬇").clicked() {
                                    action = Some(FieldsModification::Swap(n,n+1));
                                    undo.register_modification("move field", undo::ModificationType::Finished);
                                }
                            });

                        });

                        gui_blocks::widget_text(&mut field.name, &mut ui, "Name", gui_blocks::TextWidgetType::SingleLine, undo);

                        ui.horizontal(|mut ui| {
                            gui_blocks::widget_combobox_inline(&mut field.access, &mut ui, "Access", Some(&format!("field access{}",n)),
                                match can_access_as_register {
                                    false => Some(model_gui::AccessTypeField::AsRegister),
                                    true => None}, undo);
                            gui_blocks::widget_combobox_inline(&mut field.location, &mut ui, "Location", Some(&format!("field location{}",n)),
                                match can_location_as_register {
                                    false => Some(model_gui::LocationTypeField::AsRegister),
                                    true => None}, undo);

                            if (register_location_core && field.location == model_gui::LocationTypeField::AsRegister) || field.location == model_gui::LocationTypeField::Core {
                                gui_blocks::widget_combobox_inline(&mut field.core_use_read_enable, &mut ui, "use read enable", 
                                    Some(&format!("core use re field {}",n)),
                                    match can_use_re_as_register {
                                        false => Some(model_gui::CoreSignalPropertyField::AsRegister),
                                        true => None}, undo);
                                gui_blocks::widget_combobox_inline(&mut field.core_use_write_enable, &mut ui, "use write enable",
                                    Some(&format!("core use we field {}",n)),
                                    match can_use_we_as_register {
                                        false => Some(model_gui::CoreSignalPropertyField::AsRegister),
                                        true => None}, undo);
                            }
                        });
                        ui.horizontal(|mut ui| {
                            gui_blocks::widget_combobox_inline(&mut field.signal_type, &mut ui, "Signal type", 
                                Some(&format!("field signal type {}",n)), None, undo);
                            gui_blocks::widget_vectorvalue_inline(&mut field.reset, &mut ui, "reset value", 
                                Some(&format!("field signal reset value {}",n)), undo);
                        });
                        gui_blocks::widget_text(&mut field.description, &mut ui, "Description", gui_blocks::TextWidgetType::MultiLine, undo);
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
                },
                Some(FieldsModification::Swap(a,b)) => {
                    register.fields.swap(a,b);
                },
                None => ()
            }
        }

    });

}

