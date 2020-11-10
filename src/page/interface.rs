//! page to edit an interface, with registers list

use seed::{prelude::*, *};

use super::super::mdf_format;
use super::super::mdf_format::Interface;
use super::super::mdf_format::InterfaceType;
use super::super::Model;
use super::super::Msg;
use super::super::PageType;
use super::super::Urls;

use super::super::utils;
use super::html_elements;

use super::register;

use std::str::FromStr;

// URL constants
const URL_NEW: &str = "new";
const URL_REGISTER: &str = "register";

// ID constants
const ID_ADDRESS_WIDTH: &str = "inputAddressWidth";
const ID_DATA_WIDTH: &str = "inputDataWidth";

/// types of page for interface edit
pub enum InterfacePage {
    /// edit the interface with the given number
    Num(usize),
    /// create a new interface
    NewInterface,
}

/// generate the url corresponding to an interface page
pub fn interface_url(url: Url, interface_page: InterfacePage) -> Url {
    match interface_page {
        InterfacePage::Num(interface_number) => url.add_path_part(interface_number.to_string()),

        InterfacePage::NewInterface => url.add_path_part(URL_NEW),
    }
}

/// called with the webapp url is changed to an interface page url
pub fn change_url(mut url: seed::browser::url::Url, model: &mut Model) -> PageType {
    match url.next_path_part() {
        None => PageType::NotFound,
        Some(URL_NEW) => new_interface(model),
        Some(number_string) => {
            match number_string.parse::<usize>() {
                Ok(index) => {
                    if index < model.mdf_data.interfaces.len() {
                        // check if we are just refering to the interface (URL stops here) ir a register (URL continues)
                        match url.next_path_part() {
                            None => PageType::Interface(index),
                            Some(URL_REGISTER) => super::register::change_url(url, index, model),
                            Some(_) => PageType::NotFound,
                        }
                    } else {
                        PageType::NotFound
                    }
                }
                Err(_) => PageType::NotFound,
            }
        }
    }
}

fn new_interface(model: &mut Model) -> PageType {
    model.mdf_data.interfaces.push(Interface::new());
    let new_page_type = PageType::Interface(model.mdf_data.interfaces.len() - 1);

    super::super::Urls::new(model.base_url.clone())
        .from_page_type(new_page_type)
        .go_and_replace();
    new_page_type
}

// ------ ------
//    Update
// ------ ------

/// generated messages handling interfaces
#[derive(Clone)]
pub enum InterfaceMsg {
    /// delete the given interface
    Delete(usize),
    /// move the given interface up in the list
    MoveUp(usize),
    /// move the given interface down in the list
    MoveDown(usize),
    /// change an interface name
    NameChanged(usize, String),
    /// change an interface protocol type
    TypeChanged(usize, String),
    /// change an interface description
    DescriptionChanged(usize, String),
    /// change an interface address width
    AddressWitdhChanged(usize, String),
    /// change an interface data width
    DataWidthChanged(usize, String),
}

/// process the interface messages
pub fn update(msg: InterfaceMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        InterfaceMsg::Delete(index) => {
            if index < model.mdf_data.interfaces.len() {
                model.mdf_data.interfaces.remove(index);
            }
        }
        InterfaceMsg::MoveUp(index) => {
            if (index < model.mdf_data.interfaces.len()) && (index > 0) {
                model.mdf_data.interfaces.swap(index - 1, index);
            }
        }
        InterfaceMsg::MoveDown(index) => {
            if index < model.mdf_data.interfaces.len() - 1 {
                model.mdf_data.interfaces.swap(index, index + 1);
            }
        }

        InterfaceMsg::NameChanged(index, new_name) => {
            model.mdf_data.interfaces[index].name = new_name;
            orders.skip();
        }
        InterfaceMsg::TypeChanged(index, new_type_name) => {
            match InterfaceType::from_str(&new_type_name) {
                Ok(new_type) => {
                    model.mdf_data.interfaces[index].interface_type = new_type;
                    orders.skip();
                }

                _ => seed::log!("error while converting from string to interface type"),
            }
        }
        InterfaceMsg::DescriptionChanged(index, new_description) => {
            model.mdf_data.interfaces[index].description =
                utils::textarea_to_opt_vec_str(&new_description);

            orders.skip();
        }

        InterfaceMsg::AddressWitdhChanged(index, new_width) => {
            orders.skip();

            match utils::validate_field(ID_ADDRESS_WIDTH, &new_width, |field_value| {
                utils::option_num_from_str(field_value)
            }) {
                Ok(value) => model.mdf_data.interfaces[index].address_width = value,
                Err(_) => (),
            };
        }

        InterfaceMsg::DataWidthChanged(index, new_width) => {
            orders.skip();

            match utils::validate_field(ID_DATA_WIDTH, &new_width, |field_value| {
                utils::option_num_from_str(field_value)
            }) {
                Ok(value) => model.mdf_data.interfaces[index].data_width = value,
                Err(_) => (),
            };
        }
    }
}

