use std::collections::HashMap;
use gtk::gdk::RGBA;
use crate::settings::appearance_settings::AppearanceSettings;
use crate::settings::monitor::monitor_configuration::MonitorConfiguration;

#[derive(Debug)]
pub struct HyprlandSettings {
    pub program_settings: HashMap<String, String>,
    pub monitor_configurations: Vec<MonitorConfiguration>,
    pub appearance_settings: AppearanceSettings,
}

impl HyprlandSettings {
    pub fn new() -> Self {
        let program_settings = HashMap::new();
        let appearance_settings = AppearanceSettings::new(
            String::new(),
            false,
            false,
            0.0,
            0.0,
            0.0,
            RGBA::new(0.0, 0.0, 0.0, 0.0),
            RGBA::new(0.0, 0.0, 0.0, 0.0),
            false,
            false,
            0.0,
            0.0,
            false,
            0.0,
            0.0,
            false,
            0.0,
            0.0,
            RGBA::new(0.0, 0.0, 0.0, 0.0),
            false,
            0.0,
            0,
            0.0,
            String::new(),
            String::new(),
            false,
            false,
        );
        
        Self {
            program_settings,
            monitor_configurations: Vec::new(),
            appearance_settings,
        }
    }
}