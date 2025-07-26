use std::collections::HashMap;
use gtk::gdk::RGBA;
use crate::config::appearance_settings::AppearanceSettings;
use crate::monitor::monitor_setting::MonitorSetting;

#[derive(Debug)]
pub struct HyprlandSettings {
    pub program_settings: HashMap<String, String>,
    pub monitor_settings: Vec<MonitorSetting>,
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
            monitor_settings: Vec::new(),
            appearance_settings,
        }
    }
}