//! page to edit an interface
//! main page for the project
#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::app::HdlWizardApp;
use crate::gui_blocks;
use crate::gui_blocks::callback;

#[inline_props]
pub fn Content<'a>(
    cx: Scope<'a>,
    app_data: &'a UseRef<HdlWizardApp>,
    interface_num: usize
) -> Element<'a> {
    if let Some(interface) = app_data.read().data.model.interfaces.get(*interface_num) {
        cx.render(rsx! {
            h1 { class:"title page-title", "Interface" },
            div { class:"m-4",
                gui_blocks::TextGeneric {
                    app_data: app_data,
                    update_int: callback( |interface, value : &String| interface.name = value.clone()),
                    gui_label: "Name",
                    undo_label: "change interface name",
                    value: interface.name.clone()              
                },
                gui_blocks::EnumWidget {
                    app_data: app_data,
                    update_int: callback( |interface, value | interface.interface_type = *value),
                    gui_label: "Type",
                    undo_label: "change interface type",
                    value: interface.interface_type
                },
                gui_blocks::TextArea {
                    app_data: app_data,
                    update_int: callback( |interface, value | interface.description = value.clone()),
                    gui_label: "Description",
                    undo_label: "change description",
                    value: interface.description.clone()
                },
                gui_blocks::AutoManuText {
                    app_data: app_data,
                    update_int: callback( |interface, value | interface.address_width = *value),
                    gui_label: "Address width",
                    undo_label: "change address width",
                    value: interface.address_width,
                    default: 32
                },
                gui_blocks::AutoManuText {
                    app_data: app_data,
                    update_int: callback( |interface, value | interface.data_width = *value),
                    gui_label: "Data width",
                    undo_label: "change data width",
                    value: interface.data_width,
                    default: 32
                },
            },
        })
    } else {
        cx.render(rsx! {
            p { "Wrong interface"}
        })
    }
}

/*pub fn panel(
    interface_num: usize,
    interface: &mut model_gui::Interface,
    ctx: &egui::CtxRef,
    _frame: &epi::Frame,
    undo: &mut undo::Undo,
) -> Option<page::PageType> {
    let mut return_value = None;

    egui::CentralPanel::default().show(ctx, |mut ui| {
        ui.spacing_mut().item_spacing.y = 10.0;

        ui.heading("Interface");

        gui_blocks::widget_text(
            &mut interface.name,
            &mut ui,
            "Name",
            gui_blocks::TextWidgetType::SingleLine,
            undo,
        );
        gui_blocks::widget_text(
            &mut interface.description,
            &mut ui,
            "Description",
            gui_blocks::TextWidgetType::MultiLine,
            undo,
        );

        gui_blocks::widget_combobox(
            &mut interface.interface_type,
            &mut ui,
            "Interface Type",
            None,
            undo,
        );

        gui_blocks::widget_auto_manual_u32(
            &mut interface.address_width,
            &mut ui,
            "Address width",
            false,
            undo,
        );
        gui_blocks::widget_auto_manual_u32(
            &mut interface.data_width,
            &mut ui,
            "Data width",
            false,
            undo,
        );
        ui.separator();

        ui.horizontal(|ui| {
            ui.heading("Registers:");
            if ui.button("New").clicked() {
                interface.registers.push(Default::default());
                return_value = Some(page::PageType::Register(
                    interface_num,
                    interface.registers.len() - 1,
                ));
                undo.register_modification("create new register", undo::ModificationType::Finished);
            }
        });

        let num_registers = interface.registers.len();
        let register_names: Vec<String> = interface
            .registers
            .iter()
            .map(|int| int.name.clone())
            .collect();

        ui.add_space(5.0);
        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                egui::Grid::new("registers_grid")
                    .striped(true)
                    .spacing([30.0, 5.0])
                    .show(ui, |ui| {
                        for (n, register_name) in register_names.iter().enumerate() {
                            let register_page_type = page::PageType::Register(interface_num, n);
                            if ui.selectable_label(false, register_name).clicked() {
                                return_value = Some(register_page_type);
                            }
                            ui.horizontal(|ui| {
                                if ui.button("🗑").clicked() {
                                    interface.registers.remove(n);
                                    undo.register_modification(
                                        "delete register",
                                        undo::ModificationType::Finished,
                                    );
                                }
                                ui.add_enabled_ui(n > 0, |ui| {
                                    if ui.button("⬆").clicked() {
                                        interface.registers.swap(n - 1, n);
                                        undo.register_modification(
                                            "move register",
                                            undo::ModificationType::Finished,
                                        );
                                    }
                                });
                                ui.add_enabled_ui(n < (num_registers - 1), |ui| {
                                    if ui.button("⬇").clicked() {
                                        interface.registers.swap(n, n + 1);
                                        undo.register_modification(
                                            "move register",
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

    return_value
}
*/