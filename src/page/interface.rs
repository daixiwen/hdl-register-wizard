//! page to edit an interface, with registers list

use seed::{prelude::*, *};

use crate::mdf_format;
use crate::mdf_format::Interface;
use crate::mdf_format::InterfaceType;
use crate::Model;
use crate::Msg;
use crate::PageType;
use crate::Urls;

use crate::utils;
use super::html_elements;

use super::register;

use std::str::FromStr;
use std::mem;

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
/// returns the page to display and possibly a message to undo the
/// operation executed with switching to that url
pub fn change_url(mut url: seed::browser::url::Url, model: &mut Model) -> (PageType, Option<Msg>) {
    match url.next_path_part() {
        None => (PageType::NotFound, None),
        Some(URL_NEW) => new_interface(model),
        Some(number_string) => {
            match number_string.parse::<usize>() {
                Ok(index) => {
                    if index < model.mdf_data.interfaces.len() {
                        // check if we are just refering to the interface (URL stops here) ir a register (URL continues)
                        match url.next_path_part() {
                            None => (PageType::Interface(index), None),
                            Some(URL_REGISTER) => super::register::change_url(url, index, model),
                            Some(_) => (PageType::NotFound, None),
                        }
                    } else {
                        (PageType::NotFound, None)
                    }
                }
                Err(_) => (PageType::NotFound, None),
            }
        }
    }
}

fn new_interface(model: &mut Model) -> (PageType, Option<Msg>) {
    model.mdf_data.interfaces.push(Interface::new());
    let interface_number = model.mdf_data.interfaces.len() - 1;
    let new_page_type = PageType::Interface(interface_number);

    // generate the undo action
    let undo = Msg::Interface(interface_number, InterfaceMsg::Delete);

    crate::Urls::new(&model.base_url)
        .from_page_type(new_page_type)
        .go_and_replace();
    (new_page_type, Some(undo))
}

// ------ ------
//    Update
// ------ ------

/// generated messages handling interfaces
#[derive(Clone)]
pub enum InterfaceMsg {
    /// delete the given interface
    Delete,
    /// restore a deleted interface (undo)
    Restore(std::rc::Rc<Interface>),
    /// move the given interface up in the list
    MoveUp,
    /// move the given interface down in the list
    MoveDown,
    /// change an interface name
    NameChanged(String),
    /// change an interface protocol type
    TypeChanged(String),
    /// change an interface description
    DescriptionChanged(String),
    /// change an interface address width
    AddressWitdhChanged(String),
    /// change an interface data width
    DataWidthChanged(String),
}

