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
  #[serde(rename = "AddressWidth", skip_serializing_if = "Option::is_none")]
  pub address_width : Option<u32>,
  #[serde(rename = "DataWidth",skip_serializing_if = "Option::is_none")]
  pub data_width : Option<u32>,
  pub registers : Vec<Register>,
}

impl Interface{
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
pub struct Register {

}
