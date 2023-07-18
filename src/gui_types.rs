//! Types used in different parts of the GUI

use crate::utils;   

/// trait for all types that can be edited as a string to provide a validate function
/// this function will be called by the GUI to report to the user whether the value
/// is valid or not
pub trait Validable {
    fn validate_pattern() -> &'static str;
}

/// implemenation of validable for string... anything
impl Validable for String {
    fn validate_pattern() -> &'static str {
        ".+"
    }
}

/// implementation of validable for an unsigned integer. Only digits
impl Validable for u32 {
    fn validate_pattern() -> &'static str {
        "\\d+"
    }
}

/// implementation of validable for a VectorValue. Either digits, 0x with hexadecilam digits, 0d with
/// decimal digits, or 0b with a binary string
impl Validable for utils::VectorValue {
    fn validate_pattern() -> &'static str {
        "0x[0-9a-fA-F]+|0d\\d+|0b[01]+|\\d+"
    }
}
