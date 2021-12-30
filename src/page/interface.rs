//! page to edit an interface
use eframe::{egui, epi};
use crate::mdf_format;

pub fn panel(interface : &mut mdf_format::Interface, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Interface");

        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut interface.name);
        });

        ui.separator();
    });
}
