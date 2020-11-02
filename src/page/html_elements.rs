//! common HTML blocks genetators for different parts of the app

use super::super::Msg;
use seed::{prelude::*, *};

/// a single line text edit on full width
pub fn text_field_full_line(
    id: &str,
    label: &str,
    value: &str,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
    invalid_feedback: Option<&str>,
) -> Node<Msg> {
    div![
        C!["form-group row"],
        label![
            C!["col-sm-2 col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            C!["col-sm-10"],
            input![
                C!["form-control"],
                attrs! {
                  At::Type => "text",
                  At::Id => id,
                  At::Value => value,
                },
                input_ev(Ev::Change, handler),
            ],
            IF!(invalid_feedback.is_some() => div![
              C!["invalid-feedback"],
              invalid_feedback.unwrap()
            ])
        ]
    ]
}

/// a select input generated from an enum, on full width
pub fn select_field_full_line<
    T: strum::IntoEnumIterator + std::string::ToString + std::cmp::PartialEq<T>,
>(
    id: &str,
    label: &str,
    selected: &T,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
) -> Node<Msg> {
    div![
        C!["form-group row"],
        label![
            C!["col-sm-2 col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            C!["col-sm-10"],
            select![
                C!["form-control"],
                attrs! {
                  At::Id => id,
                  At::Value =>&selected.to_string(),
                },
                input_ev(Ev::Change, handler),
                T::iter()
                    .map(|entry| option![
                        IF!(&entry == selected =>
                        attrs!{
                          At::Selected => "selected",
                        }),
                        entry.to_string(),
                    ])
                    .collect::<Vec<_>>(),
            ]
        ]
    ]
}

/// a multiple line text area on full width
pub fn textarea_field(
    id: &str,
    label: &str,
    value: &str,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
) -> Node<Msg> {
    div![
        C!["form-group row"],
        label![
            C!["col-sm-2 col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            C!["col-sm-10"],
            textarea![
                C!["form-control"],
                attrs! {
                  At::Type => "text",
                  At::Id => id,
                  At::Value => value,
                },
                input_ev(Ev::Change, handler),
            ]
        ]
    ]
}

/// table header, generated from a vector of strings
pub fn table_header(labels: Vec<&str>) -> Node<Msg> {
    thead![tr![labels
        .iter()
        .map(|label| th![
            attrs! {
              At::Scope => "col"
            },
            label
        ])
        .collect::<Vec<_>>()]]
}
