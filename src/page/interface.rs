use seed::{prelude::*, *};

use super::super::Model;
use super::super::PageType;
use super::super::mdf_format::Interface;
use super::super::mdf_format::InterfaceType;
use super::super::mdf_format;
use strum::IntoEnumIterator;
use super::super::Msg;

use super::super::utils;

use std::str::FromStr;

// URL constants
const URL_NEW: &str = "new";

// ID constants
const ID_ADDRESS_WIDTH: &str = "inputAddressWidth";
const ID_DATA_WIDTH: &str = "inputDataWidth";

// ------ ------
//     Urls
// ------ ------
pub enum InterfacePage {
  Num(usize),
  NewInterface
}

pub fn interface_url (url: Url, interface_page : InterfacePage) -> Url {
  match interface_page {
    InterfacePage::Num(interface_number) =>
      url.add_path_part(interface_number.to_string()),

    InterfacePage::NewInterface =>
      url.add_path_part(URL_NEW),
  }
}

pub fn change_url(url: Option<&str>, model: &mut Model) -> PageType {
  match url
  {
    None => PageType::NotFound,
    Some(URL_NEW) => new_interface(model),
    Some(number_string) => {
      match number_string.parse::<usize>() {
        Ok(index) => {
          if index < model.mdf_data.interfaces.len()
          {
            PageType::Interface(index)
          }
          else {
            PageType::NotFound
          }
        }
        Err(_) => {
          PageType::NotFound
        }
      }
    }
  }
}

