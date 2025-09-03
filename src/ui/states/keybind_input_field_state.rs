use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;

#[derive(Clone)]
pub struct KeybindInputFieldState {
    pub input_text: String,
    pub configuration: Option<KeyBindConfiguration>,
}