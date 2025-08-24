use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::settings::appearance_settings::AppearanceSettings;
use crate::settings::keybinds_settings::KeyBindsSettings;
use crate::settings::monitor::monitor_configuration::MonitorConfiguration;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HyprlandSettings {
    pub programs: HashMap<String, String>,
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
    pub appearance_settings: AppearanceSettings,
    pub key_bind_settings: KeyBindsSettings,
    pub startup_programs: HashMap<String, String>,
}