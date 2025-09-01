use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;

#[derive(Clone)]
pub struct KeybindInputState {
    pub configuration: Option<KeyBindConfiguration>,
}