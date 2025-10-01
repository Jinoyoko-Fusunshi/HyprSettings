use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;

#[derive(Clone, Default)]
pub struct KeybindInputState {
    pub configuration: Option<KeyBindConfiguration>,
}