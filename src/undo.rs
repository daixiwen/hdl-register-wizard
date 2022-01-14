//! Undo functionality

use eframe::egui;
use std::default;

pub struct Undo {
    current_focus : Option<egui::Id>,
    previous_focus : Option<egui::Id>
}

impl default::Default for Undo {
    /// create an empty model
    fn default() -> Undo {
        Undo {
            current_focus: None,
            previous_focus: None,
        }
    }
}

impl Undo {
    pub fn update_focus(&mut self, focus : Option<egui::Id>) {
        self.previous_focus = self.current_focus;
        self.current_focus = focus;
    }

    pub fn lost_focus(&self, object : egui::Id) -> bool {
        (Some(object) == self.previous_focus) && (Some(object) != self.current_focus)
    }
}
