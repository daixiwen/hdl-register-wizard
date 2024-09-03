//! The App structure holds all the current state for the application

#![allow(non_snake_case)]
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::tao;

use std::sync::Arc;
use crate::file_formats;
use crate::navigation;
use crate::page;
use crate::settings;
use crate::undo;
use crate::generate::templates;
use crate::keys::{KeyAction,key_down_event};

#[cfg(not(target_arch = "wasm32"))]
use std::cell::RefCell;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{BufReader, BufWriter};
#[cfg(not(target_arch = "wasm32"))]
use crate::assets;

#[cfg(target_arch = "wasm32")]
// name of the storage key for app configuration
const STORAGE_NAME: &str = "hdl_register_wizard_storage";

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

/// Structure holding all the data that will be saved at shutdown and reloaded at restart.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HdlWizardAppSaveData {
    /// The model the application is currently working on
    pub model: Arc<file_formats::mdf::Mdf>,

    /// The file name that was last used, wither load or saved as. Cleared when using the 'new' command
    pub current_file_name: Option<String>,

    /// last used path for the files
    pub current_path: String,

    /// application settings
    pub settings: settings::Settings,

    /// data specific to the host target (wasm or desktop)
    pub target: HdlWizardAppSaveTarget,
}

/// target specific save data for desktop
#[cfg(not(target_arch = "wasm32"))]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HdlWizardAppSaveTarget {
    /// window position
    pub window_pos: RefCell<tao::dpi::PhysicalPosition<i32>>,

    /// window size
    pub window_size: RefCell<tao::dpi::PhysicalSize<u32>>,
}

/// target specific save data for wasm, empty
#[cfg(target_arch = "wasm32")]
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HdlWizardAppSaveTarget {}

/// Application state, that will be passed to all GUI elements for rendering
pub struct HdlWizardApp {

    /// saved part of the application state
    pub data: HdlWizardAppSaveData,

    /// undo cache
    pub undo: undo::Undo,


    // gui state
    /// whether the burger menu is currently open
    pub burger_menu: bool,

    /// whether the live help is currently active
    pub live_help: bool,

    /// current page being displayed in the app
    pub page_type: page::PageType,

    /// if some, contains an error message to be displayed at next render
    pub error_message: Option<String>,

    /// if some, containts a notification message that should briefly appear
    pub notification: Option<String>,

    /// if some and with the wasm target, contains a base64 endoded string with the saved file to give to the broaswe
    pub web_file_save: Option<String>,

    /// indicate if a documentation preview generation is requested
    pub generate_preview : bool
}

/// reasonable defaults for the saved data structure
impl Default for HdlWizardAppSaveData {
    fn default() -> Self {
        Self {
            model: Default::default(),
            settings: Default::default(),
            target: Default::default(),
            current_file_name: None,
            current_path: Default::default()
        }
    }
}

/// default window position and size for the desktop app
#[cfg(not(target_arch = "wasm32"))]
impl Default for HdlWizardAppSaveTarget {
    fn default() -> Self {
        Self {
            window_pos: RefCell::new(tao::dpi::PhysicalPosition::new(0, 100)),
            window_size: RefCell::new(tao::dpi::PhysicalSize::new(1024, 800)),
        }
    }
}

/// currently no data for the wasm specific structure
#[cfg(target_arch = "wasm32")]
impl Default for HdlWizardAppSaveTarget {
    fn default() -> Self {
        Self {}
    }
}

/// default for the full application state
impl Default for HdlWizardApp {
    fn default() -> Self {
        Self {
            data: Default::default(),
            burger_menu: false,
            live_help: false,
            page_type: page::PageType::Project,
            undo: Default::default(),
            error_message: None,
            notification: Some("could not load settings".to_owned()),
            web_file_save: None,
            generate_preview : false
        }
    }
}

