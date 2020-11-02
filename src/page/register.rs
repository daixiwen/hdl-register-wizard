//! page to edit a register in the model, with optional fields list

use seed::{prelude::*, *};

use super::super::Model;
use super::super::PageType;

use super::super::mdf_format::Register;
use super::super::mdf_format::Address;
use super::super::mdf_format::AddressStride;
use super::super::mdf_format::AccessType;
use super::super::mdf_format::SignalType;
use super::super::mdf_format::VectorValue;
use super::super::mdf_format::LocationType;
use super::super::mdf_format::RadixType;

use strum::IntoEnumIterator;
use super::super::Msg;

use super::super::utils;

use std::str::FromStr;

// URL constants
const URL_NEW: &str = "new";

// ID constants
const ID_REG_WIDTH: &str = "inputRegisterWidth";
const ID_RESET_VALUE: &str = "inputResetValue";
const ID_ADDR_SINGLE_VALUE: &str = "addrSingleValue";
const ID_ADDR_STRIDE_VALUE: &str = "addrStrideValue";
const ID_ADDR_STRIDE_COUNT: &str = "addrStrideCount";
const ID_ADDR_STRIDE_INCR : &str = "addrStrideIncrement";


// text constant
const TXT_SPEC_IN_FIELDS: &str ="(specify in fields)";

// ------ ------
//     Urls
// ------ ------
/// different pages, each having its url within an interface
pub enum RegisterPage {
    /// edit the register with the given number
  Num(usize),
    /// create a register
  NewRegister
}

/// generate an url for a specific register page, built upon an interface url
pub fn register_url (url: Url, register_page : RegisterPage) -> Url {
  match register_page {
    RegisterPage::Num(register_number) =>
      url.add_path_part(register_number.to_string()),

    RegisterPage::NewRegister =>
      url.add_path_part(URL_NEW),
  }
}

/// called when the url is changed to a register one
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

/// messages related to registers
#[derive(Clone)]
pub enum RegisterMsg {
      /// delete th register
    Delete(usize),
      /// move the register up in the list
    MoveUp(usize),
      /// move the register down in the list
    MoveDown(usize),
      /// register name changed
    NameChanged(usize, String),
      /// sent when the address type is changed to `auto`
    AddressAutoSelected(usize),
      /// sent when the address type is changed to single
    AddressSingleSelected(usize),
      /// sent when the address type is changed to stride
    AddressStrideSelected(usize),
      /// sent when the register summary is changed
    SummaryChanged(usize, String),
      /// sent when the register description is changed
    DescriptionChanged(usize, String),
      /// sent when the address value in single mode is changed
    AddrSingleChanged(usize, String),
      /// sent when the address start in stride mode is changed
    AddrStrideValueChanged(usize, String),
      /// sent when the address stride count is changed
    AddrStrideCountChanged(usize, String),
      /// sent when the address stride increase is changed
    AddrStrideIncrementChanged(usize, String),
      /// sent with the register width is changed
    WidthChanged(usize, String),
      /// sent when the access type of the register is changed
    AccessTypeChanged(usize, String),
      /// sent when the signal type for the register is changed
    SignalTypeChanged(usize, String),
      /// sent when the register's reset value is changed
    ResetValueChanged(usize, String),
      /// sent when the register's location is changed
    LocationChanged(usize, String),
      /// sent when the read enable core property is changed
    CorePropReadEnable(usize, web_sys::Event),
      /// sent when the write enable core property is changed
    CorePropWriteEnable(usize, web_sys::Event)
}

