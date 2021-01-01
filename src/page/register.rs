//! page to edit a register in the model, with optional fields list

use seed::{prelude::*, *};

use crate::Model;
use crate::PageType;
use crate::Urls;

use crate::mdf_format;
use crate::mdf_format::AccessType;
use crate::mdf_format::Address;
use crate::mdf_format::AddressStride;
use crate::mdf_format::LocationType;
use crate::mdf_format::RadixType;
use crate::mdf_format::Register;
use crate::mdf_format::SignalType;
use crate::mdf_format::VectorValue;

use crate::Msg;

use crate::utils;
use super::html_elements;
use super::field;

use std::str::FromStr;
use std::mem;

// URL constants
const URL_NEW: &str = "new";
const URL_FIELD: &str = "field";

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
) -> (PageType, Option<Msg>) {
    match url.next_path_part() {
        None => (PageType::NotFound, None),
        Some(URL_NEW) => new_register(interface_num, model),
        Some(number_string) => match number_string.parse::<usize>() {
            Ok(index) => {
                if index < model.mdf_data.interfaces[interface_num].registers.len() {
                    // check if we are just refering to the interface (URL stops here) ir a register (URL continues)
                    match url.next_path_part() {
                        None => (PageType::Register(interface_num, index), None),
                        Some(URL_FIELD) => super::field::change_url(url, interface_num, index, model),
                        Some(_) => (PageType::NotFound, None),
                    }
                } else {
                    (PageType::NotFound, None)
                }
            }
            Err(_) => (PageType::NotFound, None),
        },
    }
}

fn new_register(interface_num: usize, model: &mut Model) -> (PageType, Option<Msg>) {
    model.mdf_data.interfaces[interface_num].registers.push(Register::new());
    let register_num = model.mdf_data.interfaces[interface_num].registers.len() - 1;
    let new_page_type = PageType::Register(interface_num, register_num);

    // generate the undo action
    let undo = Msg::Register(interface_num, register_num, RegisterMsg::Delete);

    crate::Urls::new(model.base_url.clone())
        .from_page_type(new_page_type)
        .go_and_replace();
    (new_page_type, Some(undo))
}

// ------ ------
//    Update
// ------ ------

/// messages related to registers
#[derive(Clone)]
pub enum RegisterMsg {
    /// delete th register
    Delete,
    /// restore a deleted interface (undo)
    Restore(std::rc::Rc<Register>),
    /// move the register up in the list
    MoveUp,
    /// move the register down in the list
    MoveDown,
    /// register name changed
    NameChanged(String),
    /// restore an address (undo)
    RestoreAddress(Address),
    /// sent when the address type is changed to `auto`
    AddressAutoSelected,
    /// sent when the address type is changed to single
    AddressSingleSelected,
    /// sent when the address type is changed to stride
    AddressStrideSelected,
    /// sent when the register summary is changed
    SummaryChanged(String),
    /// sent when the register description is changed
    DescriptionChanged(String),
    /// sent when the address value in single mode is changed
    AddrSingleChanged(String),
    /// sent when the address start in stride mode is changed
    AddrStrideValueChanged(String),
    /// sent when the address stride count is changed
    AddrStrideCountChanged(String),
    /// sent when the address stride increase is changed
    AddrStrideIncrementChanged(String),
    /// sent with the register width is changed
    WidthChanged(String),
    /// sent when the access type of the register is changed
    AccessTypeChanged(String),
    /// sent when the signal type for the register is changed
    SignalTypeChanged(String),
    /// sent when the register's reset value is changed
    ResetValueChanged(String),
    /// sent when the register's location is changed
    LocationChanged(String),
    /// sent when the read enable core property is changed
    CorePropReadEnable(web_sys::Event),
    /// sent when the write enable core property is changed
    CorePropWriteEnable(web_sys::Event),
    /// restore the read enable core property (undo)
    RestoreCorePropReadEnable(Option<bool>),
    /// restore the write enable core property (undo)
    RestoreCorePropWriteEnable(Option<bool>),
}

