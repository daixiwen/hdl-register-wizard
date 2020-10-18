use seed::{prelude::*, *};
use super::Msg;

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

// utilities functions for the fields using arrays of strings (description mostly)
// convert to string, for example to put in a table. Outputs at most the first line
pub fn opt_vec_str_to_str(field : &Option<Vec<String>>) -> Node<Msg> {
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

