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
use crate::gui_types;
use crate::utils;

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
    pub address_width : gui_types::AutoManualU32,
    /// width of the data bus.
    pub data_width: gui_types::AutoManualU32,
    /// list of registers
    pub registers: Vec<Register>,
}

impl InterfaceGUI {
    /// create a new empty interface with type SBI
    pub fn new() -> InterfaceGUI {
        InterfaceGUI {
            name: String::new(),
            description: String::new(),
            interface_type: InterfaceType::SBI,
            registers: Vec::new(),
            address_width: gui_types::AutoManualU32::new(),
            data_width: gui_types::AutoManualU32::new()
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

#[derive(Serialize, Deserialize, Clone)]
/// structure representing an register in the GUI
pub struct Register {
    /// register name
    pub name: String,
    /// register address type
    pub address: AddressType,
    /// for non auto address: (first) address value
    pub address_value: gui_types::VectorValue,
    /// for stride address: number of registers
    pub address_count: gui_types::VectorValue,
    /// for stride address: increment between registers
    pub address_incr: gui_types::VectorValue,
    /// quick description of register
    pub summary: String,
    /// longer description of register
    pub description: String,
    /// register width. Can be auto only if fields are used
    pub width: gui_types::AutoManualU32,
    /// read/write access type for register
    pub access: AccessType,
    /// register location.  
    pub location: LocationType,
    /// signal properties when in core: read enable
    pub core_use_read_enable: CoreSignalProperty,
    /// signal properties when in core: write enable
    pub core_use_write_enable: CoreSignalProperty,
    /// list of fields elements. If not empty the following parameters are ignored
    pub fields: Vec<mdf_format::Field>,
    /// signal type
    pub signal_type: utils::SignalType,
    /// reset value
    pub reset: gui_types::VectorValue,
}

impl Register {
    pub fn new() -> Register {
        Register {
            name: String::new(),
            address: AddressType::Auto,
            address_value: gui_types::VectorValue::new(),
            address_count: gui_types::VectorValue::new(),
            address_incr: gui_types::VectorValue::new(),
            summary: String::new(),
            description: String::new(),
            width: gui_types::AutoManualU32::new(),
            access: AccessType::ReadWrite,
            location: LocationType::Core,
            core_use_read_enable: CoreSignalProperty::No,
            core_use_write_enable: CoreSignalProperty::No,
            fields: Vec::new(),
            signal_type: utils::SignalType::StdLogicVector,
            reset: gui_types::VectorValue::new(),
        
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Register::new()
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
/// type of address for the register
pub enum AddressType {
    /// Auto, next available address
    Auto,
    /// Fixed single address
    Single,
    /// Stride, register repeated several times
    Stride,
}

#[derive(
    Serialize,
    Deserialize,
    strum_macros::ToString,
    strum_macros::EnumIter,
    strum_macros::EnumString,
    PartialEq,
    Clone,
    Copy,
)]
/// read/write access type for a register
pub enum AccessType {
    /// Read/write
    ReadWrite,
    /// Read only
    ReadOnly,
    /// Write only
    WriteOnly,
    /// Per field
    PerField
}

#[derive(
    Serialize,
    Deserialize,
    strum_macros::ToString,
    strum_macros::EnumIter,
    strum_macros::EnumString,
    PartialEq,
    Clone,
    Copy,
)]
/// location of the register.
pub enum LocationType {
    /// interface module
    Pif,
    /// user module
    Core,
    /// different value per field
    PerField
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
/// yes/no core signal property
pub enum CoreSignalProperty {
    Yes,
    No,
    PerField
}
