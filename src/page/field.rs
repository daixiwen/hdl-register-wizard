//! page to edit a register in the model, with optional fields list

use seed::{prelude::*, *};

use super::super::Model;
use super::super::PageType;

use super::super::mdf_format::AccessType;
use super::super::mdf_format::LocationType;

use super::super::mdf_format::Field;
use super::super::mdf_format::SignalType;
use super::super::mdf_format::VectorValue;
use super::super::mdf_format::FieldPosition;
use super::super::Msg;

use super::super::utils;
use super::html_elements;

use std::str::FromStr;

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
) -> PageType {
    match url.next_path_part() {
        None => PageType::NotFound,
        Some(URL_NEW) => new_field(interface_num, register_num, model),
        Some(number_string) => match number_string.parse::<usize>() {
            Ok(index) => {
                if index < model.mdf_data.interfaces[interface_num].registers[register_num].fields.len() {
                    PageType::Field(interface_num, register_num, index)
                } else {
                    PageType::NotFound
                }
            }
            Err(_) => PageType::NotFound,
        },
    }
}

fn new_field(interface: usize, register: usize, model: &mut Model) -> PageType {
    model.mdf_data.interfaces[interface]
        .registers[register]
        .fields
        .push(Field::new());
    let new_page_type = PageType::Field(
        interface,
        register,
        model.mdf_data.interfaces[interface].registers[register].fields.len() - 1,
    );

    super::super::Urls::new(model.base_url.clone())
        .from_page_type(new_page_type)
        .go_and_replace();
    new_page_type
}

// ------ ------
//    Update
// ------ ------

/// messages related to fields
#[derive(Clone)]
pub enum FieldMsg {
    /// delete the field
    Delete(usize),
    /// move the field up in the list
    MoveUp(usize),
    /// move the field down in the list
    MoveDown(usize),
    /// field name changed
    NameChanged(usize, String),
    /// sent when the field position is changed
    PositionChanged(usize, String),
    /// sent when the field description is changed
    DescriptionChanged(usize, String),
    /// sent when the access type of the field is changed
    AccessTypeChanged(usize, String),
    /// sent when the signal type for the field is changed
    SignalTypeChanged(usize, String),
    /// sent when the field's reset value is changed
    ResetValueChanged(usize, String),
    /// sent when the field's location is changed
    LocationChanged(usize, String),
    /// sent when the read enable core property is changed
    CorePropReadEnable(usize, web_sys::Event),
    /// sent when the write enable core property is changed
    CorePropWriteEnable(usize, web_sys::Event),
}

