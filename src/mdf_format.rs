use serde::{Serialize, Deserialize, de::Error};

use strum_macros;
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Mdf {
  pub name : String,
  pub interfaces : Vec<Interface>,
}

impl Mdf {

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
pub struct Interface {
  pub name : String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description : Option<Vec<String>>,
  #[serde(rename = "type")]
  pub interface_type : InterfaceType,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub address_width : Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub data_width : Option<u32>,
  #[serde(default)]
  pub registers : Vec<Register>,
}

impl Interface {
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
pub enum InterfaceType { SBI, APB3, AvalonMm}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Register {
  pub name : String,
  pub address: Address,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub summary : Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description : Option<Vec<String>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub width: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub access: Option<AccessType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub signal: Option<SignalType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reset : Option<VectorValue>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub location : Option<LocationType>,
  #[serde(default)]
  pub core_signal_properties : CoreSignalProperties,
  #[serde(default)]
  pub fields : Vec<Field>,
}

impl Register {
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
pub enum Address {
  Auto,
  Single(VectorValue),
  Stride(AddressStride)
}

#[derive(Debug, PartialEq)]
pub struct AddressStride {
  pub value : VectorValue,
  pub count : VectorValue,
  pub increment : Option<VectorValue>
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Clone, Copy)]
pub enum AccessType { RW, RO, WO}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SignalType { StdLogic, StdLogicVector, Unsigned, Signed, Boolean}

#[derive(Debug, PartialEq)]
pub struct VectorValue {
  pub value : u128,
  pub radix : RadixType
}

#[derive(PartialEq, strum_macros::ToString, Clone, Copy, Debug)]
pub enum RadixType { Binary, Decimal, Hexadecimal }

impl VectorValue {
  pub fn new () -> VectorValue {
    VectorValue {
      value: 0,
      radix : RadixType::Decimal,
    }
  }
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum LocationType { Pif, Core }

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CoreSignalProperties {

  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_read_enable : Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_write_enable : Option<bool>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
}

// methods for serializing and deserilizing a Address
impl fmt::Display for Address {
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
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
}

impl std::str::FromStr for Address {
    type Err = std::num::ParseIntError;

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

// use a raw internal type to make the deserializer automatically fill the correct enum, depending on 
// the field type as a number or a string
#[derive(Deserialize)]
#[serde(untagged)]
enum StrOrNum<'a> {
    Str(&'a str),
    Num(u64),
}

impl<'de> Deserialize<'de> for Address {
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
