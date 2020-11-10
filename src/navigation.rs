//! webapp navigation: menu top bar and navigation side bar
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use super::PageType;
use super::Urls;
use super::page::html_elements;
use super::page;

/// structure used to describe a menu entry when building the menu in HTML
pub struct MenuEntry<'a> {
    label: &'a str,
    command: MenuCommand,
}

/// menu commands, received as messages
#[derive(Clone, Copy)]
pub enum MenuCommand {
    LoadFile,
    SaveFile,
}

// ------ ------
//     Menu actions
// ------ ------
/// execute the required actions when receiving a message from a menu
pub fn do_menu(
    action: MenuCommand,
    model: &mut super::Model,
    orders: &mut impl Orders<super::Msg>,
) {
    match action {
        MenuCommand::SaveFile => {
            let filename = format!("{}.regwiz", &model.mdf_data.name);
            let jsondata =
                serde_json::to_string_pretty(&model.mdf_data).expect("serialize data to json");
            super::file_io::download_text(&filename, &jsondata);
            orders.skip();
        }
        MenuCommand::LoadFile => {
            super::file_io::choose_upload(super::FILE_INPUT_ID);
            orders.skip();
        }
    }
}

// ------ ------
//     Navigation bar
// ------ ------
/// write the top bar, including the menu, in the HTML document
pub fn navbar(model: &super::Model) -> Node<super::Msg> {
    let file_menu_entries = vec![
        MenuEntry {
            label: "Load",
            command: MenuCommand::LoadFile,
        },
        MenuEntry {
            label: "Save",
            command: MenuCommand::SaveFile,
        },
    ];

    div![
        C!["fixed-top"],
        nav![
            C!["navbar navbar-expand-lg navbar-dark bg-dark"],
            div![C!["navbar-brand"], "RegWizard"],
            button![
                C!["navbar-toggler"],
                attrs! {
                    At::Type => "button",
                    At::from("data-toggle") => "collapse",
                    At::from("data-target") => "#navbarSupportedContent",
                    At::AriaControls => "navbarSupportedContent",
                    At::AriaExpanded => "false",
                    At::AriaLabel => "Toggle navigation"
                },
                span![C!["navbar-toggler-icon"]]
            ],
            div![
                C!["collapse navbar-collapse"],
                attrs! {At::Id => "navbarSupportedContent"},
                ul![
                    C!["navbar-nav mr-auto"],
                    navbar_dropdown_menu("File", file_menu_entries),
                    navbar_item("Edit", super::PageType::Edit, model),
                    navbar_dropdown_menu("Export", Vec::<MenuEntry>::new()),
                    navbar_item("Settings", super::PageType::Settings, model),
                ],
            ],
        ],
        div![
            C!["row bg-light"],
            match model.active_page {
                PageType::Edit | PageType::Interface(_) | PageType::Register(_, _) =>
                    div![C!["col-md3 col-xl-2 mx-3 text-secondary"], " ",],
                _ => empty![],
            },
            div![
                C!["col"],
                top_toolbar(model),
            ]
        ]
    ]
}

/// generate a single entry with no submenu in the top navbar
pub fn navbar_item(
    label: &str,
    page_type: super::PageType,
    model: &super::Model,
) -> Node<super::Msg> {
    li![
        C!["nav-item"],
        a![
            C![if model.active_page == page_type {
                "nav-link active"
            } else {
                "nav-link"
            }],
            attrs! {At::Href => super::Urls::new(&model.base_url).from_page_type(page_type)},
            //      ev(Ev::Click, move |_| super::Msg::ChangePage(page_type)),
            label
        ]
    ]
}

/// generate an entry with a submenu in the top navbar
pub fn navbar_dropdown_menu(label: &str, entries: Vec<MenuEntry>) -> Node<super::Msg> {
    let menu_id = &format!("{}Dropdown", label);

    li![
        C!["nav-item dropdown"],
        a![
            C!["nav-link dropdown-toggle"],
            attrs! {
                At::Href => "#",
                At::Id => menu_id,
                At::from("role") => "button"
                At::from("data-toggle") => "dropdown",
                At::AriaHasPopup => "true",
                At::AriaExpanded => "false",
            },
            label,
        ],
        div![
            C!["dropdown-menu"],
            attrs! {
              At::AriaLabelledBy => menu_id,
            },
            entries
                .iter()
                .map(|entry| navbar_dropdown_menu_entry(entry))
                .collect::<Vec<_>>(),
        ],
    ]
}

