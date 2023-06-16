//! Undo functionality

use crate::file_formats::mdf;
use crate::page;
use std::default;

pub struct Undo {
    //    current_focus: Option<egui::Id>,
    //    previous_focus: Option<egui::Id>,
    undo_list: Vec<UndoState>,
    redo_list: Vec<UndoState>,
}

#[derive(Clone)]
pub struct UndoState {
    pub change_description: String,
    pub model: mdf::Mdf,
    pub page_type: page::PageType,
}

impl default::Default for Undo {
    /// create an empty object
    fn default() -> Undo {
        Undo {
            undo_list: Default::default(),
            redo_list: Default::default(),
        }
    }
}

impl default::Default for UndoState {
    fn default() -> UndoState {
        UndoState {
            change_description: Default::default(),
            model: Default::default(),
            page_type: page::PageType::Project,
        }
    }
}

impl Undo {
    pub fn register_modification(
        &mut self,
        description: &str,
        model: &mdf::Mdf,
        page_type: &page::PageType,
    ) {
        self.undo_list.push(UndoState {
            change_description: description.to_owned(),
            model: model.clone(),
            page_type: page_type.clone(),
        });

        self.redo_list.clear();
    }

    pub fn get_undo_description(&self) -> Option<String> {
        let num_elements = self.undo_list.len();
        if num_elements > 1 {
            Some(
                self.undo_list
                    .get(num_elements - 1)
                    .unwrap()
                    .change_description
                    .to_owned(),
            )
        } else {
            None
        }
    }

    pub fn get_redo_description(&self) -> Option<String> {
        let num_elements = self.redo_list.len();
        if num_elements > 0 {
            Some(
                self.redo_list
                    .get(num_elements - 1)
                    .unwrap()
                    .change_description
                    .to_owned(),
            )
        } else {
            None
        }
    }

    pub fn apply_undo(&mut self) -> Option<UndoState> {
        let num_elements = self.undo_list.len();

        if num_elements > 1 {
            let latest = self.undo_list.pop().unwrap();
            let latest_page = latest.page_type.clone();
            self.redo_list.push(latest);

            let previous_model = self.undo_list.get(num_elements - 2).unwrap().model.clone();
            //let previous_page = self.undo_list.get(num_elements - 2).unwrap().page_type.clone();

            Some(UndoState {
                change_description: Default::default(),
                model: previous_model,
                page_type: latest_page,
            })
        } else {
            None
        }
    }

    pub fn apply_redo(&mut self) -> Option<UndoState> {
        let num_elements = self.undo_list.len();

        if num_elements > 0 {
            let state = self.redo_list.pop().unwrap();

            self.undo_list.push(state.clone());

            Some(state)
        } else {
            None
        }
    }
}
