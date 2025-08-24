use std::cell::RefCell;
use std::rc::Rc;
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::controls::keybinds::keybind_input::KeybindInput;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_component::UpdatableComponent;

#[derive(Clone)]
pub struct KeybindInputManager {
    keybind_input: Rc<RefCell<KeybindInput>>,
}

pub enum KeybindInputEvent {
    ConfigurationChanged(Option<KeyBindConfiguration>)
}

impl KeybindInputManager {
    pub fn new(keybind_input: Rc<RefCell<KeybindInput>>) -> Self {
        Self {
            keybind_input
        }
    }

    pub fn send_event(&self, keybind_input_event: KeybindInputEvent) {
        match keybind_input_event {
            KeybindInputEvent::ConfigurationChanged(configuration) => {
                let state = KeybindInputState {
                    configuration
                };
                self.keybind_input.borrow_mut().update_ui(state);
            }
        }
    }
}