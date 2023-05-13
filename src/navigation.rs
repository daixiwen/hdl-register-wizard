//! GUI navigation: both the menu bar and the left sidebar
#![allow(non_snake_case)]
use crate::app::HdlWizardApp;
use crate::page::PageType;
use dioxus::prelude::*;

#[inline_props]
pub fn NavBar<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>
) -> Element<'a> {
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

    cx.render(rsx! {
        nav { class: "navbar is-link", role: "navigation", aria_label: "main navigation",
            div { class: "navbar-brand",
                div { class: "navbar-item",
                    a { class: "navbar-item has-text-white",
                        i { class: "fa-solid fa-house",
                            onclick: move |_| app_data.with_mut(|app| {
                                app.page_type = PageType::Project;
                                }),
                        }
                    }
                    "{app_data.read().data.model.name}",
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
                                            p { class:"has-text-black is-size-7", "undo {undo}"}
                                        }
                                    }
                                }
                            }
                        )
                    } else {
                        rsx!(
                            div { class:"navbar-item",
                                i {
                                    class: "fa-solid fa-rotate-left has-text-grey-light"
                                }
                            }
                        )
                    }
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
                                            p { class:"has-text-black is-size-7", "redo {redo}"}
                                        }
                                    }
                                }
                            }
                        )
                    } else {
                        rsx!(
                            div { class:"navbar-item",
                                i {
                                    class: "fa-solid fa-rotate-right has-text-grey-light"
                                }
                            }
                        )
                    }
                }
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
                    div { class: "navbar-item has-dropdown is-hoverable",
                        a { class: "navbar-link", "File" }
                        div { class: "navbar-dropdown",
                            a { class: "navbar-item", i { class:"fa-solid fa-file mr-1"}, "New" }
                            a { class: "navbar-item", i { class:"fa-solid fa-folder-open mr-1"}, "Open..." }
                            a { class: "navbar-item", i { class:"fa-solid fa-floppy-disk mr-1"}, "Save" }
                            a { class: "navbar-item", i { class:"fa-solid fa-file-export mr-1"}, "Save as..." }
                            hr { class: "navbar-divider" }
                            a { class: "navbar-item", i { class:"fa-solid fa-person-walking-arrow-right mr-1"}, "Quit" }
                        }
                    }
                    a { class: "navbar-item", "Settings" }
                }
                div { class: "navbar-end",
                    div { class: "navbar-item",
                        div { class: "field",
                            label { class: "checkbox",
                                input {
                                    r#type: "checkbox",
                                    checked: "{live_help}",
                                    onchange: move |evt| app_data.with_mut(|data| data.live_help = evt.value == "true")
                                }
                                "Live help"
                            }
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn SideBar<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>
) -> Element<'a> {
    cx.render(rsx! {
        aside { class: "panel ext-sticky m-5",
            p { class: "panel-heading", "Registers" }
            div { class: "panel-block",
                nav { class: "menu",
                    ul { class: "menu-list",
                    }
                }
            }
        }
    })
}

/*pub fn navigate(app: &mut HdlWizardApp, ctx: &egui::CtxRef, _frame: &epi::Frame) {
    egui::SidePanel::left("side_panel").show(ctx, |ui| {
        ui.add_space(5.0);
        ui.heading("Navigation");

        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    if ui
                        .selectable_label(app.page_type == page::PageType::Project, &app.model.name)
                        .clicked()
                    {
                        app.page_type = page::PageType::Project;
                    }
                    for (n, interface) in app.model.interfaces.iter().enumerate() {
                        let interface_page_type = page::PageType::Interface(n);

                        if ui
                            .selectable_label(
                                app.page_type == interface_page_type,
                                format!("  {}", &interface.name),
                            )
                            .clicked()
                        {
                            app.page_type = interface_page_type;
                        }

                        for (r, register) in interface.registers.iter().enumerate() {
                            let register_page_type = page::PageType::Register(n, r);

                            if ui
                                .selectable_label(
                                    app.page_type == register_page_type,
                                    format!("    {}", &register.name),
                                )
                                .clicked()
                            {
                                app.page_type = register_page_type;
                            }
                        }
                    }
                });
            });
    });
}
*/