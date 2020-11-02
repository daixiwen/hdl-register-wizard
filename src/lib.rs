//! # HDL Register Wizard
//!
//! This is a webapp that can generate VHDL code and documentation to create hardware registers accessible on a memory mapped bus. It can load and save files in the Model Description Format developped by Bitvis for its [Register Wizard](https://bitvis.no/dev-tools/register-wizard/). Files saved by this webapp should be usable by Bitvis' tool.
//!
//! ## Project Status
//!
//! The project is under development and is not currently usable. The aim for the first release is to be able to load and save MDF files, as the [Bitvis Register Wizard](https://bitvis.no/dev-tools/register-wizard/) currently lacks a GUI.
//! A future release will also be able to generate code and documentation.
//!
//! The code is currently hosted on [Github](https://github.com/daixiwen/hdl-register-wizard).
//! ## Project License
//!
//! The project uses an MIT license.

#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

pub mod file_io;
pub mod mdf_format;
pub mod navigation;

pub mod page;
mod tests;
pub mod utils;

// URLs
const SETTINGS: &str = "settings";
const INTERFACE: &str = "interface";
const REGISTER: &str = "register";

// IDs
/// HTML ID of a hidden file input element used to open files
pub const FILE_INPUT_ID: &str = "hidden_file_input";

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    let base_url = url.to_base_url();
    orders
        .subscribe(Msg::UrlChanged)
        .notify(subs::UrlChanged(url));
    Model {
        base_url,
        active_page: PageType::Edit,
        mdf_data: mdf_format::Mdf::new(),
    }
}

// ------ ------
//     Model
// ------ ------

/// application state
pub struct Model {
    /// URL of the webapp main page.
    /// All the other URLS used in the app are relative to this one
    base_url: Url,

    /// indicates what page is currently displayed in the app
    active_page: PageType,

    /// structure holding the full registers description
    mdf_data: mdf_format::Mdf,
}

/// enumeration describing the currently displayed page
#[derive(Copy, Clone, PartialEq)]
pub enum PageType {
    Edit,
    Interface(usize),
    Register(usize, usize),
    Settings,
    NotFound,
}

// ------ ------
//     Urls
// ------ ------

struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
    fn settings(self) -> Url {
        self.base_url().add_path_part(SETTINGS)
    }

    /// Generate a URL relative to an interface.
    ///
    /// The InterfacePage enum decribes any page that can be displayed in order to work on an interface
    pub fn interface(self, interface_page: page::interface::InterfacePage) -> Url {
        page::interface::interface_url(self.base_url().add_path_part(INTERFACE), interface_page)
    }

    /// Generate a URL relative to a register in an interface
    /// the RegisterPage enum decribes any page that can be displayed in order to work on a register
    pub fn register(
        self,
        interface_num: usize,
        register_page: page::register::RegisterPage,
    ) -> Url {
        page::register::register_url(
            page::interface::interface_url(
                self.base_url().add_path_part(INTERFACE),
                page::interface::InterfacePage::Num(interface_num),
            )
            .add_path_part(REGISTER),
            register_page,
        )
    }
    fn from_page_type(self, page: PageType) -> Url {
        match page {
            PageType::Edit => self.home(),
            PageType::Settings => self.settings(),
            PageType::Interface(index) => {
                self.interface(page::interface::InterfacePage::Num(index))
            }
            PageType::Register(interface_num, index) => {
                self.register(interface_num, page::register::RegisterPage::Num(index))
            }
            _ => self.home(),
        }
    }
}

// ------ ------
//    Update
// ------ ------

