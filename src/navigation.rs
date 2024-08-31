//! GUI navigation: both the menu bar and the left sidebar
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use crate::file_formats::mdf;
use crate::file_io;
use crate::page::PageType;
use crate::keys::KeyAction;
use dioxus::prelude::*;
use crate::generate;
use tera::Tera;

/// quit menu item for the desktop application
#[cfg(not(target_arch = "wasm32"))]
#[component]
pub fn Quit() -> Element {
    let desktop = consume_context::<dioxus_desktop::DesktopContext>();

    rsx! {
        hr { class: "navbar-divider" }
        a { 
            class: "navbar-item",
            onclick: move |_| desktop.close(),
            i { class: "fa-solid fa-person-walking-arrow-right mr-1" },
            "Quit"
        }
    }
}

/// quit menu item, not used for the webapp
#[cfg(target_arch = "wasm32")]
#[component]
pub fn Quit() -> Element {

    rsx! {}
}

/// Menu bar
#[component]
pub fn NavBar(app_data: Signal<HdlWizardApp>, templates: Signal<tera::Result<Tera>>, key_action : Signal<Option<KeyAction>>) -> Element {
    let burger_menu = app_data.read().burger_menu;
    let live_help = app_data.read().live_help;

    let burger_class = match burger_menu {
        false => "navbar-burger".to_owned(),
        true => "navbar-burger is-active".to_owned(),
    };
    let navmenu_class = match burger_menu {
        false => "navbar-menu".to_owned(),
        true => "navbar-menu is-active".to_owned(),
    };

    rsx! {
        nav { class: "navbar is-link", role: "navigation", aria_label: "main navigation",
            div { class: "navbar-brand",
                div { class: "navbar-item",
                    a { class: "navbar-item has-text-white",
                        i {
                            class: "fa-solid fa-house",
                            onclick: move |_| {
                                app_data
                                    .with_mut(|app| {
                                        app.page_type = PageType::Project;
                                    })
                            }
                        }
                    }
                    "{app_data.read().data.model.name}"
                },

                // undo button
                {
                    if let Some(undo) = app_data.read().undo.get_undo_description() {
                        rsx!(
                            div { class:"navbar-item dropdown is-hoverable",
                                a { class: "dropdown-trigger has-text-white",
                                    i {
                                        class: "fa-solid fa-rotate-left",
                                        aria_haspopup:"true",
                                        aria_controls:"dropdown-menu-undo",
                                        onclick : move | _ | app_data.with_mut(|data| data.apply_undo())
                                    }
                                },
                                div { class:"dropdown-menu", id:"dropdown-menu-undo", role:"menu",
                                    div { class:"dropdown-content",
                                        div { class:"dropdown-item",
                                            p { class:"has-text-current is-size-7", "undo {undo}"}
                                        }
                                    }
                                }
                            }
                        )
                    } else {
                        // the undo list is empty, put a disabled icon
                        rsx!(
                            div { class:"navbar-item",
                                i {
                                    class: "fa-solid fa-rotate-left has-text-grey-light"
                                }
                            }
                        )
                    }
                },

                // redo button
                {
                    if let Some(redo) = app_data.read().undo.get_redo_description() {
                        rsx!(
                            div { class:"navbar-item dropdown is-hoverable",
                                a { class: "dropdown-trigger has-text-white",
                                    i {
                                        class: "fa-solid fa-rotate-right",
                                        aria_haspopup:"true",
                                        aria_controls:"dropdown-menu-redo",
                                        onclick : move | _ | app_data.with_mut(|data| data.apply_redo())
                                    }
                                },
                                div { class:"dropdown-menu", id:"dropdown-menu-redo", role:"menu",
                                    div { class:"dropdown-content",
                                        div { class:"dropdown-item",
                                            p { class:"has-text-current is-size-7", "redo {redo}"}
                                        }
                                    }
                                }
                            }
                        )
                    } else {
                        // the redo list is empty, put a disabled icon
                        rsx!(
                            div { class:"navbar-item",
                                i {
                                    class: "fa-solid fa-rotate-right has-text-grey-light"
                                }
                            }
                        )
                    }
                },
                a {
                    role: "button",
                    class: "{burger_class}",
                    aria_label: "menu",
                    aria_expanded: "false",
                    "data-target": "navbarBasicExample",
                    onclick: move |_| app_data.with_mut(|data| data.burger_menu = !burger_menu),
                    span { aria_hidden: "true" }
                    span { aria_hidden: "true" }
                    span { aria_hidden: "true" }
                }
            }

            div { id: "navbarBasicExample", class: "{navmenu_class}",
                div { class: "navbar-start",
                    // File menu
                    div { class: "navbar-item has-dropdown is-hoverable",
                        a { class: "navbar-link", "File" }
                        div { class: "navbar-dropdown",
                            a {
                                class: "navbar-item",
                                onclick: move |_| {
                                    app_data
                                        .with_mut(|data| {
                                            data.data.model = Default::default();
                                            data.data.current_file_name = None;
                                            data.page_type = PageType::Project;
                                            data.register_undo("new project")
                                        })
                                },
                                i { class: "fa-solid fa-file mr-1" }
                                "New"
                            }
                            file_io::Open { app_data: app_data, key_action : key_action }
                            file_io::Save { app_data: app_data }
                            file_io::SaveAs { app_data: app_data }
                            Quit {}
                        }
                    }

                    // Settings menu
                    a { class: "navbar-item", "Settings" }

                    // Generate menu
                    generate::output::Menu {
                        app_data : app_data,
                        templates : templates
                    }
                }
                div { class: "navbar-end",
                    div { class: "navbar-item",
                        // box to enable live help
                        div { class: "field",
                            label { class: "checkbox",
                                input {
                                    r#type: "checkbox",
                                    checked: "{live_help}",
                                    onchange: move |evt| app_data.with_mut(|data| data.live_help = evt.value() == "true")
                                }
                                " Live help "
                            }
                        }
                    }
                }
            }
        }
    }
}

