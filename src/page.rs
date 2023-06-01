//! app pages
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum PageType {
    Project,
    Interface(usize),
    Register(usize, usize, Option<usize>),
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
        },
        PageType::Interface(interface_num) => {
            cx.render(rsx! {
                interface::Content { 
                    app_data: app_data,
                    interface_num: *interface_num
                }
            })
        },
        PageType::Register(interface_num, register_num, field_num) => {
            cx.render(rsx! {
                register::Content { 
                    app_data: app_data,
                    interface_num: *interface_num,
                    register_num: *register_num,
                    field_num: *field_num
                }
            })
        },        
/*        _ =>{
            cx.render(rsx! {
                p { "Not implemented yet"}
            })
        }*/
    }
}
