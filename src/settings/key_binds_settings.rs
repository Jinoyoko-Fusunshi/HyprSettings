use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::settings::key_binds::{system_keybind::SystemKeybind};
use crate::settings::key_binds::custom_keybind::CustomKeybind;
use crate::settings::key_binds::key_bind_configuration::KeyBindConfiguration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindsSettings {
    program_key_binds: HashMap<SystemKeybind, KeyBindConfiguration>,
    custom_key_binds: HashMap<String, CustomKeybind>
}

impl KeyBindsSettings {
    pub fn new() -> Self {
        Self {
            program_key_binds: HashMap::new(),
            custom_key_binds: HashMap::new(),
        }
    }

    pub fn set_program_key_bind(
        &mut self, system_keybind: SystemKeybind, keybind_configuration: KeyBindConfiguration
    ) {
        self.program_key_binds.insert(system_keybind, keybind_configuration);
    }

    pub fn get_program_key_bind(&self, system_keybind: SystemKeybind) -> Option<KeyBindConfiguration> {
        self.program_key_binds.get(&system_keybind).cloned()
    }

    pub fn set_custom_key_bind(&mut self, custom_keybind_name: String, custom_keybind: CustomKeybind) {
        self.custom_key_binds.insert(custom_keybind_name, custom_keybind);
    }

    pub fn remove_custom_key_bind(&mut self, custom_keybind_name: String) {
        self.custom_key_binds.remove(&custom_keybind_name);
    }

    pub fn get_custom_key_binds(&self) -> Vec<(String, CustomKeybind)> {
        self.custom_key_binds.iter()
            .map(|(system_keybind, keybind_configuration)| {
                (system_keybind.clone(), keybind_configuration.clone())
            })
            .collect()
    }
}