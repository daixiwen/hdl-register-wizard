use serde::{Serialize, Deserialize};

use strum_macros;

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
pub struct Interface {
  pub name : String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description : Option<Vec<String>>,
  #[serde(rename = "type")]
  pub interface_type : InterfaceType,
  pub registers : Vec<Register>,
}

#[derive(Serialize, Deserialize, strum_macros::ToString)]
pub enum InterfaceType { SBI, APB3, AvalonMm}

#[derive(Serialize, Deserialize)]
pub struct Register {

}