fn new_interface(model: &mut Model) -> PageType {
  model.mdf_data.interfaces.push(Interface::new());
  let new_page_type = PageType::Interface(model.mdf_data.interfaces.len()-1);

  super::super::Urls::new(model.base_url.clone())
    .from_page_type(new_page_type)
    .go_and_replace();
  new_page_type  
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
#[derive(Clone)]
pub enum InterfaceMsg {
    Delete(usize),
    MoveUp(usize),
    MoveDown(usize),
    NameChanged(usize, String),
    TypeChanged(usize, String),
    DescriptionChanged(usize, String),
    AddressWitdhChanged(usize, String),
    DataWidthChanged(usize, String)
}

pub fn update(msg: InterfaceMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
  match msg {
    InterfaceMsg::Delete(index) => {
      if index < model.mdf_data.interfaces.len() {
        model.mdf_data.interfaces.remove(index);
      }
    },
    InterfaceMsg::MoveUp(index) => {
      if (index < model.mdf_data.interfaces.len()) && (index > 0) {
        model.mdf_data.interfaces.swap(index-1, index);
      }
    },
    InterfaceMsg::MoveDown(index) => {
      if  index < model.mdf_data.interfaces.len()-1 {
        model.mdf_data.interfaces.swap(index, index+1);
      }
    },

    InterfaceMsg::NameChanged(index, new_name) => {
      model.mdf_data.interfaces[index].name = new_name;
      orders.skip();
    },
    InterfaceMsg::TypeChanged(index, new_type_name) => {
      match InterfaceType::from_str(&new_type_name)
      {
        Ok(new_type) => {
          model.mdf_data.interfaces[index].interface_type = new_type;
          orders.skip();
        },

        _ =>
          seed::log!("error while converting from string to interface type"),
      }
    },
    InterfaceMsg::DescriptionChanged(index, new_description) => {
      model.mdf_data.interfaces[index].description =
        if new_description.is_empty() {
          None
        }
        else {
          Some(new_description.split("\n").map(|s|s.to_string()).collect())
        };

      orders.skip();
    },

    InterfaceMsg::AddressWitdhChanged(index, new_width) => {
      orders.skip();

      match utils::validate_field(
          ID_ADDRESS_WIDTH, &new_width, | field_value | utils::option_num_from_str(field_value)) {
        Ok(value) => model.mdf_data.interfaces[index].address_width = value,
        Err(_) => ()
      };
    },

    InterfaceMsg::DataWidthChanged(index, new_width) => {
      orders.skip();

      match utils::validate_field(
          ID_DATA_WIDTH, &new_width, | field_value | utils::option_num_from_str(field_value)) {
        Ok(value) => model.mdf_data.interfaces[index].data_width = value,
        Err(_) => ()
      };
    }

  }
}


// ------ ------
//     View
// ------ ------


// `view` describes what to display.
pub fn view(model: &Model, index: usize) -> Node<Msg> {
  let interface = &model.mdf_data.interfaces[index];

  div![
    div![
      C!["my-3"],
      a![
        C!["btn btn-primary"],
        attrs!{
          At::Href => super::super::Urls::new(&model.base_url).from_page_type(PageType::Edit),
        },
        "Back"
      ]
    ],
    h3![
      C!["my-2"],
      "Interface"],
    div![
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => "inputName",
          },
          "Name"
        ],
        div![
          C!["col-sm-10"],
          input![
            C!["form-control"],
            attrs!{
              At::Type => "text",
              At::Id => "inputName",
              At::Value => &interface.name,
            },
            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
          ]
        ]
      ],
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => "inputType",
          },
          "Type"
        ],
        div![
          C!["col-sm-10"],
          select![
            C!["form-control"],
            attrs!{
              At::Id => "inputType",
              At::Value => &interface.interface_type.to_string(),
            },
            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::TypeChanged(index, input))),
            InterfaceType::iter().map(|interface_type|
              option![
                IF!(&interface_type == &interface.interface_type =>
                  attrs!{
                    At::Selected => "selected",
                  }),
                interface_type.to_string(),
              ] 
            ).collect::<Vec<_>>(),
          ]
        ]
      ],
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => "inputDescription",
          },
          "Description"
        ],
        div![
          C!["col-sm-10"],
          textarea![
            C!["form-control"],
            attrs!{
              At::Type => "text",
              At::Id => "inputDescription",
              At::Value => match &interface.description {
                None => String::new(),
                Some(str_vector) => str_vector.join("\n"),
              },
            },
            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::DescriptionChanged(index, input))),
          ]
        ]
      ], 
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => ID_ADDRESS_WIDTH,
          },
          "Address width"
        ],
        div![
          C!["col-sm-10 was-validated]"],
          input![
            C!["form-control"],
            attrs!{
              At::Type => "text",
              At::Id => ID_ADDRESS_WIDTH,
              At::Value => match &interface.address_width {
                None => String::new(),
                Some(width) => width.to_string(),
              },
            },
            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::AddressWitdhChanged(index, input))),
          ],
          div![
            C!["invalid-feedback"],
            "please write a decimal value or leave empty for automatic"
          ],
        ]
      ],
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => ID_DATA_WIDTH,
          },
          "Data width"
        ],
        div![
          C!["col-sm-10"],
          input![
            C!["form-control"],
            attrs!{
              At::Type => "text",
              At::Id => ID_DATA_WIDTH,
              At::Value => match &interface.data_width {
                None => String::new(),
                Some(width) => width.to_string(),
              },
            },
            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::DataWidthChanged(index, input))),
          ],
          div![
            C!["invalid-feedback"],
            "please write a decimal value or leave empty for automatic"
          ],
        ]
      ],
    ],
    h3![
      C!["my-2"],
      "Registers"],
    table![
      C!["table table-striped"],
      thead![
        tr![
          th![
            attrs!{
              At::Scope => "col"
            },
            "name"
          ],
          th![
            attrs!{
              At::Scope => "col"
            },
            "address"
          ],
          th![
            attrs!{
              At::Scope => "col"
            },
            "summary"
          ],
          th![
            attrs!{
              At::Scope => "col"
            },
            "actions"
          ],
        ]
      ],
      tbody![
        interface.registers.iter().enumerate().map(|(ref_index, register)| register_table_row(&model, index, ref_index, &register)).collect::<Vec<_>>(),
        tr![
          td![],
          td![],
          td![],
          td![

            "placeholder for future 'add' button"
          ],           
        ]
      ]
    ]
  ]
}

fn register_table_row(model: &Model, index : usize, ref_index : usize, register : &mdf_format::Register) -> Node<Msg>
{
  tr![
    td![
      &register.name
    ],
    td![
      &register.address
    ],
    td![
      utils::opt_vec_str_to_str(&register.summary),
    ],
    td![
/*      in_table_button_url(index, "✎", "primary",
        &Urls::new(&model.base_url).interface(InterfacePage::Num(index)), true),
      in_table_button_msg(index, "✖", "danger",
        Msg::Interface(InterfaceMsg::Delete(index)), true),
      in_table_button_msg(index, "▲", "primary", 
        Msg::Interface(InterfaceMsg::MoveUp(index)), index != 0),
      in_table_button_msg(index, "▼", "primary",
        Msg::Interface(InterfaceMsg::MoveDown(index)), index != model.mdf_data.interfaces.len()-1),
*/    ],    
  ]
}