/// Message used to communicate between the GUI and the system core.
///
/// Some messages are regrouped under a single entry as an enum holding
/// more information about a specific message in a specific context.
#[derive(Clone)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Menu(navigation::MenuCommand),
    Edit(page::edit::EditMsg),
    Interface(page::interface::InterfaceMsg),
    Register(usize, page::register::RegisterMsg),
    UploadStart(web_sys::Event),
    UploadText(String),
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.active_page = match url.next_path_part() {
                None => PageType::Edit,
                Some(SETTINGS) => PageType::Settings,
                Some(INTERFACE) => page::interface::change_url(url, model),
                Some(_) => PageType::NotFound,
            }
        }
        Msg::Menu(action) => navigation::do_menu(action, model, orders),

        Msg::Edit(edit_msg) => page::edit::update(edit_msg, model, orders),

        Msg::Interface(interface_msg) => page::interface::update(interface_msg, model, orders),

        Msg::Register(interface_num, register_msg) => {
            page::register::update(register_msg, interface_num, model, orders)
        }

        Msg::UploadStart(event) => {
            file_io::upload_file(event, orders);
            orders.skip();
        }

        Msg::UploadText(text) => {
            let decode: Result<mdf_format::Mdf, serde_json::Error> = serde_json::from_str(&text);
            match decode {
                Ok(decoded) => model.mdf_data = decoded,

                Err(error) => {
                    modal_window(
                        "upload error",
                        &format!("error while reading file:<br>{}", error),
                    );
                    orders.skip();
                }
            }
        }
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        navigation::navbar(model),
        div![
            C!["container-fluid"],
            div![
                C!["row"],
                match model.active_page {
                    PageType::Edit | PageType::Interface(_) | PageType::Register(_, _) =>
                        div![C!["col-md3 col-xl-2 bd-sidebar"], "sidebar",],
                    _ => empty![],
                },
                div![
                    C!["col"],
                    div![match model.active_page {
                        PageType::Edit => page::edit::view(model),
                        PageType::Settings => page::settings::view(model),
                        PageType::Interface(index) => page::interface::view(model, index),
                        PageType::Register(interface_num, reg_num) =>
                            page::register::view(model, interface_num, reg_num),
                        _ => div!["404 not found"],
                    },]
                ]
            ]
        ],
        div![
            attrs! {
              At::Style => "display: none",
            },
            input![
                attrs! {
                  At::Id => FILE_INPUT_ID,
                  At::Type => "file",
                  At::Accept => ".regwiz,.mdf,text/plain"
                },
                ev(Ev::Change, |event| Msg::UploadStart(event))
            ]
        ],
        div![attrs! {
          At::Id => "modal",
        }]
    ]
}

/// makes a modal window (alert) appear in front of the rest of the page
pub fn modal_window(title: &str, content: &str) {
    let modal_window = seed::document().get_element_by_id("modal").unwrap();

    modal_window.set_inner_html(&format!(r##"
<div class="modal fade" id="function_modal" tabindex="-1" aria-labelledby="function_modal_label" aria-hidden="true">
  <div class="modal-dialog">
  <div class="modal-content">
    <div class="modal-header">
    <h5 class="modal-title" id="function_modal_label">{}</h5>
    <button type="button" class="close" data-dismiss="modal" aria-label="Close">
      <span aria-hidden="true">&times;</span>
    </button>
    </div>
    <div class="modal-body">
    <p>{}</p>
    </div>
    <div class="modal-footer">
    <button type="button" class="btn btn-secondary" data-dismiss="modal">Close</button>
    </div>
  </div>
  </div>
</div>
<button style="display: none" type="button" data-toggle="modal" data-target="#function_modal" id="function_modal_button">
</button>
    "##, title, content));

    // we could also directly call the javascript instead of clicking on the button
    let button = seed::document()
        .get_element_by_id("function_modal_button")
        .unwrap();
    let event = seed::document()
        .create_event("MouseEvents")
        .expect("should be able to call createEvent()")
        .dyn_into::<web_sys::MouseEvent>()
        .ok()
        .expect("should be a MouseEvent");
    event.init_mouse_event_with_can_bubble_arg_and_cancelable_arg("click", true, true);
    let _ = button.dispatch_event(&event);
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
/// This function is invoked by `init` function in `index.html`. Start the application
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