/// process a register message
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

    RegisterMsg::NameChanged(index, new_name) => {
      model.mdf_data.interfaces[interface_num].registers[index].name = new_name;
      orders.skip();
    },
    RegisterMsg::AddressAutoSelected(index) => {
      model.mdf_data.interfaces[interface_num].registers[index].address = 
        Address::Auto;
    },
    RegisterMsg::AddressSingleSelected(index) => {
      model.mdf_data.interfaces[interface_num].registers[index].address =
          match &model.mdf_data.interfaces[interface_num].registers[index].address {

        Address::Stride(stride) =>
             Address::Single(stride.value.clone()),
        _ => Address::Single(VectorValue::new()),
      }
      // no skipping so that the view is refreshed and the correct inputs activated/deactivated
    },
    RegisterMsg::AddressStrideSelected(index) => {
      model.mdf_data.interfaces[interface_num].registers[index].address =
          match &model.mdf_data.interfaces[interface_num].registers[index].address {

        Address::Single(single) =>
          Address::Stride(AddressStride{
            value : single.clone(),
            count : VectorValue{value: 1, radix: RadixType::Decimal},
            increment : None}),
        _ =>
          Address::Stride(AddressStride{
            value : VectorValue::new(),
            count : VectorValue{value: 1, radix: RadixType::Decimal},
            increment : None}),
      }
      // no skipping so that the view is refreshed and the correct inputs activated/deactivated
    },

    RegisterMsg::SummaryChanged(index, new_summary) => {
      model.mdf_data.interfaces[interface_num].registers[index].summary =
          utils::textarea_to_opt_vec_str(&new_summary);

      orders.skip();
    },
    RegisterMsg::DescriptionChanged(index, new_description) => {
      model.mdf_data.interfaces[interface_num].registers[index].description =
          utils::textarea_to_opt_vec_str(&new_description);

      orders.skip();
    },

    RegisterMsg::AddrSingleChanged(index, new_addr) => {
      orders.skip();

      match utils::validate_field(
          ID_ADDR_SINGLE_VALUE, &new_addr, | field_value |VectorValue::from_str(field_value)) {
        Ok(value) => model.mdf_data.interfaces[interface_num].registers[index].address = Address::Single(value),
        Err(_) => ()
      };
    },

    RegisterMsg::AddrStrideValueChanged(index, new_addr) => {
      orders.skip();

      match &model.mdf_data.interfaces[interface_num].registers[index].address {
        Address::Stride(stride) =>
          match utils::validate_field(
              ID_ADDR_STRIDE_VALUE, &new_addr, | field_value |VectorValue::from_str(field_value)) {
            Ok(value) => model.mdf_data.interfaces[interface_num].registers[index].address =
              Address::Stride(AddressStride{
                value, count: stride.count, increment: stride.increment
              }),
            Err(_) => ()
          },
        _ => ()
      }
    },

    RegisterMsg::AddrStrideCountChanged(index, new_count) => {
      orders.skip();

      match &model.mdf_data.interfaces[interface_num].registers[index].address {
        Address::Stride(stride) =>
          match utils::validate_field(
              ID_ADDR_STRIDE_COUNT, &new_count, | field_value |VectorValue::from_str(field_value)) {
            Ok(count) => model.mdf_data.interfaces[interface_num].registers[index].address =
              Address::Stride(AddressStride{
                value: stride.value, count: count, increment: stride.increment
              }),
            Err(_) => ()
          },
        _ => ()
      }
    },

    RegisterMsg::AddrStrideIncrementChanged(index, new_count) => {
      orders.skip();

      match &model.mdf_data.interfaces[interface_num].registers[index].address {
        Address::Stride(stride) =>
          match utils::validate_field(
              ID_ADDR_STRIDE_INCR, &new_count, | field_value | utils::option_vectorval_from_str(field_value)) {
            Ok(increment) => model.mdf_data.interfaces[interface_num].registers[index].address =
              Address::Stride(AddressStride{
                value: stride.value, count: stride.count, increment
              }),
            Err(_) => ()
          },
        _ => ()
      }
    },

    RegisterMsg::WidthChanged(index, new_width) => {
      orders.skip();

      match utils::validate_field(
          ID_REG_WIDTH, &new_width, | field_value | utils::option_num_from_str(field_value)) {
        Ok(width) => model.mdf_data.interfaces[interface_num].registers[index].width = width,
        Err(_) => ()
      }
    },

    RegisterMsg::AccessTypeChanged(index, new_type_name) => {
      match AccessType::from_str(&new_type_name)
      {
        Ok(new_type) => {
          model.mdf_data.interfaces[interface_num].registers[index].access = Some(new_type);

          // put a default value for use_write_enabled if it is not set yet
          if (new_type != AccessType::RO) && 
              (model.mdf_data.interfaces[interface_num].registers[index].location == Some(LocationType::Core)) &&
              (model.mdf_data.interfaces[interface_num].registers[index].core_signal_properties.use_write_enable.is_none())
          {
            model.mdf_data.interfaces[interface_num].registers[index].core_signal_properties.use_write_enable = Some(true);
          }
        },

        _ => {
          if new_type_name == TXT_SPEC_IN_FIELDS {
            model.mdf_data.interfaces[interface_num].registers[index].access = None;
          }
          else {
            seed::log!("error while converting from string to interface type")
          }
        },
      }
    },

    RegisterMsg::SignalTypeChanged(index, new_type_name) => {
      orders.skip();

      match SignalType::from_str(&new_type_name)
      {
        Ok(new_type) => {
          model.mdf_data.interfaces[interface_num].registers[index].signal = Some(new_type);
        },

        _ => {
          if new_type_name == TXT_SPEC_IN_FIELDS {
            model.mdf_data.interfaces[interface_num].registers[index].signal = None;
          }
          else {
            seed::log!("error while converting from string to signal type")
          }
        },
      }
    },

    RegisterMsg::ResetValueChanged(index, new_value) => {
      orders.skip();

      match utils::validate_field(
          ID_RESET_VALUE, &new_value, | field_value | utils::option_vectorval_from_str(field_value)) {
        Ok(reset_value) => model.mdf_data.interfaces[interface_num].registers[index].reset = reset_value,
        Err(_) => ()
      }
    },

    RegisterMsg::LocationChanged(index, new_location_name) => {
      match LocationType::from_str(&new_location_name)
      {
        Ok(location) => {
          model.mdf_data.interfaces[interface_num].registers[index].location = Some(location);
          // put a default value for use_write_enabled if it is not set yet
          if (location == LocationType::Core) && 
              (model.mdf_data.interfaces[interface_num].registers[index].access != Some(AccessType::RO)) &&
              (model.mdf_data.interfaces[interface_num].registers[index].core_signal_properties.use_write_enable.is_none())
          {
            model.mdf_data.interfaces[interface_num].registers[index].core_signal_properties.use_write_enable = Some(true);
          }
        },

        _ => {
          if new_location_name == TXT_SPEC_IN_FIELDS {
            model.mdf_data.interfaces[interface_num].registers[index].location = None;
          }
          else {
            seed::log!("error while converting from string to location")
          }
        },
      }
    },

    RegisterMsg::CorePropReadEnable(index, event) => {

      model.mdf_data.interfaces[interface_num].registers[index]
          .core_signal_properties.use_read_enable = 
          Some(utils::target_checked(&event));
    }

    RegisterMsg::CorePropWriteEnable(index, event) => {

      model.mdf_data.interfaces[interface_num].registers[index]
          .core_signal_properties.use_write_enable = 
          Some(utils::target_checked(&event));
    }
  }
}


