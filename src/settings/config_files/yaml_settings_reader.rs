use std::fs;
use crate::settings::config_files::settings_reader::SettingsReader;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct YamlSettingsReader {
    deserialized_settings: HyprlandSettings
}

impl SettingsReader<HyprlandSettings> for YamlSettingsReader {
    fn read_from_config(&mut self) {
        let yaml_file_content = fs::read_to_string("hyprsettings.yaml").expect("Cannot create YAML file.");
        self.deserialized_settings = serde_yaml::from_str(yaml_file_content.as_str()).expect("Cannot deserialize YAML file.");
    }

    fn deserialize_settings(&mut self) -> HyprlandSettings {
        self.deserialized_settings.clone()
    }
}

impl YamlSettingsReader {
    pub fn new() -> Self {
        Self {
            deserialized_settings: Default::default()
        }
    }
}