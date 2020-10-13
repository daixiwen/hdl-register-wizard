use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Mdf {
  pub name : String,
}

impl Mdf {

  pub fn new () -> Mdf
  {
    Mdf {
      name : String::new(),
    }
  }
}

