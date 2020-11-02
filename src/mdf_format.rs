//! Model description file structures module

use serde::{Serialize, Deserialize, de::Error};

use strum_macros;
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
/// model description file. This structure hold all the model, and can be
/// imported or exported as JSON
pub struct Mdf {
  /// file name
  pub name : String,
  /// list of interfaces
  pub interfaces : Vec<Interface>,
}

impl Mdf {

  /// create an empty model
  pub fn new () -> Mdf
  {
    Mdf {
      name : String::new(),
      interfaces : Vec::new()
    }
  }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// structure representing an interface in the model
pub struct Interface {
  /// interface name
  pub name : String,
  /// description for the interface
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description : Option<Vec<String>>,
  /// interface type (protocol used)
  #[serde(rename = "type")]
  pub interface_type : InterfaceType,
  /// width of the address bus.
  /// if empty, automatically caculated from the highest register address
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address_width : Option<u32>,
  /// width of the data bus.
  /// if empty, automatically caculated from the widest register
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data_width : Option<u32>,
  /// list of registers
  #[serde(default)]
  pub registers : Vec<Register>,
}

impl Interface {
  /// create a new empty interface with type SBI
  pub fn new () -> Interface {
    Interface {
      name : String::new(),
      description : None,
      interface_type : InterfaceType::SBI,
      registers : Vec::<Register>::new(),
      address_width: None,
      data_width: None
    }
  }
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq)]
/// type of interface. Only SBI is officially spported by the Bitvis tool RegisterWizard
pub enum InterfaceType {
  /// SBI protocol, defined by Bitvis 
  SBI, 
  /// APB3 protocol, used in ARM systems among others
  APB3, 
  /// Avalon Memory mapped interface, used in Altera/Intel FPGA designs
  AvalonMm}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// structure describing a register within an interface
pub struct Register {
  /// register name
  pub name : String,
  /// register address
  pub address: Address,
  /// quick description of register
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary : Option<Vec<String>>,
  /// longer description of register
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description : Option<Vec<String>>,
  /// register width. Can be None if fields are used
  #[serde(skip_serializing_if = "Option::is_none")]
  pub width: Option<u32>,
  /// read/write access type for register. Can be None if fields are used and every field has an access type
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access: Option<AccessType>,
  /// signal type. Must be None if and only if fields are used
  #[serde(skip_serializing_if = "Option::is_none")]
  pub signal: Option<SignalType>,
  /// reset value. Must be None if and only if fields are used
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reset : Option<VectorValue>,
  /// register location.  Can be None if fields are used and every field has a location
  #[serde(skip_serializing_if = "Option::is_none")]
  pub location : Option<LocationType>,
  /// signal properties
  #[serde(default)]
  pub core_signal_properties : CoreSignalProperties,
  /// list of fields elements
  #[serde(default)]
  pub fields : Vec<Field>,
}

impl Register {
  /// create a new register, with an auto address, rw access, 32 bit wide std_logic_vector
  pub fn new () -> Register {
    Register {
      name : String::new(),
      address: Address::Auto,
      summary : None,
      description : None,
      width : Some(32),
      access : Some(AccessType::RW),
      signal : Some(SignalType::StdLogicVector),
      reset : Some(VectorValue::new()),
      location : Some(LocationType::Pif),
      core_signal_properties : CoreSignalProperties {
        use_read_enable : None,
        use_write_enable : None},
      fields : Vec::new()
    }
  }
}

#[derive(Debug, PartialEq)]
/// diferent ways of defining a register address
pub enum Address {
  /// automatic address, the first available spot is used
  Auto,
  /// fixed unique address
  Single(VectorValue),
  /// fixed group of addresses
  Stride(AddressStride)
}

#[derive(Debug, PartialEq)]
/// structure to represent a stride address definition, where the register can be repeated several times
pub struct AddressStride {
  /// starting value
  pub value : VectorValue,
  /// number of addresses
  pub count : VectorValue,
  /// increment between two addresses. If None, use the register size as increment
  pub increment : Option<VectorValue>
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Clone, Copy)]
/// read/write access type for a register
pub enum AccessType { 
  /// Read/write
  RW, 
  /// Read only
  RO, 
  /// Write only
  WO}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
