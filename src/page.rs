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
    cx.render(rsx! {
        if let Some(error_message) = &app_data.read().error_message {
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
                                    "Error"
                                },
                                button { 
                                    class:"delete",
                                    onclick: move |_| app_data.with_mut(|app| {app.clear_error();})
                                }
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
        match &app_data.read().page_type {
            PageType::Project => {
                rsx! {
                    project::Content { app_data: app_data}
                }
            },
            PageType::Interface(interface_num) => {
                rsx! {
                    interface::Content { 
                        app_data: app_data,
                        interface_num: *interface_num
                    }
                }
            },
            PageType::Register(interface_num, register_num, field_num) => {
                rsx! {
                    register::Content { 
                        app_data: app_data,
                        interface_num: *interface_num,
                        register_num: *register_num,
                        field_num: *field_num
                    }
                }
            },        
    /*        _ =>{
                cx.render(rsx! {
                    p { "Not implemented yet"}
                })
            }*/
        }
    })
}
