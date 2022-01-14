//! page to edit an interface
use eframe::{egui, epi};
use crate::model_gui;
use crate::undo;
use strum::IntoEnumIterator;

pub fn panel(interface : &mut model_gui::InterfaceGUI, ctx: &egui::CtxRef, _frame: &epi::Frame, undo: &mut undo::Undo) {
    egui::CentralPanel::default().show(ctx, |mut ui| {
        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Interface");

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut interface.name));
        });

        ui.horizontal(|ui| {
            ui.label("Description:");
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut interface.description));
        });

        ui.horizontal(|ui| {
            ui.label("Interface Type:");
            egui::ComboBox::from_id_source("interface type")
                .selected_text(interface.interface_type.to_string())
                .show_ui(ui, |ui| { 
                    for int_type in model_gui::InterfaceType::iter() {
                        ui.selectable_value(&mut interface.interface_type, int_type, int_type.to_string());
                    }
                });
        });

        widget_auto_manual_u32(&mut interface.address_width, &mut ui, "Address width:", undo);
        widget_auto_manual_u32(&mut interface.data_width, &mut ui, "Data width:", undo);
        ui.separator();
    });
}

pub fn widget_auto_manual_u32(value : &mut model_gui::AutoManualU32, ui: &mut  egui::Ui, label: &str, undo : &mut undo::Undo) {

    ui.horizontal(|ui| {
        ui.label(label);
        ui.checkbox(&mut value.is_auto, "automatic");
        ui.label(" or manual:");
        ui.add_enabled_ui(! value.is_auto, |ui| {
            let mut textedit = egui::TextEdit::singleline(&mut value.value_str).id_source(&label);
            if !value.str_valid {
                textedit = textedit.text_color(egui::Color32::RED);
            }
            let response = ui.add_sized([30.0, ui.available_size()[1]], textedit);
            if response.changed() {
                value.validate();
            }
            if undo.lost_focus(response.id) {
                println!("last focus in {:?}", label);
                if ! value.str_valid {
                    value.str_valid = true;
                    value.value_str = value.value_int.to_string();
                }
            }
        });
    });
}
