#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]
#![allow(non_snake_case)]

use dioxus_desktop::{Config, WindowBuilder};
use hdl_register_wizard::app;

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let app_settings = app::HdlWizardApp::try_load();
    let window_pos = app_settings.data.window_pos;
    let window_size = app_settings.data.window_size;

    dioxus_desktop::launch_cfg(
        app::App,
        Config::default().with_window(WindowBuilder::new().with_resizable(true)
            .with_inner_size(window_size).with_position(window_pos).with_title("HDL Register Wizard")),
    );
}
