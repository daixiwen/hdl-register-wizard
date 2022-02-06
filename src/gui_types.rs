//! Types used in different parts of the GUI

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::utils;

/// egui doesn't have a way of controlling an integer value from a text edit
/// so we have a type that regroups both a string and the integer value
#[derive(
    Serialize,
    Deserialize,
    Clone,
)]
/// integer value
pub struct GuiU32 {
    /// value actually in the GUI, whether it is valid or not
    pub value_str: String,
    /// if asserted, value currently in the GUI is valid
    pub str_valid : bool,
    /// value as an integer. Stored seperately than the string so that we can put it back
    /// to the last known valid int value if the string is invalid
    pub value_int : u32
}

impl GuiU32 {
    pub fn new() -> GuiU32 {
        GuiU32 {
            value_str : String::new(),
            str_valid : false,
            value_int : 0
        }
    }

    /// validate the string when it is changed
    pub fn validate(&mut self) {
        match u32::from_str(& self.value_str) {
            Ok(value) => {
                self.str_valid = true;
                self.value_int = value;
            },
            Err(_) => {
                self.str_valid = false;
            }
        }
    }
    
}

impl Default for GuiU32 {
    fn default() -> Self {
        GuiU32::new()
    }
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
)]
/// value that can either be "auto" or an integer manual value
pub struct AutoManualU32 {
    /// if asserted, automatically caculated
    pub is_auto: bool,
    /// manual value
    pub manual : GuiU32
}

impl AutoManualU32 {
    /// create a new empty interface with type SBI
    pub fn new() -> AutoManualU32 {
        AutoManualU32 {
            is_auto : true,
            manual  : GuiU32::new()
        }
    }
}

impl Default for AutoManualU32 {
    fn default() -> Self {
        AutoManualU32::new()
    }
}

#[derive(Serialize, Deserialize, Clone)]
// GUI variant of utils::VectorValue, that combines the value with a
// string to temporarily store the value edited in the GUI
pub struct VectorValue {
    // GUI edit value
    pub value_str : String,
    // actual stored value
    pub value : utils::VectorValue,
    /// if asserted, value currently in the GUI is valid
    pub str_valid : bool,
}

impl VectorValue {
    pub fn new() -> VectorValue {
        VectorValue {
            value_str : String::new(),
            value : utils::VectorValue::new(),
            str_valid : false
        }
    }
}

impl Default for VectorValue {
    fn default() -> Self {
        VectorValue::new()
    }
}

#[derive(
    Serialize,
    Deserialize,
    Clone,
)]
/// value that can either be "auto" or a manual vector value
pub struct AutoManualVectorValue {
    /// if asserted, automatically caculated
    pub is_auto: bool,
    /// manual value
    pub manual : VectorValue
}

impl AutoManualVectorValue {
    /// create a new empty interface with type SBI
    pub fn new() -> AutoManualVectorValue {
        AutoManualVectorValue {
            is_auto : true,
            manual  : VectorValue::new()
        }
    }
}

impl Default for AutoManualVectorValue {
    fn default() -> Self {
        AutoManualVectorValue::new()
    }
}
