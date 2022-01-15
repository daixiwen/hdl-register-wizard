//! main page for the project
use eframe::{egui, epi};
use crate::app::HdlWizardApp;
use crate::page;
use crate::gui_blocks;
use crate::undo;

pub fn panel(app : &mut HdlWizardApp, ctx: &egui::CtxRef, _frame: &epi::Frame) {
    egui::CentralPanel::default().show(ctx, |mut ui| {
//        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Hdl Register Wizard Project");

        ui.add_space(10.0);
        
        gui_blocks::widget_text(&mut app.model.name, &mut ui, "Project Name", gui_blocks::TextWidgetType::SingleLine, &mut app.undo);

        ui.separator();

        ui.horizontal(|ui| {
            ui.heading("Interfaces:");
            if ui.button("New").clicked() {
                app.model.interfaces.push(Default::default());
                app.page_type = page::PageType::Interface(app.model.interfaces.len()-1);
                app.undo.register_modification("create new interface", undo::ModificationType::Finished);
            }
        });

        let num_interfaces = app.model.interfaces.len();
        let interface_names : Vec<String> = app.model.interfaces.iter().map(|int| int.name.clone()).collect();

        ui.add_space(5.0);
        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, | ui | {
            egui::Grid::new("interfaces_grid")
                .striped(true)
                .spacing([30.0, 5.0])
                .show(ui, |ui| {
                    for (n, interface_name) in interface_names.iter().enumerate() {
                        let interface_page_type = page::PageType::Interface(n);
                        if ui.selectable_label(false, interface_name).clicked() {
                            app.page_type = interface_page_type;
                        }
                        ui.horizontal(|ui| {
                            if ui.button("🗑").clicked() {
                                app.model.interfaces.remove(n);
                                app.undo.register_modification("delete interface", undo::ModificationType::Finished);
                            }
                            ui.add_enabled_ui(n > 0, |ui| {
                                if ui.button("⬆").clicked() {
                                    app.model.interfaces.swap(n-1,n);
                                    app.undo.register_modification("move interface", undo::ModificationType::Finished);
                                }
                            });
                            ui.add_enabled_ui(n < (num_interfaces - 1), |ui| {
                                if ui.button("⬇").clicked() {
                                    app.model.interfaces.swap(n,n+1);
                                    app.undo.register_modification("move interface", undo::ModificationType::Finished);
                                }
                            });
                        });
                        ui.end_row();
                    }
                });
        });
    });
}
