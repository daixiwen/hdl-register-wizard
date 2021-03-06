//! common HTML blocks genetators for different parts of the app

use crate::Msg;
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

/// a single line text edit as a part of a line
pub fn text_field_sub_line(
    id: &str,
    label: &str,
    value: &str,
    disabled: bool,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
    invalid_feedback: Option<&str>,
) -> Node<Msg> {
    div![
        C!["col-auto flex-nowrap form-group mb-2"], //
        label![
            C!["col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            C!["col-auto"],
            input![
                C!["form-control"],
                attrs! {
                  At::Type => "text",
                  At::Id => id,
                  At::Value => value,
                },
                IF!(disabled => attrs!{At::Disabled => "disabled"}),
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
                  At::Value => &selected.to_string(),
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

/// a select input generated from an option enum, on sub line
pub fn select_option_field_sub_line<
    T: 'static +  strum::IntoEnumIterator + std::string::ToString + std::cmp::PartialEq<T> + Copy,
>(
    id: &str,
    label: &str,
    selected: &Option<T>,
    text_if_none: &'static str,
    handler: impl FnOnce(String) -> Msg + 'static + Clone + Copy,
) -> Node<Msg> {
    div![
        C!["col-auto flex-nowrap form-group mb-2"],
        label![
            C!["col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            button![
                C!["mx-3 form-control btn btn-outline-dark dropdown-toggle"],
                attrs! {
                    At::Id => id,
                    At::Type => "button",
                    At::from("data-toggle") => "dropdown",
                    At::AriaHasPopup => "true",
                    At::AriaExpanded => "false"
                },
                IF!(text_if_none.is_empty() && selected.is_none() =>
                    attrs! {
                        At::Disabled => "disabled"
                    }),
                IF!(selected.is_none() => text_if_none ),
                IF!(selected.is_some() => selected.unwrap().to_string()),
            ],
            div![
                C!["dropdown-menu"],
                attrs! {
                    At::AriaLabelledBy => id
                },
                IF!(!text_if_none.is_empty() =>
                    button![
                        C!["dropdown-item"],
                        attrs!{
                            At::Type => "button"
                        },
                        IF!(selected.is_none() =>
                            C!["active"]
                        ),
                        text_if_none,
                        ev(Ev::Click, move |_| handler(text_if_none.to_string()))
                    ]
                ),
                IF!(!text_if_none.is_empty() || selected.is_some() =>
                    T::iter()
                        .map(|entry| button![
                            C!["dropdown-item"],
                            attrs!{
                                At::Type => "button"
                            },                            
                            IF!(selected == &Some(entry) =>
                                C!["active"]
                            ),
                            entry.to_string(),
                            ev(Ev::Click, move |_| handler(entry.to_string()))
                        ])
                        .collect::<Vec<_>>()
                ),
            ]
        ]
    ]
}

/// a select input generated from an option enum, on sub line (old version with a select html element)
pub fn old_select_option_field_sub_line<
    T: strum::IntoEnumIterator + std::string::ToString + std::cmp::PartialEq<T> + Copy,
>(
    id: &str,
    label: &str,
    selected: &Option<T>,
    text_if_none: &str,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
) -> Node<Msg> {
    div![
        C!["col-auto flex-nowrap form-group mb-2"],
        label![
            C!["col-sm-2 col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            C!["col-sm-10"],
            form![
                attrs!{
                    At::AutoComplete => "off"
                },
                // workaround a 15 years old bug in Firefox where it wouldn't always show the selected option if the select tag is not inside a form tag
                select![
                    C!["form-control"],
                    attrs! {
                        At::Id => id,
                        At::AutoComplete => "off"
                    },
                    IF!(text_if_none.is_empty() && selected.is_none() =>
                        attrs! {
                            At::Disabled => "disabled"
                        }),
                    input_ev(Ev::Change, handler),
                    IF!(!text_if_none.is_empty() =>
                        option![
                            IF!(selected.is_none() =>
                            attrs!{
                                At::Selected => "selected",
                            }),
                            text_if_none,
                        ]
                    ),
                    IF!(!text_if_none.is_empty() || selected.is_some() =>
                        T::iter()
                            .map(|entry| option![
                                IF!(selected == &Some(entry) =>
                                    attrs!{
                                        At::Selected => "selected",
                                    }
                                ),
                                entry.to_string(),
                            ])
                            .collect::<Vec<_>>()
                    ),
                ]
            ]
        ]
    ]
}

/// a select input generated from an option enum, on sub line
pub fn select_field_sub_line<
    T: 'static +  strum::IntoEnumIterator + std::string::ToString + std::cmp::PartialEq<T> + std::clone::Clone,
>(
    id: &str,
    label: &str,
    selected: &T,
    handler: impl FnOnce(String) -> Msg + 'static + Clone + Copy,
) -> Node<Msg> {
    div![
        C!["col-auto flex-nowrap form-group mb-2"],
        label![
            C!["col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            button![
                C!["mx-3 form-control btn btn-outline-dark dropdown-toggle"],
                attrs! {
                    At::Id => id,
                    At::Type => "button",
                    At::from("data-toggle") => "dropdown",
                    At::AriaHasPopup => "true",
                    At::AriaExpanded => "false"
                },
                selected.to_string()
            ],
            div![
                C!["dropdown-menu"],
                attrs! {
                    At::AriaLabelledBy => id
                },
                T::iter()
                    .map(|entry| button![
                        C!["dropdown-item"],
                        attrs!{
                            At::Type => "button"
                        },                            
                        IF!(selected == &entry =>
                            C!["active"]
                        ),
                        entry.to_string(),
                        ev(Ev::Click, move |_| handler(entry.to_string()))
                    ])
                    .collect::<Vec<_>>(),
            ]
        ]
    ]
}

/// a select input generated from an option enum, on sub line
pub fn old_select_field_sub_line<
    T: strum::IntoEnumIterator + std::string::ToString + std::cmp::PartialEq<T>,
>(
    id: &str,
    label: &str,
    selected: &T,
    handler: impl FnOnce(String) -> Msg + 'static + Clone,
) -> Node<Msg> {
    div![
        C!["col-auto flex-nowrap form-group mb-2"],
        label![
            C!["col-sm-2 col-form-label"],
            attrs! {
              At::For => id,
            },
            label
        ],
        div![
            C!["col-sm-10"],
            form![
                // workaround a 15 years old bug in Firefox where it wouldn't always show the selected option if the select tag is not inside a form tag (still doesnt work always)
                attrs!{
                    At::AutoComplete => "off"
                },
                select![
                    C!["form-control"],
                    attrs! {
                        At::Id => id,
                        At::AutoComplete => "off"
                    },
                    input_ev(Ev::Change, handler),
                    T::iter()
                        .map(|entry| option![
                            IF!(selected == &entry =>
                                attrs!{
                                    At::Selected => "selected",
                                }
                            ),
                            entry.to_string(),
                        ])
                        .collect::<Vec<_>>(),
                ]
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

/// add a button that sends to the given url
pub fn toolbar_button_url(label: &str, url: &Url, enabled: bool) -> Node<Msg> {
    let url_str = if enabled {
        url.to_string()
    }
    else {
        "#".to_string()
    };

    a![
        C![&format!("cstm-toolbar-btn cstm-{}-btn mx-1", label)],
        IF![enabled => 
            C![&format!("cstm-{}-btn-enabled", label)]
        ],
        IF![!enabled => 
            C![&format!("cstm-{}-btn-disabled", label)]
        ],
        attrs! {
            At::Href => &url_str
        },
        span![
            label,
        ],
    ]
}

/// add a button that sends the given message
pub fn toolbar_button_msg(label: &str, msg: Msg, enabled: bool) -> Node<Msg> {
    a![
        C![&format!("cstm-toolbar-btn cstm-{}-btn mx-1", label)],
        attrs! {
            At::Href => "#"
        },
        IF![enabled => 
            C![&format!("cstm-{}-btn-enabled", label)]
        ],
        IF![!enabled => 
            C![&format!("cstm-{}-btn-disabled", label)]
        ],
        span![
            label,
        ],
        IF![enabled => 
            ev(Ev::Click, move |_| msg)
        ],
    ]
}
