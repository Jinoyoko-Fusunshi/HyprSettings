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
        let appearance_settings = AppearanceSettings {
            wallpaper_path: String::new(),
            force_default_wallpaper: false,
            disable_hyprland_logo: false,
            inner_gab: 0.0,
            outer_gab: 0.0,
            border_size: 0.0,
            active_border_color: RGBA::new(0.0, 0.0, 0.0, 0.0),
            inactive_border_color: RGBA::new(0.0, 0.0, 0.0, 0.0),
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
            shadow_color: RGBA::new(0.0, 0.0, 0.0, 0.0),
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
            program_settings,
            monitor_configurations: Vec::new(),
            appearance_settings,
        }
    }
}