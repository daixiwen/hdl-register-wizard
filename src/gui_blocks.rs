//! building blocks for GUI
//! 

use eframe::egui;
use crate::undo;
use crate::gui_types;
use strum;

pub fn widget_auto_manual_u32(value : &mut gui_types::AutoManualU32, ui: &mut  egui::Ui, label: &str, undo : &mut undo::Undo) {

    ui.horizontal(|ui| {
        ui.label(format!("{}:",label));
        if ui.checkbox(&mut value.is_auto, "automatic").changed() {
            undo.register_modification(&format!("{} {}", label, match value.is_auto {
                true => "set to automatic",
                false => "set to manual"}), 
                undo::ModificationType::Finished);
        }
        ui.label(" or manual:");
        ui.add_enabled_ui(! value.is_auto, |ui| {
            let mut textedit = egui::TextEdit::singleline(&mut value.value_str).id_source(&label);
            if !value.str_valid {
                textedit = textedit.text_color(egui::Color32::RED);
            }
            let response = ui.add_sized([30.0, ui.available_size()[1]], textedit);
            if response.changed() {
                value.validate();
                undo.register_modification(&format!("{} change",label).to_lowercase(), undo::ModificationType::OnGoing(response.id));
            }
            if undo.lost_focus(response.id) && ! value.str_valid {
                value.str_valid = true;
                value.value_str = value.value_int.to_string();
            }
        });
    });
}

pub enum TextWidgetType {
    SingleLine, MultiLine
}

pub fn widget_text(value : &mut String, ui: &mut  egui::Ui, label: &str, widget_type : TextWidgetType, undo : &mut undo::Undo) {
    ui.horizontal(|ui| {
        ui.label(format!("{}:", label));
        let response = ui.add_sized(ui.available_size(), match widget_type {
            TextWidgetType::SingleLine => egui::TextEdit::singleline(value),
            TextWidgetType::MultiLine  => egui::TextEdit::multiline(value)
        } );

        if response.changed() {
            undo.register_modification(&format!("{} change",label).to_lowercase(), undo::ModificationType::OnGoing(response.id));
        }
    });
}

pub fn widget_combobox<S : strum::IntoEnumIterator + ToString + PartialEq + Copy>
        (value : &mut S, ui: &mut  egui::Ui, label: &str, undo : &mut undo::Undo) {
    ui.horizontal(|ui| {
        let previous_value = *value;

        ui.label(format!("{}:", label));
        egui::ComboBox::from_id_source(label)
            .selected_text(value.to_string())
            .show_ui(ui, |ui| { 
                for int_type in S::iter() {
                    ui.selectable_value(value, int_type, int_type.to_string());
                }
            });

        // egui doesn't signal a change of the combobox in the response object, so we
        // detect a change manually
        if *value != previous_value {
            undo.register_modification("changed interface type", undo::ModificationType::Finished);
        }
    });
}
