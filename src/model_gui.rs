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
use crate::gui_types;
use crate::utils;

#[derive(Serialize, Deserialize, Clone)]
/// model description. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct Model {
    /// file name
    pub name: String,
    /// list of interfaces
    pub interfaces: Vec<Interface>,
}

impl Default for Model {
    /// create an empty model
    fn default() -> Model {
        Model {
            name: String::new(),
            interfaces: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
/// structure representing an interface in the model
pub struct Interface {
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

impl Interface {
    /// create a new empty interface with type SBI
    pub fn new() -> Interface {
        Interface {
            name: String::new(),
            description: String::new(),
            interface_type: InterfaceType::SBI,
            registers: Vec::new(),
            address_width: gui_types::AutoManualU32::new(),
            data_width: gui_types::AutoManualU32::new()
        }
    }
}

impl Default for Interface {
    fn default() -> Self {
        Interface::new()
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
#[serde(default)]
/// structure representing an register in the GUI
pub struct Register {
    /// register name
    pub name: String,
    /// quick description of register
    pub summary: String,
    /// longer description of register
    pub description: String,
    /// (first) address value
    pub address_value: gui_types::AutoManualVectorValue,
    /// is it a stride address?
    pub address_stride : bool,
    /// for stride address: number of registers
    pub address_count: gui_types::VectorValue,
    /// for stride address: increment between registers
    pub address_incr: gui_types::AutoManualVectorValue,
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
    pub fields: Vec<Field>,
    /// signal type
    pub signal_type: utils::SignalType,
    /// reset value
    pub reset: gui_types::VectorValue,
    /// string used by the gui to represent the bitfield. Updated with the update_field() method
    #[serde(skip)]
    pub bitfield : String,
    /// which field is currently hovered by the mouse, if any
    #[serde(skip)]
    pub hovered_field : Option<usize>
}

impl Register {
    pub fn new() -> Register {
        Register {
            name: String::new(),
            address_value: gui_types::AutoManualVectorValue::new(),
            address_stride: false,
            address_count: gui_types::VectorValue::new(),
            address_incr: gui_types::AutoManualVectorValue::new(),
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
            bitfield : String::new(),
            hovered_field : None
        }
    }

    pub fn calculate_width(&self, interface_width : &gui_types::AutoManualU32) -> Result<usize, String> {
        if ! self.width.is_auto {
            Ok(self.width.manual.value_int as usize)
        } else if  ! interface_width.is_auto {
            Ok(interface_width.manual.value_int as usize)
        } else if self.fields.is_empty() {
            Err(format!("register {} has no width specified", self.name))
        } else {
            let mut max_bit_num = 0;
            for field in &self.fields {
                max_bit_num = usize::max(max_bit_num, usize::max(field.position_start.value_int as usize, field.position_end.value_int as usize) + 1);
            }
            Ok(max_bit_num)
        }
    }

    pub fn update_bitfield(&mut self, interface_width : &gui_types::AutoManualU32) {
 
        if let Ok(total_reg_size) = self.calculate_width(interface_width) {
            self.bitfield = "e".repeat(total_reg_size);

            // check that the bitfield values are not over the register size
            let mut total_size = total_reg_size;
            for field in &self.fields {
                let highest_pos = usize::max(field.position_start.value_int as usize, field.position_end.value_int as usize);

                if highest_pos + 1 > total_size {
                    self.bitfield.insert_str(0, &"*".repeat(highest_pos + 1 - total_size));
                    total_size = highest_pos + 1;
                }
            }

            for (n, field) in self.fields.iter().enumerate() {
                let min = usize::max(0,total_size - 1 - usize::max(field.position_start.value_int as usize, field.position_end.value_int as usize));
                let max = usize::max(0,total_size - 1 - usize::min(field.position_start.value_int as usize, field.position_end.value_int as usize));
                if let Some(field_slice) = self.bitfield.get(min..max+1) {
                    let mut new_char = 'u';
                    if self.hovered_field == Some(n) {
                        new_char = 'h'
                    }
                    let field_slice : String = field_slice.chars().map({ | c | match c {
                        'e' => new_char,
                        'h' => '*',
                        _ => '*'
                    }}).collect();
    
                    self.bitfield.replace_range(min..max+1, &field_slice);
                }
                
            }    
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
    #[strum(serialize="Read / Write")]
    ReadWrite,
    /// Read only
    #[strum(serialize="Read Only")]
    ReadOnly,
    /// Write only
    #[strum(serialize="Write Only")]
    WriteOnly,
    /// Per field
    #[strum(serialize="Per Field")]    
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
    #[strum(serialize="Per Field")]    
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
    #[strum(serialize="Per Field")]    
    PerField
}

#[derive(Serialize, Deserialize, Clone)]
/// structure representing a field element in a register
pub struct Field {
    /// field name
    pub name: String,
    /// field position
    pub position_start: gui_types::GuiU32,
    pub position_end: gui_types::GuiU32,
    /// description of the register field
    pub description: String,
    /// read/write access type for register. Can be None if fields are used and every field has an access type
    pub access: AccessTypeField,
    /// signal type
    pub signal_type: utils::SignalType,
    /// reset value
    pub reset: gui_types::VectorValue,
    /// register location.  Can be None if field has a location
    pub location: LocationTypeField,
    /// signal properties when in core: read enable
    pub core_use_read_enable: CoreSignalPropertyField,
    /// signal properties when in core: write enable
    pub core_use_write_enable: CoreSignalPropertyField,
}

impl Field {
    pub fn new() -> Self {
        Field {
            name : String::new(),
            position_start : gui_types::GuiU32::new(),
            position_end : gui_types::GuiU32::new(),
            description : String::new(),
            access: AccessTypeField::ReadWrite,
            signal_type: utils::SignalType::StdLogicVector,
            reset: gui_types::VectorValue::new(),
            location: LocationTypeField::Core,
            core_use_read_enable: CoreSignalPropertyField::No,
            core_use_write_enable: CoreSignalPropertyField::No,
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
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
    Copy,
)]
/// read/write access type for a register
pub enum AccessTypeField {
    /// Read/write
    #[strum(serialize="Read / Write")]
    ReadWrite,
    /// Read only
    #[strum(serialize="Read Only")]
    ReadOnly,
    /// Write only
    #[strum(serialize="Write Only")]
    WriteOnly,
    /// use register setting
    #[strum(serialize="As Register")]    
    AsRegister
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
pub enum LocationTypeField {
    /// interface module
    Pif,
    /// user module
    Core,
    /// different value per field
    #[strum(serialize="As Register")]    
    AsRegister
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
pub enum CoreSignalPropertyField {
    Yes,
    No,
    #[strum(serialize="As Register")]    
    AsRegister
}
