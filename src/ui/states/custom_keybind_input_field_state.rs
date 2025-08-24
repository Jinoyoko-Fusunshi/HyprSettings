use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;

#[derive(Clone)]
pub struct CustomKeybindInputFieldState {
    pub previous_shortcut_name: Option<String>,
    pub shortcut_name: Option<String>,
    pub command: Option<String>,
    pub keybind: Option<KeyBindConfiguration>
}