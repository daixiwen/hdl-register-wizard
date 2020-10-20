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
  pub address: String,
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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub core_signal_properties : Option<CoreSignalProperties>,
  #[serde(default)]
  pub fields : Vec<Field>,
}

impl Register {
  pub fn new () -> Register {
    Register {
      name : String::new(),
      address: String::new(),
      summary : None,
      description : None,
      width : Some(32),
      access : Some(AccessType::RW),
      signal : Some(SignalType::StdLogicVector),
      reset : Some(VectorValue::new()),
      location : Some(LocationType::Pif),
      core_signal_properties : None,
      fields : Vec::new()
    }
  }
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq)]
pub enum AccessType { RW, RO, WO}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum SignalType { StdLogic, StdLogicVector, Unsigned, Signed, Boolean}

#[derive(Debug, PartialEq)]
pub struct VectorValue {
  pub value : u128,
  pub radix : RadixType
}

#[derive(PartialEq, strum_macros::ToString, Debug)]
pub enum RadixType { Binary, Decimal, Hexadecimal }

impl VectorValue {
  pub fn new () -> VectorValue {
    VectorValue {
      value: 0,
      radix : RadixType::Decimal,
    }
  }
}

#[derive(Serialize, Deserialize, strum_macros::ToString, strum_macros::EnumIter, strum_macros::EnumString, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LocationType { Pif, Core }

#[derive(Serialize, Deserialize)]
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

// use a raw internal type to make the deserializer automatically fill the correct enum, depending on 
// the field type as a number or a string
#[derive(Deserialize)]
#[serde(untagged)]
enum StrOrNum<'a> {
    Str(&'a str),
    Num(u64),
}

impl std::str::FromStr for VectorValue {
    type Err = std::num::ParseIntError;

  fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
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
