//! several utilities and types used in the project, both on the gui side
//! and on the file I/O side
use crate::gui_types;
use serde::{de::Error, Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use strum_macros;

#[derive(Debug, PartialEq, Copy, Clone)]
/// structure used to represent a vector or integer value, with both the value itself and the radix type
pub struct VectorValue {
    /// integer value itself
    pub value: u128,
    /// radix used to import, export display or edit the value
    pub radix: RadixType,
}

#[derive(PartialEq, strum_macros::ToString, Clone, Copy, Debug)]
/// radix type for a VectorValue
pub enum RadixType {
    /// binary representation (0b*)
    Binary,
    /// decimal representation (0d* or direct integer value)
    Decimal,
    /// hexadecimal reprentation
    Hexadecimal,
}

impl VectorValue {
    /// create a new vector value, with a decimal radix and value 0
    pub fn new() -> VectorValue {
        VectorValue {
            value: 0,
            radix: RadixType::Decimal,
        }
    }
}

impl From<u128> for VectorValue {
    fn from(value: u128) -> Self {
        VectorValue {
            value,
            radix: RadixType::Hexadecimal,
        }
    }
}

impl Default for VectorValue {
    fn default() -> Self {
        VectorValue::new()
    }
}
// methods for serializing and deserilizing a VectorValue
impl fmt::Display for VectorValue {
    /// convert a VectorValue to a string, using the specified radix
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.radix {
            RadixType::Decimal => write!(f, "{}", self.value),

            RadixType::Binary => write!(f, "{:#0b}", self.value),

            RadixType::Hexadecimal => write!(f, "{:#0x}", self.value),
        }
    }
}

impl Serialize for VectorValue {
    /// serialize the VectorValue to a string
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if (self.radix == RadixType::Decimal) && (self.value <= u32::MAX.into()) {
            // JSON supports bigger integers than u32, and is in theory unlimited, but in practise
            // when using doubles the maximum is somewhere between u32::MAX and u64::MAX. Not taking
            // any chances
            serializer.serialize_u32(self.value as u32)
        } else {
            serializer.serialize_str(&self.to_string())
        }
    }
}

impl std::str::FromStr for VectorValue {
    type Err = std::num::ParseIntError;

    /// create a vector value from a string value, determing the radix from the prefix
    fn from_str(s: &str) -> Result<Self, std::num::ParseIntError> {
        if s.len() < 3 {
            let value = s.parse()?;
            Ok(VectorValue {
                value,
                radix: RadixType::Decimal,
            })
        } else {
            match &s[0..2] {
                "0x" | "0X" => {
                    let value = u128::from_str_radix(&s[2..], 16)?;
                    Ok(VectorValue {
                        value,
                        radix: RadixType::Hexadecimal,
                    })
                }

                "0d" | "0D" => {
                    let value = s[2..].parse()?;
                    Ok(VectorValue {
                        value,
                        radix: RadixType::Decimal,
                    })
                }

                "0b" | "0B" => {
                    let value = u128::from_str_radix(&s[2..], 2)?;
                    Ok(VectorValue {
                        value,
                        radix: RadixType::Binary,
                    })
                }

                _ => {
                    let value = s.parse()?;
                    Ok(VectorValue {
                        value,
                        radix: RadixType::Decimal,
                    })
                }
            }
        }
    }
}

/// use a raw internal type to make the deserializer automatically fill the correct enum, depending on
/// the field type as a number or a string
#[derive(Deserialize)]
#[serde(untagged)]
pub enum StrOrNum {
    Str(String),
    Num(u64),
}

impl<'de> Deserialize<'de> for VectorValue {
    /// deserialize a VectorValue
    fn deserialize<D>(deserializer: D) -> Result<VectorValue, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw = StrOrNum::deserialize(deserializer)?;

        match raw {
            StrOrNum::Str(s) => match VectorValue::from_str(&s) {
                Ok(v) => Ok(v),
                Err(_) => Err(D::Error::custom(&format!(
                    "couldn't parse string '{}' as a vector value",
                    s
                ))),
            },

            StrOrNum::Num(n) => Ok(VectorValue {
                value: n.into(),
                radix: RadixType::Decimal,
            }),
        }
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
    Boolean,
}

/// convert to string for a text area, each line separated by an end of line
pub fn opt_vec_str_to_textarea(field: &Option<Vec<String>>) -> String {
    match field {
        None => String::new(),
        Some(str_vector) => str_vector.join("\n"),
    }
}

/// convert from string from a text area, each line separated by an end of line
pub fn textarea_to_opt_vec_str(value_str: &str) -> Option<Vec<String>> {
    if value_str.is_empty() {
        None
    } else {
        Some(value_str.split('\n').map(|s| s.to_string()).collect())
    }
}

/// convert from an Option<u32> to an AutoManualU32
pub fn opt_u32_to_automanual(entry: &Option<u32>) -> gui_types::AutoManualU32 {
    match entry {
        None => gui_types::AutoManualU32 {
            is_auto: true,
            ..Default::default()
        },
        Some(value) => gui_types::AutoManualU32 {
            is_auto: false,
            manual: gui_types::GuiU32 {
                value_str: value.to_string(),
                str_valid: true,
                value_int: *value,
            },
        },
    }
}

/// convert from a AutoManualU32 to an Option<u32>
pub fn automanual_to_opt_u32(gui_field: &gui_types::AutoManualU32) -> Option<u32> {
    if gui_field.is_auto {
        None
    } else {
        Some(gui_field.manual.value_int)
    }
}
