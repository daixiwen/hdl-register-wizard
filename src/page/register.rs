//! page to edit a register
use eframe::{egui, epi};
use crate::model_gui;
use crate::undo;
use crate::gui_blocks;
use crate::page;

pub fn panel(interface_num : usize, register_num : usize, register : &mut model_gui::Register, ctx: &egui::CtxRef, 
        _frame: &epi::Frame, undo: &mut undo::Undo) 
        -> Option<page::PageType> {

    let mut return_value = None;

    egui::CentralPanel::default().show(ctx, |mut ui| {
        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Register");

        gui_blocks::widget_text(&mut register.name, &mut ui, "Name", gui_blocks::TextWidgetType::SingleLine, undo);
    });

    return_value
}

