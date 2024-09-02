use dioxus::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum KeyAction {
    OpenFile
}

pub fn key_down_event(event: Event<KeyboardData>, key_action : Signal<Option<KeyAction>>) {
    let keystring = event.data.key().to_string();
    let key = (keystring.as_str(), event.data.modifiers());

    let new_action = match key {
        ("o", Modifiers::CONTROL) => Some(KeyAction::OpenFile),
        _ => None
    };

    let mut key_action: Signal<Option<KeyAction>> = key_action.clone();

    key_action.replace(new_action);
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
