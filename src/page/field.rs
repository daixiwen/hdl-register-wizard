//! page to edit a register in the model, with optional fields list

use seed::{prelude::*, *};

use crate::Model;
use crate::PageType;

use crate::mdf_format::AccessType;
use crate::mdf_format::LocationType;

use crate::mdf_format::Field;
use crate::mdf_format::SignalType;
use crate::mdf_format::VectorValue;
use crate::mdf_format::FieldPosition;
use crate::Msg;

use crate::utils;
use super::html_elements;

use std::str::FromStr;
use std::mem;

// URL constants
const URL_NEW: &str = "new";

// ID constants
const ID_RESET_VALUE: &str = "inputResetValue";
const ID_POSITION: &str = "inputPosition";


const TXT_SPEC_IN_REGISTER: &str = "(specified in register)";

// ------ ------
//     Urls
// ------ ------
/// different pages, each having its url within a register
pub enum FieldPage {
    /// edit the field with the given number
    Num(usize),
    /// create a field
    NewField,
}

/// generate an url for a specific field page, built upon a register url
pub fn field_url(url: Url, field_page: FieldPage) -> Url {
    match field_page {
        FieldPage::Num(field_number) => url.add_path_part(field_number.to_string()),

        FieldPage::NewField => url.add_path_part(URL_NEW),
    }
}

/// called when the url is changed to a register one
pub fn change_url(
    mut url: seed::browser::url::Url,
    interface_num: usize,
    register_num: usize,
    model: &mut Model,
) -> (PageType, Option<Msg>) {
    match url.next_path_part() {
        None => (PageType::NotFound, None),
        Some(URL_NEW) => new_field(interface_num, register_num, model),
        Some(number_string) => match number_string.parse::<usize>() {
            Ok(index) => {
                if index < model.mdf_data.interfaces[interface_num].registers[register_num].fields.len() {
                    (PageType::Field(interface_num, register_num, index), None)
                } else {
                    (PageType::NotFound, None)
                }
            }
            Err(_) => (PageType::NotFound, None),
        },
    }
}

fn new_field(interface_num: usize, register_num: usize, model: &mut Model) -> (PageType, Option<Msg>) {
    model.mdf_data.interfaces[interface_num]
        .registers[register_num]
        .fields
        .push(Field::new());
    let field_num = model.mdf_data.interfaces[interface_num].registers[register_num].fields.len() - 1;
    let new_page_type = PageType::Field(interface_num, register_num, field_num);

    // generate the undo action
    let undo = Msg::Field(interface_num, register_num, field_num, FieldMsg::Delete);

    // remove some of the register options that become illegal
    // if fields are present
    model.mdf_data.interfaces[interface_num].registers[register_num].clean();

    crate::Urls::new(model.base_url.clone())
        .from_page_type(new_page_type)
        .go_and_replace();
    (new_page_type, Some(undo))
}

// ------ ------
//    Update
// ------ ------