/// process a register message
pub fn update(
    interface_num: usize,
    register_num: usize,
    msg: RegisterMsg,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
    )
            -> Option<Msg> {

    let num_registers = match msg {
        RegisterMsg::Restore(_) => model.mdf_data.interfaces[interface_num].registers.len() + 1,
        _ => model.mdf_data.interfaces[interface_num].registers.len() 
    };
    
    let registers = &mut model.mdf_data.interfaces[interface_num].registers;

    if register_num >= num_registers {
        None
    }
    else {
        match msg {
            RegisterMsg::Delete => {
                Some(Msg::Register(interface_num, register_num, RegisterMsg::Restore(
                    std::rc::Rc::new(
                        registers.remove(register_num)
                        ))))
            }

            RegisterMsg::Restore(register) => {
                match std::rc::Rc::<mdf_format::Register>::try_unwrap(register) {
                    Ok(register_obj) => { 
                        registers.insert(register_num,register_obj);
                        Some(Msg::Register(interface_num, register_num, RegisterMsg::Delete))
                    },
                    _ => {
                        seed::log!("error recovering register object");
                        None
                    },
                }
            }

            RegisterMsg::MoveUp => {
                if register_num > 0 {
                    registers.swap(register_num - 1, register_num);
                    Some(Msg::Register(interface_num, register_num-1, RegisterMsg::MoveDown))
                }
                else {
                    None
                }
            }

            RegisterMsg::MoveDown => {
                if register_num < num_registers - 1 {
                    registers.swap(register_num, register_num + 1);
                    Some(Msg::Register(interface_num, register_num+1, RegisterMsg::MoveUp))
                }
                else {
                    None
                }
            }

            RegisterMsg::NameChanged(new_name) => {
                let old_name = mem::replace(&mut registers[register_num].name, new_name);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::NameChanged(old_name)))
            }

            RegisterMsg::AddressAutoSelected => {
                let old_address = mem::replace(&mut registers[register_num].address, Address::Auto);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
            }

            RegisterMsg::AddressSingleSelected => {
                let new_address = match &registers[register_num].address {
                        Address::Stride(stride) => Address::Single(stride.value.clone()),
                        _ => Address::Single(VectorValue::new()),
                    };
                let old_address = mem::replace(&mut registers[register_num].address, new_address);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
            }

            RegisterMsg::AddressStrideSelected => {
                let new_address = match &registers[register_num].address {
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
                    };
                let old_address = mem::replace(&mut registers[register_num].address, new_address);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
            }

            RegisterMsg::RestoreAddress(new_address) => {
                let old_address = mem::replace(&mut registers[register_num].address, new_address);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
            }

            RegisterMsg::SummaryChanged(new_summary) => {
                let old_summary = utils::opt_vec_str_to_textarea(&registers[register_num].summary);
                registers[register_num].summary = utils::textarea_to_opt_vec_str(&new_summary);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::SummaryChanged(old_summary)))                
            }

            RegisterMsg::DescriptionChanged(new_description) => {
                let old_description = utils::opt_vec_str_to_textarea(&registers[register_num].description);
                registers[register_num].description = utils::textarea_to_opt_vec_str(&new_description);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::DescriptionChanged(old_description)))                
            }

            RegisterMsg::AddrSingleChanged(new_addr) => {
                match utils::validate_field(ID_ADDR_SINGLE_VALUE, &new_addr, |field_value| {
                    VectorValue::from_str(field_value)
                }) {
                    Ok(value) => {
                        let old_address = mem::replace(&mut registers[register_num].address, Address::Single(value));
                        Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
                    }
                    Err(_) => {
                        orders.skip();
                        None
                    }
                }
            }

            RegisterMsg::AddrStrideValueChanged(new_addr) => {
                match &registers[register_num].address {
                    Address::Stride(stride) => {
                        match utils::validate_field(ID_ADDR_STRIDE_VALUE, &new_addr, |field_value| {
                            VectorValue::from_str(field_value)
                        }) {
                            Ok(value) => {
                                let new_address = Address::Stride(AddressStride {
                                        value,
                                        count: stride.count,
                                        increment: stride.increment,
                                    });
                                let old_address = mem::replace(&mut registers[register_num].address, new_address);
                                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
                            }
                            Err(_) => {
                                orders.skip();
                                None
                            }
                        }
                    }
                    _ => None,
                }
            }

            RegisterMsg::AddrStrideCountChanged(new_count) => {
                match &registers[register_num].address {
                    Address::Stride(stride) => {
                        match utils::validate_field(ID_ADDR_STRIDE_COUNT, &new_count, |field_value| {
                            VectorValue::from_str(field_value)
                        }) {
                            Ok(count) => {
                                let new_address = Address::Stride(AddressStride {
                                        value: stride.value,
                                        count: count,
                                        increment: stride.increment,
                                    });
                                let old_address = mem::replace(&mut registers[register_num].address, new_address);
                                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
                            }
                            Err(_) => {
                                orders.skip();
                                None
                            }
                        }
                    }
                    _ => None,
                }
            }

            RegisterMsg::AddrStrideIncrementChanged(new_count) => {
                match &registers[register_num].address {
                    Address::Stride(stride) => {
                        match utils::validate_field(ID_ADDR_STRIDE_INCR, &new_count, |field_value| {
                            utils::option_vectorval_from_str(field_value)
                        }) {
                            Ok(increment) => {
                                let new_address = Address::Stride(AddressStride {
                                        value: stride.value,
                                        count: stride.count,
                                        increment,
                                    });
                                let old_address = mem::replace(&mut registers[register_num].address, new_address);
                                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreAddress(old_address)))
                            }
                            Err(_) => {
                                orders.skip();
                                None
                            }
                        }
                    }
                    _ => None,
                }
            }

            RegisterMsg::WidthChanged(new_width) => {
                match utils::validate_field(ID_REG_WIDTH, &new_width, |field_value| {
                    utils::option_num_from_str(field_value)
                }) {
                    Ok(width) => {
                        let old_width = mem::replace(&mut registers[register_num].width, width);
                        Some(Msg::Register(interface_num, register_num, RegisterMsg::WidthChanged(utils::option_type_to_str(&old_width))))
                    }
                    Err(_) => {
                        orders.skip();
                        None
                    }
                }
            }

            RegisterMsg::AccessTypeChanged(new_type_name) => {
                let old_type = match AccessType::from_str(&new_type_name) {
                    Ok(new_type) => {
                        // put a default value for use_write_enabled if it is not set yet
                        if (new_type != AccessType::RO)
                            && (registers[register_num].location == Some(LocationType::Core))
                            && (registers[register_num].core_signal_properties.use_write_enable.is_none()) {

                            registers[register_num].core_signal_properties.use_write_enable = Some(true);
                        }

                        mem::replace(&mut registers[register_num].access, Some(new_type))
                    }

                    _ => {
                        if new_type_name == TXT_SPEC_IN_FIELDS {
                            mem::replace(&mut registers[register_num].access, None)
                        } else {
                            seed::log!("error while converting from string to interface type");
                            registers[register_num].access
                        }
                    }
                };

                match old_type {
                    None => Some(Msg::Register(interface_num, register_num, RegisterMsg::AccessTypeChanged(TXT_SPEC_IN_FIELDS.to_string()))),
                    Some(type_elem) => Some(Msg::Register(interface_num, register_num, RegisterMsg::AccessTypeChanged(type_elem.to_string())))
                }
            }

            RegisterMsg::SignalTypeChanged(new_type_name) => {
                let old_signal = match SignalType::from_str(&new_type_name) {
                    Ok(new_type) => {
                         mem::replace(&mut registers[register_num].signal, Some(new_type))
                    }

                    _ => {
                        if new_type_name == TXT_SPEC_IN_FIELDS {
                            mem::replace(&mut registers[register_num].signal, None)
                        } else {
                            seed::log!("error while converting from string to signal type");
                            registers[register_num].signal
                        }
                    }
                };

                match old_signal {
                    None => Some(Msg::Register(interface_num, register_num, RegisterMsg::SignalTypeChanged(TXT_SPEC_IN_FIELDS.to_string()))),
                    Some(type_elem) => Some(Msg::Register(interface_num, register_num, RegisterMsg::SignalTypeChanged(type_elem.to_string())))
                }
            }

            RegisterMsg::ResetValueChanged(new_value) => {

                match utils::validate_field(ID_RESET_VALUE, &new_value, |field_value| {
                    utils::option_vectorval_from_str(field_value)
                }) {
                    Ok(reset_value) => {
                        let old_value = mem::replace(&mut registers[register_num].reset, reset_value);
                        Some(Msg::Register(interface_num, register_num, RegisterMsg::ResetValueChanged(utils::option_type_to_str(&old_value))))
                    }
                    Err(_) => {
                        orders.skip();
                        None
                    }
                }
            }

            RegisterMsg::LocationChanged(new_location_name) => {
                let old_location = match LocationType::from_str(&new_location_name) {
                    Ok(location) => {

                        // put a default value for use_write_enabled if it is not set yet
                        if (location == LocationType::Core)
                            && (registers[register_num].access
                                != Some(AccessType::RO))
                            && (registers[register_num]
                                .core_signal_properties
                                .use_write_enable
                                .is_none())
                        {
                            registers[register_num]
                                .core_signal_properties
                                .use_write_enable = Some(true);
                        }
                        // peform a cleanup of the now illegal core properties
                        // if location was just changed to pif
                        else if location == LocationType::Pif
                        {
                            registers[register_num].clean();
                        }

                        mem::replace(&mut registers[register_num].location, Some(location))
                    }

                    _ => {
                        if new_location_name == TXT_SPEC_IN_FIELDS {
                            mem::replace(&mut registers[register_num].location, None)
                        } else {
                            seed::log!("error while converting from string to location");
                            registers[register_num].location
                        }
                    }
                };

                match old_location {
                    None => Some(Msg::Register(interface_num, register_num, RegisterMsg::LocationChanged(TXT_SPEC_IN_FIELDS.to_string()))),
                    Some(loc) => Some(Msg::Register(interface_num, register_num, RegisterMsg::LocationChanged(loc.to_string())))
                }
            }

            RegisterMsg::CorePropReadEnable(event) => {
                let old_prop = mem::replace(&mut registers[register_num].core_signal_properties.use_read_enable, 
                    Some(utils::target_checked(&event)));
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreCorePropReadEnable(old_prop)))
            }

            RegisterMsg::RestoreCorePropReadEnable(prop) => {
                let old_prop = mem::replace(&mut registers[register_num].core_signal_properties.use_read_enable, prop);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreCorePropReadEnable(old_prop)))
            }

            RegisterMsg::CorePropWriteEnable(event) => {
                let old_prop = mem::replace(&mut registers[register_num].core_signal_properties.use_write_enable,
                    Some(utils::target_checked(&event)));
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreCorePropWriteEnable(old_prop)))
            }

            RegisterMsg::RestoreCorePropWriteEnable(prop) => {
                let old_prop = mem::replace(&mut registers[register_num].core_signal_properties.use_write_enable, prop);
                Some(Msg::Register(interface_num, register_num, RegisterMsg::RestoreCorePropWriteEnable(old_prop)))
            }
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
        html_elements::text_field_full_line(
            "inputName",
            "Name",
            &register.name,
            move | input | Msg::Register(interface_index, register_index, RegisterMsg::NameChanged(input)),
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
              ev(Ev::Click, move | _ | Msg::Register(interface_index, register_index, RegisterMsg::AddressAutoSelected)),
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
                  ev(Ev::Change, move | _ | Msg::Register(interface_index, register_index, RegisterMsg::AddressSingleSelected)),
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
                move | input | Msg::Register(interface_index, register_index, RegisterMsg::AddrSingleChanged(input)),
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
                  ev(Ev::Click, move | _ | Msg::Register(interface_index, register_index, RegisterMsg::AddressStrideSelected)),
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
                  move | input | Msg::Register(interface_index, register_index, RegisterMsg::AddrStrideValueChanged(input)),
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
                  move | input | Msg::Register(interface_index, register_index, RegisterMsg::AddrStrideCountChanged(input)),
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
                  move | input | Msg::Register(interface_index, register_index, RegisterMsg::AddrStrideIncrementChanged(input)),
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
          move | input | Msg::Register(interface_index, register_index, RegisterMsg::SummaryChanged(input))
      ),
      html_elements::textarea_field(
          "inputDescription",
          "Description",
          &utils::opt_vec_str_to_textarea(&register.description),
          move | input | Msg::Register(interface_index, register_index, RegisterMsg::DescriptionChanged(input))
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
                    "Type:",
                    &register.signal,
                    "",
                    move | input | Msg::Register(interface_index, register_index, RegisterMsg::SignalTypeChanged(input))
                ),
                html_elements::select_option_field_sub_line(
                    "inputAccess",
                    "Access:",
                    &register.access,
                    "",
                    move | input | Msg::Register(interface_index, register_index, RegisterMsg::AccessTypeChanged(input))
                ),
                html_elements::select_option_field_sub_line(
                    "inputLocation",
                    "Location:",
                    &register.location,
                    TXT_SPEC_IN_FIELDS,
                    move | input | Msg::Register(interface_index, register_index, RegisterMsg::LocationChanged(input))
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
                    !register.fields.is_empty(),
                    move | input | Msg::Register(interface_index, register_index, RegisterMsg::WidthChanged(input)),
                    Some("please write a decimal value or leave empty for automatic when using fields")
                ),
                html_elements::text_field_sub_line(
                    ID_RESET_VALUE,
                    "Reset:",
                    & match &register.reset {
                        None => String::new(),
                        Some(value) => value.to_string(),
                    },
                    !register.fields.is_empty(),
                    move | input | Msg::Register(interface_index, register_index, RegisterMsg::ResetValueChanged(input)),
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
            ev(Ev::Change, move | event | Msg::Register(interface_index, register_index, RegisterMsg::CorePropReadEnable(event))),
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
            ev(Ev::Change, move | event | Msg::Register(interface_index, register_index, RegisterMsg::CorePropWriteEnable(event))),
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
    // Fields table
    h3![C!["my-2"], "Fields"],
    table![
        C!["table table-striped"],
        html_elements::table_header(vec!["", "name", "position", "description"]),
        tbody![
            register
                .fields
                .iter()
                .enumerate()
                .map(|(field_index, field)| field_table_row(
                    &model, interface_index, register_index, field_index, &field
                ))
                .collect::<Vec<_>>(),
            tr![
                td![
                    C!["cstm-small-btn"],
                    html_elements::toolbar_button_url(
                    "add",
                    &Urls::new(&model.base_url)
                        .field(interface_index, register_index, field::FieldPage::NewField),
                    true
                ),],
                td![],
                td![],
                td![
                    C!["w-100"],
                ],
            ]
        ]
    ]
  ]
}

