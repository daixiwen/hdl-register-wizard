//! Model description file version 1.0 import/export

use serde::{de::Error, Deserialize, Serialize};

use crate::utils;
use std::convert::From;
use std::convert::TryInto;
use std::default::Default;
use std::fmt;
use std::str::FromStr;
use strum_macros;

#[derive(Serialize, Deserialize, Clone)]
/// model description file. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct Mdf {
    /// file name
    pub name: String,
    /// list of interfaces
    pub interfaces: Vec<Interface>,
}

impl Default for Mdf {
    /// create an empty model
    fn default() -> Mdf {
        Mdf {
            name: "New Project".to_owned(),
            interfaces: Vec::new(),
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    strum_macros::Display,
    strum_macros::EnumIter,
    strum_macros::EnumString,
    PartialEq,
    Clone,
    Copy,
)]
/// type of interface. Only SBI is officially spported by the Bitvis tool RegisterWizard
pub enum InterfaceType {
    /// SBI protocol, defined by Bitvis
    SBI,
    /// APB3 protocol, used in ARM systems among others
    APB3,
    /// Avalon Memory mapped interface, used in Altera/Intel FPGA designs
    AvalonMm,
    /// AXI4 light Memory mapped interface,  used in ARM systems among others
    AXI4Light,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// structure representing an interface in the model
pub struct Interface {
    /// interface name
    pub name: String,
    /// description for the interface
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// interface type (protocol used)
    #[serde(rename = "type")]
    pub interface_type: InterfaceType,
    /// width of the address bus.
    /// if empty, automatically caculated from the highest register address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_width: Option<u32>,
    /// width of the data bus.
    /// if empty, automatically caculated from the widest register
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_width: Option<u32>,
    /// list of registers
    #[serde(default)]
    pub registers: Vec<Register>,
}

impl Interface {
    /// create a new empty interface with type SBI
    pub fn new() -> Interface {
        Interface {
            name: String::new(),
            description: None,
            interface_type: InterfaceType::SBI,
            registers: Vec::<Register>::new(),
            address_width: None,
            data_width: None,
        }
    }
}

impl Default for Interface {
    fn default() -> Self {
        Interface::new()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// structure describing a register within an interface
pub struct Register {
    /// register name
    pub name: String,
    /// register address
    pub address: Address,
    /// quick description of register
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<Vec<String>>,
    /// longer description of register
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// register width. Can be None if fields are used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    /// read/write access type for register. Can be None if fields are used and every field has an access type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<AccessType>,
    /// signal type. Must be None if and only if fields are used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal: Option<utils::SignalType>,
    /// reset value. Must be None if and only if fields are used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset: Option<utils::VectorValue>,
    /// register location.  Can be None if fields are used and every field has a location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    /// signal properties
    #[serde(default)]
    #[serde(skip_serializing_if = "CoreSignalProperties::must_skip")]
    pub core_signal_properties: CoreSignalProperties,
    /// list of fields elements
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<Field>,
}

impl Register {
    /// create a new register, with an auto address, rw access, 32 bit wide std_logic_vector
    pub fn new() -> Register {
        Register {
            name: String::new(),
            address: Default::default(),
            summary: None,
            description: None,
            width: Some(32),
            access: Some(AccessType::RW),
            signal: Some(utils::SignalType::StdLogicVector),
            reset: Some(utils::VectorValue::new()),
            location: Some(LocationType::Pif),
            core_signal_properties: CoreSignalProperties {
                use_read_enable: None,
                use_write_enable: None,
            },
            fields: Vec::new(),
        }
    }
}

impl Default for Register {
    fn default() -> Self {
        Register::new()
    }
}

#[derive(Debug, PartialEq, Clone)]
/// diferent ways of defining a register address
pub struct Address {
    /// if none, automatic address. If some, defined address
    pub value: Option<utils::VectorValue>,
    /// stride option
    pub stride: Option<AddressStride>,
}

#[derive(Debug, PartialEq, Clone)]
/// structure to represent a stride address definition, where the register can be repeated several times
pub struct AddressStride {
    /// number of addresses
    pub count: utils::VectorValue,
    /// increment between two addresses. If None, use the register size as increment
    pub increment: Option<utils::VectorValue>,
}

impl Address {
    pub fn new() -> Self {
        Address {
            value: None,
            stride: None,
        }
    }
}

impl AddressStride {
    pub fn new() -> Self {
        AddressStride {
            count: Default::default(),
            increment: None,
        }
    }
}

impl Default for Address {
    fn default() -> Self {
        Address::new()
    }
}

impl Default for AddressStride {
    fn default() -> Self {
        AddressStride::new()
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
pub enum AccessType {
    /// Read/write
    RW,
    /// Read only
    RO,
    /// Write only
    WO,
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
#[serde(rename_all = "lowercase")]
/// location of the register.
pub enum LocationType {
    /// interface module
    Pif,
    /// user module
    Core,
}

#[derive(Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
/// extra properties for signals located in core
pub struct CoreSignalProperties {
    /// generate a signal to indicate to the core when the signal is read
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_read_enable: Option<bool>,
    /// generate a signal to indicate to the core when the signal is written
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_write_enable: Option<bool>,
}

impl CoreSignalProperties {
    pub fn must_skip(&self) -> bool {
        self.use_read_enable.is_none() && self.use_write_enable.is_none()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
/// structure representing a field element in a register
pub struct Field {
    /// field name
    pub name: String,
    /// field position
    pub position: FieldPosition,
    /// description of the register field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    /// read/write access type for register. Can be None if fields are used and every field has an access type
    pub access: AccessType,
    /// signal type
    pub signal: utils::SignalType,
    /// reset value
    pub reset: utils::VectorValue,
    /// register location.  Can be None if field has a location
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationType>,
    /// signal properties
    #[serde(default)]
    pub core_signal_properties: CoreSignalProperties,
}

impl Field {
    /// create a new field, with a single bit 0, type std_logic
    pub fn new() -> Field {
        Field {
            name: String::new(),
            position: FieldPosition::Single(0),
            description: None,
            access: AccessType::RW,
            signal: utils::SignalType::StdLogic,
            reset: utils::VectorValue::new(),
            location: Some(LocationType::Pif),
            core_signal_properties: CoreSignalProperties {
                use_read_enable: None,
                use_write_enable: None,
            },
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Field::new()
    }
}

#[derive(Debug, PartialEq, Clone)]
/// diferent ways of defining a field position
pub enum FieldPosition {
    /// single bit
    Single(u32),
    /// field(msb, lsb)
    Field(u32, u32),
}

impl fmt::Display for Address {
    /// conversion to string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value_str = match self.value {
            None => "auto".to_string(),
            Some(address_value) => address_value.to_string(),
        };
        match &self.stride {
            None => {
                write!(f, "{}", value_str)
            }
            Some(stride) => match stride.increment {
                None => {
                    write!(f, "{}:stride:{}", value_str, stride.count)
                }
                Some(increment) => {
                    write!(f, "{}:stride:{}:{}", value_str, stride.count, increment)
                }
            },
        }
    }
}

impl Address {
    /// conversion to string for human reading in tables
    pub fn nice_str(&self) -> String {
        let value_str = match self.value {
            None => "auto".to_string(),
            Some(address_value) => address_value.to_string(),
        };
        match &self.stride {
            None => value_str,
            Some(stride) => {
                let count_minus_one = utils::VectorValue {
                    value: stride.count.value - 1,
                    radix: stride.count.radix,
                };

                match stride.increment {
                    None => {
                        format!("{} + (0..{})", value_str, count_minus_one)
                    }
                    Some(increment) => {
                        format!("{} + (0..{})*{}", value_str, count_minus_one, increment)
                    }
                }
            }
        }
    }
}

impl Serialize for Address {
    /// address serializing. The MDF model gives a special string format for stride addresses instead of using several fields
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::str::FromStr for Address {
    type Err = &'static str;

    /// conversion from string to address, using the format described in the mdf specification for stride addresses
    fn from_str(s: &str) -> Result<Self, &'static str> {
        let elements: Vec<&str> = s.split(':').collect();
        let (value_str, stride) = match elements.len() {
            1 => (s, None),
            3 => {
                if elements[1] == "stride" {
                    (
                        elements[0],
                        Some(AddressStride {
                            count: utils::VectorValue::from_str(elements[2])
                                .map_err(|_| "could not parse stride count")?,
                            increment: None,
                        }),
                    )
                } else {
                    return Err("'stride' keyword expected");
                }
            }
            4 => {
                if elements[1] == "stride" {
                    (
                        elements[0],
                        Some(AddressStride {
                            count: utils::VectorValue::from_str(elements[2])
                                .map_err(|_| "could not parse stride count")?,
                            increment: Some(
                                utils::VectorValue::from_str(elements[3])
                                    .map_err(|_| "could not parse stride increment")?,
                            ),
                        }),
                    )
                } else {
                    return Err("'stride' keyword expected");
                }
            }

            _ => return Err("bad number of arguments between ':'"),
        };
        let address_value = if value_str == "auto" {
            None
        } else {
            Some(
                utils::VectorValue::from_str(value_str)
                    .map_err(|_| "could not parse address value")?,
            )
        };

        Ok(Address {
            value: address_value,
            stride,
        })
    }
}

impl<'de> Deserialize<'de> for Address {
    /// deserialize into an address
    fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = utils::StrOrNum::deserialize(deserializer)?;

        match raw {
            utils::StrOrNum::Str(s) => match Address::from_str(&s) {
                Ok(a) => Ok(a),
                Err(_) => Err(D::Error::custom(&format!(
                    "couldn't parse string '{}' as a vector value",
                    s
                ))),
            },

            utils::StrOrNum::Num(n) => Ok(Address {
                value: Some(utils::VectorValue {
                    value: n.into(),
                    radix: utils::RadixType::Decimal,
                }),
                stride: None,
            }),
        }
    }
}

impl fmt::Display for FieldPosition {
    /// conversion to string
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            FieldPosition::Single(position) => write!(f, "{}", position),

            FieldPosition::Field(msb, lsb) => write!(f, "{}:{}", msb, lsb),
        }
    }
}

impl Serialize for FieldPosition {
    /// address serializing. The MDF model gives a special string format for stride addresses instead of using several fields
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::str::FromStr for FieldPosition {
    type Err = std::num::ParseIntError;

    /// conversion from string to field position, using the format described in the mdf specification: single value, or msb:lsb
    fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
        let elements: Vec<&str> = s.split(':').collect();
        match elements.len() {
            1 => Ok(FieldPosition::Single(u32::from_str(s)?)),
            2 => Ok(FieldPosition::Field(
                u32::from_str(elements[0])?,
                u32::from_str(elements[1])?,
            )),

            _ => Err(u32::from_str("abc").err().unwrap()),
        }
    }
}

impl<'de> Deserialize<'de> for FieldPosition {
    /// deserialize into a field position
    fn deserialize<D>(deserializer: D) -> Result<FieldPosition, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = utils::StrOrNum::deserialize(deserializer)?;

        match raw {
            utils::StrOrNum::Str(s) => match FieldPosition::from_str(&s) {
                Ok(a) => Ok(a),
                Err(_) => Err(D::Error::custom(&format!(
                    "couldn't parse string '{}' as a field position",
                    s
                ))),
            },

            utils::StrOrNum::Num(n) => Ok(FieldPosition::Single(n.try_into().unwrap())),
        }
    }
}
