use crate::models::rgba_color::RGBAColor;
use crate::models::settings::lockscreen_settings::LockScreenSettings;

pub struct LockscreenProvider {
    settings: LockScreenSettings
}

impl LockscreenProvider {
    pub fn new(settings: LockScreenSettings) -> Self {
        Self {
            settings
        }
    }
    
    pub fn set_hide_cursor(&mut self, state: bool) {
        self.settings.hide_cursor = state;
    }

    pub fn get_hide_cursor(&self) -> bool {
        self.settings.hide_cursor
    }

    pub fn set_grace(&mut self, grace: f32) {
        self.settings.grace = grace;
    }

    pub fn get_grace(&self) -> f32 {
        self.settings.grace
    }

    pub fn set_fall_timeout(&mut self, timeout: u32) {
        self.settings.fall_timeout = timeout;
    }

    pub fn get_fall_timeout(&self) -> u32 {
        self.settings.fall_timeout
    }

    pub fn set_lockscreen_wallpaper(&mut self, path: String) {
        self.settings.lockscreen_wallpaper = path;
    }

    pub fn get_lockscreen_wallpaper(&self) -> Option<String> {
        let lockscreen_wallpaper = self.settings.lockscreen_wallpaper.clone();
        if lockscreen_wallpaper.is_empty() {
            return None;
        }

        Some(lockscreen_wallpaper)
    }

    pub fn set_lockscreen_blur_size(&mut self, size: u32) {
        self.settings.blur_size = size;
    }

    pub fn get_lockscreen_blur_size(&self) -> u32 {
        self.settings.blur_size
    }

    pub fn set_lockscreen_blur_passes(&mut self, passes: u32) {
        self.settings.blur_passes = passes;
    }

    pub fn get_lockscreen_blur_passes(&self) -> u32 {
        self.settings.blur_passes
    }

    pub fn set_noise(&mut self, noise: f32) {
        self.settings.noise = noise;
    }

    pub fn get_noise(&self) -> f32 {
        self.settings.noise
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.settings.contrast = contrast;
    }

    pub fn get_contrast(&self) -> f32 {
        self.settings.contrast
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.settings.brightness = brightness;
    }

    pub fn get_brightness(&self) -> f32 {
        self.settings.brightness
    }

    pub fn set_vibrancy(&mut self, vibrancy: f32) {
        self.settings.vibrancy = vibrancy;
    }

    pub fn get_vibrancy(&self) -> f32 {
        self.settings.vibrancy
    }

    pub fn set_input_width(&mut self, width: u32) {
        self.settings.input_width = width;
    }

    pub fn get_input_width(&self) -> u32 {
        self.settings.input_width
    }

    pub fn set_input_height(&mut self, height: u32) {
        self.settings.input_height = height;
    }

    pub fn get_input_height(&self) -> u32 {
        self.settings.input_height
    }

    pub fn set_input_outline_thickness(&mut self, thickness: u32) {
        self.settings.input_outline_thickness = thickness;
    }

    pub fn get_input_outline_thickness(&self) -> u32 {
        self.settings.input_outline_thickness
    }

    pub fn set_input_dots_size(&mut self, size: u32) {
        self.settings.input_dots_size = size;
    }

    pub fn get_input_dots_size(&self) -> u32 {
        self.settings.input_dots_size
    }

    pub fn set_input_dots_spacing(&mut self, spacing: u32) {
        self.settings.input_dots_spacing = spacing;
    }

    pub fn get_input_dots_spacing(&self) -> u32 {
        self.settings.input_dots_spacing
    }

    pub fn set_input_dots_center(&mut self, state: bool) {
        self.settings.input_dots_center = state;
    }

    pub fn get_input_dots_center(&self) -> bool {
        self.settings.input_dots_center
    }

    pub fn set_input_outer_color(&mut self, color: RGBAColor) {
        self.settings.input_outer_color = color;
    }

    pub fn get_input_outer_color(&self) -> RGBAColor {
        self.settings.input_outer_color.clone()
    }

    pub fn set_input_inner_color(&mut self, color: RGBAColor) {
        self.settings.input_inner_color = color;
    }

    pub fn get_input_inner_color(&self) -> RGBAColor {
        self.settings.input_inner_color.clone()
    }

    pub fn set_input_font_color(&mut self, color: RGBAColor) {
        self.settings.input_font_color = color;
    }