fn field_table_row(
    model: &Model,
    interface_index: usize,
    register_index: usize,
    field_index: usize,
    field: &mdf_format::Field,
) -> Node<Msg> {
    tr![
        td![
            div![
                C!["text-nowrap btn-group cstm-small-btn"],
                html_elements::toolbar_button_url(
                    "edit",
                    &Urls::new(&model.base_url).field(interface_index, register_index, field::FieldPage::Num(field_index)),
                    true
                ),
                html_elements::toolbar_button_msg(
                    "delete",
                    Msg::Field(interface_index, register_index, field_index, field::FieldMsg::Delete),
                    true
                ),
                html_elements::toolbar_button_msg(
                    "up",
                    Msg::Field(interface_index, register_index, field_index, field::FieldMsg::MoveUp),
                    field_index != 0
                ),
                html_elements::toolbar_button_msg(
                    "down",
                    Msg::Field(interface_index, register_index, field_index, field::FieldMsg::MoveDown),
                    field_index != model.mdf_data.interfaces[interface_index].registers[register_index].fields.len() - 1
                ),
            ],
        ],
        td![
            C!["text-nowrap"],
            &field.name],
        td![
            C!["text-nowrap"],
            &field.position.to_string()],
        td![
            C!["w-100"],
            utils::opt_vec_str_to_summary(&field.description),],
    ]
}