/// process a field message
pub fn update(
    msg: FieldMsg,
    interface_num: usize,
    register_num: usize,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
) {
    let num_fields = model.mdf_data.interfaces[interface_num].registers[register_num].fields.len();

    match msg {
        FieldMsg::Delete(index) => {
            if index < num_fields {
                model.mdf_data.interfaces[interface_num].registers[register_num]
                    .fields
                    .remove(index);
            }
        }
        FieldMsg::MoveUp(index) => {
            if (index < num_fields) && (index > 0) {
                model.mdf_data.interfaces[interface_num].registers[register_num]
                    .fields
                    .swap(index - 1, index);
            }
        }
        FieldMsg::MoveDown(index) => {
            if index < num_fields - 1 {
                model.mdf_data.interfaces[interface_num].registers[register_num]
                    .fields
                    .swap(index, index + 1);
            }
        }

        FieldMsg::NameChanged(index, new_name) => {
            model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].name = new_name;
            orders.skip();
        }

        FieldMsg::PositionChanged(index, new_position) => {
            match utils::validate_field(ID_POSITION, &new_position, |field_value| {
                FieldPosition::from_str(field_value)}) {

                Ok(pos) => model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].position = pos,
                _ => ()
            }

            orders.skip();
        }
        FieldMsg::DescriptionChanged(index, new_description) => {
            model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].description =
                utils::textarea_to_opt_vec_str(&new_description);

            orders.skip();
        }

        FieldMsg::AccessTypeChanged(index, new_type_name) => {
            match AccessType::from_str(&new_type_name) {
                Ok(new_type) => {
                    model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].access =
                        Some(new_type);

                    // put a default value for use_write_enabled if it is not set yet
                    if (new_type != AccessType::RO)
                        && (model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].location
                            == Some(LocationType::Core))
                        && (model.mdf_data.interfaces[interface_num].registers[register_num].fields[index]
                            .core_signal_properties
                            .use_write_enable
                            .is_none())
                    {
                        model.mdf_data.interfaces[interface_num].registers[register_num].fields[index]
                            .core_signal_properties
                            .use_write_enable = Some(true);
                    }
                }

                _ => seed::log!("error while converting from string to interface type")
            }
        }

        FieldMsg::SignalTypeChanged(index, new_type_name) => {
            orders.skip();

            match SignalType::from_str(&new_type_name) {
                Ok(new_type) => {
                    model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].signal =
                        new_type;
                }

                _ => seed::log!("error while converting from string to signal type")
            }
        }

        FieldMsg::ResetValueChanged(index, new_value) => {
            orders.skip();

            match utils::validate_field(ID_RESET_VALUE, &new_value, |field_value| {
                VectorValue::from_str(field_value)
            }) {
                Ok(reset_value) => {
                    model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].reset = reset_value
                }
                Err(_) => (),
            }
        }

        FieldMsg::LocationChanged(index, new_location_name) => {
            match LocationType::from_str(&new_location_name) {
                Ok(location) => {
                    model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].location =
                        Some(location);
                    // put a default value for use_write_enabled if it is not set yet
                    if (location == LocationType::Core)
                        && (model.mdf_data.interfaces[interface_num].registers[register_num].fields[index].access
                            != Some(AccessType::RO))
                        && (model.mdf_data.interfaces[interface_num].registers[register_num].fields[index]
                            .core_signal_properties
                            .use_write_enable
                            .is_none())
                    {
                        model.mdf_data.interfaces[interface_num].registers[register_num].fields[index]
                            .core_signal_properties
                            .use_write_enable = Some(true);
                    }
                }

                _ =>  seed::log!("error while converting from string to location")
            }
        }

        FieldMsg::CorePropReadEnable(index, event) => {
            model.mdf_data.interfaces[interface_num].registers[index]
                .core_signal_properties
                .use_read_enable = Some(utils::target_checked(&event));
        }

        FieldMsg::CorePropWriteEnable(index, event) => {
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
            move | input | Msg::Field(interface_index, register_index, FieldMsg::NameChanged(field_index, input)),
            None
        ),
      html_elements::textarea_field(
          "inputDescription",
          "Description",
          &utils::opt_vec_str_to_textarea(&field.description),
          move | input | Msg::Field(interface_index, register_index, FieldMsg::DescriptionChanged(field_index, input))
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
                    move | input | Msg::Field(interface_index, register_index, FieldMsg::SignalTypeChanged(field_index, input))
                ),
                html_elements::text_field_sub_line(
                    ID_POSITION,
                    "position",
                    &field.position.to_string(),
                    false,
                    move | input | Msg::Field(interface_index, register_index, FieldMsg::PositionChanged(field_index, input)),
                    Some("please provide either a single bit number, or the combination msb:lsb"),
                ),
                html_elements::text_field_sub_line(
                    ID_RESET_VALUE,
                    "Reset:",
                    &field.reset.to_string(),
                    false,
                    move | input | Msg::Field(interface_index, register_index, FieldMsg::ResetValueChanged(field_index, input)),
                    Some("please use a decimal, hexadecimal (0x*) or binary (0b*) value or leave empty when using fields")
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

                html_elements::select_option_field_sub_line(
                    "inputAccess",
                    "Access:",
                    &field.access,
                    TXT_SPEC_IN_REGISTER,
                    move | input | Msg::Field(interface_index, register_index, FieldMsg::AccessTypeChanged(field_index, input))
                ),
                html_elements::select_option_field_sub_line(
                    "inputLocation",
                    "Location:",
                    &field.location,
                    TXT_SPEC_IN_REGISTER,
                    move | input | Msg::Field(interface_index, register_index, FieldMsg::LocationChanged(field_index, input))
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
                (field.access == Some(AccessType::WO)) =>
              attrs!{ At::Disabled => "disabled"}),
            ev(Ev::Change, move | event | Msg::Field(interface_index, register_index, FieldMsg::CorePropReadEnable(field_index, event))),
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
                (field.access == Some(AccessType::RO)) =>
              attrs!{ At::Disabled => "disabled"}),
            ev(Ev::Change, move | event | Msg::Field(interface_index, register_index, FieldMsg::CorePropWriteEnable(field_index, event))),
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