// ------ ------
//     View
// ------ ------

/// display the interface page
pub fn view(model: &Model, index: usize) -> Node<Msg> {
    let interface = &model.mdf_data.interfaces[index];
    let address_width_value = match &interface.address_width {
        None => String::new(),
        Some(width) => width.to_string(),
    };
    let data_width_value = match &interface.data_width {
        None => String::new(),
        Some(width) => width.to_string(),
    };

    div![
        // Interface fields
        html_elements::text_field_full_line(
            "inputName",
            "Name",
            &interface.name,
            move |input| Msg::Interface(InterfaceMsg::NameChanged(index, input)),
            None
        ),
        html_elements::textarea_field(
            "inputDescription",
            "Description",
            &utils::opt_vec_str_to_textarea(&interface.description),
            move |input| Msg::Interface(InterfaceMsg::DescriptionChanged(index, input))
        ),
        div![
            C!["form-group row"],
            div![
                C!["col-sm-2 col-form-label"],
                "Parameters"
            ],
            div![
                C!["col-sm-10"],
                div![
                    C!["form-row align-items-center form-inline ml-4"],
                    html_elements::select_field_full_line(
                        "inputType",
                        "Protocol",
                        &interface.interface_type,
                        move |input| Msg::Interface(InterfaceMsg::TypeChanged(index, input))
                    ),
                    html_elements::text_field_full_line(
                        ID_ADDRESS_WIDTH,
                        "Address width",
                        &address_width_value,
                        move |input| Msg::Interface(InterfaceMsg::AddressWitdhChanged(index, input)),
                        Some("please write a decimal value or leave empty for automatic")
                    ),
                    html_elements::text_field_full_line(
                        ID_DATA_WIDTH,
                        "Data width",
                        &data_width_value,
                        move |input| Msg::Interface(InterfaceMsg::DataWidthChanged(index, input)),
                        Some("please write a decimal value or leave empty for automatic")
                    )
                ]
            ]
        ],
        // Registers table
        h3![C!["my-2"], "Registers"],
        table![
            C!["table table-striped"],
            html_elements::table_header(vec!["", "name", "address", "summary"]),
            tbody![
                interface
                    .registers
                    .iter()
                    .enumerate()
                    .map(|(reg_index, register)| register_table_row(
                        &model, index, reg_index, &register
                    ))
                    .collect::<Vec<_>>(),
                tr![
                    td![
                        C!["cstm-small-btn"],
                        html_elements::toolbar_button_url(
                        "add",
                        &Urls::new(&model.base_url)
                            .register(index, register::RegisterPage::NewRegister),
                        true
                    ),],
                    td![],
                    td![],
                    td![],
                ]
            ]
        ]
    ]
}

fn register_table_row(
    model: &Model,
    index: usize,
    reg_index: usize,
    register: &mdf_format::Register,
) -> Node<Msg> {
    tr![
        td![
            div![
                C!["text-nowrap btn-group cstm-small-btn"],
                html_elements::toolbar_button_url(
                    "edit",
                    &Urls::new(&model.base_url).register(index, register::RegisterPage::Num(reg_index)),
                    true
                ),
                html_elements::toolbar_button_msg(
                    "delete",
                    Msg::Register(index, register::RegisterMsg::Delete(reg_index)),
                    true
                ),
                html_elements::toolbar_button_msg(
                    "up",
                    Msg::Register(index, register::RegisterMsg::MoveUp(reg_index)),
                    reg_index != 0
                ),
                html_elements::toolbar_button_msg(
                    "down",
                    Msg::Register(index, register::RegisterMsg::MoveDown(reg_index)),
                    reg_index != model.mdf_data.interfaces[index].registers.len() - 1
                ),
            ],
        ],
        td![
            C!["text-nowrap"],
            &register.name],
        td![
            C!["text-nowrap"],
            &register.address.nice_str()],
        td![
            C!["w-100"],
            utils::opt_vec_str_to_summary(&register.summary),],
    ]
}
