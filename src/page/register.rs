use seed::{prelude::*, *};

use super::super::Model;
use super::super::PageType;

use super::super::mdf_format::Register;
use super::super::mdf_format::Address;
//use super::super::mdf_format::AddressStride;
use super::super::mdf_format::AccessType;
use super::super::mdf_format::SignalType;
//use super::super::mdf_format::VectorValue;
use super::super::mdf_format::LocationType;
//use super::super::mdf_format::RadixType;
//use super::super::mdf_format::CoreSignalProperties;
//use super::super::mdf_format::Field;

//use super::super::mdf_format;
use strum::IntoEnumIterator;
use super::super::Msg;

use super::super::utils;

//use std::str::FromStr;

// URL constants
const URL_NEW: &str = "new";

// ID constants
const ID_REG_WIDTH: &str = "inputRegisterWidth";
const ID_RESET_VALUE: &str = "inputResetValue";

// text constant
const TXT_SPEC_IN_FIELDS: &str ="(specify in fields)";

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


// ------ ------
//     View
// ------ ------


// `view` describes what to display.
pub fn view(model: &Model, interface_index: usize, register_index: usize) -> Node<Msg> {
  let interface = &model.mdf_data.interfaces[interface_index];
  let register = &interface.registers[register_index];

  div![
    div![
      C!["my-3"],
      a![
        C!["btn btn-primary"],
        attrs!{
          At::Href => super::super::Urls::new(&model.base_url).from_page_type(PageType::Interface(interface_index)),
        },
        "Back"
      ]
    ],
    h3![
      C!["my-2"],
      "Register in Interface ", &interface.name],
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
              At::Value => &register.name,
            },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
          ]
        ]
      ],
      div![
        C!["form-group row"],
        div![
          C!["col-sm-2 col-form-label"],
          "Address"
        ],
        div![
          C!["col-sm-10"],
          div![
            C!["form-check"],
            input![
              C!["form-check-input"],
              attrs!{
                At::Type => "radio",
                At::Name => "addressRadio",
                At::Value => "auto",
              },
              IF!(register.address == Address::Auto =>
                attrs!{At::Checked => "checked"}),
              id!["addressAuto"],
            ],
            label![
              C!["form-check-label"],
              attrs!{
                At::For => "addressAuto"
              },
              "Auto"
            ]
          ],
          div![
            C!["form-check"],
            div![
              C!["form-row align-items-center my-2"],
              div![
                C!["col-auto"],
                input![
                  C!["form-check-input"],
                  attrs!{
                    At::Type => "radio",
                    At::Name => "addressRadio",
                    At::Value => "single",
                  },
                  match &register.address {
                    Address::Single(_) => attrs!{ At::Checked => "checked"},
                    _ => attrs!{},
                  },
                  id!["addressSingle"],
                ],
                label![
                  C!["form-check-label"],
                  attrs!{
                    At::For => "addressSingle"
                  },
                  "Single:"
                ]
              ],
              div![
                C!["col-auto"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => "singleValue",
                  },
                  "Value"
                ],
              ],
              div![
                C!["col-auto"],
                input![
                  C!["form-control"],
                  attrs!{
                    At::Type => "text",
                    At::Id => "singleValue",
                  },
                  match &register.address {
                    Address::Single(v) => attrs!{ At::Value => &v.to_string()},
                    _ => attrs!{At::Disabled => "disabled"},
                  },
        //            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
                  div![
                    C!["invalid-feedback"],
                    "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
                  ],
                ],
              ],
            ],
          ],
          div![
            C!["form-check"],
            div![
              C!["form-row align-items-center"],
              div![
                C!["col-auto"],
                input![
                  C!["form-check-input"],
                  attrs!{
                    At::Type => "radio",
                    At::Name => "addressRadio",
                    At::Value => "stride",
                  },
                  match &register.address {
                    Address::Stride(_) => attrs!{ At::Checked => "checked"},
                    _ => attrs!{},
                  },
                  id!["addressStride"],
                ],
                label![
                  C!["form-check-label"],
                  attrs!{
                    At::For => "addressStride"
                  },
                  "Stride:"
                ],
              ],
              div![
                C!["col-auto"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => "strideStart",
                  },
                  "Start"
                ],
              ],
              div![
                C!["col-auto"],
                input![
                  C!["form-control"],
                  attrs!{
                    At::Type => "text",
                    At::Id => "strideStart",
                  },
                  match &register.address {
                    Address::Stride(s) => attrs!{ At::Value => &s.value.to_string()},
                    _ => attrs!{At::Disabled => "disabled"},
                  },
        //            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
                  div![
                    C!["invalid-feedback"],
                    "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
                  ],
                ],
              ],
              div![
                C!["col-auto"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => "strideCount",
                  },
                  "Count"
                ],
              ],
              div![
                C!["col-auto"],
                input![
                  C!["form-control"],
                  attrs!{
                    At::Type => "text",
                    At::Id => "strideCount",
                  },
                  match &register.address {
                    Address::Stride(s) => attrs!{ At::Value => &s.count.to_string()},
                    _ => attrs!{At::Disabled => "disabled"},
                  },
        //            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
                  div![
                    C!["invalid-feedback"],
                    "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
                  ],
                ],
              ],
              div![
                C!["col-auto"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => "strideIncr",
                  },
                  "Increment"
                ],
              ],
              div![
                C!["col-auto"],
                input![
                  C!["form-control"],
                  attrs!{
                    At::Type => "text",
                    At::Id => "strideIncr",
                  },
                  match &register.address {
                    Address::Stride(s) => 
                      match &s.increment {
                        None => attrs!{ At::Value => ""},
                        Some(incr) => attrs!{ At::Value => &incr.to_string()},
                      }
                      
                    _ => attrs!{At::Disabled => "disabled"},
                  },
        //            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
                  div![
                    C!["invalid-feedback"],
                    "please use a decimal, hexadecimal (0x*) or binary (0b*) value or leave empty for auto"
                  ],
                ],
              ],
            ]
          ],    
        ],
      ],
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => "inputSummary",
          },
          "Summary"
        ],
        div![
          C!["col-sm-10"],
          textarea![
            C!["form-control"],
            attrs!{
              At::Type => "text",
              At::Id => "inputSummary",
              At::Value => utils::opt_vec_str_to_textarea(&register.summary),
            },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::DescriptionChanged(index, input))),
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
              At::Value => utils::opt_vec_str_to_textarea(&register.description),
            },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::DescriptionChanged(index, input))),
          ]
        ]
      ], 
      div![
        C!["form-group row"],
        label![
          C!["col-sm-2 col-form-label"],
          attrs!{
            At::For => ID_REG_WIDTH,
          },
          "Width (bits)"
        ],
        div![
          C!["col-sm-10"],
          input![
            C!["form-control"],
            attrs!{
              At::Type => "text",
              At::Id => ID_REG_WIDTH,
              At::Value => match &interface.data_width {
                None => String::new(),
                Some(width) => width.to_string(),
              },
            },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::DataWidthChanged(index, input))),
          ],
          div![
            C!["invalid-feedback"],
            "please write a decimal value or leave empty for automatic when using fields"
          ],
        ]
      ],
    ],
    div![
      C!["form-group row"],
      label![
        C!["col-sm-2 col-form-label"],
        attrs!{
          At::For => "inputAccess",
        },
        "Access type"
      ],
      div![
        C!["col-sm-10"],
        select![
          C!["form-control"],
          attrs!{
            At::Id => "inputAccess",
            At::Value => match &register.access {
              Some(access) => access.to_string(),
              None => TXT_SPEC_IN_FIELDS.to_string()
            },
          },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::TypeChanged(index, input))),
          option![
            IF!(&register.access == &Option::<AccessType>::None  =>
              attrs!{
                At::Selected => "selected",
              }),
            TXT_SPEC_IN_FIELDS,
          ],
          AccessType::iter().map(|access_type|
            option![
              IF!(&register.access == &Some(access_type)=>
                attrs!{
                  At::Selected => "selected",
                }),
              access_type.to_string(),
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
          At::For => "inputSignal",
        },
        "Signal type"
      ],
      div![
        C!["col-sm-10"],
        select![
          C!["form-control"],
          attrs!{
            At::Id => "inputSignal",
            At::Value => match &register.signal {
              Some(signal) => signal.to_string(),
              None => TXT_SPEC_IN_FIELDS.to_string()
            },
          },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::TypeChanged(index, input))),
          option![
            IF!(&register.signal == &Option::<SignalType>::None  =>
              attrs!{
                At::Selected => "selected",
              }),
            TXT_SPEC_IN_FIELDS,
          ],
          SignalType::iter().map(|signal_type|
            option![
              IF!(&register.signal == &Some(signal_type)=>
                attrs!{
                  At::Selected => "selected",
                }),
              signal_type.to_string(),
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
          At::For => ID_RESET_VALUE,
        },
        "Reset value"
      ],
      div![
        C!["col-sm-10"],
        input![
          C!["form-control"],
          attrs!{
            At::Type => "text",
            At::Id => ID_RESET_VALUE,
          },
          match &register.reset {
            Some(v) => attrs!{ At::Value => &v.to_string()},
            _ => attrs!{},
          },
  //            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::NameChanged(index, input))),
        ],
        div![
          C!["invalid-feedback"],
          "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
        ],
      ],
    ],
  
    div![
      C!["form-group row"],
      label![
        C!["col-sm-2 col-form-label"],
        attrs!{
          At::For => "inputLocation",
        },
        "Location"
      ],
      div![
        C!["col-sm-10"],
        select![
          C!["form-control"],
          attrs!{
            At::Id => "inputLocation",
            At::Value => match &register.location {
              Some(loc) => loc.to_string(),
              None => TXT_SPEC_IN_FIELDS.to_string()
            },
          },
//            input_ev(Ev::Change, move | input | Msg::Interface(InterfaceMsg::TypeChanged(index, input))),
          option![
            IF!(&register.location == &Option::<LocationType>::None  =>
              attrs!{
                At::Selected => "selected",
              }),
            TXT_SPEC_IN_FIELDS,
          ],
          LocationType::iter().map(|location_type|
            option![
              IF!(&register.location == &Some(location_type)=>
                attrs!{
                  At::Selected => "selected",
                }),
              location_type.to_string(),
            ] 
          ).collect::<Vec<_>>(),
        ]
      ]
    ],

    div![
      C!["form-group row"],
      label![
        C!["col-sm-2 col-form-label"],
        "Core signal properties"
      ],
      div![
        C!["col-sm-10"],
        div![
          C!["form-check"],
          input![
            C!["form-check-input"],
            attrs!{
              At::Type => "checkbox",
              At::Value => "",
              At::Id => "inputUseReadEnable",
            },
            IF!(register.core_signal_properties.use_read_enable == Some(true) =>
              attrs!{ At::Checked => "checked"}),
          ],
          label![
            C!["form-check-label"],
            attrs!{
              At::For => "inputUseReadEnable"
            },
            "Use read enable signal"
          ],
        ],
        div![
          C!["form-check"],
          input![
            C!["form-check-input"],
            attrs!{
              At::Type => "checkbox",
              At::Value => "",
              At::Id => "inputUseWriteEnable",
            },
            IF!(register.core_signal_properties.use_write_enable == Some(true) =>
              attrs!{ At::Checked => "checked"}),
          ],
          label![
            C!["form-check-label"],
            attrs!{
              At::For => "inputUseWriteEnable"
            },
            "Use write enable signal"
          ],
        ],
      ],
    ],


/*      div![
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
          C!["col-sm-10"],
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
    ]*/
  ]
}
/*
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