use crate::models::keybinds::custom_keybind::CustomKeybind;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::models::keybinds::system_keybind::SystemKeybind;
use crate::models::settings::keybind_settings::KeyBindSettings;

pub struct KeybindProvider {
    settings: KeyBindSettings,
}

impl KeybindProvider {
    pub fn new(settings: KeyBindSettings) -> Self {
        Self {
            settings,
        }
    }
    
    pub fn set_keybind(
        &mut self, system_keybind: SystemKeybind, keybind_configuration: KeyBindConfiguration
    ) {
        self.settings.program_keybinds.insert(system_keybind, keybind_configuration);
    }

    pub fn get_keybind(&self, system_keybind: SystemKeybind) -> Option<KeyBindConfiguration> {
        self.settings.program_keybinds.get(&system_keybind).cloned()
    }

    pub fn set_custom_keybind(&mut self, custom_keybind_name: Option<String>, custom_keybind: Option<CustomKeybind>) {
        if let Some(name) = custom_keybind_name {
            if let Some(configuration) = custom_keybind {
                self.settings.custom_keybinds.insert(name, configuration);
            }
        }
    }

    pub fn remove_custom_keybind(&mut self, custom_keybind_name: String) {
        self.settings.custom_keybinds.remove(&custom_keybind_name);
    }

    pub fn get_custom_keybinds(&self) -> Vec<(String, CustomKeybind)> {
        self.settings.custom_keybinds.iter()
            .map(|(system_keybind, keybind_configuration)| {
                (system_keybind.clone(), keybind_configuration.clone())
            })
            .collect()
    }
    
    pub fn get_settings(&self) -> KeyBindSettings {
        self.settings.clone()
    }
}