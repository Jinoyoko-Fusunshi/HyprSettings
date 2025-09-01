use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::providers::hyprland_settings_provider::appearance_settings::AppearanceSettings;
use crate::providers::hyprland_settings_provider::keybinds_settings::KeyBindsSettings;

pub const HYPRLAND_CONFIG_ENTRY: &str = "HyprLandConfig";
pub const VIRTUAL_TERMINAL_ENTRY: &str = "VirtualTerminal";
pub const FILE_MANAGER_ENTRY: &str = "FileManager";
pub const QUICK_SEARCH_ENTRY: &str = "QuickSearch";
pub const LOCK_SCREEN_ENTRY: &str = "LockScreen";
pub const NOTIFICATION_HANDLER_ENTRY: &str = "NotificationHandler";

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HyprlandSettings {
    pub programs: HashMap<String, String>,
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
    pub appearance_settings: AppearanceSettings,
    pub key_bind_settings: KeyBindsSettings,
    pub startup_programs: HashMap<String, String>,
}