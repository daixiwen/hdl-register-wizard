//! page to edit a register in the model, with optional fields list

use seed::{prelude::*, *};

use super::super::Model;
use super::super::PageType;

use super::super::mdf_format::AccessType;
use super::super::mdf_format::Address;
use super::super::mdf_format::AddressStride;
use super::super::mdf_format::LocationType;
use super::super::mdf_format::RadixType;
use super::super::mdf_format::Register;
use super::super::mdf_format::SignalType;
use super::super::mdf_format::VectorValue;

use super::super::Msg;

use super::super::utils;
use super::html_elements;

use std::str::FromStr;

// URL constants
const URL_NEW: &str = "new";

// ID constants
const ID_REG_WIDTH: &str = "inputRegisterWidth";
const ID_RESET_VALUE: &str = "inputResetValue";
const ID_ADDR_SINGLE_VALUE: &str = "addrSingleValue";
const ID_ADDR_STRIDE_VALUE: &str = "addrStrideValue";
const ID_ADDR_STRIDE_COUNT: &str = "addrStrideCount";
const ID_ADDR_STRIDE_INCR: &str = "addrStrideIncrement";

// text constant
const TXT_SPEC_IN_FIELDS: &str = "(specify in fields)";

// ------ ------
//     Urls
// ------ ------
/// different pages, each having its url within an interface
pub enum RegisterPage {
    /// edit the register with the given number
    Num(usize),
    /// create a register
    NewRegister,
}

/// generate an url for a specific register page, built upon an interface url
pub fn register_url(url: Url, register_page: RegisterPage) -> Url {
    match register_page {
        RegisterPage::Num(register_number) => url.add_path_part(register_number.to_string()),

        RegisterPage::NewRegister => url.add_path_part(URL_NEW),
    }
}

/// called when the url is changed to a register one
pub fn change_url(
    mut url: seed::browser::url::Url,
    interface_num: usize,
    model: &mut Model,
) -> PageType {
    match url.next_path_part() {
        None => PageType::NotFound,
        Some(URL_NEW) => new_register(interface_num, model),
        Some(number_string) => match number_string.parse::<usize>() {
            Ok(index) => {
                if index < model.mdf_data.interfaces[interface_num].registers.len() {
                    PageType::Register(interface_num, index)
                } else {
                    PageType::NotFound
                }
            }
            Err(_) => PageType::NotFound,
        },
    }
}

fn new_register(interface: usize, model: &mut Model) -> PageType {
    model.mdf_data.interfaces[interface]
        .registers
        .push(Register::new());
    let new_page_type = PageType::Register(
        interface,
        model.mdf_data.interfaces[interface].registers.len() - 1,
    );

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
    CorePropWriteEnable(usize, web_sys::Event),
}