/// messages related to fields
#[derive(Clone)]
pub enum FieldMsg {
    /// delete the field
    Delete,
    /// restore a deleted field (undo)
    Restore(std::rc::Rc<Field>),
    /// move the field up in the list
    MoveUp,
    /// move the field down in the list
    MoveDown,
    /// field name changed
    NameChanged(String),
    /// sent when the field position is changed
    PositionChanged(String),
    /// sent when the field description is changed
    DescriptionChanged(String),
    /// sent when the access type of the field is changed
    AccessTypeChanged(String),
    /// sent when the signal type for the field is changed
    SignalTypeChanged(String),
    /// sent when the field's reset value is changed
    ResetValueChanged(String),
    /// sent when the field's location is changed
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

/// process a field message
pub fn update(
    msg: FieldMsg,
    interface_num: usize,
    register_num: usize,
    field_num: usize,
    model: &mut Model,
    orders: &mut impl Orders<Msg>) -> Option<Msg> {
    let num_fields = match msg {
        FieldMsg::Restore(_) =>         model.mdf_data.interfaces[interface_num].registers  [register_num].fields.len() + 1,
        _ => model.mdf_data.interfaces[interface_num].registers[register_num].fields.len()
    };

    let reg_location = model.mdf_data.interfaces[interface_num].registers[register_num].location;
    let fields = &mut model.mdf_data.interfaces[interface_num].registers[register_num].fields;

    if field_num >= num_fields {
        None
    }
    else {
        match msg {
            FieldMsg::Delete => {
                let undo_msg = Msg::Field(interface_num, register_num, field_num, FieldMsg::Restore(
                        std::rc::Rc::new(fields.remove(field_num))));

                if model.mdf_data.interfaces[interface_num].registers[register_num].fields.is_empty() {
                    // if fields become empty again, put back some default values
                    // in the register parameters that were removed
                    let reg = &mut model.mdf_data.interfaces[interface_num].registers[register_num];
                    reg.width = Some(32);
                    reg.access = Some(AccessType::RW);
                    reg.signal = Some(SignalType::StdLogicVector);
                    reg.reset = Some(VectorValue::new());
                }
                Some(undo_msg)
            }

            FieldMsg::Restore(field) => {
                match std::rc::Rc::<Field>::try_unwrap(field) {
                    Ok(field_obj) => {
                        fields.insert(field_num, field_obj);
                        Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::Delete))
                    },
                    _ => {
                        seed::log!("error recovering field object");
                        None
                    },
                }
            }

            FieldMsg::MoveUp => {
                if field_num > 0 {
                    fields.swap(field_num - 1, field_num);
                    Some(Msg::Field(interface_num, register_num, field_num - 1, FieldMsg::MoveDown))
                }
                else {
                    None
                }
            }

            FieldMsg::MoveDown => {
                if field_num < num_fields - 1 {
                    fields.swap(field_num, field_num + 1);
                    Some(Msg::Field(interface_num, register_num, field_num + 1, FieldMsg::MoveUp))
                }
                else {
                    None
                }
            }

            FieldMsg::NameChanged(new_name) => {
                let old_name = mem::replace(&mut fields[field_num].name, new_name);
                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::NameChanged(old_name)))
            }

            FieldMsg::PositionChanged(new_position) => {
                let old_position = fields[field_num].position.to_string();
                match utils::validate_field(ID_POSITION, &new_position, |field_value| {
                    FieldPosition::from_str(field_value)}) {

                    Ok(pos) => {
                        fields[field_num].position = pos;
                        Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::PositionChanged(old_position)))
                    },
                    _ => {
                        orders.skip();
                        None
                    },
                }
            }

            FieldMsg::DescriptionChanged(new_description) => {
                let old_description = utils::opt_vec_str_to_textarea(&fields[field_num].description);
                fields[field_num].description = utils::textarea_to_opt_vec_str(&new_description);
                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::DescriptionChanged(old_description)))
            }

            FieldMsg::AccessTypeChanged(new_type_name) => {
                let old_type = fields[field_num].access.to_string();

                match AccessType::from_str(&new_type_name) {
                    Ok(new_type) => {
                        fields[field_num].access = new_type;

                        // put a default value for use_write_enabled if it is not set yet
                        if (new_type != AccessType::RO)
                            && (fields[field_num].location == Some(LocationType::Core))
                            && (fields[field_num].core_signal_properties.use_write_enable.is_none())
                        {
                            fields[field_num].core_signal_properties.use_write_enable = Some(true);
                        }

                        Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::AccessTypeChanged(old_type)))
                    }

                    _ => {
                        seed::log!("error while converting from string to access type");
                        None
                    }
                }
            }

            FieldMsg::SignalTypeChanged(new_type_name) => {
                let old_type = fields[field_num].signal.to_string();

                match SignalType::from_str(&new_type_name) {
                    Ok(new_type) => {
                        fields[field_num].signal = new_type;
                        Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::SignalTypeChanged(old_type)))
                    }

                    _ => {
                        seed::log!("error while converting from string to signal type");
                        None
                    }
                }
            }

            FieldMsg::ResetValueChanged(new_value) => {
                let old_reset_value = fields[field_num].reset.to_string();

                match utils::validate_field(ID_RESET_VALUE, &new_value, |field_value| {
                    VectorValue::from_str(field_value)
                }) {
                    Ok(reset_value) => {
                        fields[field_num].reset = reset_value
                    }
                    Err(_) => ()
                };

                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::ResetValueChanged(old_reset_value)))
            }

            FieldMsg::LocationChanged(new_location_name) => {
                let old_location = match fields[field_num].location {
                    Some(location) => location.to_string(),
                    _ => TXT_SPEC_IN_REGISTER.to_string()
                };

                match LocationType::from_str(&new_location_name) {
                    Ok(location) => {
                        fields[field_num].location = Some(location);
                        // put a default value for use_write_enabled if it is not set yet
                        if (location == LocationType::Core)
                            && (fields[field_num].access != AccessType::RO)
                            && (fields[field_num].core_signal_properties.use_write_enable.is_none())
                        {
                            fields[field_num].core_signal_properties.use_write_enable = Some(true);
                        }
                        else {
                            // in some cases the core signal properties need to be cleaned
                            fields[field_num].clean(reg_location);
                        }
                    }

                    _ => {
                        if new_location_name == TXT_SPEC_IN_REGISTER {
                            fields[field_num].location = None;
                            fields[field_num].clean(reg_location);
                        } else {
                            seed::log!("error while converting from string to location")
                        }
                    }
                };
                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::LocationChanged(old_location)))
            }

            FieldMsg::CorePropReadEnable(event) => {
                let old_prop = mem::replace(&mut fields[field_num].core_signal_properties.use_read_enable, 
                    Some(utils::target_checked(&event)));
                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::RestoreCorePropReadEnable(old_prop)))
            }

            FieldMsg::RestoreCorePropReadEnable(prop) => {
                let old_prop = mem::replace(&mut fields[field_num].core_signal_properties.use_read_enable, prop);
                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::RestoreCorePropReadEnable(old_prop)))
            }

            FieldMsg::CorePropWriteEnable(event) => {
                let old_prop = mem::replace(&mut fields[field_num].core_signal_properties.use_write_enable,
                    Some(utils::target_checked(&event)));

                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::RestoreCorePropWriteEnable(old_prop)))
            }

            FieldMsg::RestoreCorePropWriteEnable(prop) => {
                let old_prop = mem::replace(&mut fields[field_num].core_signal_properties.use_write_enable, prop);
                Some(Msg::Field(interface_num, register_num, field_num, FieldMsg::RestoreCorePropWriteEnable(old_prop)))
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

/// build an html view for the register
pub fn view(model: &Model, interface_index: usize, register_index: usize, field_index: usize) -> Node<Msg> {
    let interface = &model.mdf_data.interfaces[interface_index];
    let register = &interface.registers[register_index];
    let field = &register.fields[field_index];

    div![
    div![
        html_elements::text_field_full_line(
            "inputName",
            "Name",
            &field.name,
            move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::NameChanged(input)),
            None
        ),
      html_elements::textarea_field(
          "inputDescription",
          "Description",
          &utils::opt_vec_str_to_textarea(&field.description),
          move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::DescriptionChanged(input))
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
                html_elements::select_field_sub_line(
                    "inputSignal",
                    "Type:",
                    &field.signal,
                    move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::SignalTypeChanged(input))
                ),
                html_elements::text_field_sub_line(
                    ID_POSITION,
                    "position",
                    &field.position.to_string(),
                    false,
                    move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::PositionChanged(input)),
                    Some("please provide either a single bit number, or the combination msb:lsb"),
                ),
                html_elements::text_field_sub_line(
                    ID_RESET_VALUE,
                    "Reset:",
                    &field.reset.to_string(),
                    false,
                    move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::ResetValueChanged(input)),
                    Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value")
                ),
            ]
        ]
    ],
    div![
        C!["form-group row"],
        div![
            C!["col-sm-2 col-form-label"],
            "Register"
        ],
        div![
            C!["col-sm-10"],
            div![
                C!["form-row align-items-center form-inline"],

                html_elements::select_field_sub_line(
                    "inputAccess",
                    "Access:",
                    &field.access,
                    move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::AccessTypeChanged(input))
                ),
                html_elements::select_option_field_sub_line(
                    "inputLocation",
                    "Location:",
                    &field.location,
                    TXT_SPEC_IN_REGISTER,
                    move | input | Msg::Field(interface_index, register_index, field_index, FieldMsg::LocationChanged(input))
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
            IF!(field.core_signal_properties.use_read_enable == Some(true) =>
              attrs!{ At::Checked => "checked"}),
            IF!((field.location == Some(LocationType::Pif)) ||
                (field.access == AccessType::WO) =>
              attrs!{ At::Disabled => "disabled"}),
            ev(Ev::Change, move | event | Msg::Field(interface_index, register_index, field_index, FieldMsg::CorePropReadEnable(event))),
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
            IF!(field.core_signal_properties.use_write_enable == Some(true) =>
              attrs!{ At::Checked => "checked"}),
            IF!((field.location == Some(LocationType::Pif)) ||
                (field.access == AccessType::RO) =>
              attrs!{ At::Disabled => "disabled"}),
            ev(Ev::Change, move | event | Msg::Field(interface_index, register_index, field_index, FieldMsg::CorePropWriteEnable(event))),
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
    ]]
}
