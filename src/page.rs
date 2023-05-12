//! app pages
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum PageType {
    Project,
    Interface(usize),
    Register(usize, usize),
}

pub mod interface;
pub mod project;
pub mod register;

#[inline_props]
pub fn Content<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>
) -> Element<'a> {
    match &app_data.read().page_type {
        PageType::Project => {
            cx.render(rsx! {
                project::Content { app_data: app_data}
            })
        }
        _ =>{
            cx.render(rsx! {
                p { "Not implemented yet"}
            })
        }
    }
}
