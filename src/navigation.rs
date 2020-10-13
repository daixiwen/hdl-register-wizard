#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

pub struct MenuEntry<'a> {
  label : &'a str,
  command : MenuCommand,
}

#[derive(Clone, Copy)]
pub enum MenuCommand {
    LoadFile,
    SaveFile
}

// ------ ------
//     Menu actions
// ------ ------
pub fn do_menu(action: MenuCommand, model: &mut super::Model, orders: &mut impl Orders<super::Msg>) {
    match action {
        MenuCommand::SaveFile => {
            let filename = format!("{}.mdf", &model.mdf_data.name);
            let jsondata = serde_json::to_string(&model.mdf_data).expect("serialize data to json");
            super::file_io::download_text(&filename, &jsondata);
            orders.skip();
        },
        MenuCommand::LoadFile => {
        
            super::file_io::choose_upload("hidden_file_input");
            orders.skip();
        },
    }
}

// ------ ------
//     Navigation bar
// ------ ------

pub fn navbar(model: &super::Model) -> Node<super::Msg> {
  let file_menu_entries = vec![
    MenuEntry{
      label : "Load",
      command : MenuCommand::LoadFile
    },
    MenuEntry{
      label : "Save",
      command : MenuCommand::SaveFile
    },
  ];

  nav![
    C!["navbar navbar-expand-lg navbar-dark bg-dark"],
    div![
        C!["navbar-brand"],
        "RegWizard"],
    button![
        C!["navbar-toggler"],
        attrs!{
            At::Type => "button",
            At::from("data-toggle") => "collapse",
            At::from("data-target") => "#navbarSupportedContent",
            At::AriaControls => "navbarSupportedContent",
            At::AriaExpanded => "false",
            At::AriaLabel => "Toggle navigation"
        },
        span![
            C!["navbar-toggler-icon"]
        ]
    ], 
    div![
        C!["collapse navbar-collapse"],
        attrs!{At::Id => "navbarSupportedContent"},
        ul![
            C!["navbar-nav mr-auto"],
            navbar_dropdown_menu("File", file_menu_entries),
            navbar_item("Edit", super::PageType::Edit, model),
            navbar_dropdown_menu("Export", Vec::<MenuEntry>::new()),
            navbar_item("Settings", super::PageType::Settings, model),
        ],
    ],
  ]
}

pub fn navbar_item(label: &str, page_type: super::PageType, model: &super::Model) -> Node<super::Msg> {
  li![
    C!["nav-item"],
    a![
      C![
        if model.active_page == page_type
        {
          "nav-link active"
        }
        else {
          "nav-link"
        }
      ], 
      attrs!{At::Href => super::Urls::new(&model.base_url).from_page_type(page_type)},
//      ev(Ev::Click, move |_| super::Msg::ChangePage(page_type)),
      label]]
}

pub fn navbar_dropdown_menu(label: &str, entries: Vec<MenuEntry>) -> Node<super::Msg> {
  let menu_id = &format!("{}Dropdown", label);

  li![
    C!["nav-item dropdown"],
    a![
      C!["nav-link dropdown-toggle"],
      attrs!{
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
      attrs!{
        At::AriaLabelledBy => menu_id,
      },
      entries.iter().map(|entry| navbar_dropdown_menu_entry(entry)).collect::<Vec<_>>(),
    ],
  ]
}

pub fn navbar_dropdown_menu_entry(entry: &MenuEntry) -> Node<super::Msg> {
  let command = entry.command;

  a![
    C!["dropdown-item"],
    attrs!{
      At::Href => "#",
    },
    ev(Ev::Click, move |_| super::Msg::Menu(command)),
    entry.label
  ]
}
