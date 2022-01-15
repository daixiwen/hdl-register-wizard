//! page to edit an interface
use eframe::{egui, epi};
use crate::model_gui;
use crate::undo;
use crate::gui_blocks;

pub fn panel(interface : &mut model_gui::InterfaceGUI, ctx: &egui::CtxRef, _frame: &epi::Frame, undo: &mut undo::Undo) {
    egui::CentralPanel::default().show(ctx, |mut ui| {
        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Interface");

        gui_blocks::widget_text(&mut interface.name, &mut ui, "Name", gui_blocks::TextWidgetType::SingleLine, undo);
        gui_blocks::widget_text(&mut interface.description, &mut ui, "Description", gui_blocks::TextWidgetType::MultiLine, undo);

        gui_blocks::widget_combobox(&mut interface.interface_type, &mut ui, "Interface Type", undo);

        gui_blocks::widget_auto_manual_u32(&mut interface.address_width, &mut ui, "Address width", undo);
        gui_blocks::widget_auto_manual_u32(&mut interface.data_width, &mut ui, "Data width", undo);
        ui.separator();
    });
}
