//! Types used in different parts of the GUI

use serde::{Deserialize, Serialize};
use std::str::FromStr;
use crate::utils;

#[derive(
    Serialize,
    Deserialize,
    Clone,
)]
/// value that can either be "auto" or an integer manual value
pub struct AutoManualU32 {
    /// value actually in the GUI for manual, whether it is valid or not
    pub value_str: String,
    /// if asserted, automatically caculated
    pub is_auto: bool,
    /// if asserted, value currently in the GUI is valid
    pub str_valid : bool,
    /// value as an integer. Stored seperately than the string so that we can put it back
    /// to the last known valid int value if the string is invalid
    pub value_int : u32
}

impl AutoManualU32 {
    /// create a new empty interface with type SBI
    pub fn new() -> AutoManualU32 {
        AutoManualU32 {
            value_str : String::new(),
            is_auto : true,
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
    pub edit_std : String,
    // actual stored value
    pub value : utils::VectorValue
}

impl VectorValue {
    pub fn new() -> VectorValue {
        VectorValue {
            edit_std : String::new(),
            value : utils::VectorValue::new()
        }
    }
}

impl Default for VectorValue {
    fn default() -> Self {
        VectorValue::new()
    }
}
