//! several utilities used in the project

use super::mdf_format::VectorValue;
use std::str::FromStr;

/// convert an option to a string
pub fn option_type_to_str<T>(object: &Option<T>) -> String
    where T: std::fmt::Display {

    match object {
        None => String::new(),
        Some(t) => t.to_string()
    }
}

/// convert a string to an Option<name>
pub fn option_num_from_str(string_input: &str) -> Result<Option<u32>, std::num::ParseIntError> {
    if string_input.is_empty() {
        Ok(None)
    } else {
        Ok(Some(string_input.parse()?))
    }
}

/// convert a string to an Option<VectorValue>
pub fn option_vectorval_from_str(
    string_input: &str,
) -> Result<Option<VectorValue>, std::num::ParseIntError> {
    if string_input.is_empty() {
        Ok(None)
    } else {
        match VectorValue::from_str(string_input) {
            Ok(value) => Ok(Some(value)),
            Err(error) => Err(error),
        }
    }
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