/// process the interface messages. Returns a message that can be used
/// to undo the operation described in the message
pub fn update(index: usize, msg: InterfaceMsg, model: &mut Model, _orders: &mut impl Orders<Msg>) -> Option<Msg> {
    let num_interfaces = match msg {
        InterfaceMsg::Restore(_) => model.mdf_data.interfaces.len() + 1,
        _ => model.mdf_data.interfaces.len() 
    };

    if index >= num_interfaces {
        // index is out of bounds
        None
    }
    else {
        match msg {
            InterfaceMsg::Delete => {
                Some(Msg::Interface(index, InterfaceMsg::Restore(
                    std::rc::Rc::new(model.mdf_data.interfaces.remove(index)))))
            }

            InterfaceMsg::Restore(interface) => {
                match std::rc::Rc::<mdf_format::Interface>::try_unwrap(interface) {
                    Ok(interface_obj) => { 
                        model.mdf_data.interfaces.insert(index,interface_obj);
                        Some(Msg::Interface(index, InterfaceMsg::Delete))
                    },
                    _ => {
                        seed::log!("error recovering interface object");
                        None
                    },
                }
            }
            InterfaceMsg::MoveUp => {
                if index > 0 {
                    model.mdf_data.interfaces.swap(index - 1, index);
                    Some(Msg::Interface(index-1, InterfaceMsg::MoveDown))
                }
                else {
                    None
                }
            }
            InterfaceMsg::MoveDown => {
                if index < num_interfaces - 1 {
                    model.mdf_data.interfaces.swap(index, index + 1);
                    Some(Msg::Interface(index+1, InterfaceMsg::MoveUp))
                }
                else {
                    None
                }
            }

            InterfaceMsg::NameChanged(new_name) => {
                let old_name = mem::replace(&mut model.mdf_data.interfaces[index].name, new_name);
                Some(Msg::Interface(index,InterfaceMsg::NameChanged(old_name)))                
            }

            InterfaceMsg::TypeChanged(new_type_name) => {
                match InterfaceType::from_str(&new_type_name) {
                    Ok(new_type) => {
                        let old_type = model.mdf_data.interfaces[index].interface_type.to_string();
                        model.mdf_data.interfaces[index].interface_type = new_type;
                        Some(Msg::Interface(index, InterfaceMsg::TypeChanged(old_type )))
                    }

                    _ => {
                        seed::log!("error while converting from string to interface type");
                        None
                    }
                }
            }
            InterfaceMsg::DescriptionChanged(new_description) => {
                let old_description = utils::opt_vec_str_to_textarea(&model.mdf_data.interfaces[index].description);
                model.mdf_data.interfaces[index].description =
                    utils::textarea_to_opt_vec_str(&new_description);
                Some(Msg::Interface(index, InterfaceMsg::DescriptionChanged(old_description)))
            }

            InterfaceMsg::AddressWitdhChanged(new_width) => {

                let old_add_width = match model.mdf_data.interfaces[index].address_width {
                            None => String::new(),
                            Some(width) => width.to_string()
                };
                match utils::validate_field(ID_ADDRESS_WIDTH, &new_width, |field_value| {
                    utils::option_num_from_str(field_value)
                }) {
                    Ok(value) => {
                        model.mdf_data.interfaces[index].address_width = value;
                        Some(Msg::Interface(index, InterfaceMsg::AddressWitdhChanged(old_add_width)))
                    }
                    Err(_) => None
                }
            }

            InterfaceMsg::DataWidthChanged(new_width) => {
                let old_dat_width = match model.mdf_data.interfaces[index].data_width {
                        None => String::new(),
                        Some(width) => width.to_string()
                };
                match utils::validate_field(ID_DATA_WIDTH, &new_width, |field_value| {
                    utils::option_num_from_str(field_value)
                }) {
                    Ok(value) => {
                        model.mdf_data.interfaces[index].data_width = value;
                        Some(Msg::Interface(index, InterfaceMsg::DataWidthChanged(old_dat_width)))
                    }
                    Err(_) => None
                }
            }
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
            move |input| Msg::Interface(index, InterfaceMsg::NameChanged(input)),
            None
        ),
        html_elements::textarea_field(
            "inputDescription",
            "Description",
            &utils::opt_vec_str_to_textarea(&interface.description),
            move |input| Msg::Interface(index, InterfaceMsg::DescriptionChanged(input))
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
                    C!["form-row align-items-center form-inline ml-1"],
                    html_elements::select_field_sub_line(
                        "inputType",
                        "Protocol",
                        &interface.interface_type,
                        move |input| Msg::Interface(index, InterfaceMsg::TypeChanged(input))
                    ),
                    html_elements::text_field_sub_line(
                        ID_ADDRESS_WIDTH,
                        "Address width",
                        &address_width_value,
                        false,
                        move |input| Msg::Interface(index, InterfaceMsg::AddressWitdhChanged(input)),
                        Some("please write a decimal value or leave empty for automatic")
                    ),
                    html_elements::text_field_sub_line(
                        ID_DATA_WIDTH,
                        "Data width",
                        &data_width_value,
                        false,
                        move |input| Msg::Interface(index, InterfaceMsg::DataWidthChanged(input)),
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
                    td![
                        C!["w-100"],
                    ],
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
                    Msg::Register(index, reg_index, register::RegisterMsg::Delete),
                    true
                ),
                html_elements::toolbar_button_msg(
                    "up",
                    Msg::Register(index, reg_index, register::RegisterMsg::MoveUp),
                    reg_index != 0
                ),
                html_elements::toolbar_button_msg(
                    "down",
                    Msg::Register(index, reg_index, register::RegisterMsg::MoveDown),
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
