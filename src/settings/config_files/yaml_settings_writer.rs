use std::cell::RefCell;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use crate::settings::config_files::settings_writer::SettingsWriter;
use crate::settings::hyprland_settings::HyprlandSettings;

pub struct YamlSettingsWriter {
    settings: Rc<RefCell<HyprlandSettings>>,
    serialized_settings: String
}

impl SettingsWriter for YamlSettingsWriter {
    fn serialize_settings(&mut self) {
        let settings_clone = self.settings.clone();
        let settings = <RefCell<HyprlandSettings> as Clone>::clone(&settings_clone).into_inner();
        let serialized_settings = serde_yaml::to_string(&settings).expect("Cannot serialize settings to YAML formatted string.");

        self.serialized_settings = serialized_settings;
    }

    fn write_to_config_file(&self) -> Result<(), String> {
        if self.serialized_settings.is_empty() {
            return Err(String::from("Settings hasn't been serialized and cannot be writing to the config file"));
        }
        
        let mut yaml_file = File::create("hyprsettings.yaml").expect("Cannot create YAML file.");
        yaml_file.write_all(self.serialized_settings.as_bytes()).expect("Cannot write settings as bytes to YAML file.");

        Ok(())
    }
}

impl YamlSettingsWriter {
    pub fn new(settings: &Rc<RefCell<HyprlandSettings>>) -> Self {
        Self {
            settings: settings.clone(),
            serialized_settings: "".to_string(),
        }
    }
}