//! Undo functionality

use super::Msg;
use super::Model;
use super::PageType;

use seed::prelude::Orders;

use std::collections::VecDeque;
use std::convert::TryInto;

/// Messages for the undo functions
#[derive(Clone)]
pub enum UndoMsg {
    /// Undo last operation
    Undo,

    /// Redo last undo
    Redo
}

pub struct Undo {
    undo_steps: VecDeque<(Msg, PageType)>,
    redo_steps: VecDeque<(Msg, PageType)>
}

pub fn update(msg: UndoMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        UndoMsg::Undo => {
            let (undo_msg, undo_page) = model.undo.get_undo_message();
            let is_settings = match undo_msg {
                Msg::Settings(_) => true,
                _ => false
            };
            let current_page = model.active_page;
            let redo = super::process_message(undo_msg, model, orders);
            model.active_page = undo_page;
            super::Urls::new(&model.base_url).from_page_type(undo_page).go_and_replace();
    
            match redo {
                Some(redo_message) => model.undo.redo_steps.push_back((redo_message, current_page)),
                None => ()
            }

            // the model was modified, store the modifications
            if is_settings {
                super::store_settings(model, orders);
            }
            else {
                super::store_data(model, orders);
            }
        },
        UndoMsg::Redo =>  {
            let (redo_msg, redo_page) = model.undo.get_redo_message();
            let is_settings = match redo_msg {
                Msg::Settings(_) => true,
                _ => false
            };
            let current_page = model.active_page;
            let undo = super::process_message(redo_msg, model, orders);
            model.active_page = redo_page;
            super::Urls::new(&model.base_url).from_page_type(redo_page).go_and_replace();
    
            match undo {
                Some(undo_message) => model.undo.undo_steps.push_back((undo_message, current_page)),
                None => ()
            }

            // the model was modified, store the modifications
            if is_settings {
                super::store_settings(model, orders);
            }
            else {
                super::store_data(model, orders);
            }
        },
    }
}

impl Undo {
    pub fn new() -> Undo {
        Undo {
            undo_steps: VecDeque::new(),
            redo_steps: VecDeque::new(),
        }
    }

    pub fn register_message(&mut self, max_undo_level: u32, reverse_msg: Option<Msg>, page_type: PageType) {
        match reverse_msg {
            Some(undo_message) => {
                self.undo_steps.push_back((undo_message, page_type));
                self.redo_steps.clear();

                while self.undo_steps.len() > max_undo_level.try_into().unwrap() {
                    self.undo_steps.pop_front();
                }
            },
            None => ()
        };
    }

    fn get_undo_message(&mut self) -> (Msg, PageType) {
        self.undo_steps.pop_back().unwrap()
    }

    fn get_redo_message(&mut self) -> (Msg, PageType) {
        self.redo_steps.pop_back().unwrap()
    }

    pub fn has_undo(&self) -> bool {
        !self.undo_steps.is_empty()
    }

    pub fn has_redo(&self) -> bool {
        !self.redo_steps.is_empty()
    }
}