/// generates a register list from the given vector list
#[component]
pub fn RegistersList(app_data: Signal<HdlWizardApp>, list: Vec<(String, PageType)>) -> Element {
    rsx! {
        { list.iter().map( | (name, reg_page) | {
            let reg_page = reg_page.to_owned();
            rsx! {
                li {
                    a {
                        onclick: move |_| app_data.with_mut(|app| {
                            app.page_type = reg_page.clone();
                        }),
                        "{name}"
                    }
                }
            }
        })}
    }
}

/// Left sidebar, listing all the registers
#[component]
pub fn SideBar(app_data: Signal<HdlWizardApp>) -> Element {
    // build a list of all registers, within a list of all interfaces
    let registers = app_data
        .read()
        .data
        .model
        .interfaces
        .iter()
        .enumerate()
        .map(|(n_int, interface)| {
            (
                interface.name.clone(),
                PageType::Interface(n_int),
                interface
                    .registers
                    .iter()
                    .enumerate()
                    .map(|(n_reg, register)| {
                        (
                            register.name.clone(),
                            PageType::Register(n_int, n_reg, None),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    // build the menu from the last iterator. If there is only one interface, just put the registers in the list.
    // If there are several interfaces, put a list of interfaces and the registers as a sublist
    let menu = match registers.len() {
        1 => {
            let (_, _, registers_list) = &registers[0];

            rsx! {

                RegistersList { app_data: app_data, list: registers_list.clone() }
                li {
                    a {
                        onclick: move |_| {
                            app_data
                                .with_mut(|app| {
                                    app.get_mut_model().interfaces[0].registers.push(mdf::Register::new());
                                    app
                                        .page_type = PageType::Register(
                                        0,
                                        app.data.model.interfaces[0].registers.len() - 1,
                                        None,
                                    );
                                    app.register_undo("create register")
                                })
                        },
                        class: "has-text-primary",
                        "( new )"
                    }
                }
            }
        }
        _ => rsx! {
            { registers.iter().map(
                | (interface_name, interface_page, registers) | {
                    let new_page = interface_page.clone();
                    rsx! {
                        li {
                            a {
                                onclick: move |_| app_data.with_mut(|app| {
                                    app.page_type = new_page.clone();
                                    }),
                                "{interface_name}"
                            },
                            ul {
                                RegistersList {
                                    app_data : app_data,
                                    list: registers.clone()
                                }
                            }
                        }
                    }
                })
            }
        },
    };
    rsx! {
        aside { class: "panel ext-sticky m-5 is-link",
            p { class: "panel-heading", "Registers" }
            div { class: "panel-block",
                nav { class: "menu",
                    ul { class: "menu-list", { menu } }
                }
            }
        }
    }
}
