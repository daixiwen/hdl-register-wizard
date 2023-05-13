//! main page for the project
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::HdlWizardApp;
use crate::gui_blocks;
use std::cell::RefCell;
use crate::file_formats::mdf;
use crate::page::PageType;

// builds a line in the table with all the interfaces
#[inline_props]
fn TableLine<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>,
    interface_number: usize,
    interface_name: String,
    interface_type: mdf::InterfaceType
) -> Element<'a> {
    let num_of_interfaces = 2;//app_data.read().data.model.interfaces.len();
    let up_disabled = *interface_number == 0;
    let down_disabled = *interface_number == num_of_interfaces-1;

    cx.render(rsx! {
        tr {
            td { "{interface_name}"},
            td { "{interface_type.to_string()}"},
            td { 
                div { class:"buttons are-small ext-buttons-in-table",
                    button { class:"button is-primary", disabled:"{up_disabled}",
                        onclick: move | _evt | println!("click"),
                        span {
                            class:"icon is_small",
                            i { class:"fa-solid fa-caret-up"}
                        }
                    }
                    button { class:"button is-primary", disabled:"{down_disabled}",
                        span {
                            class:"icon is_small",
                            i { class:"fa-solid fa-caret-down"}
                        }
                    }
                    button { class:"button is-link",
                        span {
                            class:"icon is_small",
                            i { class:"fa-solid fa-pen"}
                        }
                    }
                    button { class:"button is-danger",
                        span {
                            class:"icon is_small",
                            i { class:"fa-solid fa-trash"}
                        }
                    }
                }
            }
        }
    })
}

#[inline_props]
pub fn Content<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>
) -> Element<'a> {
    let project_name = app_data.read().data.model.name.clone();

    // extract a list of interfaces and types
    let int_list = app_data.read().data.model.interfaces
        .iter().enumerate().map(
            |(n, int)| (n, int.name.clone(), int.interface_type.clone())).collect::<Vec<_>>();

    // now build some items from that list
    let int_items = int_list.iter().map( |(n, int_name, int_type) | {
                rsx!(
                    TableLine {
                        app_data: app_data,
                        interface_number: *n,
                        interface_name: int_name.clone(),
                        interface_type: *int_type
                        key: "{int_name}{n}"
                    }
                )
            }
        );

    cx.render(rsx! {
        div { class: "container",
            h1 { class:"title page-title", "HDL Register Wizard Project" },
            div { class:"m-4",
            gui_blocks::TextGeneric {
                app_data: app_data,
                update_model: RefCell::new(Box::new( |model, value : &String| model.name = value.clone())),
                gui_label: "Name",
                undo_label: "change project name",
                value: project_name              
                }
            }
            h2 { class:"subtitle page-title", "Interfaces"},
            table {
                class:"table is-striped is-hoverable is-fullwidth",
                thead {
                    tr {
                        th { "Name"},
                        th { "Type"},
                        th {}
                    }
                },
                tbody {
                    int_items
                }
            }
            div { class:"buttons",
                button { class:"button is-primary",
                    onclick: move |_| app_data.with_mut(|app| {
                        app.data.model.interfaces.push(mdf::Interface::new());
                        app.page_type = PageType::Interface(app.data.model.interfaces.len()-1);
                        app.register_undo("create interface")
                        }),
                    "New interface"
                }
            }
        }
    })
}

/*
pub fn panel(app: &mut HdlWizardApp, ctx: &egui::CtxRef, _frame: &epi::Frame) {
    egui::CentralPanel::default().show(ctx, |mut ui| {
        //        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Hdl Register Wizard Project");

        ui.add_space(10.0);

        gui_blocks::widget_text(
            &mut app.model.name,
            &mut ui,
            "Project Name",
            gui_blocks::TextWidgetType::SingleLine,
            &mut app.undo,
        );

        ui.separator();

        ui.horizontal(|ui| {
            ui.heading("Interfaces:");
            if ui.button("New").clicked() {
                app.model.interfaces.push(Default::default());
                app.page_type = page::PageType::Interface(app.model.interfaces.len() - 1);
                app.undo.register_modification(
                    "create new interface",
                    undo::ModificationType::Finished,
                );
            }
        });

        let num_interfaces = app.model.interfaces.len();
        let interface_names: Vec<String> = app
            .model
            .interfaces
            .iter()
            .map(|int| int.name.clone())
            .collect();

        ui.add_space(5.0);
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                egui::Grid::new("interfaces_grid")
                    .striped(true)
                    .spacing([30.0, 5.0])
                    .show(ui, |ui| {
                        for (n, interface_name) in interface_names.iter().enumerate() {
                            let interface_page_type = page::PageType::Interface(n);
                            if ui.selectable_label(false, interface_name).clicked() {
                                app.page_type = interface_page_type;
                            }
                            ui.horizontal(|ui| {
                                if ui.button("🗑").clicked() {
                                    app.model.interfaces.remove(n);
                                    app.undo.register_modification(
                                        "delete interface",
                                        undo::ModificationType::Finished,
                                    );
                                }
                                ui.add_enabled_ui(n > 0, |ui| {
                                    if ui.button("⬆").clicked() {
                                        app.model.interfaces.swap(n - 1, n);
                                        app.undo.register_modification(
                                            "move interface",
                                            undo::ModificationType::Finished,
                                        );
                                    }
                                });
                                ui.add_enabled_ui(n < (num_interfaces - 1), |ui| {
                                    if ui.button("⬇").clicked() {
                                        app.model.interfaces.swap(n, n + 1);
                                        app.undo.register_modification(
                                            "move interface",
                                            undo::ModificationType::Finished,
                                        );
                                    }
                                });
                            });
                            ui.end_row();
                        }
                    });
            });
    });
}
*/