/// process a register message
pub fn update(
    msg: RegisterMsg,
    interface_num: usize,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
) {
    match msg {
        RegisterMsg::Delete(index) => {
            if index < model.mdf_data.interfaces[interface_num].registers.len() {
                model.mdf_data.interfaces[interface_num]
                    .registers
                    .remove(index);
            }
        }
        RegisterMsg::MoveUp(index) => {
            if (index < model.mdf_data.interfaces[interface_num].registers.len()) && (index > 0) {
                model.mdf_data.interfaces[interface_num]
                    .registers
                    .swap(index - 1, index);
            }
        }
        RegisterMsg::MoveDown(index) => {
            if index < model.mdf_data.interfaces[interface_num].registers.len() - 1 {
                model.mdf_data.interfaces[interface_num]
                    .registers
                    .swap(index, index + 1);
            }
        }

        RegisterMsg::NameChanged(index, new_name) => {
            model.mdf_data.interfaces[interface_num].registers[index].name = new_name;
            orders.skip();
        }
        RegisterMsg::AddressAutoSelected(index) => {
            model.mdf_data.interfaces[interface_num].registers[index].address = Address::Auto;
        }
        RegisterMsg::AddressSingleSelected(index) => {
            model.mdf_data.interfaces[interface_num].registers[index].address =
                match &model.mdf_data.interfaces[interface_num].registers[index].address {
                    Address::Stride(stride) => Address::Single(stride.value.clone()),
                    _ => Address::Single(VectorValue::new()),
                }
            // no skipping so that the view is refreshed and the correct inputs activated/deactivated
        }
        RegisterMsg::AddressStrideSelected(index) => {
            model.mdf_data.interfaces[interface_num].registers[index].address =
                match &model.mdf_data.interfaces[interface_num].registers[index].address {
                    Address::Single(single) => Address::Stride(AddressStride {
                        value: single.clone(),
                        count: VectorValue {
                            value: 1,
                            radix: RadixType::Decimal,
                        },
                        increment: None,
                    }),
                    _ => Address::Stride(AddressStride {
                        value: VectorValue::new(),
                        count: VectorValue {
                            value: 1,
                            radix: RadixType::Decimal,
                        },
                        increment: None,
                    }),
                }
            // no skipping so that the view is refreshed and the correct inputs activated/deactivated
        }

        RegisterMsg::SummaryChanged(index, new_summary) => {
            model.mdf_data.interfaces[interface_num].registers[index].summary =
                utils::textarea_to_opt_vec_str(&new_summary);

            orders.skip();
        }
        RegisterMsg::DescriptionChanged(index, new_description) => {
            model.mdf_data.interfaces[interface_num].registers[index].description =
                utils::textarea_to_opt_vec_str(&new_description);

            orders.skip();
        }

        RegisterMsg::AddrSingleChanged(index, new_addr) => {
            orders.skip();

            match utils::validate_field(ID_ADDR_SINGLE_VALUE, &new_addr, |field_value| {
                VectorValue::from_str(field_value)
            }) {
                Ok(value) => {
                    model.mdf_data.interfaces[interface_num].registers[index].address =
                        Address::Single(value)
                }
                Err(_) => (),
            };
        }

        RegisterMsg::AddrStrideValueChanged(index, new_addr) => {
            orders.skip();

            match &model.mdf_data.interfaces[interface_num].registers[index].address {
                Address::Stride(stride) => {
                    match utils::validate_field(ID_ADDR_STRIDE_VALUE, &new_addr, |field_value| {
                        VectorValue::from_str(field_value)
                    }) {
                        Ok(value) => {
                            model.mdf_data.interfaces[interface_num].registers[index].address =
                                Address::Stride(AddressStride {
                                    value,
                                    count: stride.count,
                                    increment: stride.increment,
                                })
                        }
                        Err(_) => (),
                    }
                }
                _ => (),
            }
        }

        RegisterMsg::AddrStrideCountChanged(index, new_count) => {
            orders.skip();

            match &model.mdf_data.interfaces[interface_num].registers[index].address {
                Address::Stride(stride) => {
                    match utils::validate_field(ID_ADDR_STRIDE_COUNT, &new_count, |field_value| {
                        VectorValue::from_str(field_value)
                    }) {
                        Ok(count) => {
                            model.mdf_data.interfaces[interface_num].registers[index].address =
                                Address::Stride(AddressStride {
                                    value: stride.value,
                                    count: count,
                                    increment: stride.increment,
                                })
                        }
                        Err(_) => (),
                    }
                }
                _ => (),
            }
        }

        RegisterMsg::AddrStrideIncrementChanged(index, new_count) => {
            orders.skip();

            match &model.mdf_data.interfaces[interface_num].registers[index].address {
                Address::Stride(stride) => {
                    match utils::validate_field(ID_ADDR_STRIDE_INCR, &new_count, |field_value| {
                        utils::option_vectorval_from_str(field_value)
                    }) {
                        Ok(increment) => {
                            model.mdf_data.interfaces[interface_num].registers[index].address =
                                Address::Stride(AddressStride {
                                    value: stride.value,
                                    count: stride.count,
                                    increment,
                                })
                        }
                        Err(_) => (),
                    }
                }
                _ => (),
            }
        }

        RegisterMsg::WidthChanged(index, new_width) => {
            orders.skip();

            match utils::validate_field(ID_REG_WIDTH, &new_width, |field_value| {
                utils::option_num_from_str(field_value)
            }) {
                Ok(width) => {
                    model.mdf_data.interfaces[interface_num].registers[index].width = width
                }
                Err(_) => (),
            }
        }

        RegisterMsg::AccessTypeChanged(index, new_type_name) => {
            match AccessType::from_str(&new_type_name) {
                Ok(new_type) => {
                    model.mdf_data.interfaces[interface_num].registers[index].access =
                        Some(new_type);

                    // put a default value for use_write_enabled if it is not set yet
                    if (new_type != AccessType::RO)
                        && (model.mdf_data.interfaces[interface_num].registers[index].location
                            == Some(LocationType::Core))
                        && (model.mdf_data.interfaces[interface_num].registers[index]
                            .core_signal_properties
                            .use_write_enable
                            .is_none())
                    {
                        model.mdf_data.interfaces[interface_num].registers[index]
                            .core_signal_properties
                            .use_write_enable = Some(true);
                    }
                }

                _ => {
                    if new_type_name == TXT_SPEC_IN_FIELDS {
                        model.mdf_data.interfaces[interface_num].registers[index].access = None;
                    } else {
                        seed::log!("error while converting from string to interface type")
                    }
                }
            }
        }

        RegisterMsg::SignalTypeChanged(index, new_type_name) => {
            orders.skip();

            match SignalType::from_str(&new_type_name) {
                Ok(new_type) => {
                    model.mdf_data.interfaces[interface_num].registers[index].signal =
                        Some(new_type);
                }

                _ => {
                    if new_type_name == TXT_SPEC_IN_FIELDS {
                        model.mdf_data.interfaces[interface_num].registers[index].signal = None;
                    } else {
                        seed::log!("error while converting from string to signal type")
                    }
                }
            }
        }

        RegisterMsg::ResetValueChanged(index, new_value) => {
            orders.skip();

            match utils::validate_field(ID_RESET_VALUE, &new_value, |field_value| {
                utils::option_vectorval_from_str(field_value)
            }) {
                Ok(reset_value) => {
                    model.mdf_data.interfaces[interface_num].registers[index].reset = reset_value
                }
                Err(_) => (),
            }
        }

        RegisterMsg::LocationChanged(index, new_location_name) => {
            match LocationType::from_str(&new_location_name) {
                Ok(location) => {
                    model.mdf_data.interfaces[interface_num].registers[index].location =
                        Some(location);
                    // put a default value for use_write_enabled if it is not set yet
                    if (location == LocationType::Core)
                        && (model.mdf_data.interfaces[interface_num].registers[index].access
                            != Some(AccessType::RO))
                        && (model.mdf_data.interfaces[interface_num].registers[index]
                            .core_signal_properties
                            .use_write_enable
                            .is_none())
                    {
                        model.mdf_data.interfaces[interface_num].registers[index]
                            .core_signal_properties
                            .use_write_enable = Some(true);
                    }
                }

                _ => {
                    if new_location_name == TXT_SPEC_IN_FIELDS {
                        model.mdf_data.interfaces[interface_num].registers[index].location = None;
                    } else {
                        seed::log!("error while converting from string to location")
                    }
                }
            }
        }

        RegisterMsg::CorePropReadEnable(index, event) => {
            model.mdf_data.interfaces[interface_num].registers[index]
                .core_signal_properties
                .use_read_enable = Some(utils::target_checked(&event));
        }

        RegisterMsg::CorePropWriteEnable(index, event) => {
            model.mdf_data.interfaces[interface_num].registers[index]
                .core_signal_properties
                .use_write_enable = Some(utils::target_checked(&event));
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
        C!["my-3  cstm-big-btn"],
        html_elements::toolbar_button_url(
            "back",
            &super::super::Urls::new(&model.base_url).from_page_type(PageType::Interface(interface_index)),
            true
        ),
    ],
    h3![
      C!["my-2"],
      "Register in Interface ", &interface.name],
    div![
        html_elements::text_field_full_line(
            "inputName",
            "Name",
            &register.name,
            move | input | Msg::Register(interface_index, RegisterMsg::NameChanged(register_index, input)),
            None
        ),
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
            C!["form-check my-2"],
            div![
              C!["form-row align-items-center form-inline"],
              div![
                C!["col-auto flex-nowrap form-group ml-n4"],
                input![
                  C!["form-check-input ml-1"],
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
              html_elements::text_field_sub_line(
                ID_ADDR_SINGLE_VALUE,
                "Value",
                & match &register.address {
                    Address::Single(v) => v.to_string(),
                    _ => String::new() },
                match &register.address {
                    Address::Single(_) => false,
                    _ => true },
                move | input | Msg::Register(interface_index, RegisterMsg::AddrSingleChanged(register_index, input)),
                Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value")),
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
              html_elements::text_field_sub_line(
                  ID_ADDR_STRIDE_VALUE,
                  "Start",
                  & match &register.address {
                      Address::Stride(s) => s.value.to_string(),
                      _ => String::new(),
                    },
                  match &register.address {
                      Address::Stride(_) => false,
                      _ => true },
                  move | input | Msg::Register(interface_index, RegisterMsg::AddrStrideValueChanged(register_index, input)),
                  Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value")
              ),
              html_elements::text_field_sub_line(
                  ID_ADDR_STRIDE_COUNT,
                  "Count",
                  & match &register.address {
                      Address::Stride(s) => s.count.to_string(),
                      _ => String::new(),
                    },
                  match &register.address {
                      Address::Stride(_) => false,
                      _ => true },
                  move | input | Msg::Register(interface_index, RegisterMsg::AddrStrideCountChanged(register_index, input)),
                  Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value")
              ),
              html_elements::text_field_sub_line(
                  ID_ADDR_STRIDE_INCR,
                  "Increment",
                  & match &register.address {
                      Address::Stride(s) =>
                        match &s.increment {
                          None => String::new(),
                          Some(incr) => incr.to_string()},
                      _ => String::new(),
                      },
                  match &register.address {
                      Address::Stride(_) => false,
                      _ => true },
                  move | input | Msg::Register(interface_index, RegisterMsg::AddrStrideIncrementChanged(register_index, input)),
                  Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value or leave empty for auto")
              ),
            ]
          ],
        ],
      ],
      html_elements::textarea_field(
          "inputSummary",
          "Summary",
          &utils::opt_vec_str_to_textarea(&register.summary),
          move | input | Msg::Register(interface_index,RegisterMsg::SummaryChanged(register_index, input))
      ),
      html_elements::textarea_field(
          "inputDescription",
          "Description",
          &utils::opt_vec_str_to_textarea(&register.description),
          move | input | Msg::Register(interface_index,RegisterMsg::DescriptionChanged(register_index, input))
      ),
    ],
    div![
        C!["form-group row"],
        div![
            C!["col-sm-2 col-form-label"],
            "Signal"
        ],
        div![
            C!["col-sm-10"],
            div![
                C!["form-row align-items-center form-inline"],
            html_elements::select_option_field_sub_line(
                "inputSignal",
                "type:",
                &register.signal,
                TXT_SPEC_IN_FIELDS,
                move | input | Msg::Register(interface_index, RegisterMsg::SignalTypeChanged(register_index, input))
            ),
            html_elements::select_option_field_sub_line(
                "inputAccess",
                "access:",
                &register.access,
                TXT_SPEC_IN_FIELDS,
                move | input | Msg::Register(interface_index, RegisterMsg::AccessTypeChanged(register_index, input))
            ),
            html_elements::select_option_field_sub_line(
                "inputLocation",
                "location:",
                &register.location,
                TXT_SPEC_IN_FIELDS,
                move | input | Msg::Register(interface_index, RegisterMsg::LocationChanged(register_index, input))
            ),

            ]
        ]
    ],

    div![
        C!["form-group row"],
        div![
            C!["col-sm-2 col-form-label"],
            "Value"
        ],
        div![
            C!["col-sm-10"],
            div![
                C!["form-row align-items-center form-inline"],
                html_elements::text_field_sub_line(
                    ID_REG_WIDTH,
                    "Width (bits):",
                    & match &register.width {
                        None => String::new(),
                        Some(width) => width.to_string(),
                    },
                    false,
                    move | input | Msg::Register(interface_index, RegisterMsg::WidthChanged(register_index, input)),
                    Some("please write a decimal value or leave empty for automatic when using fields")
                ),
                html_elements::text_field_sub_line(
                    ID_RESET_VALUE,
                    "Reset:",
                    & match &register.reset {
                        None => String::new(),
                        Some(value) => value.to_string(),
                    },
                    false,
                    move | input | Msg::Register(interface_index, RegisterMsg::ResetValueChanged(register_index, input)),
                    Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value or leave empty when using fields")
                ),
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
