use std::collections::HashMap;
use gtk::gdk::RGBA;
use serde::{Deserialize, Serialize};
use crate::settings::appearance_settings::AppearanceSettings;
use crate::settings::key_binds_settings::KeyBindsSettings;
use crate::settings::monitor::monitor_configuration::MonitorConfiguration;
use crate::settings::rgba_color::RGBAColor;

pub const HYPRLAND_CONFIG_ENTRY: &str = "HyprLandConfig";
pub const VIRTUAL_TERMINAL_ENTRY: &str = "VirtualTerminal";
pub const FILE_MANAGER_ENTRY: &str = "FileManager";
pub const QUICK_SEARCH_ENTRY: &str = "QuickSearch";
pub const LOCK_SCREEN_ENTRY: &str = "LockScreen";
pub const NOTIFICATION_HANDLER_ENTRY: &str = "NotificationHandler";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyprlandSettings {
    pub programs: HashMap<String, String>,
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
    pub appearance_settings: AppearanceSettings,
    pub key_bind_settings: KeyBindsSettings,
    pub startup_programs: HashMap<String, String>,
}

impl HyprlandSettings {
    pub fn new() -> Self {
        let programs = HashMap::new();
        let appearance_settings = AppearanceSettings {
            wallpaper_path: String::new(),
            force_default_wallpaper: false,
            disable_hyprland_logo: false,
            inner_gab: 0.0,
            outer_gab: 0.0,
            border_size: 0.0,
            active_border_color: RGBAColor::new(RGBA::new(0.0, 0.0, 0.0, 0.0)),
            inactive_border_color: RGBAColor::new(RGBA::new(0.0, 0.0, 0.0, 0.0)),
            resize_on_border: false,
            allow_tearing: false,
            rounding: 0.0,
            rounding_power: 0.0,
            dim_inactive: false,
            active_opacity: 0.0,
            inactive_opacity: 0.0,
            active_shadow: false,
            shadow_range: 0.0,
            shadow_render_power: 0.0,
            shadow_color: RGBAColor::new(RGBA::new(0.0, 0.0, 0.0, 0.0)),
            active_blur: false,
            blur_size: 0.0,
            blur_passes: 0,
            blur_vibrancy: 0.0,
            layout: String::new(),
            master_status: String::new(),
            pseudo_tiling: false,
            split_preservation: false,
        };
        
        Self {
            programs,
            monitor_configurations: HashMap::new(),
            appearance_settings,
            key_bind_settings: KeyBindsSettings::new(),
            startup_programs: HashMap::new(),       
        }
    }

    pub fn get_monitor_ports(&self) -> Vec<String> {
        self.monitor_configurations
            .keys()
            .map(|monitor_port| monitor_port.clone())
            .collect()
    }
}