/// load the application state from the save file
#[cfg(not(target_arch = "wasm32"))]
fn load_app_data() -> Result<HdlWizardAppSaveData, std::io::Error> {
    if let Some(path) = assets::data_file_path() {
        // read the settings
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Read the JSON contents of the file as an instance of `HdlWizardAppSaveData`.
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
    }
}

/// fetch the application state from the local storage in the browser
#[cfg(target_arch = "wasm32")]
fn load_app_data() -> Result<HdlWizardAppSaveData, std::io::Error> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(Some(data)) = storage.get_item(STORAGE_NAME) {
                return Ok(serde_json::from_str(&data)?);
            }
        }
    } 
    Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
}

/// save the application state to the save file
#[cfg(not(target_arch = "wasm32"))]
fn save_app_data(data: &HdlWizardAppSaveData) -> Result<(), std::io::Error> {
    if let Some(path) = assets::data_file_path() {
        // check that the parent dir exists
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                std::fs::create_dir_all(parent)?;
            }
        }

        // create the settings file
        let file = File::create(path)?;
        let writer = BufWriter::new(file);

        // Write the JSON contents of the file as an instance of `User`.
        serde_json::to_writer(writer, data)?;

        Ok(())
    } else {
        Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
    }
}

/// write the application state to the local storage in the browser
#[cfg(target_arch = "wasm32")]
fn save_app_data(data: &HdlWizardAppSaveData) -> Result<(), std::io::Error> {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(data_string) = serde_json::to_string(data) {
                if storage.set_item(STORAGE_NAME, &data_string).is_ok() {
                    return Ok(());
                } 
            }
        }
    } 
    Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
}

/// we use the destructor to save the application state before the application exits
#[cfg(not(target_arch = "wasm32"))]
impl Drop for HdlWizardApp {
    fn drop(&mut self) {
        if let Err(error) = save_app_data(&self.data) {
            println!("Error while writing application configuration: {}", error);
        }
    }
}

impl HdlWizardApp {
    /// attempt to restore the state from a previous run and if not use a default state 
    pub fn try_load() -> Self {
        let data = match load_app_data() {
            Ok(data) => data,
            Err(error) => {
                println!("Error while reading application configuration: {}", error);
                Default::default()
            }
        };

        Self {
            data,
            burger_menu: false,
            live_help: false,
            undo: Default::default(),
            page_type: page::PageType::Project,
            error_message: None,
            notification: None,
            web_file_save: None,
            generate_preview: false
        }
    }

    /// register that a state change took place, so that it can be undone
    pub fn register_undo(&mut self, description: &str) {
        self.undo
            .register_modification(description, &self.data.model, &self.data.current_file_name, &self.page_type)
    }

    /// undo the last change, registering it so that it can be redone if asked
    pub fn apply_undo(&mut self) {
        if let Some(new_state) = self.undo.apply_undo() {
            self.data.model = Arc::new(new_state.model);
            self.page_type = new_state.page_type;
            self.data.current_file_name = new_state.file_name;
        }
    }

    /// redo the last change that was undone
    pub fn apply_redo(&mut self) {
        if let Some(new_state) = self.undo.apply_redo() {
            self.data.model = Arc::new(new_state.model);
            self.page_type = new_state.page_type;
            self.data.current_file_name = new_state.file_name;
        }
    }

    /// if the given result is ok, do nothing. If it is an error, extract the error message so that it can be
    /// displayed on the next round
    pub fn test_result(&mut self, result: Result<(), String>) {
        if let Err(message) = result {
            self.error_message = Some(message);
        }
    }

    /// clear the displayed error message
    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    /// return a mutable reference to the model
    pub fn get_mut_model(&mut self) -> &mut file_formats::mdf::Mdf {
        Arc::make_mut(&mut self.data.model)
    }
}