/// generate the HTML description of a single entry in the submenu
fn navbar_dropdown_menu_entry(entry: &MenuEntry) -> Node<super::Msg> {
    let command = entry.command;

    a![
        C!["dropdown-item"],
        attrs! {
          At::Href => "#",
        },
        ev(Ev::Click, move |_| super::Msg::Menu(command)),
        entry.label
    ]
}

/// generate the icons toolbar
fn top_toolbar(model: &super::Model) -> Node<super::Msg> {
    // find the different urls
    let page_back = match model.active_page {
        PageType::Edit                        => None,
        PageType::Interface(_)                => Some(PageType::Edit),
        PageType::Register(interface_num, _)  => Some(PageType::Interface(interface_num)),
        PageType::Settings                    => None,
        _                           => None,
    };
    let page_prev = match model.active_page {
        PageType::Edit                        => None,
        PageType::Interface(interface_num) => 
            if interface_num > 0
            {
                Some(PageType::Interface(interface_num-1))
            }
            else {
                None
            },
        PageType::Register(interface_num, reg_num) =>             
            if reg_num > 0
            {
                Some(PageType::Register(interface_num,reg_num-1))
            }
            else {
                None
            },
        PageType::Settings                    => None,
        _                                     => None,
    };
    let page_next = match model.active_page {
        PageType::Edit                        => None,
        PageType::Interface(interface_num) => 
            if interface_num < model.mdf_data.interfaces.len()-1
            {
                Some(PageType::Interface(interface_num+1))
            }
            else {
                None
            },
        PageType::Register(interface_num, reg_num) =>             
            if interface_num < model.mdf_data.interfaces.len() &&
                reg_num < model.mdf_data.interfaces[interface_num].registers.len()-1
            {
                Some(PageType::Register(interface_num,reg_num+1))
            }
            else {
                None
            },
        PageType::Settings                    => None,
        _                                     => None,
    };
    let url_new = match model.active_page {
        PageType::Edit                        => None,
        PageType::Interface(_)                =>
            Some(Urls::new(&model.base_url).interface(page::interface::InterfacePage::NewInterface)),
        PageType::Register(interface_num, _)  =>             
            Some(Urls::new(&model.base_url)
                            .register(interface_num, page::register::RegisterPage::NewRegister)),
        PageType::Settings                    => None,
        _                                     => None,
    };
    div![
        C!["my-1"],
        span![
            C!["h2 mr-4"],
            match model.active_page {
                PageType::Edit                        => "File summary".to_string(),
                PageType::Interface(_)                => "Interface".to_string(),
                PageType::Register(interface_num, _)  =>             
                    {
                        let interface_name = if interface_num < model.mdf_data.interfaces.len() {
                            &model.mdf_data.interfaces[interface_num].name
                        }
                        else {
                            ""
                        };
                        if interface_name.is_empty() {
                            "Register".to_string()
                        }
                        else {
                            format!("{} / register", interface_name)
                        }
                    },
                PageType::Settings                    => "Settings".to_string(),
                PageType::NotFound                    => "Not found".to_string(),
            }
        ],
        span![
            C!["cstm-big-btn"],
            IF![model.active_page == PageType::Edit =>
                html_elements::toolbar_button_url(
                    "new",
                    &Urls::new(&model.base_url).home(),
                    true
                )
            ],
            match page_back {
                Some(page)  =>    
                    html_elements::toolbar_button_url(
                        "back",
                        &Urls::new(&model.base_url).from_page_type(page),
                        true
                ),

                None =>
                    empty!(),
            },
            match page_prev {
                Some(page)  =>    
                    html_elements::toolbar_button_url(
                        "left",
                        &Urls::new(&model.base_url).from_page_type(page),
                        true
                    ),

                None =>
                    html_elements::toolbar_button_url(
                        "left",
                        &model.base_url,
                        false
                    )
            },
            match page_next {
                Some(page)  =>    
                    html_elements::toolbar_button_url(
                        "right",
                        &Urls::new(&model.base_url).from_page_type(page),
                        true
                    ),
                None =>
                    html_elements::toolbar_button_url(
                        "right",
                        &model.base_url,
                        false
                    )
            },
            IF!(url_new.is_some() =>
                html_elements::toolbar_button_url(
                    "add",
                    &url_new.unwrap(),
                    true
                )
            )
        ]
    ]
}
