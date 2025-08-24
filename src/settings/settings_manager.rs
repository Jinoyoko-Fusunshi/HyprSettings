use std::collections::HashMap;
use std::fs;
use std::process::Command;
use crate::settings::config_files::settings_reader::SettingsReader;
use crate::settings::config_files::yaml_settings_reader::YamlSettingsReader;
use crate::settings::hyprland_settings::HyprlandSettings;
use crate::settings::keybinds::custom_keybind::CustomKeybind;
use crate::settings::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::settings::keybinds::system_keybind::SystemKeybind;
use crate::settings::monitor::monitor_configuration::MonitorConfiguration;
use crate::settings::monitor::monitor_info_parser::MonitorInfoParser;
use crate::settings::rgba_color::RGBAColor;

const HYPRLAND_CONFIG_ENTRY: &str = "HyprLandConfig";
const VIRTUAL_TERMINAL_ENTRY: &str = "VirtualTerminal";
const FILE_MANAGER_ENTRY: &str = "FileManager";
const QUICK_SEARCH_ENTRY: &str = "QuickSearch";
const LOCK_SCREEN_ENTRY: &str = "LockScreen";
const NOTIFICATION_HANDLER_ENTRY: &str = "NotificationHandler";

pub struct SettingsManager {
    settings: HyprlandSettings
}

impl SettingsManager {
    pub fn new() -> Self {
        let settings = Self::get_yaml_config_settings();

        Self {
            settings
        }
    }

    pub fn get_yaml_config_settings() -> HyprlandSettings {
        const HYPRSETTINGS_CONFIG_FILE: &str = "hyprsettings.yaml";

        let mut settings: HyprlandSettings = Default::default();
        if fs::exists(HYPRSETTINGS_CONFIG_FILE).expect("Cannot verify existence of settings file") {
            let mut settings_reader = YamlSettingsReader::new();
            settings_reader.read_from_config();
            settings = settings_reader.deserialize_settings();
        } else {
            Self::load_monitor_default_settings(&mut settings);
        }

        settings
    }

    fn load_monitor_default_settings(settings: &mut HyprlandSettings) {
        let output = Command::new("wlr-randr")
            .output()
            .expect("Error during wlrandr execution");

        let output_string = String::from_utf8(output.stdout)
            .expect("Failed to parse wlr-randr output");

        let mut monitor_info_parser = MonitorInfoParser::new();
        monitor_info_parser.parse_output(&output_string);
        let monitor_information = monitor_info_parser.get_result();

        let max_monitor_configurations: HashMap<String, MonitorConfiguration> = monitor_information
            .iter()
            .map(|monitor_information| {

                let port = monitor_information.port_name.clone();
                let configuration = MonitorConfiguration {
                    enabled: true,
                    information: monitor_information.clone(),
                    video_mode: monitor_information.max_video_mode.clone()
                };

                (port, configuration)
            })
            .collect();

        settings.monitor_configurations = max_monitor_configurations;
    }

    pub fn set_hyprland_config_program_path(&mut self, path: String) {
        self.set_program_path(HYPRLAND_CONFIG_ENTRY.to_string(), path);
    }

    pub fn set_terminal_program_path(&mut self, path: String) {
        self.set_program_path(VIRTUAL_TERMINAL_ENTRY.to_string(), path);
    }

    pub fn set_files_program_path(&mut self, path: String) {
        self.set_program_path(FILE_MANAGER_ENTRY.to_string(), path);
    }

    pub fn set_quick_search_program_path(&mut self, path: String) {
        self.set_program_path(QUICK_SEARCH_ENTRY.to_string(), path);
    }

    pub fn set_lockscreen_program_path(&mut self, path: String) {
        self.set_program_path(LOCK_SCREEN_ENTRY.to_string(), path);
    }

    pub fn set_notifications_program_path(&mut self, path: String) {
        self.set_program_path(NOTIFICATION_HANDLER_ENTRY.to_string(), path);
    }

    pub fn get_hyprland_config_program_path(&self) -> Option<String> {
        self.get_program_path(HYPRLAND_CONFIG_ENTRY.to_string())
    }

    pub fn get_terminal_program_path(&self) -> Option<String> {
        self.get_program_path(VIRTUAL_TERMINAL_ENTRY.to_string())
    }

    pub fn get_files_program_path(&self) -> Option<String> {
        self.get_program_path(FILE_MANAGER_ENTRY.to_string())
    }

    pub fn get_quick_search_program_path(&self) -> Option<String> {
        self.get_program_path(QUICK_SEARCH_ENTRY.to_string())
    }

    pub fn get_lockscreen_program_path(&self) -> Option<String> {
        self.get_program_path(LOCK_SCREEN_ENTRY.to_string())
    }

    pub fn get_notifications_program_path(&self) -> Option<String> {
        self.get_program_path(NOTIFICATION_HANDLER_ENTRY.to_string())
    }

    fn set_program_path(&mut self, program: String, path: String) {
        if path.is_empty() {
            self.settings.programs.remove(&program);
            return;
        } 

        self.settings.programs.insert(program, path);
    }