/// signal type used for the register
pub enum SignalType {
  /// VHDL IEEE 1164 std_logic 
  StdLogic, 
  /// VHDL IEEE 1164 dst_logic_vector
  StdLogicVector, 
  /// VHDL IEEE numeric unsigned
  Unsigned, 
  /// VHDL IEEE numeric signed
  Signed, 
  /// VHDL boolean
  Boolean}

#[derive(Debug, PartialEq, Copy, Clone)]
/// structure used to represent a vector or integer value, with both the value itself and the radix type
pub struct VectorValue {
  /// integer value itself
  pub value : u128,
  /// radix used to import, export display or edit the value
  pub radix : RadixType
}

#[derive(PartialEq, strum_macros::ToString, Clone, Copy, Debug)]
/// radix type for a VectorValue
pub enum RadixType { 
  /// binary representation (0b*)
  Binary, 
  /// decimal representation (0d* or direct integer value)
  Decimal, 
  /// hexadecimal reprentation
  Hexadecimal }

impl VectorValue {
  /// create a new vector value, with a decimal radix and value 0
  pub fn new () -> VectorValue {
    VectorValue {
      value: 0,
      radix : RadixType::Decimal,
    }
  }
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
/// location of the register.
pub enum LocationType { 
  /// interface module
  Pif, 
  /// user module
  Core }

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
/// extra properties for signals located in core
pub struct CoreSignalProperties {

  /// generate a signal to indicate to the core when the signal is read
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_read_enable : Option<bool>,
  /// generate a signal to indicate to the core when the signal is written
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_write_enable : Option<bool>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// structure representing a field element in a register
pub struct Field {
}

impl fmt::Display for Address {
  /// conversion to string
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

    match &self {
      Address::Auto =>
        write!(f, "auto"),

      Address::Single(value) =>
        write!(f, "{}", &value.to_string()),

      Address::Stride(stride) => match &stride.increment {
        None =>
          write!(f, "{}:stride:{}", &stride.value.to_string(), &stride.count.to_string()),
        Some(inc) =>
          write!(f, "{}:stride:{}:{}", &stride.value.to_string(), &stride.count.to_string(),
            &inc.to_string()),
      }
    }
  }
}

impl Address {
  /// conversion to string for human reading in tables
  pub fn nice_str(&self) -> String {
    match &self {
      Address::Auto =>
        "auto".to_string(),

      Address::Single(value) =>
        value.to_string(),

      Address::Stride(stride) =>  {
        let count_minus_one = VectorValue {
          value : stride.count.value - 1,
          radix : stride.count.radix };

        match &stride.increment {
 
        None =>
          format!("{} + (0..{})", &stride.value.to_string(),&count_minus_one.to_string()),
        Some(inc) =>
          format!("{} + (0..{})*{}", &stride.value.to_string(), &count_minus_one.to_string(),
            &inc.to_string()),
        }
      }
    }
  }
}

impl Serialize for Address {
  /// address serializing. The MDF model gives a special string format for stride addresses instead of using several fields 
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

impl std::str::FromStr for Address {
    type Err = std::num::ParseIntError;

