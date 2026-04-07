//! Application settings

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::default::Default;

#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
/// model description file. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct Settings {
    /// dark mode
    pub dark_mode: Option<bool>,
    /// undo level
    pub undo_level: u32,
    /// user templates
    pub user_templates: BTreeMap<String, String>,
}

impl Default for Settings {
    /// create an empty model
    fn default() -> Settings {
        Settings {
            dark_mode: None,
            undo_level: 10,
            user_templates: Default::default(),
        }
    }
}
