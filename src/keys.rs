use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum KeyAction {
    New, OpenFile, SaveFile, SaveFileAs, Quit, Undo, Redo, Preview
}

#[cfg(not(target_arch = "wasm32"))]
pub fn key_down_event(event: Event<KeyboardData>, key_action : Signal<Option<KeyAction>>) {
    let keystring = event.data.key().to_string();
    let key = (keystring.as_str(), event.data.modifiers());

    let new_action = match key {
        ("n", Modifiers::CONTROL) => Some(KeyAction::New),
        ("o", Modifiers::CONTROL) => Some(KeyAction::OpenFile),
        ("s", Modifiers::CONTROL) => Some(KeyAction::SaveFile),
        ("S", modifiers) =>  {
            if modifiers == Modifiers::CONTROL | Modifiers::SHIFT {
                Some(KeyAction::SaveFileAs)
            } else {
                None
            }
        },
        ("q", Modifiers::CONTROL) => Some(KeyAction::Quit),
        ("z", Modifiers::CONTROL) => Some(KeyAction::Undo),
        ("Z", modifiers) =>  {
            if modifiers == Modifiers::CONTROL | Modifiers::SHIFT {
                Some(KeyAction::Redo)
            } else {
                None
            }
        },
        ("y", Modifiers::CONTROL) => Some(KeyAction::Redo),
        ("p", Modifiers::CONTROL) => Some(KeyAction::Preview),
        _ => None
    };

    let mut key_action: Signal<Option<KeyAction>> = key_action.clone();

    key_action.replace(new_action);
}

#[cfg(target_arch = "wasm32")]
pub fn key_down_event(_event: Event<KeyboardData>, _key_action : Signal<Option<KeyAction>>) {
}

pub fn key_event_check(key_action : Option<Signal<Option<KeyAction>>>, wanted_action : Option<KeyAction>) -> bool {
    match (key_action,wanted_action) {
        (Some(key_action),Some(wanted_action)) => {
            let action = key_action.read().clone();
            if action == Some(wanted_action) {
                let mut key_action = key_action.clone();
                key_action.replace(None);  
        
                true
            } else {
                false
            }        
        },
        _ => false,
    }
}