    fn get_program_path(&self, program: String) -> Option<String> {
        let program_path = self.settings.programs.get(&program);
        program_path.cloned()
    }

    pub fn set_monitor_state(&mut self, monitor_port: String, state: bool) {
        let configuration = self.settings.monitor_configurations.get_mut(&monitor_port).unwrap();
        configuration.enabled = state;
    }

    pub fn set_monitor_width(&mut self, monitor_port: String, width: u32) {
        let configuration = self.settings.monitor_configurations.get_mut(&monitor_port).unwrap();
        configuration.video_mode.width_resolution = width;
    }

    pub fn set_monitor_height(&mut self, monitor_port: String, height: u32) {
        let configuration = self.settings.monitor_configurations.get_mut(&monitor_port).unwrap();
        configuration.video_mode.height_resolution = height;
    }

    pub fn set_monitor_refresh_rate(&mut self, monitor_port: String, refresh_rate: u32) {
        let configuration = self.settings.monitor_configurations.get_mut(&monitor_port).unwrap();
        configuration.video_mode.refresh_rate = refresh_rate;
    }
    
    pub fn get_monitor_configurations(&self) -> HashMap<String, MonitorConfiguration> {
        self.settings.monitor_configurations.clone()
    }

    pub fn set_keybind(&mut self, system_keybind: SystemKeybind, keybind_configuration: KeyBindConfiguration) {
        self.settings.key_bind_settings.set_program_key_bind(system_keybind, keybind_configuration)
    }

    pub fn get_keybind(&self, system_keybind: SystemKeybind) -> Option<KeyBindConfiguration> {
        self.settings.key_bind_settings.get_program_key_bind(system_keybind)
    }

    pub fn set_custom_keybind(&mut self, shortcut_name: Option<String>, keybind: Option<CustomKeybind>) {
        self.settings.key_bind_settings.set_custom_key_bind(shortcut_name, keybind);
    }
    
    pub fn remove_custom_keybind(&mut self, shortcut_name: String) {
        self.settings.key_bind_settings.remove_custom_key_bind(shortcut_name);
    }

    pub fn get_custom_keybinds(&self) -> Vec<(String, CustomKeybind)> {
        self.settings.key_bind_settings.get_custom_keybinds()
    }

    pub fn add_program(&mut self, program: String, path: String) {
        self.settings.programs.insert(program, path);
    }

    pub fn remove_program(&mut self, program: String) {
        self.settings.programs.remove(&program);
    }

    pub fn get_program(&self, program: &str) -> Option<String> {
        self.settings.programs.get(program).cloned()
    }

    pub fn get_program_names(&self) -> Vec<String> {
        self.settings.programs.iter()
            .map(|(program_name, _)| program_name.to_string())
            .collect()
    }

    pub fn add_startup_program(&mut self, program: String, path: String) {
        self.settings.startup_programs.insert(program, path);
    }

    pub fn remove_startup_program(&mut self, program: String) {
        self.settings.startup_programs.remove(&program);
    }

    pub fn get_startup_programs(&self) -> HashMap<String, String> {
        self.settings.startup_programs.clone()
    }

    pub fn get_settings(&self) -> &HyprlandSettings {
        &self.settings
    }

    pub fn set_wallpaper_path(&mut self, path: String) {
        self.settings.appearance_settings.wallpaper_path = path;
    }

    pub fn get_wallpaper_path(&self) -> String {
        self.settings.appearance_settings.wallpaper_path.clone()
    }

    pub fn set_force_default_wallpaper(&mut self, state: bool) {
        self.settings.appearance_settings.force_default_wallpaper = state;
    }

    pub fn get_force_default_wallpaper(&self) -> bool {
        self.settings.appearance_settings.force_default_wallpaper
    }

    pub fn disable_hyprland_logo(&mut self, state: bool) {
        self.settings.appearance_settings.disable_hyprland_logo = state;
    }

    pub fn get_disable_hyprland_logo(&self) -> bool {
        self.settings.appearance_settings.disable_hyprland_logo
    }

    pub fn set_inner_gab(&mut self, gab: f64) {
        self.settings.appearance_settings.inner_gab = gab;
    }

    pub fn get_inner_gab(&self) -> f64 {
        self.settings.appearance_settings.inner_gab
    }

    pub fn set_outer_gab(&mut self, gab: f64) {
        self.settings.appearance_settings.outer_gab = gab;
    }

    pub fn get_outer_gab(&self) -> f64 {
        self.settings.appearance_settings.outer_gab
    }

    pub fn set_border_size(&mut self, size: f64) {
        self.settings.appearance_settings.border_size = size;
    }

    pub fn get_border_size(&self) -> f64 {
        self.settings.appearance_settings.border_size
    }

    pub fn set_active_border_color(&mut self, color: RGBAColor) {
        self.settings.appearance_settings.active_border_color = color;
    }

