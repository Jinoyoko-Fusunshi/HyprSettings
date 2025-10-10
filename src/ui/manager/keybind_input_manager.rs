use std::cell::RefCell;
use std::rc::Rc;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::ui::controls::keybinds::keybind_input::KeybindInput;
use crate::ui::manager::control_manager::ControlManager;
use crate::ui::states::keybind_input_state::KeybindInputState;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::RcMut;

#[derive(Clone)]
pub struct KeybindInputManager {
    keybind_input: RcMut<KeybindInput>,
}

impl ControlManager<KeybindInput, KeybindInputEvent> for KeybindInputManager {
    fn send_event(&self, event: KeybindInputEvent) {
        match event {
            KeybindInputEvent::ConfigurationChanged(configuration) => {
                let state = KeybindInputState {
                    configuration
                };
                self.keybind_input.borrow_mut().update_state(state.clone());
            }
        }
    }

    fn get_control(&self) -> RcMut<KeybindInput> {
        self.keybind_input.clone()
    }
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
}