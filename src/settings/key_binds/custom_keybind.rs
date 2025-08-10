use serde::{Deserialize, Serialize};
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomKeybind {
    command: String,
    keybind: KeyBindConfiguration
}

impl CustomKeybind {
    pub fn new(command: String, keybind: KeyBindConfiguration) -> Self {
        Self {
            command,
            keybind
        }
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_configuration(&self) -> &KeyBindConfiguration {
        &self.keybind
    }
}