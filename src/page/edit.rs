#![allow(clippy::wildcard_imports)]

//! Main edit page, with the model name and the list of interfaces

use super::super::mdf_format;
use super::super::Model;
use super::super::Msg;
use super::super::Urls;
use super::html_elements;
use super::interface::InterfaceMsg;
use super::interface::InterfacePage;
use seed::{prelude::*, *};

use super::super::utils;

/// messages for the edit page
#[derive(Clone)]
pub enum EditMsg {
    /// Model name changed
    NameChanged(String),
}

/// message handling for the edit page
pub fn update(msg: EditMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        EditMsg::NameChanged(new_name) => {
            model.mdf_data.name = new_name;
            orders.skip();
        }
    }
}

/// edit page view
pub fn view(model: &Model) -> Node<Msg> {
    div![
        div![
            C!["my-3"],
            button![
                attrs! {
                  At::Type => "button",
                  At::Class => "btn btn-primary",
                },
                "New Model"
            ]
        ],
        div![
            h3![C!["my-2"], "Model Description File"],
            html_elements::text_field_full_line(
                "inputName",
                "Name",
                &model.mdf_data.name,
                move |input| Msg::Edit(EditMsg::NameChanged(input)),
                None
            ),
            h3![C!["my-2"], "Interfaces"],
            table![
                C!["table table-striped"],
                html_elements::table_header(vec!["name", "type", "description", "actions"]),
                tbody![
                    model
                        .mdf_data
                        .interfaces
                        .iter()
                        .enumerate()
                        .map(|(index, interface)| interface_table_row(&model, index, &interface))
                        .collect::<Vec<_>>(),
                    tr![
                        td![],
                        td![],
                        td![],
                        td![html_elements::in_table_button_url(
                            "Add",
                            "primary",
                            &Urls::new(&model.base_url).interface(InterfacePage::NewInterface),
                            true
                        ),],
                    ]
                ]
            ]
        ]
    ]
}

fn interface_table_row(
    model: &Model,
    index: usize,
    interface: &mdf_format::Interface,
) -> Node<Msg> {
    tr![
        td![&interface.name],
        td![interface.interface_type.to_string()],
        td![utils::opt_vec_str_to_summary(&interface.description),],
        td![
            html_elements::in_table_button_url(
                "✎",
                "primary",
                &Urls::new(&model.base_url).interface(InterfacePage::Num(index)),
                true
            ),
            html_elements::in_table_button_msg(
                "✖",
                "danger",
                Msg::Interface(InterfaceMsg::Delete(index)),
                true
            ),
            html_elements::in_table_button_msg(
                "▲",
                "primary",
                Msg::Interface(InterfaceMsg::MoveUp(index)),
                index != 0
            ),
            html_elements::in_table_button_msg(
                "▼",
                "primary",
                Msg::Interface(InterfaceMsg::MoveDown(index)),
                index != model.mdf_data.interfaces.len() - 1
            ),
        ],
    ]
}
