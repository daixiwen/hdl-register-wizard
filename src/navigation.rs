//! main page for the project
use eframe::{egui, epi};
use crate::app::HdlWizardApp;
use crate::page;

pub fn navigate(app : &mut HdlWizardApp, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.add_space(5.0);
        ui.heading("Navigation");

        ui.add_space(10.0);

        egui::ScrollArea::vertical().auto_shrink([false; 2]).show(ui, | ui | {
            ui.vertical(| ui | {
                if ui.selectable_label(app.page_type == page::PageType::Project, &app.model.name).clicked() {
                    app.page_type = page::PageType::Project;
                }
                for (n, interface) in app.model.interfaces.iter().enumerate() {
                    let interface_page_type = page::PageType::Interface(n);
                    
                    if ui.selectable_label(
                        app.page_type == interface_page_type, 
                        format!("  {}", &interface.name)).clicked() {
                            app.page_type = interface_page_type;
                    }
                }
            });
        });
    });
}