// ------ ------
//     View
// ------ ------


/// build an html view for the register
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
            input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::NameChanged(register_index, input))),
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
              ev(Ev::Click, move | _ | Msg::Register(interface_index, RegisterMsg::AddressAutoSelected(register_index))),
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
                  ev(Ev::Change, move | _ | Msg::Register(interface_index, RegisterMsg::AddressSingleSelected(register_index))),
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
                    At::For => ID_ADDR_SINGLE_VALUE,
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
                    At::Id => ID_ADDR_SINGLE_VALUE,
                  },
                  match &register.address {
                    Address::Single(v) => attrs!{ At::Value => &v.to_string()},
                    _ => attrs!{At::Disabled => "disabled"},
                  },
                  input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::AddrSingleChanged(register_index, input))),
                ],
                div![
                  C!["invalid-feedback"],
                  "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
                ],
              ],
            ],
          ],
          div![
            C!["form-check"],
            div![
              C!["form-row align-items-center form-inline"],
              div![
                C!["col-auto flex-nowrap form-group ml-n4"],
                input![
                  C!["form-check-input ml-1"],
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
                  ev(Ev::Click, move | _ | Msg::Register(interface_index, RegisterMsg::AddressStrideSelected(register_index))),
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
                C!["col-auto  flex-nowrap form-group"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => ID_ADDR_STRIDE_VALUE,
                  },
                  "Start"
                ],
                div![
                  C!["m-2"],
                  input![
                    C!["form-control"],
                    attrs!{
                      At::Type => "text",
                      At::Id => ID_ADDR_STRIDE_VALUE,
                    },
                    match &register.address {
                      Address::Stride(s) => attrs!{ At::Value => &s.value.to_string()},
                      _ => attrs!{At::Disabled => "disabled"},
                    },
                    input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::AddrStrideValueChanged(register_index, input))),
                  ],
                  div![
                    C!["invalid-feedback"],
                    "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
                  ],
                ],
              ],
              div![
                C!["col-auto flex-nowrap form-group"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => ID_ADDR_STRIDE_COUNT,
                  },
                  "Count"
                ],
                div![
                  C!["m-2"],
                  input![
                    C!["form-control"],
                    attrs!{
                      At::Type => "text",
                      At::Id => ID_ADDR_STRIDE_COUNT,
                    },
                    match &register.address {
                      Address::Stride(s) => attrs!{ At::Value => &s.count.to_string()},
                      _ => attrs!{At::Disabled => "disabled"},
                    },
                    input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::AddrStrideCountChanged(register_index, input))),
                  ],
                  div![
                    C!["invalid-feedback"],
                    "please use a decimal, hexadecimal (0x*) or binary (0b*) value"
                  ],
                ],
              ],
              div![
                C!["col-auto flex-nowrap form-group"],
                label![
                  C!["col-form-label"],
                  attrs!{
                    At::For => ID_ADDR_STRIDE_INCR,
                  },
                  "Increment"
                ],
                div![
                  C!["m-2"],
                  input![
                    C!["form-control m-2"],
                    attrs!{
                      At::Type => "text",
                      At::Id => ID_ADDR_STRIDE_INCR,
                    },
                    match &register.address {
                      Address::Stride(s) => 
                        match &s.increment {
                          None => attrs!{ At::Value => ""},
                          Some(incr) => attrs!{ At::Value => &incr.to_string()},
                        }
                        
                      _ => attrs!{At::Disabled => "disabled"},
                    },
                    input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::AddrStrideIncrementChanged(register_index, input))),
                  ],
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
            input_ev(Ev::Change, move | input | Msg::Register(interface_index,RegisterMsg::SummaryChanged(register_index, input))),
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
            input_ev(Ev::Change, move | input | Msg::Register(interface_index,RegisterMsg::DescriptionChanged(register_index, input))),
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
              At::Value => match &register.width {
                None => String::new(),
                Some(width) => width.to_string(),
              },
            },
            input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::WidthChanged(register_index, input))),
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
          input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::AccessTypeChanged(register_index, input))),
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
          input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::SignalTypeChanged(register_index, input))),
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
          input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::ResetValueChanged(register_index, input))),
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
          input_ev(Ev::Change, move | input | Msg::Register(interface_index, RegisterMsg::LocationChanged(register_index, input))),
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
            IF!((register.location == Some(LocationType::Pif)) ||
                (register.access == Some(AccessType::WO)) =>
              attrs!{ At::Disabled => "disabled"}),
            ev(Ev::Change, move | event | Msg::Register(interface_index, RegisterMsg::CorePropReadEnable(register_index, event))),
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
            IF!((register.location == Some(LocationType::Pif)) ||
                (register.access == Some(AccessType::RO)) =>
              attrs!{ At::Disabled => "disabled"}),
            ev(Ev::Change, move | event | Msg::Register(interface_index, RegisterMsg::CorePropWriteEnable(register_index, event))),
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
  ]
}
