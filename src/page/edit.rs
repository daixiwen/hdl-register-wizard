#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use super::super::Model;
use super::super::Msg;

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
        "New"
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
    ]
  ]
}
