//! Undo functionality

use super::Msg;
use super::Model;
use super::PageType;
use seed::prelude::subs;
use seed::prelude::Orders;
use super::utils;

use super::page::edit::EditMsg;
use super::page::interface::InterfaceMsg;

use std::collections::VecDeque;

/// Messages for the undo functions
#[derive(Clone)]
pub enum UndoMsg {
    /// Undo last operation
    Undo,

    /// Redo last undo
    Redo
}

pub struct Undo {
    undo_steps: VecDeque<(Msg, PageType)>
}

pub fn update(msg: UndoMsg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        UndoMsg::Undo => {
            let reverse_action = model.undo.get_reverse_message();
            super::process_message(reverse_action.0, model, orders);
            model.active_page = reverse_action.1;
            super::Urls::new(&model.base_url).from_page_type(reverse_action.1).go_and_replace();
        },
        UndoMsg::Redo => (),
    }
}

impl Undo {
    pub fn new() -> Undo {
        Undo {
            undo_steps: VecDeque::new(),
        }
    }

    pub fn register_message(&mut self, reverse_msg: Option<Msg>, page_type: PageType) {
        match reverse_msg {
            Some(undo_message) => self.undo_steps.push_back((undo_message, page_type)),
            None => ()
        };
    }

    fn get_reverse_message(&mut self) -> (Msg, PageType) {
        self.undo_steps.pop_back().unwrap()
    }

    pub fn has_undo(&self) -> bool {
        !self.undo_steps.is_empty()
    }
}

/// From a received message, generates a new message to reverse the action
pub fn reverse_msg(msg: Msg, model: &Model) -> Option<Msg> {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(_)) => None,

        Msg::Menu(_) => None,

        Msg::Undo(_) => None,

        Msg::Edit(edit_msg) => match edit_msg {
            EditMsg::NameChanged(_) =>
                Some(Msg::Edit(EditMsg::NameChanged(model.mdf_data.name.clone())))
        },

        Msg::Interface(interface_msg) => match interface_msg {

                // the undo for this function is done in page::interface::update because we need to move the deleted interface object
            InterfaceMsg::Delete(_) => None,

            InterfaceMsg::Restore(_ ,_) => None,

            InterfaceMsg::MoveUp(index) => 
                Some(Msg::Interface(InterfaceMsg::MoveDown(index-1))),

            InterfaceMsg::MoveDown(index) =>
                Some(Msg::Interface(InterfaceMsg::MoveUp(index+1))),

            InterfaceMsg::NameChanged(index, _) =>
                Some(Msg::Interface(InterfaceMsg::NameChanged(index, model.mdf_data.interfaces[index].name.clone()))),

            InterfaceMsg::TypeChanged(index, _) =>
                Some(Msg::Interface(InterfaceMsg::TypeChanged(index, model.mdf_data.interfaces[index].interface_type.to_string()))),

            InterfaceMsg::DescriptionChanged(index, _) => 
                Some(Msg::Interface(InterfaceMsg::DescriptionChanged(index,
                    utils::opt_vec_str_to_textarea(&model.mdf_data.interfaces[index].description)))),

            InterfaceMsg::AddressWitdhChanged(index, _) =>
                Some(Msg::Interface(InterfaceMsg::AddressWitdhChanged(index,
                    match model.mdf_data.interfaces[index].address_width {
                        None => String::new(),
                        Some(width) => width.to_string()
                    }))),

            InterfaceMsg::DataWidthChanged(index, _) =>
                Some(Msg::Interface(InterfaceMsg::DataWidthChanged(index,
                    match model.mdf_data.interfaces[index].data_width {
                        None => String::new(),
                        Some(width) => width.to_string()
                    }))),
        },

        Msg::Register(_interface_num, _register_msg) => None,

        Msg::Field(_interface_num, _register_num, _field_msg) => None,

        Msg::UploadStart(_event) => None,

        Msg::UploadText(_text) => None
    }    
}