  /// conversion from string to address, using the format described in the mdf specification for stride addresses
  fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
    if s == "auto"
    {
      Ok(Address::Auto)
    }
    else {
      let elements :Vec<&str> = s.split(":").collect();
      match elements.len() {
        1 => Ok(Address::Single(VectorValue::from_str(s)?)),
        3 => if elements[1] == "stride" {
          Ok(Address::Stride(AddressStride {
            value : VectorValue::from_str(elements[0])?,
            count : VectorValue::from_str(elements[2])?,
            increment : None }))
        }
        else {
          Err(u32::from_str("abc").err().unwrap())
        },
        4 => if elements[1] == "stride" {
          Ok(Address::Stride(AddressStride {
            value : VectorValue::from_str(elements[0])?,
            count : VectorValue::from_str(elements[2])?,
            increment : Some(VectorValue::from_str(elements[3])?) }))
        }
        else {
          Err(u32::from_str("abc").err().unwrap())
        },

        _ => Err(u32::from_str("abc").err().unwrap())

      }
      
    }
  }
}

/// use a raw internal type to make the deserializer automatically fill the correct enum, depending on 
/// the field type as a number or a string
#[derive(Deserialize)]
#[serde(untagged)]
enum StrOrNum<'a> {
    Str(&'a str),
    Num(u64),
}

impl<'de> Deserialize<'de> for Address {
  /// deserialize into an address
  fn deserialize<D>(deserializer: D) -> Result<Address, D::Error>
    where D: serde::Deserializer<'de>,
  {
    let raw = StrOrNum::deserialize(deserializer)?;

    match raw {
      StrOrNum::Str(s) =>
        match Address::from_str(s) {
          Ok(a) => Ok(a),
          Err(_) => Err(D::Error::custom(&format!("couldn't parse string '{}' as a vector value", s))),
        },

      StrOrNum::Num(n) =>
        Ok(Address::Single(VectorValue { value: n.into(), radix: RadixType::Decimal}))
    }
  }
}

// methods for serializing and deserilizing a VectorValue
impl fmt::Display for VectorValue {
  /// convert a VectorValue to a string, using the specified radix
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

    match &self.radix {
      RadixType::Decimal =>
        write!(f, "{}", self.value),

      RadixType::Binary =>
        write!(f, "{:#0b}", self.value),

      RadixType::Hexadecimal =>
        write!(f, "{:#0x}", self.value),
    }
  }
}

impl Serialize for VectorValue {
  /// serialize the VectorValue to a string
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
  {
    if (self.radix == RadixType::Decimal) && (self.value <= u32::MAX.into()) {
      // JSON supports bigger integers than u32, and is in theory unlimited, but in practise
      // when using doubles the maximum is somewhere between u32::MAX and u64::MAX. Not taking
      // any chances
      serializer.serialize_u32(self.value as u32)
    }
    else {
      serializer.serialize_str(&self.to_string())
    }
  }
}

impl std::str::FromStr for VectorValue {
    type Err = std::num::ParseIntError;

  /// create a vector value from a string value, determing the radix from the prefix
  fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
    if s.len() < 3 {
      let value  = u128::from_str_radix(&s,10)?;
      Ok(VectorValue {
        value,
        radix : RadixType::Decimal
      })      
    }
    else {
      match &s[0..2] {
        "0x" | "0X" => {
          let value  = u128::from_str_radix(&s[2..],16)?;
          Ok(VectorValue {
            value,
            radix : RadixType::Hexadecimal
          })
        },

        "0d" | "0D" => {
          let value  = u128::from_str_radix(&s[2..],10)?;
          Ok(VectorValue {
            value,
            radix : RadixType::Decimal
          })
        },

        "0b" | "0B" => {
          let value  = u128::from_str_radix(&s[2..],2)?;
          Ok(VectorValue {
            value,
            radix : RadixType::Binary
          })
        },

        _ => {
          let value  = u128::from_str_radix(&s,10)?;
          Ok(VectorValue {
            value,
            radix : RadixType::Decimal
          })
        },
      }
    }
  }
}

impl<'de> Deserialize<'de> for VectorValue {
  /// deserialize a VectorValue
  fn deserialize<D>(deserializer: D) -> Result<VectorValue, D::Error>
    where D: serde::Deserializer<'de>,
  {
    let raw = StrOrNum::deserialize(deserializer)?;

    match raw {
      StrOrNum::Str(s) =>
        match VectorValue::from_str(s) {
          Ok(v) => Ok(v),
          Err(_) => Err(D::Error::custom(&format!("couldn't parse string '{}' as a vector value", s))),
        },

      StrOrNum::Num(n) =>
        Ok(VectorValue { value: n.into(), radix: RadixType::Decimal})
    }
  }
}