/// generate the live help column, depending on the displayed page
#[component]
pub fn LiveHelp(app_data: Signal<HdlWizardApp>, page_type: page::PageType, live_help_setting: bool) -> Element {
    if live_help_setting {
        // content is html generated from markdown, included in the application
        let (title, contents) = match page_type {
            page::PageType::Project => ("Project", include_str!(concat!(env!("OUT_DIR"), "/live_help/project.html")).to_owned()),
            page::PageType::Interface(_) => ("Interface", include_str!(concat!(env!("OUT_DIR"), "/live_help/interface.html")).to_owned()),
            page::PageType::Register(int, reg, field) => {
                // we need to determine if this register is a bitfield, and if it has a bitfield selected, as
                // this will change which documentation will be shown
                let is_bitfield = {
                    if let Some(interface) = app_data.read().data.model.interfaces.get(int) {
                        if let Some(register) = interface.registers.get(reg) {
                            register.signal.is_none()
                        } else { false}
                    } else {false}
                };

                let has_bitfield_selected = field.is_some();

                // load the different parts
                let register_top = include_str!(concat!(env!("OUT_DIR"), "/live_help/register-top.html"));
                let register_med_bitfield = include_str!(concat!(env!("OUT_DIR"), "/live_help/register-med-bitfield.html"));
                let register_med_normal = include_str!(concat!(env!("OUT_DIR"), "/live_help/register-med-normal.html"));
                let register_access = include_str!(concat!(env!("OUT_DIR"), "/live_help/register-access.html"));
                let register_bottom_normal = include_str!(concat!(env!("OUT_DIR"), "/live_help/register-bottom-normal.html"));
                let bitfield_top = include_str!(concat!(env!("OUT_DIR"), "/live_help/bitfield-top.html"));
                let bitfield_bottom = include_str!(concat!(env!("OUT_DIR"), "/live_help/bitfield-bottom.html"));

                // and combine them depending on the context
                let text = match (is_bitfield, has_bitfield_selected) {
                    (false, _) => [register_top, register_med_normal, register_access, register_bottom_normal].concat(),
                    (true, false) => [register_top, register_med_bitfield].concat(),
                    (true, true) => [register_top, register_med_bitfield, bitfield_top, register_access, bitfield_bottom].concat()
                };

                ("Register", text)
            },
            page::PageType::Preview => ("Preview", include_str!(concat!(env!("OUT_DIR"), "/live_help/preview.html")).to_owned()),
//                _ => ("WIP","<p>Not written yet</p>".to_owned()) 
            page::PageType::ChangeRegisterField(_,_,_) => ("", String::new())
        };
        rsx!(
            aside { class: "panel ext-sticky m-5 is-link ext-livehelp",
                p { class: "panel-heading", "{title}" }
                div { 
                    class: "panel-block content",
                    article {
                        dangerous_inner_html : "{contents}"
                    }
                }
            }    
        )
    } else {
        rsx!("")
    }
}

const STYLE_CSS : &str = include_str!("./style.css");

/// for windows, convert an url to a filesystem one
/// We need to use the custom resolver from Dioxus, because the webview2 from Microsoft
/// doesn't allow to use the file:// URL from a webapp with a custom protocol. For that the
/// URL needs to be of the form http://dioxus.{path}.
#[cfg(windows)]
fn convert_url(orig_path: &str) -> String {
    let converted = orig_path.to_owned().to_lowercase();
    let converted = converted.replace(":", "");
    let converted = converted.replace("\\", "/");

    format!("http://dioxus.{converted}")
}

/// for unix, no convertion needs to be done, the URL are in the form dioxus://{path} so
/// all paths are accessible without modification
#[cfg(unix)]
fn convert_url(orig_path: &str) -> String {
    orig_path.to_owned()
}

