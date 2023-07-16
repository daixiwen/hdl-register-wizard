//! The App structure holds all the current state for the application

#![allow(non_snake_case)]
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::tao;

use crate::file_formats;
use crate::navigation;
use crate::page;
use crate::settings;
use crate::undo;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::use_window;
#[cfg(not(target_arch = "wasm32"))]
use directories_next::ProjectDirs;
#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::{BufReader, BufWriter};
#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

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
    pub model: file_formats::mdf::Mdf,

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
    pub window_pos: tao::dpi::PhysicalPosition<i32>,

    /// window size
    pub window_size: tao::dpi::PhysicalSize<u32>,
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
            window_pos: tao::dpi::PhysicalPosition::new(0, 100),
            window_size: tao::dpi::PhysicalSize::new(1024, 800),
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
        }
    }
}

/// path used for the settings and state save file
#[cfg(not(target_arch = "wasm32"))]
fn data_file_path() -> Option<PathBuf> {
    match ProjectDirs::from("", "Sylvain Tertois", "HDL Register Wizard") {
        Some(proj) => Some(proj.config_dir().to_path_buf()),
        _ => None,
    }
}

/// load the application state from the save file
#[cfg(not(target_arch = "wasm32"))]
fn load_app_data() -> Result<HdlWizardAppSaveData, std::io::Error> {
    if let Some(path) = data_file_path() {
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
    if let Some(path) = data_file_path() {
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
            self.data.model = new_state.model;
            self.page_type = new_state.page_type;
            self.data.current_file_name = new_state.file_name;
        }
    }

    /// redo the last change that was undone
    pub fn apply_redo(&mut self) {
        if let Some(new_state) = self.undo.apply_redo() {
            self.data.model = new_state.model;
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
}

#[inline_props]
/// generate the live help column, depending on the displayed page
pub fn LiveHelp(cx: Scope, page_type: page::PageType, live_help_setting: bool) -> Element<'_> {
    cx.render(
        if *live_help_setting {
            // content is html generated from markdown, included in the application
            let (title, contents) = match page_type {
                page::PageType::Project => ("Project", include_str!(concat!(env!("OUT_DIR"), "/live_help/project.html"))),
                _ => ("WIP","<p>Not written yet</p>") 
            };
            rsx!(
                aside { class: "panel ext-sticky m-5 ext-livehelp",
                    p { class: "panel-heading", "{title}" }
                    div { 
                        class: "panel-block",
                        article {
                            dangerous_inner_html : "{contents}"
                        }
                    }
                }    
            )
        } else {
            rsx!("")
        }
    )
}

/// application main function, for both web and desktop
pub fn App<'a>(cx: Scope<'a>) -> Element<'a> {
    // this structure holds all the application data and will be sent over all the GUI modules
    let app_data = use_ref(cx, || {
        let mut app = HdlWizardApp::try_load();
        app.register_undo("initial load");
        app
    });

    // I didn't find a clean way to get an event when the window size is changed yet, so for now I just update the position
    // and size at each re-render
    #[cfg(not(target_arch = "wasm32"))]
    {
        let window = use_window(cx).webview.as_ref().window();
        let size = window.inner_size();
        let pos = window.inner_position();

        if let Ok(pos) = pos {
            app_data.write_silent().data.target.window_pos = pos;
            app_data.write_silent().data.target.window_size = size;
        }
    }

    // when a webapp, update storage at each change
    #[cfg(target_arch = "wasm32")]
    if let Err(error) = save_app_data(&app_data.read().data) {
        println!("Error while writing application configuration: {}", error);
    }

    let page_type = app_data.read().page_type.to_owned();
    let live_help_setting = app_data.read().live_help.to_owned();

    // page render
    cx.render(rsx! {
        link {
            href: "https://cdn.jsdelivr.net/npm/bulma@0.9.4/css/bulma.min.css",
            rel: "stylesheet"
        }
        script { src: "https://kit.fontawesome.com/e5a7832160.js", crossorigin: "anonymous" }
        div {
            style { include_str!("./style.css") }
            navigation::NavBar { app_data: app_data }
            div { class: "columns",
                navigation::SideBar {
                    app_data: app_data 
                }
                div { 
                    class: "column ext-sticky mr-4", 
                    page::Content { 
                        app_data: app_data 
                    } 
                }
                LiveHelp {
                    page_type: page_type,
                    live_help_setting: live_help_setting
                }
            }
        }
    })
}
