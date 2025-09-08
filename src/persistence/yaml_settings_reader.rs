use std::fs;
use crate::persistence::settings_reader::SettingsReader;
use crate::models::settings::hyprland_settings::HyprlandSettings;

pub const HYPRSETTINGS_CONFIG_FILE: &str = "hyprsettings.yaml";

pub struct YamlSettingsReader {
    deserialized_settings: HyprlandSettings
}

impl SettingsReader<HyprlandSettings> for YamlSettingsReader {
    fn read_from_config(&mut self) {
        let yaml_file_content = fs::read_to_string(HYPRSETTINGS_CONFIG_FILE)
            .expect("Cannot create YAML file.");

        self.deserialized_settings = serde_yaml::from_str(yaml_file_content.as_str())
            .expect("Cannot deserialize YAML file.");
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

    pub fn config_file_exists() -> bool {
        if fs::exists(HYPRSETTINGS_CONFIG_FILE).expect("Cannot verify existence of settings file") {
            true
        } else {
            false
        }
    }
}