/// application main function, for both web and desktop
pub fn App() -> Element {
    // this structure holds all the application data and will be sent over all the GUI modules
    let app_data = use_signal( || {
        let mut app = HdlWizardApp::try_load();
        app.register_undo("initial load");
        app
    });
  
    let key_action: Signal<Option<KeyAction>> = use_signal( || {None });

    let templates = use_signal( || {templates::gen_templates()});
    let templates_ok = templates.peek().is_ok();

    // There is no clean way to get an event when the window size or position is changed. The tao WindowEvent is dropped
    // in dioxus-desktop/launch.rs launch_virtual_dom_blocking(). So we just get the position and size at each re-render
    // we a using Refcells to be able to change them without triggering a redraw

    #[cfg(not(target_arch = "wasm32"))]
    {
        let window = &dioxus_desktop::use_window().window;
        let size = window.inner_size();
        let pos = window.inner_position();

        if let Ok(pos) = pos {
            let app_data_peek = app_data.peek();

            app_data_peek.data.target.window_pos.replace(pos.clone());
            app_data_peek.data.target.window_size.replace(size.clone());
        }
    }

    // when a webapp, update storage at each change
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = save_app_data(&app_data.read().data) {
        println!("Error while writing application configuration: {}", error);
    }

    // main stylesheet
    //- on the webapp, get it from the web
    #[cfg(target_arch = "wasm32")]
    let css_path : &str = "https://cdn.jsdelivr.net/npm/bulma@1.0.2/css/bulma.min.css";

    //- on the desktop app, use a local file
    #[cfg(not(target_arch = "wasm32"))]
    let css_path : String = convert_url(&assets::find_asset("css/bulma.css")
        .expect("didn't find bulma css file").into_os_string().into_string().unwrap());

    // fontawesome import
    //- on the webapp, use the kit from fontawesome.com. Maybe should change this at one point
    #[cfg(target_arch = "wasm32")]
    let fontawesome_import : Element = rsx!(
        script { src: "https://kit.fontawesome.com/e5a7832160.js", crossorigin: "anonymous" }
    );

    //- on the desktop app, use local files
    #[cfg(not(target_arch = "wasm32"))]
    let fontawesome_import : Element = {
        let fontawesome_path : String = convert_url(&assets::find_asset("css/fontawesome.css")
            .expect("didn't find fontawesome css file").into_os_string().into_string().unwrap());
        let brands_path : String = convert_url(&assets::find_asset("css/brands.css")
            .expect("didn't find fontawesome brands css file").into_os_string().into_string().unwrap());
        let solid_path : String = convert_url(&assets::find_asset("css/solid.css")
            .expect("didn't find fontawesome solid css file").into_os_string().into_string().unwrap());

        rsx!(
            link { href: brands_path, rel: "stylesheet" }
            link { href: solid_path, rel: "stylesheet" }
            link { href: fontawesome_path, rel: "stylesheet" }
    )};

    // general variables for page
    let page_type = app_data.read().page_type.to_owned();
    let live_help_setting = app_data.read().live_help.to_owned();
    
    // page render
    let render = rsx! {
        link {
            href: css_path,
            rel: "stylesheet"
        }
        {fontawesome_import}
        div {
            onkeydown: move |event | key_down_event(event, key_action),
            tabindex: -1,
            id: "mainwindow",
            style { {STYLE_CSS} },
            {
                if templates_ok {
                    rsx! {
                        navigation::NavBar { 
                            app_data: app_data,
                            templates: templates,
                            key_action: key_action
                        }
                        div { class: "columns",
                            navigation::SideBar {
                                app_data: app_data 
                            }
                            div { 
                                class: "column ext-sticky mr-4", 
                                page::Content { 
                                    app_data: app_data,
                                    templates: templates
                                } 
                            }
                            LiveHelp {
                                app_data: app_data,
                                page_type: page_type,
                                live_help_setting: live_help_setting
                            }
                        }    
                    }        
                } else {
                    let error_message = templates.peek().as_ref().unwrap_err().to_string();

                    rsx! {
                        div {
                            class: "modal is-active",
                            div {
                                class:"modal-background"
                            },
                            div {
                                class:"modal-content",
                                article {
                                    class: "message is-danger",
                                    div {
                                        class:"message-header",
                                        p {
                                            "Template parsing error"
                                        },
                                    }
                                    div {
                                        class: "message-body",
                                        "{error_message}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };

    render
}
