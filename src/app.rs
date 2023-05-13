#![allow(non_snake_case)]
use dioxus_desktop::tao;

use crate::file_formats;
use crate::navigation;
use crate::page;
use crate::settings;
use crate::undo;
//use crate::utils;
use dioxus_desktop::use_window;
use directories_next::ProjectDirs;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufWriter};

// import the prelude to get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct HdlWizardAppSaveData {
    pub model: file_formats::mdf::Mdf,
    pub settings: settings::Settings,

    // window size and position
    pub window_pos : tao::dpi::PhysicalPosition<i32>,
    pub window_size : tao::dpi::PhysicalSize<u32>,
}
// Application state data, including what will be saved in data, and volatile data in the other fields
pub struct HdlWizardApp {

    pub undo: undo::Undo,

    pub data: HdlWizardAppSaveData,

    // gui state
    pub burger_menu: bool,
    pub live_help: bool,
    pub page_type: page::PageType,
}

impl Default for HdlWizardAppSaveData {
    fn default() -> Self {
        Self {
            model: Default::default(),
            settings: Default::default(),
            window_pos : tao::dpi::PhysicalPosition::new(0,100),
            window_size : tao::dpi::PhysicalSize::new(1024,800),
        }
    }
}

impl Default for HdlWizardApp {
    fn default() -> Self {
        Self {
            data: Default::default(),
            burger_menu : false,
            live_help: false,
            page_type: page::PageType::Project,
            undo: Default::default(),
        }
    }
}

// fin the path for the save file
fn data_file_path() -> Option<PathBuf> {
    match ProjectDirs::from("", "Sylvain Tertois",  "HDL Register Wizard") {
        Some(proj) => Some(proj.config_dir().to_path_buf()),
        _ => None
    }
}

fn load_app_data() -> Result<HdlWizardAppSaveData, std::io::Error> {
    if let Some(path) = data_file_path() {
        // read the settings
        let file = File::open(path)?;
        let reader = BufReader::new(file);
    
        // Read the JSON contents of the file as an instance of `HdlWizardAppSaveData`.
        let data = serde_json::from_reader(reader)?;
        Ok(data)
    }
    else {
        Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
    }
}

fn save_app_data(data: &HdlWizardAppSaveData) -> Result<(), std::io::Error>{
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
    }
    else {
        Err(std::io::Error::from(std::io::ErrorKind::AddrNotAvailable))
    }
}

impl Drop for HdlWizardApp {
    fn drop(&mut self) {
        if let Err(error) = save_app_data(&self.data) {
            println!("Error while writing application configuration: {}", error);
        }
    }
}

impl HdlWizardApp {
    pub fn try_load() -> Self {
        let data = match load_app_data() {
            Ok(data) => data,
            Err(error) => { println!("Error while reading application configuration: {}", error); Default::default()}
        };

        Self {
            data,
            burger_menu : false,
            live_help: false,
            undo: Default::default(),
            page_type: page::PageType::Project
        }
    }

    pub fn register_undo(&mut self, description : &str) {
        self.undo.register_modification(description, &self.data.model, &self.page_type)
    }

    pub fn apply_undo(&mut self) {
        if let Some(new_state) = self.undo.apply_undo() {
            self.data.model = new_state.model;
            self.page_type = new_state.page_type;
        }
    }

    pub fn apply_redo(&mut self) {
        if let Some(new_state) = self.undo.apply_redo() {
            self.data.model = new_state.model;
            self.page_type = new_state.page_type;
        }
    }
}

pub fn App<'a>(cx: Scope<'a>) -> Element<'a> {
    // this structure holds all the application data and will be sent over all the GUI modules
    let app_data = use_ref(cx, || {
        let mut app = HdlWizardApp::try_load();
        app.register_undo("initial load");
        app });

    // I didn't find a clean way to get an event when the window size is changed yet, so for now I just update the position
    // and size at each re-render
    let window = use_window(cx).webview.as_ref().window();
    let size = window.inner_size();
    let pos = window.inner_position();

    if let Ok(pos) = pos {
        app_data.write_silent().data.window_pos = pos;
        app_data.write_silent().data.window_size = size;
    }

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
                navigation::SideBar { app_data: app_data},
                div { class: "column ext-sticky mr-4",

                    page::Content {
                        app_data: app_data
                    }
                }
            }
        }
    })
}
