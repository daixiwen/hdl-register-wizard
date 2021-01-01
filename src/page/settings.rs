//! settings page (TBD)

#![allow(clippy::wildcard_imports)]

use crate::Model;
use crate::Msg;
use seed::{prelude::*, *};

use super::html_elements;
use crate::settings::MultiWindow;
use crate::utils;

// ID constants
const ID_UNDO_LEVEL: &str = "inputUndoLevel";

// ------ ------
//    Update
// ------ ------

/// messages handling settings
#[derive(Clone)]
pub enum SettingsMsg {
    /// change the undo level
    UndoLevelChanged(String),
    MultiIndependentSelected,
    MultiViewsSelected
}

fn validate_undo_level(string_input: &str) -> Result<u32, std::num::ParseIntError> {
    match u32::from_str_radix(string_input, 10) {
        Ok(value) => {
            if value >= 1 {
                Ok(value)
            }
            else {
                // cause a parse error
                u32::from_str_radix("z", 10)
            }
        }
        Err(error) => Err(error)
    }
}

/// process a settings message
pub fn update(
    msg: SettingsMsg,
    model: &mut Model,
    orders: &mut impl Orders<Msg>,
    )
            -> Option<Msg> {

    match msg {
        SettingsMsg::UndoLevelChanged(new_level) => {
            match utils::validate_field(ID_UNDO_LEVEL, &new_level, |field_value| {
                    validate_undo_level(field_value) }) {
                Ok(level) => {
                    let old_level = std::mem::replace(&mut model.settings.undo_level, level);
                    Some(Msg::Settings(SettingsMsg::UndoLevelChanged(old_level.to_string())))
                }
                Err(_) => {
                    orders.skip();
                    None
                }
            }
        }

        SettingsMsg::MultiIndependentSelected => {
            match std::mem::replace(&mut model.settings.multi_window, MultiWindow::Independent) {
                // only put an undo is the previous choice was different
                MultiWindow::Views => Some(Msg::Settings(SettingsMsg::MultiViewsSelected)),
                _ => None
            }
        }

        SettingsMsg::MultiViewsSelected => {
            match std::mem::replace(&mut model.settings.multi_window, MultiWindow::Views) {
                // only put an undo is the previous choice was different
                MultiWindow::Independent => Some(Msg::Settings(SettingsMsg::MultiIndependentSelected)),
                _ => None                
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    div![
    // Undo level
        html_elements::text_field_full_line(
            ID_UNDO_LEVEL,
            "Number of undo levels",
            &model.settings.undo_level.to_string(),
            move |input| Msg::Settings(SettingsMsg::UndoLevelChanged(input)),
            Some("please use a decimal value of 1 or more")
        ),
        div![
            C!["form-group row"],
            div![
                C!["col-sm-2 col-form-label"],
                "Multi window behaviour"
            ],
            div![
                C!["col-sm-10"],
                div![
                    C!["form-check"],
                    input![
                        C!["form-check-input"],
                        attrs!{
                            At::Type => "radio",
                            At::Name => "multiWindow",
                            At::Value => "independent",
                        },
                        IF!(model.settings.multi_window == MultiWindow::Independent =>
                            attrs!{At::Checked => "checked"}),
                        id!["multiIndependent"],
                        ev(Ev::Click, move | _ | Msg::Settings(SettingsMsg::MultiIndependentSelected)),
                    ],
                    label![
                        C!["form-check-label"],
                        attrs!{
                            At::For => "multiIndependent"
                        },
                        "Independent view of different files"
                    ]
                ],
                div![
                    C!["form-check"],
                    input![
                        C!["form-check-input"],
                        attrs!{
                            At::Type => "radio",
                            At::Name => "multiWindow",
                            At::Value => "views",
                        },
                        IF!(model.settings.multi_window == MultiWindow::Views =>
                            attrs!{At::Checked => "checked"}),
                        id!["multiViews"],
                        ev(Ev::Click, move | _ | Msg::Settings(SettingsMsg::MultiViewsSelected)),
                    ],
                    label![
                        C!["form-check-label"],
                        attrs!{
                            At::For => "multiViews"
                        },
                        "Multiple views of the same file"
                    ]
                ],
            ],
        ]
    ]
}
