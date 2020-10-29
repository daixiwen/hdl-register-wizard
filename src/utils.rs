use seed::{prelude::*, *};
use super::Msg;
use super::mdf_format::VectorValue;
use std::str::FromStr;

// utility to mark a form field as invalid if its contents can't be parsed
pub fn validate_field<F,T,E>(field_id: &str, field_new_value: &str, decode_value: F) -> Result<T,E> 
    where  F: Fn(&str) -> Result<T,E>
{
  let result = decode_value(field_new_value);
  let elem = seed::document().get_element_by_id(field_id)
        .expect("should find element");

  match result {
    Ok(_) => elem.set_class_name("form-control"),
    Err(_) => elem.set_class_name("form-control  is-invalid"),
  };

  result
}

pub fn option_num_from_str(string_input: &str) -> Result<Option<u32>, std::num::ParseIntError> {
  if string_input.is_empty() {
    Ok(None)
  }
  else {
    match u32::from_str_radix(string_input, 10) {
      Ok(value) => Ok(Some(value)),
      Err(error) => Err(error)
    }
  }
}

pub fn option_vectorval_from_str(string_input: &str) -> Result<Option<VectorValue>, std::num::ParseIntError> {
  if string_input.is_empty() {
    Ok(None)
  }
  else {
    match VectorValue::from_str(string_input) {
      Ok(value) => Ok(Some(value)),
      Err(error) => Err(error)
    }
  }
}

// utilities functions for the fields using arrays of strings (description mostly)
// convert to string, for example to put in a table. Outputs at most the first line
pub fn opt_vec_str_to_summary(field : &Option<Vec<String>>) -> Node<Msg> {
  match field {
    None => empty![],
    Some(vec_str) => {
      match vec_str.len() {
        0 => empty![],
        1 => plain![vec_str[0].clone()],
        _ => plain![format!("{} ...",vec_str[0])],
      }
    }
  }
}

// convert to string for a text area, each line separated by an end of line
pub fn opt_vec_str_to_textarea(field : &Option<Vec<String>>) -> String {
  match field {
    None => String::new(),
    Some(str_vector) => str_vector.join("\n"),
  }
}

// convert from string from a text area, each line separated by an end of line
pub fn textarea_to_opt_vec_str(value_str: &String) -> Option<Vec<String>> {
  if value_str.is_empty() {
    None
  }
  else {
    Some(value_str.split("\n").map(|s|s.to_string()).collect())
  }
}

// returns whether the input element that is target from an event is checked or not
// panics if target is not an input element
pub fn target_checked(event: &web_sys::Event) -> bool {
  return event
    .target()
    .as_ref()
    .expect("cant get target")
    .dyn_ref::<web_sys::HtmlInputElement>()
    .expect("cant get right element")
    .checked();
}