//! Model description file structures module

use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize)]
/// model description file. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct Settings {
    /// undo level
    pub undo_level: u32,
    /// multi window behaviour
    pub multi_window: MultiWindow,
}

impl Default for Settings {
    /// create an empty model
    fn default() -> Settings {
        Settings {
            undo_level: 10,
            multi_window: MultiWindow::Independent,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
/// how the app should behave when multiple windows are opened
pub enum MultiWindow {
    /// independent views of different models
    Independent,
    /// mulitple views of the same file
    Views
}