    pub fn get_input_font_color(&self) -> RGBAColor {
        self.settings.input_font_color.clone()
    }

    pub fn set_input_placeholder_text(&mut self, text: String) {
        self.settings.input_placeholder_text = text;
    }

    pub fn get_input_placeholder_text(&self) -> Option<String> {
        let input_placeholder_text = self.settings.input_placeholder_text.clone();
        if input_placeholder_text.is_empty() {
            return None;
        }

        Some(input_placeholder_text.clone())
    }

    pub fn set_hide_input(&mut self, state: bool) {
        self.settings.hide_input = state;
    }

    pub fn get_hide_input(&self) -> bool {
        self.settings.hide_input
    }

    pub fn set_input_x_position(&mut self, position: u32) {
        self.settings.input_x_position = position;
    }

    pub fn get_input_x_position(&self) -> u32 {
        self.settings.input_x_position
    }

    pub fn set_input_y_position(&mut self, position: u32) {
        self.settings.input_y_position = position;
    }

    pub fn get_input_y_position(&self) -> u32 {
        self.settings.input_y_position
    }

    pub fn set_input_vertical_alignment(&mut self, alignment: String) {
        self.settings.input_vertical_alignment = alignment;
    }

    pub fn get_input_vertical_alignment(&self) -> Option<String> {
        let input_vertical_alignment =
            self.settings.input_vertical_alignment.clone();
        if input_vertical_alignment.is_empty() {
            return None;
        }

        Some(input_vertical_alignment)
    }

    pub fn set_input_horizontal_alignment(&mut self, alignment: String) {
        self.settings.input_horizontal_alignment = alignment;
    }

    pub fn get_input_horizontal_alignment(&self) -> Option<String> {
        let input_horizontal_alignment =
            self.settings.input_horizontal_alignment.clone();
        if input_horizontal_alignment.is_empty() {
            return None;
        }

        Some(input_horizontal_alignment)
    }

    pub fn set_display_text(&mut self, text: String) {
        self.settings.display_text = text;
    }

    pub fn get_display_text(&self) -> Option<String> {
        let display_text = self.settings.display_text.clone();
        if display_text.is_empty() {
            return None;
        }

        Some(display_text.clone())
    }

    pub fn set_display_text_color(&mut self, color: RGBAColor) {
        self.settings.display_text_color = color;
    }

    pub fn get_display_text_color(&self) -> RGBAColor {
        self.settings.display_text_color.clone()
    }

    pub fn set_display_text_font_size(&mut self, size: u32) {
        self.settings.display_text_font_size = size;
    }

    pub fn get_display_text_font_size(&self) -> u32 {
        self.settings.display_text_font_size
    }

    pub fn set_display_text_font(&mut self, font: String) {
        self.settings.display_text_font = font;
    }

    pub fn get_display_text_font(&self) -> Option<String> {
        let display_text_font = self.settings.display_text_font.clone();
        if display_text_font.is_empty() {
            return None;
        }

        Some(display_text_font)
    }

    pub fn set_display_text_x_position(&mut self, position: u32) {
        self.settings.display_text_x_position = position;
    }

    pub fn get_display_text_x_position(&self) -> u32 {
        self.settings.display_text_x_position
    }

    pub fn set_display_text_y_position(&mut self, position: u32) {
        self.settings.display_text_y_position = position;
    }

    pub fn get_display_text_y_position(&self) -> u32 {
        self.settings.display_text_y_position
    }

    pub fn set_display_text_vertical_alignment(&mut self, alignment: String) {
        self.settings.display_text_vertical_alignment = alignment;
    }

    pub fn get_display_text_vertical_alignment(&self) -> Option<String> {
        let display_text_vertical_alignment =
            self.settings.display_text_vertical_alignment.clone();
        if display_text_vertical_alignment.is_empty() {
            return None;
        }

        Some(display_text_vertical_alignment.clone())
    }

    pub fn set_display_text_horizontal_alignment(&mut self, alignment: String) {
        self.settings.display_text_horizontal_alignment = alignment;
    }

    pub fn get_display_text_horizontal_alignment(&self) -> Option<String> {
        let display_text_horizontal_alignment =
            self.settings.display_text_horizontal_alignment.clone();

        if display_text_horizontal_alignment.is_empty() {
            return None;
        }

        Some(display_text_horizontal_alignment)
    }
    
    pub fn get_settings(&self) -> LockScreenSettings {
        self.settings.clone()   
    }
}