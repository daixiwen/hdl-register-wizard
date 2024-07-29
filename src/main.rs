#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]

#[cfg(not(target_arch = "wasm32"))]
use dioxus_desktop::{Config, WindowBuilder};
use hdl_register_wizard::app;
use dioxus::prelude::*;

/// When compiling natively, open a window and launch the application
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app_settings = app::HdlWizardApp::try_load();
    let window_pos = app_settings.data.target.window_pos.borrow();
    let window_size = app_settings.data.target.window_size.borrow();

    dioxus_desktop::launch::launch(
        app::App,
        Vec::new(),
        Config::default().with_window(
            WindowBuilder::new()
                .with_resizable(true)
                .with_inner_size(window_size.to_owned())
                .with_position(window_pos.to_owned())
                .with_title("HDL Register Wizard")
            ).with_menu(None),
    );
}

/// When compiling for wasm, just launch the application from Dioxus
#[cfg(target_arch = "wasm32")]
fn main() {
    // launch the web app
    launch(app::App);
}
