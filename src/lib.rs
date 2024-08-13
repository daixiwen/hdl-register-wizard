//! # HDL Register Wizard
//!
//! This is both a desktop application and a webapp that can generate VHDL code and documentation to create hardware 
//! registers accessible on a memory mapped bus. It can load and save files in the Model Description Format developped 
//! by Bitvis for its [Register Wizard](https://bitvis.no/dev-tools/register-wizard/). Files saved by this application
//! should be usable by Bitvis' tool.
//!
//! ## Trial
//!
//!The webapp compiled from the most current sources can be accessed [here](https://daixiwen.github.io/hdl-register-wizard/).
//!
//! ## Project Status
//!
//! The project is under development and is not currently usable. The aim for the first release is to be able to load 
//! and save MDF files, as the [Bitvis Register Wizard](https://bitvis.no/dev-tools/register-wizard/) currently lacks a GUI.
//! A future release will also be able to generate code and documentation.
//!
//! The code is currently hosted on [Github](https://github.com/daixiwen/hdl-register-wizard).
//! 
//! ## Project License
//!
//! The project uses an MIT license.

#![forbid(unsafe_code)]
#![cfg_attr(not(debug_assertions), deny(warnings))] // Forbid warnings in release builds
#![warn(clippy::all, rust_2018_idioms)]

pub mod app;
pub mod file_formats;
pub mod file_io;
pub mod gui_blocks;
pub mod gui_types;
pub mod mdf_process;
pub mod navigation;
pub mod page;
pub mod settings;
pub mod undo;
pub mod utils;
pub mod generate;
#[cfg(not(target_arch = "wasm32"))]
pub mod assets;

#[cfg(test)]
mod tests;
