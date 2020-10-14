#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use super::super::Model;
use super::super::Msg;
use super::super::mdf_format;

pub enum EditMsg {
  NameChanged(String)
}

// `update` describes how to handle each `Msg`.
pub fn update(msg: EditMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        EditMsg::NameChanged(new_name) => {
           model.mdf_data.name = new_name;
           orders.skip();
        },
    }
}


pub fn view(model: &Model) -> Node<Msg> {
  div![
    div![
      C!["my-3"],
      button![
        attrs!{
          At::Type => "button",
          At::Class => "btn btn-primary",
        },
        "New Model"
      ]
    ],
    div![
      h3![
        C!["my-2"],
        "Model Description File"],
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
                At::Value => &model.mdf_data.name,
              },
//              ev(Ev::Change, |new_text| Msg::Edit(EditMsg::NameChanged(new_text))),
              input_ev(Ev::Change, move | input | Msg::Edit(EditMsg::NameChanged(input))),
            ]
          ]
        ]
      ],
      h3![
        C!["my-2"],
        "Interfaces"],
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
              "type"
            ],
            th![
              attrs!{
                At::Scope => "col"
              },
              "description"
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
          model.mdf_data.interfaces.iter().enumerate().map(|(index, interface)| interface_table_row(index, &interface)).collect::<Vec<_>>(),
          tr![
            td![],
            td![],
            td![],
            td![
              in_table_button(0, "Add", "primary"), 
            ],           
          ]
        ]
      ]
    ]
  ]
}

fn interface_table_row(index : usize, interface : &mdf_format::Interface) -> Node<Msg>
{
  tr![
    td![
      &interface.name
    ],
    td![
      interface.interface_type.to_string()
    ],
    td![
      match &interface.description
      {
        None => empty![],
        Some(description) => {
          match description.len() {
            0 => empty![],
            1 => plain![description[0].clone()],
            _ => plain![format!("{} ...",description[0])],
          }
        }
      }
    ],
    td![
      in_table_button(index, "▲", "primary"),
      in_table_button(index, "▼", "primary"),
      in_table_button(index, "✎", "primary"),
      in_table_button(index, "✖", "danger"),
    ],    
  ]
}

fn in_table_button(_index: usize, label: &str, color: &str) -> Node<Msg>
{
  a![
    C![&format!("btn btn-sm mx-1 btn-outline-{}", color)],
    attrs!{
      At::Href => "#"
    },
    label
  ]
}
