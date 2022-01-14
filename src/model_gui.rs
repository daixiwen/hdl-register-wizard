//! Model description structure for GUI
//! This is a separate scructure than the one used for (de)serialization, because some parameters in the MDF JSON file
//! are not in a form easy to process for the GUI (for example multi line texts), and with an immediate GUI it is better
//! to avoid gaving to convert data at each frame. There could have been only the GUI model structure with custom
//! implemantations for Serialize and Deserialize, but while a Serialize custom implementation is easy to write, a custom
//! deserialization function is more difficult and not very clean. Besides having different structures for GUI and 
//! (de)serialization will enable more easily support for several versions of the file format in the future or even
//! different file formats
//! 

use serde::{Deserialize, Serialize};
use strum_macros;
use std::default::Default;
/// temporaty include until I've redefined averything
use crate::mdf_format;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone)]
/// model description file. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct MdfGui {
    /// file name
    pub name: String,
    /// list of interfaces
    pub interfaces: Vec<InterfaceGUI>,
}

impl Default for MdfGui {
    /// create an empty model
    fn default() -> MdfGui {
        MdfGui {
            name: String::new(),
            interfaces: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
/// structure representing an interface in the model
pub struct InterfaceGUI {
    /// interface name
    pub name: String,
    /// description for the interface
    pub description: String,
    /// interface type (protocol used)
    pub interface_type: InterfaceType,
    /// width of the address bus.
    pub address_width : AutoManualU32,
    /// width of the data bus.
    pub data_width: AutoManualU32,
    /// list of registers
    pub registers: Vec<mdf_format::Register>,
}

impl InterfaceGUI {
    /// create a new empty interface with type SBI
    pub fn new() -> InterfaceGUI {
        InterfaceGUI {
            name: String::new(),
            description: String::new(),
            interface_type: InterfaceType::SBI,
            registers: Vec::<mdf_format::Register>::new(),
            address_width: AutoManualU32::new(),
            data_width: AutoManualU32::new()
        }
    }
}

impl Default for InterfaceGUI {
    fn default() -> Self {
        InterfaceGUI::new()
    }
}

#[derive(
    Serialize,
    Deserialize,
    strum_macros::ToString,
    strum_macros::EnumIter,
    strum_macros::EnumString,
    PartialEq,
    Clone,
    Copy
)]
/// type of interface. Only SBI is officially spported by the Bitvis tool RegisterWizard
pub enum InterfaceType {
    /// SBI protocol, defined by Bitvis
    SBI,
    /// APB3 protocol, used in ARM systems among others
    APB3,
    /// Avalon Memory mapped interface, used in Altera/Intel FPGA designs
    AvalonMm,
}

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

