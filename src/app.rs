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
    undo: undo::Undo,

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
        _ctx: &egui::CtxRef,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
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
            });
        });

        navigation::navigate(self, ctx, frame);

        match &self.page_type {
            page::PageType::Project => {
                page::project::panel(self, ctx, frame)
            },

            page::PageType::Interface(num_interface) => {
                match self.model.interfaces.get_mut(*num_interface) {
                    Some(interface) => {
                        page::interface::panel(interface, ctx, frame, &mut self.undo)
                    },

                    None => {
                        self.page_type = page::PageType::Project;
                        ctx.request_repaint();
                    }
                }
            }
        }
    }
}

