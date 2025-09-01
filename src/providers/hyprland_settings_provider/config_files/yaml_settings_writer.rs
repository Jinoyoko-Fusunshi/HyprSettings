use std::fs::File;
use std::io::Write;
use crate::providers::hyprland_settings_provider::config_files::settings_writer::SettingsWriter;
use crate::providers::hyprland_settings_provider::hyprland_settings::HyprlandSettings;

pub struct YamlSettingsWriter {
    serialized_settings: String
}

impl SettingsWriter<HyprlandSettings> for YamlSettingsWriter {
    fn serialize_settings(&mut self, settings: HyprlandSettings) {
        let serialized_settings = serde_yaml::to_string(&settings).expect("Cannot serialize settings to YAML formatted string.");
        self.serialized_settings = serialized_settings;
    }

    fn write_to_config(&self) {
        if self.serialized_settings.is_empty() {
            return;
        }
        
        let mut yaml_file = File::create("hyprsettings.yaml").expect("Cannot create YAML file.");
        yaml_file.write_all(self.serialized_settings.as_bytes()).expect("Cannot write settings as bytes to YAML file.");
    }
}

impl YamlSettingsWriter {
    pub fn new() -> Self {
        Self {
            serialized_settings: "".to_string(),
        }
    }
}