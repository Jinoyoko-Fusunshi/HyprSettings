use crate::models::rgba_color::RGBAColor;
use crate::models::settings::appearance_settings::AppearanceSettings;

pub struct AppearanceProvider {
    settings: AppearanceSettings
}

impl AppearanceProvider {
    pub fn new(settings: AppearanceSettings) -> Self {
        Self {
            settings,
        }
    }
    
    pub fn set_wallpaper_path(&mut self, path: String) {
        self.settings.wallpaper_path = path;
    }

    pub fn get_wallpaper_path(&self) -> String {
        self.settings.wallpaper_path.clone()
    }

    pub fn set_cursor_size(&mut self, size: u32) {
        self.settings.cursor_size = size;
    }

    pub fn get_cursor_size(&self) -> u32 {
        self.settings.cursor_size
    }

    pub fn set_cursor_theme(&mut self, theme: String) {
        self.settings.cursor_theme = theme;
    }

    pub fn get_cursor_theme(&self) -> String {
        self.settings.cursor_theme.clone()
    }

    pub fn set_force_default_wallpaper(&mut self, state: bool) {
        self.settings.force_default_wallpaper = state;
    }

    pub fn get_force_default_wallpaper(&self) -> bool {
        self.settings.force_default_wallpaper
    }

    pub fn disable_hyprland_logo(&mut self, state: bool) {
        self.settings.disable_hyprland_logo = state;
    }

    pub fn get_disable_hyprland_logo(&self) -> bool {
        self.settings.disable_hyprland_logo
    }

    pub fn set_inner_gab(&mut self, gab: f64) {
        self.settings.inner_gab = gab;
    }

    pub fn get_inner_gab(&self) -> f64 {
        self.settings.inner_gab
    }

    pub fn set_outer_gab(&mut self, gab: f64) {
        self.settings.outer_gab = gab;
    }

    pub fn get_outer_gab(&self) -> f64 {
        self.settings.outer_gab
    }

    pub fn set_border_size(&mut self, size: f64) {
        self.settings.border_size = size;
    }

    pub fn get_border_size(&self) -> f64 {
        self.settings.border_size
    }

    pub fn set_active_border_color(&mut self, color: RGBAColor) {
        self.settings.active_border_color = color;
    }

    pub fn get_active_border_color(&self) -> RGBAColor {
        self.settings.active_border_color.clone()
    }

    pub fn set_inactive_border_color(&mut self, color: RGBAColor) {
        self.settings.inactive_border_color = color;
    }

    pub fn get_inactive_border_color(&self) -> RGBAColor {
        self.settings.inactive_border_color.clone()
    }

    pub fn set_resize_on_border(&mut self, state: bool) {
        self.settings.resize_on_border = state;
    }

    pub fn get_resize_on_border(&self) -> bool {
        self.settings.resize_on_border
    }

    pub fn set_allow_tearing(&mut self, state: bool) {
        self.settings.allow_tearing = state;
    }

    pub fn get_allow_tearing(&self) -> bool {
        self.settings.allow_tearing
    }

    pub fn set_rounding(&mut self, rounding: f64) {
        self.settings.rounding = rounding;
    }

    pub fn get_rounding(&self) -> f64 {
        self.settings.rounding
    }

    pub fn set_rounding_power(&mut self, power: f64) {
        self.settings.rounding_power = power;
    }

    pub fn get_rounding_power(&self) -> f64 {
        self.settings.rounding_power
    }

    pub fn set_dim_inactive(&mut self, state: bool) {
        self.settings.dim_inactive = state;
    }

    pub fn get_dim_inactive(&self) -> bool {
        self.settings.dim_inactive
    }

    pub fn set_active_opacity(&mut self, opacity: f64) {
        self.settings.active_opacity = opacity;
    }

    pub fn get_active_opacity(&self) -> f64 {
        self.settings.active_opacity
    }

    pub fn set_inactive_opacity(&mut self, opacity: f64) {
        self.settings.inactive_opacity = opacity;
    }

    pub fn get_inactive_opacity(&self) -> f64 {
        self.settings.inactive_opacity
    }

    pub fn set_active_shadow(&mut self, state: bool) {
        self.settings.active_shadow = state;
    }

    pub fn get_active_shadow(&self) -> bool {
        self.settings.active_shadow
    }

    pub fn set_shadow_range(&mut self, range: f64) {
        self.settings.shadow_range = range;
    }

    pub fn get_shadow_range(&self) -> f64 {
        self.settings.shadow_range
    }

    pub fn set_shadow_render_power(&mut self, blur: f64) {
        self.settings.shadow_render_power = blur;
    }

    pub fn get_shadow_render_power(&self) -> f64 {
        self.settings.shadow_render_power
    }

    pub fn set_shadow_color(&mut self, color: RGBAColor) {
        self.settings.shadow_color = color;
    }

    pub fn get_shadow_color(&self) -> RGBAColor {
        self.settings.shadow_color.clone()
    }

    pub fn set_active_blur(&mut self, state: bool) {
        self.settings.active_blur = state;
    }

    pub fn get_active_blur(&self) -> bool {
        self.settings.active_blur
    }

    pub fn set_appearance_blur_size(&mut self, size: f64) {
        self.settings.blur_size = size;
    }

    pub fn get_appearance_blur_size(&self) -> f64 {
        self.settings.blur_size
    }

    pub fn set_appearance_blur_passes(&mut self, passes: usize) {
        self.settings.blur_passes = passes;
    }

    pub fn get_appearance_blur_passes(&self) -> usize {
        self.settings.blur_passes
    }

    pub fn set_appearance_blur_vibrancy(&mut self, blur: f64) {
        self.settings.blur_vibrancy = blur;
    }

    pub fn get_appearance_blur_vibrancy(&self) -> f64 {
        self.settings.blur_vibrancy
    }

    pub fn set_layout(&mut self, layout: String) {
        self.settings.layout = layout;
    }

    pub fn get_layout(&self) -> String {
        self.settings.layout.clone()
    }

    pub fn set_master_status(&mut self, status: String) {
        self.settings.master_status = status;
    }

    pub fn get_master_status(&self) -> String {
        self.settings.master_status.clone()
    }

    pub fn set_pseudo_tiling(&mut self, state: bool) {
        self.settings.pseudo_tiling = state;
    }

    pub fn get_pseudo_tiling(&self) -> bool {
        self.settings.pseudo_tiling
    }

    pub fn set_split_preservation(&mut self, state: bool) {
        self.settings.split_preservation = state;
    }

    pub fn get_split_preservation(&self) -> bool {
        self.settings.split_preservation
    }
    
    pub fn get_settings(&self) -> AppearanceSettings {
        self.settings.clone()   
    }
}