use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;

pub struct KeybindInputFieldState {
    pub input_text: String,
    pub configuration: Option<KeyBindConfiguration>,
}