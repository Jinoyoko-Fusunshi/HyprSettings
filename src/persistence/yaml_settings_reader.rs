use std::fs;
use crate::persistence::settings_reader::SettingsReader;
use crate::models::settings::hyprland_settings::HyprlandSettings;
use crate::persistence::hyprland_writer_utils::HyprlandWriterUtils;

pub struct YamlSettingsReader {
    deserialized_settings: HyprlandSettings
}

impl SettingsReader<HyprlandSettings> for YamlSettingsReader {
    fn read_from_config(&mut self) {
        let hyprsettings_config_file = HyprlandWriterUtils::create_hyprland_config_path("hyprsettings.yaml");
        let hyprsettings_config_file_content = fs::read_to_string(hyprsettings_config_file)
            .expect("Cannot create YAML file.");

        self.deserialized_settings = serde_yaml::from_str(hyprsettings_config_file_content.as_str())
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
        let hyprsettings_config_file = HyprlandWriterUtils::create_hyprland_config_path("hyprsettings.yaml");
        if fs::exists(hyprsettings_config_file).expect("Cannot verify existence of settings file") {
            true
        } else {
            false
        }
    }
}