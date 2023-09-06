//! app pages
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use dioxus::prelude::*;
use futures_timer::Delay;
use std::time::Duration;

#[derive(PartialEq, Clone)]
pub enum PageType {
    Project,
    Interface(usize),
    Register(usize, usize, Option<usize>),
    Preview
}

pub mod interface;
pub mod project;
pub mod register;
pub mod preview;

/// when saving a file on the webapp, create an URI that the user can click to download 
#[cfg(target_arch = "wasm32")]
#[inline_props]
pub fn FileSave<'a>(
    cx: Scope<'a>, 
    app_data: &'a UseRef<HdlWizardApp>, 
) -> Element<'a> {
    cx.render(rsx! {
        if let Some(download_uri) = &app_data.read().web_file_save {
            let file_name = match &app_data.read().data.current_file_name {
                None => format!("{}.regwiz",&app_data.read().data.model.name),
                Some(name) => name.clone(),
            };

            rsx! {
                div {
                    class: "modal is-active",
                    div {
                        class:"modal-background"
                    },
                    div {
                        class:"modal-content",
                        article {
                            class: "message is-link",
                            div {
                                class:"message-header",
                                p {
                                    "Download"
                                },
                                button {
                                    class:"delete",
                                    onclick: move |_| app_data.with_mut(|app| {app.web_file_save = None;})
                                }
                            }
                            div {
                                class: "message-body",
                                span {
                                    "Click here to download the file: "
                                },
                                a {
                                    href: "{download_uri}",
                                    download: "{file_name}",
                                    onclick: move |_| app_data.with_mut(|app| {app.web_file_save = None;}),
                                    "{file_name}"
                                }
                            }
                        }
                    }
                }
            }
        } else {
            rsx!{
                ""
            }
        }
    })
}

/// the URI file save feature is not used on desktop
#[cfg(not(target_arch = "wasm32"))]
#[inline_props]
#[allow(unused)]
pub fn FileSave<'a>(
    cx: Scope<'a>, 
    app_data: &'a UseRef<HdlWizardApp>
) -> Element<'a> {
    None
}

/// main contents
#[inline_props]
pub fn Content<'a>(cx: Scope<'a>, app_data: &'a UseRef<HdlWizardApp>) -> Element<'a> {
    let notification_timer = use_state(cx, || false);
    let page_type = app_data.read().page_type.to_owned();
    
    // notification system. We use a timer in a future to know when to remove it
    if *notification_timer.get() {
        app_data.write().notification = None;
        notification_timer.set(false);
    }

    cx.render(rsx! {
        if let Some(notification_message) = &app_data.read().notification {

            // spwan the future with the timer
            cx.spawn({
                let notification_timer = notification_timer.to_owned();

                async move {
                    Delay::new(Duration::from_secs(3)).await;
                    notification_timer.set(true);
                }
            });

            // render a notification block
            rsx! {
                div {
                    class: "ext-notification",
                    article {
                        class: "message is-warning",
                        div {
                            class:"message-header",
                            p {
                                "Note"
                            },
                            button {
                                class:"delete",
                                onclick: move |_| app_data.with_mut(|app| {app.notification = None;})
                            }
                        }
                        div {
                            class: "message-body",
                            "{notification_message}"
                        }
                    }
                }
            }
        }

        // if there is an error message to display, put it in its box
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

        // add the box for the file save when in the webapp
        rsx! {
            FileSave {
                app_data: app_data
            }
        }
        
        // fill in the contents, calling the correct module depending on the page type
        match page_type {
            PageType::Project => {
                rsx! {
                    project::Content { app_data: app_data}
                }
            },
            PageType::Interface(interface_num) => {
                rsx! {
                    interface::Content {
                        app_data: app_data,
                        interface_num: interface_num
                    }
                }
            },
            PageType::Register(interface_num, register_num, field_num) => {
                rsx! {
                    register::Content {
                        app_data: app_data,
                        interface_num: interface_num,
                        register_num: register_num,
                        field_num: field_num
                    }
                }
            },
            PageType::Preview => {
                rsx! {
                    preview::Content { app_data: app_data}
                }
            },
        },
    })
}
