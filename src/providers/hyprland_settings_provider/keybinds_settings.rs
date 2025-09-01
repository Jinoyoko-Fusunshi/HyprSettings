use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::keybinds::custom_keybind::CustomKeybind;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::models::keybinds::system_keybind::SystemKeybind;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KeyBindsSettings {
    program_keybinds: HashMap<SystemKeybind, KeyBindConfiguration>,
    custom_keybinds: HashMap<String, CustomKeybind>
}

impl KeyBindsSettings {
    pub fn set_keybind(
        &mut self, system_keybind: SystemKeybind, keybind_configuration: KeyBindConfiguration
    ) {
        self.program_keybinds.insert(system_keybind, keybind_configuration);
    }

    pub fn get_keybind(&self, system_keybind: SystemKeybind) -> Option<KeyBindConfiguration> {
        self.program_keybinds.get(&system_keybind).cloned()
    }

    pub fn set_custom_keybind(&mut self, custom_keybind_name: Option<String>, custom_keybind: Option<CustomKeybind>) {
        if let Some(name) = custom_keybind_name {
            if let Some(configuration) = custom_keybind {
                self.custom_keybinds.insert(name, configuration);
            }
        }
    }

    pub fn remove_custom_keybind(&mut self, custom_keybind_name: String) {
        self.custom_keybinds.remove(&custom_keybind_name);
    }

    pub fn get_custom_keybinds(&self) -> Vec<(String, CustomKeybind)> {
        self.custom_keybinds.iter()
            .map(|(system_keybind, keybind_configuration)| {
                (system_keybind.clone(), keybind_configuration.clone())
            })
            .collect()
    }
}