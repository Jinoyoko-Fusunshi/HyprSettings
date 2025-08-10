use std::cell::RefCell;
use std::{fs, rc::Rc};
use crate::settings::config_files::settings_reader::SettingsReader;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct YamlSettingsReader {
    deserialized_settings: HyprlandSettings
}

impl SettingsReader for YamlSettingsReader {
    fn read_from_config(&mut self) {
        let yaml_file_content = fs::read_to_string("hyprsettings.yaml").expect("Cannot create YAML file.");
        self.deserialized_settings = serde_yaml::from_str(yaml_file_content.as_str()).expect("Cannot deserialize YAML file.");
    }

    fn apply_settings(&mut self, settings: &Rc<RefCell<HyprlandSettings>>) {
        *settings.borrow_mut() = self.deserialized_settings.clone()
    }
}

impl YamlSettingsReader {
    pub fn new() -> Self {
        Self {
            deserialized_settings: HyprlandSettings::new()
        }
    }
}