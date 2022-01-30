use eframe::{egui, epi};
use crate::model_gui;
use crate::page;
use crate::navigation;
use crate::undo;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct HdlWizardApp {
    // Example stuff:
    pub model: model_gui::MdfGui,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub page_type: page::PageType,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub undo: undo::Undo,

}

impl Default for HdlWizardApp {
    fn default() -> Self {
        Self {
            model: Default::default(),
            page_type : page::PageType::Project,
            undo: Default::default()
        }
    }
}

impl epi::App for HdlWizardApp {
    fn name(&self) -> &str {
        "HDL Register Wizard"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }

        self.undo.register_modification("initial", undo::ModificationType::Finished);
        self.undo.store_undo(&self.model, &self.page_type);

        // install font and increase text size compared to default
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("dejavu".to_owned(),
            egui::FontData::from_static(include_bytes!("files/DejaVuSans.ttf"))); // .ttf and .otf supported
        fonts.fonts_for_family.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "dejavu".to_owned());

        fonts.family_and_size.insert(egui::TextStyle::Small,   (egui::FontFamily::Proportional, 12.0));
        fonts.family_and_size.insert(egui::TextStyle::Body,    (egui::FontFamily::Proportional, 16.0));
        fonts.family_and_size.insert(egui::TextStyle::Button,  (egui::FontFamily::Proportional, 16.0));
        fonts.family_and_size.insert(egui::TextStyle::Heading, (egui::FontFamily::Proportional, 22.0));
        ctx.set_fonts(fonts);

        // increase button size
        let mut style = egui::Style::default();
        style.spacing.button_padding = egui::vec2(6.0, 4.0);
        style.spacing.interact_size = egui::vec2(48.0, 24.0);
        ctx.set_style(style);
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        self.undo.update_focus(ctx.memory().focus());

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                ui.menu_button("Edit", |ui| {
                    match self.undo.get_undo_description() {
                        None => {
                            ui.add_enabled_ui(false, | ui | {
                                if ui.button("Undo").clicked() {
                                    unreachable!();
                                }
                            }); 
                        },

                        Some(change) => {
                            if ui.button(format!("Undo {}", change)).clicked() {
                                if let Some(undo_state) = self.undo.apply_undo() {
                                    self.model = undo_state.model;
                                    self.page_type = undo_state.page_type;
                                }
                            }    
                        }
                    }

                    match self.undo.get_redo_description() {
                        None => {
                            ui.add_enabled_ui(false, | ui | {
                                if ui.button("Redo").clicked() {
                                    unreachable!();
                                }
                            }); 
                        },

                        Some(change) => {
                            if ui.button(format!("Redo {}", change)).clicked() {
                                if let Some(redo_state) = self.undo.apply_redo() {
                                    self.model = redo_state.model;
                                    self.page_type = redo_state.page_type;
                                }
                            }    
                        }
                    }
                });
            });
        });

        navigation::navigate(self, ctx, frame);

        let mut change_page = None;

        match &self.page_type {
            page::PageType::Project => {
                page::project::panel(self, ctx, frame)
            },

            page::PageType::Interface(num_interface) => {
                if let Some(interface) = self.model.interfaces.get_mut(*num_interface) {
                    // display the interface page and update the displayed page if requested
                    change_page = page::interface::panel(*num_interface, interface, ctx, frame, &mut self.undo);
                } else {
                    change_page = Some(page::PageType::Project);
                }
            }

            page::PageType::Register(num_interface, num_register) => {
                if let Some(interface) = self.model.interfaces.get_mut(*num_interface) {
                    if let Some(register) = interface.registers.get_mut(*num_register) {
                        page::register::panel(register, &interface.data_width, ctx, frame, &mut self.undo);
                    } else {
                        change_page = Some(page::PageType::Interface(*num_interface));
                    }
                } else {
                    change_page = Some(page::PageType::Project);
                }
            }
        }

        if let Some(newpage) = change_page {
            self.page_type = newpage;
            ctx.request_repaint();
        }

        self.undo.store_undo(&self.model, &self.page_type);
    }
}

