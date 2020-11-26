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
    NewFile,
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
            model.mdf_data.clean();
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
        MenuCommand::NewFile => {
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
            label: "New",
            command: MenuCommand::NewFile,
        },
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
            div![C!["navbar-brand"],
                a![ 
                    "RegWizard",
                    attrs!{
                        At::Href => super::Urls::new(&model.base_url).home()
                    }
                ],
            ],
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
                PageType::Edit | PageType::Interface(_) | PageType::Register(_, _) | PageType::Field(_, _, _) =>
                    div![C!["d-none d-md-block col-md-3 col-xl-2 mx-3 text-secondary"], " ",],
                _ => div![C!["mx-3"]," "],
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
        PageType::Field(int_num, reg_num, _)  => Some(PageType::Register(int_num, reg_num)),
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
        PageType::Field(interface_num, reg_num, field_num) =>             
            if field_num > 0
            {
                Some(PageType::Field(interface_num,reg_num, field_num-1))
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
        PageType::Field(interface_num, reg_num, field_num) =>             
            if interface_num < model.mdf_data.interfaces.len() &&
                reg_num < model.mdf_data.interfaces[interface_num].registers.len() &&
                field_num < model.mdf_data.interfaces[interface_num].registers[reg_num].fields.len()-1
            {
                Some(PageType::Field(interface_num,reg_num, field_num+1))
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
        PageType::Field(interface_num, register_num, _)  =>             
            Some(Urls::new(&model.base_url)
                            .field(interface_num, register_num, page::field::FieldPage::NewField)),
        PageType::Settings                    => None,
        _                                     => None,
    };
    div![
        C!["my-1"],
        span![
            C!["h2 mr-4"],
            match model.active_page {
                PageType::Home                        => "Home".to_string(),
                PageType::Edit                        => "File summary".to_string(),
                PageType::Interface(_)                => "Interface".to_string(),
                PageType::Register(interface_num, _)  =>             
                    {
                        let int_name = interface_name(model, interface_num);
                        if int_name.is_empty() {
                            "register".to_string()
                        }
                        else {
                            format!("{} / register", int_name)
                        }
                    },
                PageType::Field(interface_num, register_num, _)  =>             
                    {
                        let int_name = interface_name(model, interface_num);
                        let int_part = if int_name.is_empty() {
                            "".to_string()
                        }
                        else {
                            format!("{} / ", int_name)
                        };
                        let reg_name = register_name(model, interface_num, register_num);
                        if reg_name.is_empty() {
                            "field".to_string()
                        }
                        else {
                            format!("{}{} / field", int_part, reg_name)
                        }
                    },
                PageType::Settings                    => "Settings".to_string(),
                PageType::NotFound                    => "Not found".to_string(),
            }
        ],
        span![
            C!["cstm-big-btn"],
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
            ),
            html_elements::toolbar_button_msg(
                "undo",
                super::Msg::Undo(super::undo::UndoMsg::Undo),
                model.undo.has_undo()
            ),
            html_elements::toolbar_button_url(
                "redo",
                &Urls::new(&model.base_url).home(),
                false
            ),
        ]
    ]
}

/// generate the sidebar
pub fn sidebar(model: &super::Model) -> Node<super::Msg> {
    nav![
        C!["nav flex-column nav-pills"],
        model
            .mdf_data
            .interfaces
            .iter()
            .enumerate()
            .map(|(index, interface)| sidebar_interface(&model, index, &interface))
            .collect::<Vec<_>>(),
    ]
}

fn sidebar_interface(model: &super::Model, int_index: usize, interface: &super::mdf_format::Interface) -> Node<super::Msg> {
    div![
        IF!(model.mdf_data.interfaces.len() > 1 =>
            a![
                C!["nav-link"],
                IF!(model.active_page == PageType::Interface(int_index) =>
                    C!["active"]),
                attrs![
                    At::Href => 
                        &Urls::new(&model.base_url).interface(page::interface::InterfacePage::Num(int_index))
                ],
                IF!(interface.name.is_empty() =>
                    em!["interface"]), 
                IF!(!interface.name.is_empty() =>
                    &interface.name)
            ]
        ),
        interface
            .registers
            .iter()
            .enumerate()
            .map(|(reg_index, register)| sidebar_register(&model, int_index, reg_index, &register))
            .collect::<Vec<_>>(),
    ]
}

fn sidebar_register(model: &super::Model, int_index: usize, reg_index: usize, register: &super::mdf_format::Register) -> Node<super::Msg> {
    a![
        C!["nav-link ml-3"],
        IF!(model.active_page == PageType::Register(int_index, reg_index) =>
            C!["active"]),
        attrs![
            At::Href => 
                &Urls::new(&model.base_url).register(int_index, page::register::RegisterPage::Num(reg_index))
        ],
        IF!(register.name.is_empty() =>
            em!["register"]), 
        IF!(!register.name.is_empty() =>
            &register.name)
    ]
}

fn interface_name ( model: &super::Model, number: usize) -> &str {
    if number < model.mdf_data.interfaces.len() {
        &model.mdf_data.interfaces[number].name
    }
    else {
        ""
    }
}

fn register_name ( model: &super::Model, interface: usize, number: usize) -> &str {
    if interface < model.mdf_data.interfaces.len() {
        if number < model.mdf_data.interfaces[interface].registers.len() {
            &model.mdf_data.interfaces[interface].registers[number].name
        }
        else {
            ""
        }
    }
    else {
        ""
    }
}
