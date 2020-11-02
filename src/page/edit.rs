#![allow(clippy::wildcard_imports)]

//! Main edit page, with the model name and the list of interfaces

use super::super::mdf_format;
use super::super::Model;
use super::super::Msg;
use super::super::Urls;
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
            div![div![
                C!["form-group row"],
                label![
                    C!["col-sm-2 col-form-label"],
                    attrs! {
                      At::For => "inputName",
                    },
                    "Name"
                ],
                div![
                    C!["col-sm-10"],
                    input![
                        C!["form-control"],
                        attrs! {
                          At::Type => "text",
                          At::Id => "inputName",
                          At::Value => &model.mdf_data.name,
                        },
                        //              ev(Ev::Change, |new_text| Msg::Edit(EditMsg::NameChanged(new_text))),
                        input_ev(Ev::Change, move |input| Msg::Edit(EditMsg::NameChanged(
                            input
                        ))),
                    ]
                ]
            ]],
            h3![C!["my-2"], "Interfaces"],
            table![
                C!["table table-striped"],
                thead![tr![
                    th![
                        attrs! {
                          At::Scope => "col"
                        },
                        "name"
                    ],
                    th![
                        attrs! {
                          At::Scope => "col"
                        },
                        "type"
                    ],
                    th![
                        attrs! {
                          At::Scope => "col"
                        },
                        "description"
                    ],
                    th![
                        attrs! {
                          At::Scope => "col"
                        },
                        "actions"
                    ],
                ]],
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
                        td![in_table_button_url(
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
            in_table_button_url(
                "✎",
                "primary",
                &Urls::new(&model.base_url).interface(InterfacePage::Num(index)),
                true
            ),
            in_table_button_msg(
                "✖",
                "danger",
                Msg::Interface(InterfaceMsg::Delete(index)),
                true
            ),
            in_table_button_msg(
                "▲",
                "primary",
                Msg::Interface(InterfaceMsg::MoveUp(index)),
                index != 0
            ),
            in_table_button_msg(
                "▼",
                "primary",
                Msg::Interface(InterfaceMsg::MoveDown(index)),
                index != model.mdf_data.interfaces.len() - 1
            ),
        ],
    ]
}

/// add a button that sends to the given url
/// TODO: move to html_elements
pub fn in_table_button_url(label: &str, color: &str, url: &Url, enabled: bool) -> Node<Msg> {
    a![
        C![&format!("btn btn-sm mx-1 btn-outline-{}", color)],
        attrs! {
          At::Href => url
        },
        IF![! enabled => attrs!{ At::Disabled => "disaled"}],
        label
    ]
}

/// add a button that sends the given message
/// TODO: move to html_elements
pub fn in_table_button_msg(label: &str, color: &str, msg: Msg, enabled: bool) -> Node<Msg> {
    button![
        C![&format!("btn btn-sm mx-1 btn-outline-{}", color)],
        attrs! {
          At::Type => "button"
        },
        IF![! enabled => attrs!{ At::Disabled => "disaled"}],
        label,
        ev(Ev::Click, move |_| msg),
    ]
}
