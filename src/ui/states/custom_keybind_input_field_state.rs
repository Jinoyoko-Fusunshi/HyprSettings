use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;

#[derive(Clone, Default)]
pub struct CustomKeybindInputFieldState {
    pub previous_shortcut_name: Option<String>,
    pub shortcut_name: Option<String>,
    pub command: Option<String>,
    pub keybind: Option<KeyBindConfiguration>
}