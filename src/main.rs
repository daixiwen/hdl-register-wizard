#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

use hdl_register_wizard::app;
#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::{Config, WindowBuilder};
#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;

/// When compiling natively, open a window and launch the application
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use hdl_register_wizard::assets;

    let app_settings = app::HdlWizardApp::try_load();
    let window_pos = app_settings.data.target.window_pos.borrow();
    let window_size = app_settings.data.target.window_size.borrow();

    let icon = match assets::find_asset("icon.png") {
        None => {
            println!("icon file not found!");
            None},
        Some(icon_path) => {
            match image::open(icon_path) {
                Ok(img) => {
                    match tao::window::Icon::from_rgba(img.to_rgba8().as_raw().to_owned(), 512, 512) {
                        Ok(data) => Some(data),
                        Err(e) => {
                            println!("Error converting icon: {e}");
                            None
                        }
                    }
                }
                Err(e) => {
                    println!("error while loading icon: {e}");
                    None
                }
            }
        }
    };
    
    dioxus_desktop::launch::launch(
        app::App,
        Vec::new(),
        Config::default().with_window(
            WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(window_size.to_owned())
                .with_position(window_pos.to_owned())
                .with_title("HDL Register Wizard")
            ).with_menu(None)
             .with_icon(icon.expect("no icon to load")),
    );
}

/// When compiling for wasm, just launch the application from Dioxus
#[cfg(target_arch = "wasm32")]
fn main() {
    // launch the web app
    launch(app::App);
}
