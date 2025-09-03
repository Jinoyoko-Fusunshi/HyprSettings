pub mod appearance_settings;
pub mod hyprland_settings;
pub mod keybinds_settings;
pub mod config_files;
pub mod lockscreen_settings;

use std::collections::HashMap;
use std::fs;
use std::process::Command;
use crate::models::keybinds::custom_keybind::CustomKeybind;
use crate::models::keybinds::key_bind_configuration::KeyBindConfiguration;
use crate::models::keybinds::system_keybind::SystemKeybind;
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::models::monitor::monitor_info_parser::MonitorInfoParser;
use crate::models::rgba_color::RGBAColor;
use config_files::settings_reader::SettingsReader;
use config_files::yaml_settings_reader::YamlSettingsReader;
use crate::providers::hyprland_settings_provider::hyprland_settings::{HyprlandSettings, FILE_MANAGER_ENTRY, HYPRLAND_CONFIG_ENTRY, LOCK_SCREEN_ENTRY, NOTIFICATION_HANDLER_ENTRY, QUICK_SEARCH_ENTRY, VIRTUAL_TERMINAL_ENTRY};

pub struct HyprlandSettingsProvider {
    settings: HyprlandSettings
}

impl HyprlandSettingsProvider {
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
        self.settings.key_bind_settings.set_keybind(system_keybind, keybind_configuration)
    }

    pub fn get_keybind(&self, system_keybind: SystemKeybind) -> Option<KeyBindConfiguration> {
        self.settings.key_bind_settings.get_keybind(system_keybind)
    }

    pub fn set_custom_keybind(&mut self, shortcut_name: Option<String>, keybind: Option<CustomKeybind>) {
        self.settings.key_bind_settings.set_custom_keybind(shortcut_name, keybind);
    }
    
    pub fn remove_custom_keybind(&mut self, shortcut_name: String) {
        self.settings.key_bind_settings.remove_custom_keybind(shortcut_name);
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

    pub fn set_appearance_blur_size(&mut self, size: f64) {
        self.settings.appearance_settings.blur_size = size;
    }

    pub fn get_appearance_blur_size(&self) -> f64 {
        self.settings.appearance_settings.blur_size
    }

    pub fn set_appearance_blur_passes(&mut self, passes: usize) {
        self.settings.appearance_settings.blur_passes = passes;
    }

    pub fn get_appearance_blur_passes(&self) -> usize {
        self.settings.appearance_settings.blur_passes
    }

    pub fn set_appearance_blur_vibrancy(&mut self, blur: f64) {
        self.settings.appearance_settings.blur_vibrancy = blur;
    }

    pub fn get_appearance_blur_vibrancy(&self) -> f64 {
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

    pub fn set_hide_cursor(&mut self, state: bool) {
        self.settings.lockscreen_settings.hide_cursor = state;
    }

    pub fn get_hide_cursor(&self) -> bool {
        self.settings.lockscreen_settings.hide_cursor
    }

    pub fn set_grace(&mut self, grace: f32) {
        self.settings.lockscreen_settings.grace = grace;
    }

    pub fn get_grace(&self) -> f32 {
        self.settings.lockscreen_settings.grace
    }

    pub fn set_fall_timeout(&mut self, timeout: u32) {
        self.settings.lockscreen_settings.fall_timeout = timeout;
    }

    pub fn get_fall_timeout(&self) -> u32 {
        self.settings.lockscreen_settings.fall_timeout
    }

    pub fn set_lockscreen_wallpaper(&mut self, path: String) {
        self.settings.lockscreen_settings.lockscreen_wallpaper = path;
    }

    pub fn get_lockscreen_wallpaper(&self) -> Option<String> {
        let lockscreen_wallpaper = self.settings.lockscreen_settings.lockscreen_wallpaper.clone();
        if lockscreen_wallpaper.is_empty() {
            return None;
        }

        Some(lockscreen_wallpaper)
    }

    pub fn set_lockscreen_blur_size(&mut self, size: u32) {
        self.settings.lockscreen_settings.blur_size = size;
    }

    pub fn get_lockscreen_blur_size(&self) -> u32 {
        self.settings.lockscreen_settings.blur_size
    }

    pub fn set_lockscreen_blur_passes(&mut self, passes: u32) {
        self.settings.lockscreen_settings.blur_passes = passes;
    }

    pub fn get_lockscreen_blur_passes(&self) -> u32 {
        self.settings.lockscreen_settings.blur_passes
    }

    pub fn set_noise(&mut self, noise: f32) {
        self.settings.lockscreen_settings.noise = noise;
    }

    pub fn get_noise(&self) -> f32 {
        self.settings.lockscreen_settings.noise
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.settings.lockscreen_settings.contrast = contrast;
    }

    pub fn get_contrast(&self) -> f32 {
        self.settings.lockscreen_settings.contrast
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.settings.lockscreen_settings.brightness = brightness;
    }

    pub fn get_brightness(&self) -> f32 {
        self.settings.lockscreen_settings.brightness
    }

    pub fn set_vibrancy(&mut self, vibrancy: f32) {
        self.settings.lockscreen_settings.vibrancy = vibrancy;
    }

    pub fn get_vibrancy(&self) -> f32 {
        self.settings.lockscreen_settings.vibrancy
    }

    pub fn set_input_width(&mut self, width: u32) {
        self.settings.lockscreen_settings.input_width = width;
    }

    pub fn get_input_width(&self) -> u32 {
        self.settings.lockscreen_settings.input_width
    }

    pub fn set_input_height(&mut self, height: u32) {
        self.settings.lockscreen_settings.input_height = height;
    }

    pub fn get_input_height(&self) -> u32 {
        self.settings.lockscreen_settings.input_height
    }

    pub fn set_input_outline_thickness(&mut self, thickness: u32) {
        self.settings.lockscreen_settings.input_outline_thickness = thickness;
    }

    pub fn get_input_outline_thickness(&self) -> u32 {
        self.settings.lockscreen_settings.input_outline_thickness
    }

    pub fn set_input_dots_size(&mut self, size: u32) {
        self.settings.lockscreen_settings.input_dots_size = size;
    }

    pub fn get_input_dots_size(&self) -> u32 {
        self.settings.lockscreen_settings.input_dots_size
    }

    pub fn set_input_dots_spacing(&mut self, spacing: u32) {
        self.settings.lockscreen_settings.input_dots_spacing = spacing;
    }

    pub fn get_input_dots_spacing(&self) -> u32 {
        self.settings.lockscreen_settings.input_dots_spacing
    }

    pub fn set_input_dots_center(&mut self, state: bool) {
        self.settings.lockscreen_settings.input_dots_center = state;
    }

    pub fn get_input_dots_center(&self) -> bool {
        self.settings.lockscreen_settings.input_dots_center
    }

    pub fn set_input_outer_color(&mut self, color: RGBAColor) {
        self.settings.lockscreen_settings.input_outer_color = color;
    }

    pub fn get_input_outer_color(&self) -> RGBAColor {
        self.settings.lockscreen_settings.input_outer_color.clone()
    }

    pub fn set_input_inner_color(&mut self, color: RGBAColor) {
        self.settings.lockscreen_settings.input_inner_color = color;
    }

    pub fn get_input_inner_color(&self) -> RGBAColor {
        self.settings.lockscreen_settings.input_inner_color.clone()
    }

    pub fn set_input_font_color(&mut self, color: RGBAColor) {
        self.settings.lockscreen_settings.input_font_color = color;
    }

    pub fn get_input_font_color(&self) -> RGBAColor {
        self.settings.lockscreen_settings.input_font_color.clone()
    }

    pub fn set_input_placeholder_text(&mut self, text: String) {
        self.settings.lockscreen_settings.input_placeholder_text = text;
    }

    pub fn get_input_placeholder_text(&self) -> Option<String> {
        let input_placeholder_text = self.settings.lockscreen_settings.input_placeholder_text.clone();
        if input_placeholder_text.is_empty() {
            return None;
        }

        Some(input_placeholder_text.clone())
    }

    pub fn set_hide_input(&mut self, state: bool) {
        self.settings.lockscreen_settings.hide_input = state;
    }

    pub fn get_hide_input(&self) -> bool {
        self.settings.lockscreen_settings.hide_input
    }

    pub fn set_input_x_position(&mut self, position: u32) {
        self.settings.lockscreen_settings.input_x_position = position;
    }

    pub fn get_input_x_position(&self) -> u32 {
        self.settings.lockscreen_settings.input_x_position
    }

    pub fn set_input_y_position(&mut self, position: u32) {
        self.settings.lockscreen_settings.input_y_position = position;
    }

    pub fn get_input_y_position(&self) -> u32 {
        self.settings.lockscreen_settings.input_y_position
    }

    pub fn set_input_vertical_alignment(&mut self, alignment: String) {
        self.settings.lockscreen_settings.input_vertical_alignment = alignment;
    }

    pub fn get_input_vertical_alignment(&self) -> Option<String> {
        let input_vertical_alignment = 
            self.settings.lockscreen_settings.input_vertical_alignment.clone();
        if input_vertical_alignment.is_empty() {
            return None;
        }

        Some(input_vertical_alignment)
    }

    pub fn set_input_horizontal_alignment(&mut self, alignment: String) {
        self.settings.lockscreen_settings.input_horizontal_alignment = alignment;
    }

    pub fn get_input_horizontal_alignment(&self) -> Option<String> {
        let input_horizontal_alignment = 
            self.settings.lockscreen_settings.input_horizontal_alignment.clone();
        if input_horizontal_alignment.is_empty() {
            return None;
        }

        Some(input_horizontal_alignment)
    }

    pub fn set_display_text(&mut self, text: String) {
        self.settings.lockscreen_settings.display_text = text;
    }

    pub fn get_display_text(&self) -> Option<String> {
        let display_text = self.settings.lockscreen_settings.display_text.clone();
        if display_text.is_empty() {
            return None;
        }

        Some(display_text.clone())
    }

    pub fn set_display_text_color(&mut self, color: RGBAColor) {
        self.settings.lockscreen_settings.display_text_color = color;
    }

    pub fn get_display_text_color(&self) -> RGBAColor {
        self.settings.lockscreen_settings.display_text_color.clone()
    }

    pub fn set_display_text_font_size(&mut self, size: u32) {
        self.settings.lockscreen_settings.display_text_font_size = size;
    }

    pub fn get_display_text_font_size(&self) -> u32 {
        self.settings.lockscreen_settings.display_text_font_size
    }

    pub fn set_display_text_font(&mut self, font: String) {
        self.settings.lockscreen_settings.display_text_font = font;
    }

    pub fn get_display_text_font(&self) -> Option<String> {
        let display_text_font = self.settings.lockscreen_settings.display_text_font.clone();
        if display_text_font.is_empty() {
            return None;
        }

        Some(display_text_font)
    }

    pub fn set_display_text_x_position(&mut self, position: u32) {
        self.settings.lockscreen_settings.display_text_x_position = position;
    }

    pub fn get_display_text_x_position(&self) -> u32 {
        self.settings.lockscreen_settings.display_text_x_position
    }

    pub fn set_display_text_y_position(&mut self, position: u32) {
        self.settings.lockscreen_settings.display_text_y_position = position;
    }

    pub fn get_display_text_y_position(&self) -> u32 {
        self.settings.lockscreen_settings.display_text_y_position
    }

    pub fn set_display_text_vertical_alignment(&mut self, alignment: String) {
        self.settings.lockscreen_settings.display_text_vertical_alignment = alignment;
    }

    pub fn get_display_text_vertical_alignment(&self) -> Option<String> {
        let display_text_vertical_alignment = 
            self.settings.lockscreen_settings.display_text_vertical_alignment.clone();
        if display_text_vertical_alignment.is_empty() {
            return None;
        }

        Some(display_text_vertical_alignment.clone())
    }

    pub fn set_display_text_horizontal_alignment(&mut self, alignment: String) {
        self.settings.lockscreen_settings.display_text_horizontal_alignment = alignment;
    }

    pub fn get_display_text_horizontal_alignment(&self) -> Option<String> {
        let display_text_horizontal_alignment = 
            self.settings.lockscreen_settings.display_text_horizontal_alignment.clone();
        
        if display_text_horizontal_alignment.is_empty() {
            return None;
        }

        Some(display_text_horizontal_alignment)
    }
}
