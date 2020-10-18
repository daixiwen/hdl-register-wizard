// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

mod navigation;
mod mdf_format;
mod file_io;

mod page;

// URLs
const SETTINGS: &str = "settings";
const INTERFACE: &str = "interface";

// IDs
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
        active_page : PageType::Edit,
        mdf_data : mdf_format::Mdf::new(),
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
pub struct Model {
    base_url : Url,
    active_page : PageType,
    mdf_data : mdf_format::Mdf
}

#[derive(Copy, Clone, PartialEq)]
pub enum PageType {
    Edit,
    Interface(usize),
    Settings,
    NotFound
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
    pub fn interface(self, interface_page : page::interface::InterfacePage) -> Url {
        page::interface::interface_url(self.base_url().add_path_part(INTERFACE), interface_page)
    }
    fn from_page_type(self, page : PageType) -> Url
    {
        match page {
            PageType::Edit => self.home(),
            PageType::Settings => self.settings(),
            PageType::Interface(index) => self.interface(page::interface::InterfacePage::Num(index)),
            _ => self.home()
        }
    }
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
#[derive(Clone)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),
    Menu(navigation::MenuCommand),
    Edit(page::edit::EditMsg),
    Interface(page::interface::InterfaceMsg),
    UploadStart(web_sys::Event),
    UploadText(String)
}


// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(mut url)) => {
            model.active_page = match url.next_path_part() {
                None => PageType::Edit,
                Some(SETTINGS) => PageType::Settings,
                Some(INTERFACE) => page::interface::change_url(url.next_path_part(), model),
                Some(_) => PageType::NotFound
            }
        },
        Msg::Menu(action) =>
            navigation::do_menu(action, model, orders),

        Msg::Edit(edit_msg) => page::edit::update(edit_msg, model, orders),

        Msg::Interface(interface_msg) => page::interface::update(interface_msg, model, orders),

        Msg::UploadStart(event) => {
            file_io::upload_file(event, orders);
            orders.skip();
        }

        Msg::UploadText(text) => {
            let decode : Result<mdf_format::Mdf, serde_json::Error> = serde_json::from_str(&text);
            match decode {
                Ok(decoded) =>
                    model.mdf_data = decoded,

                Err(error) => {
                    modal_window("upload error", &format!("error while reading file:<br>{}", error));
                    orders.skip();
                },
            }
        }
    }
}

// utility to mark a form field as invalid if its contents can't be parsed
pub fn validate_field<F,T,E>(field_id: &str, field_new_value: &str, decode_value: F) -> Result<T,E> 
    where  F: Fn(&str) -> Result<T,E>
{
  let result = decode_value(field_new_value);
  let elem = seed::document().get_element_by_id(field_id)
        .expect("should find element");

  match result {
    Ok(_) => elem.set_class_name("form-control"),
    Err(_) => elem.set_class_name("form-control  is-invalid"),
  };

  result
}

pub fn option_num_from_str(string_input: &str) -> Result<Option<u32>, std::num::ParseIntError> {
  if string_input.is_empty() {
    Ok(None)
  }
  else {
    match u32::from_str_radix(string_input, 10) {
      Ok(value) => Ok(Some(value)),
      Err(error) => Err(error)
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
        div![C!["container-fluid"],
            div![C!["row"],
                match model.active_page {
                    PageType::Edit | PageType::Interface(_) => div![
                            C!["col-md3 col-xl-2 bd-sidebar"],
                            "sidebar",
                        ],
                    _ => empty![],
                },

                div![C!["col"],
                    div![
                        match model.active_page {
                            PageType::Edit => page::edit::view(model),
                            PageType::Settings => page::settings::view(model),
                            PageType::Interface(index) => page::interface::view(model,index),
                            _ => div!["404 not found"],
                        },
                    ]
                ]
            ]
        ],
        div![
            attrs!{
                At::Style => "display: none",
            },
            input![
                attrs!{
                    At::Id => FILE_INPUT_ID,
                    At::Type => "file",
                    At::Accept => ".regwiz,.mdf,text/plain"
                },
                ev(Ev::Change, |event| Msg::UploadStart(event))
            ]
        ],
        div![
            attrs!{
                At::Id => "modal",
            }
        ]
    ]
}

pub fn modal_window(title: &str, content: &str) {
    let modal_window = seed::document()
        .get_element_by_id("modal")
        .unwrap();

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

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
