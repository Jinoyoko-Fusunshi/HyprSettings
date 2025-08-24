use serde::{Deserialize, Serialize};
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomKeybind {
    pub command: String,
    pub keybind: KeyBindConfiguration
}

impl CustomKeybind {
    pub fn new(command: String, keybind: KeyBindConfiguration) -> Self {
        Self {
            command,
            keybind
        }
    }
}