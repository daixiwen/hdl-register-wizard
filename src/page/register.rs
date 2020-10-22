use seed::{prelude::*, *};

use super::super::Model;
use super::super::PageType;

use super::super::mdf_format::Register;
//use super::super::mdf_format::Address;
//use super::super::mdf_format::AddressStride;
//use super::super::mdf_format::AccessType;
//use super::super::mdf_format::SignalType;
//use super::super::mdf_format::VectorValue;
//use super::super::mdf_format::LocationType;
//use super::super::mdf_format::RadixType;
//use super::super::mdf_format::CoreSignalProperties;
//use super::super::mdf_format::Field;

//use super::super::mdf_format;
//use strum::IntoEnumIterator;
use super::super::Msg;

//use super::super::utils;

//use std::str::FromStr;

// URL constants
const URL_NEW: &str = "new";

// ID constants


// ------ ------
//     Urls
// ------ ------
pub enum RegisterPage {
  Num(usize),
  NewRegister
}

pub fn register_url (url: Url, register_page : RegisterPage) -> Url {
  match register_page {
    RegisterPage::Num(register_number) =>
      url.add_path_part(register_number.to_string()),

    RegisterPage::NewRegister =>
      url.add_path_part(URL_NEW),
  }
}

pub fn change_url(mut url: seed::browser::url::Url, interface_num: usize, model: &mut Model) -> PageType {
  match url.next_path_part()
  {
    None => PageType::NotFound,
    Some(URL_NEW) => new_register(interface_num, model),
    Some(number_string) => {
      match number_string.parse::<usize>() {
        Ok(index) => {
          if index < model.mdf_data.interfaces[interface_num].registers.len()
          {
            PageType::Register(interface_num, index)
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

fn new_register(interface: usize, model: &mut Model) -> PageType {
  model.mdf_data.interfaces[interface].registers.push(Register::new());
  let new_page_type = PageType::Register(interface, 
    model.mdf_data.interfaces[interface].registers.len()-1);

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
pub enum RegisterMsg {
    Delete(usize),
    MoveUp(usize),
    MoveDown(usize),
/*    NameChanged(usize, String),
    TypeChanged(usize, String),
    DescriptionChanged(usize, String),
    AddressWitdhChanged(usize, String),
    DataWidthChanged(usize, String)*/
}

pub fn update(msg: RegisterMsg, interface_num: usize, model: &mut Model, orders: &mut impl Orders<Msg>) {
  match msg {
    RegisterMsg::Delete(index) => {
      if index < model.mdf_data.interfaces[interface_num].registers.len() {
        model.mdf_data.interfaces[interface_num].registers.remove(index);
      }
    },
    RegisterMsg::MoveUp(index) => {
      if (index < model.mdf_data.interfaces[interface_num].registers.len()) && (index > 0) {
        model.mdf_data.interfaces[interface_num].registers.swap(index-1, index);
      }
    },
    RegisterMsg::MoveDown(index) => {
      if  index < model.mdf_data.interfaces[interface_num].registers.len()-1 {
        model.mdf_data.interfaces[interface_num].registers.swap(index, index+1);
      }
    },

/*    InterfaceMsg::NameChanged(index, new_name) => {
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
          utils::textarea_to_opt_vec_str(&new_description);

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
*/
  }
}
/*

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
              At::Value => utils::opt_vec_str_to_textarea(&interface.description),
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
        interface.registers.iter().enumerate().map(|(reg_index, register)| register_table_row(&model, index, reg_index, &register)).collect::<Vec<_>>(),
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

fn register_table_row(model: &Model, index : usize, reg_index : usize, register : &mdf_format::Register) -> Node<Msg>
{
  tr![
    td![
      &register.name
    ],
    td![
      &register.address.nice_str()
    ],
    td![
      utils::opt_vec_str_to_summary(&register.summary),
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
*/