    pub fn get_active_border_color(&self) -> RGBAColor {
        self.settings.appearance_settings.active_border_color.clone()
    }

    pub fn set_inactive_border_color(&mut self, color: RGBAColor) {
        self.settings.appearance_settings.inactive_border_color = color;
    }

    pub fn get_inactive_border_color(&self) -> RGBAColor {
        self.settings.appearance_settings.inactive_border_color.clone()
    }

    pub fn set_resize_on_border(&mut self, state: bool) {
        self.settings.appearance_settings.resize_on_border = state;
    }

    pub fn get_resize_on_border(&self) -> bool {
        self.settings.appearance_settings.resize_on_border
    }

    pub fn set_allow_tearing(&mut self, state: bool) {
        self.settings.appearance_settings.allow_tearing = state;
    }

    pub fn get_allow_tearing(&self) -> bool {
        self.settings.appearance_settings.allow_tearing
    }

    pub fn set_rounding(&mut self, rounding: f64) {
        self.settings.appearance_settings.rounding = rounding;
    }

    pub fn get_rounding(&self) -> f64 {
        self.settings.appearance_settings.rounding
    }

    pub fn set_rounding_power(&mut self, power: f64) {
        self.settings.appearance_settings.rounding_power = power;
    }

    pub fn get_rounding_power(&self) -> f64 {
        self.settings.appearance_settings.rounding_power
    }

    pub fn set_dim_inactive(&mut self, state: bool) {
        self.settings.appearance_settings.dim_inactive = state;
    }

    pub fn get_dim_inactive(&self) -> bool {
        self.settings.appearance_settings.dim_inactive
    }

    pub fn set_active_opacity(&mut self, opacity: f64) {
        self.settings.appearance_settings.active_opacity = opacity;
    }

    pub fn get_active_opacity(&self) -> f64 {
        self.settings.appearance_settings.active_opacity
    }

    pub fn set_inactive_opacity(&mut self, opacity: f64) {
        self.settings.appearance_settings.inactive_opacity = opacity;
    }

    pub fn get_inactive_opacity(&self) -> f64 {
        self.settings.appearance_settings.inactive_opacity
    }

    pub fn set_active_shadow(&mut self, state: bool) {
        self.settings.appearance_settings.active_shadow = state;
    }

    pub fn get_active_shadow(&self) -> bool {
        self.settings.appearance_settings.active_shadow
    }

    pub fn set_shadow_range(&mut self, range: f64) {
        self.settings.appearance_settings.shadow_range = range;
    }

    pub fn get_shadow_range(&self) -> f64 {
        self.settings.appearance_settings.shadow_range
    }

    pub fn set_shadow_render_power(&mut self, blur: f64) {
        self.settings.appearance_settings.shadow_render_power = blur;
    }

    pub fn get_shadow_render_power(&self) -> f64 {
        self.settings.appearance_settings.shadow_render_power
    }

    pub fn set_shadow_color(&mut self, color: RGBAColor) {
        self.settings.appearance_settings.shadow_color = color;
    }

    pub fn get_shadow_color(&self) -> RGBAColor {
        self.settings.appearance_settings.shadow_color.clone()
    }

    pub fn set_active_blur(&mut self, state: bool) {
        self.settings.appearance_settings.active_blur = state;
    }

    pub fn get_active_blur(&self) -> bool {
        self.settings.appearance_settings.active_blur
    }

    pub fn set_blur_size(&mut self, size: f64) {
        self.settings.appearance_settings.blur_size = size;
    }

    pub fn get_blur_size(&self) -> f64 {
        self.settings.appearance_settings.blur_size
    }

    pub fn set_blur_passes(&mut self, passes: usize) {
        self.settings.appearance_settings.blur_passes = passes;
    }

    pub fn get_blur_passes(&self) -> usize {
        self.settings.appearance_settings.blur_passes
    }

    pub fn set_blur_vibrancy(&mut self, blur: f64) {
        self.settings.appearance_settings.blur_vibrancy = blur;
    }

    pub fn get_blur_vibrancy(&self) -> f64 {
        self.settings.appearance_settings.blur_vibrancy
    }

    pub fn set_layout(&mut self, layout: String) {
        self.settings.appearance_settings.layout = layout;
    }

    pub fn get_layout(&self) -> String {
        self.settings.appearance_settings.layout.clone()
    }

    pub fn set_master_status(&mut self, status: String) {
        self.settings.appearance_settings.master_status = status;
    }

    pub fn get_master_status(&self) -> String {
        self.settings.appearance_settings.master_status.clone()
    }

    pub fn set_pseudo_tiling(&mut self, state: bool) {
        self.settings.appearance_settings.pseudo_tiling = state;
    }

    pub fn get_pseudo_tiling(&self) -> bool {
        self.settings.appearance_settings.pseudo_tiling
    }

    pub fn set_split_preservation(&mut self, state: bool) {
        self.settings.appearance_settings.split_preservation = state;
    }

    pub fn get_split_preservation(&self) -> bool {
        self.settings.appearance_settings.split_preservation
    }
}
