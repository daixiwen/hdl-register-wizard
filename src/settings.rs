//! Model description file structures module

use serde::{Deserialize, Serialize};
use std::default::Default;

#[derive(Serialize, Deserialize)]
/// model description file. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct Settings {
    /// dark mode
    pub dark_mode: Option<bool>,
    /// undo level
    pub undo_level: u32,
}

impl Default for Settings {
    /// create an empty model
    fn default() -> Settings {
        Settings {
            dark_mode: None,
            undo_level: 10,
        }
    }
}
