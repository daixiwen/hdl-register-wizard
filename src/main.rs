#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]
#![windows_subsystem = "windows"]

#[cfg(target_arch = "wasm32")]
use dioxus::prelude::*;
use hdl_register_wizard::app;

/// When compiling natively, open a window and launch the application
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    use dioxus_desktop::{Config, WindowBuilder};
    use hdl_register_wizard::assets;

    let app_settings = app::HdlWizardApp::try_load();
    let window_pos = app_settings.data.target.window_pos.borrow();
    let window_size = app_settings.data.target.window_size.borrow();

    let icon = match assets::find_asset("icon.png") {
        None => {
            println!("icon file not found!");
            None
        }
        Some(icon_path) => match image::open(icon_path) {
            Ok(img) => {
                match dioxus_desktop::tao::window::Icon::from_rgba(
                    img.to_rgba8().as_raw().to_owned(),
                    512,
                    512,
                ) {
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
        },
    };

    // Webview2 needs a folder for its data
    #[cfg(windows)]
    std::env::set_var(
        "WEBVIEW2_USER_DATA_FOLDER",
        assets::webview_data_path()
            .expect("no data path for webview")
            .into_os_string(),
    );

    dioxus::LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_window(
                    WindowBuilder::new()
                        .with_resizable(true)
                        .with_inner_size(window_size.to_owned())
                        .with_position(window_pos.to_owned())
                        .with_title("HDL Register Wizard"),
                )
                .with_menu(None)
                .with_icon(icon.expect("no icon to load")),
        )
        .launch(app::App);
}

/// When compiling for wasm, just launch the application from Dioxus
#[cfg(target_arch = "wasm32")]
fn main() {
    // launch the web app
    dioxus::launch(app::App);
}
