use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::models::settings::input_settings::InputSettings;

pub struct InputProvider {
    settings: InputSettings
}

impl InputProvider {
    pub fn new(settings: InputSettings) -> Self {
        Self {
            settings
        }
    }

    pub fn set_keyboard_layout(&mut self, layout: String) {
        self.settings.keyboard_layout = layout;
    }

    pub fn get_keyboard_layout(&self) -> String {
        self.settings.keyboard_layout.clone()
    }

    pub fn set_numlock_enabled(&mut self, enabled: bool) {
        self.settings.numlock_enabled = enabled;
    }

    pub fn get_numlock_enabled(&self) -> bool {
        self.settings.numlock_enabled
    }

    pub fn set_keyboard_repeat_rate(&mut self, rate: u32) {
        self.settings.keyboard_repeat_rate = rate;
    }

    pub fn get_keyboard_repeat_rate(&self) -> u32 {
        self.settings.keyboard_repeat_rate
    }

    pub fn set_keyboard_repeat_delay(&mut self, delay: u32) {
        self.settings.keyboard_repeat_delay = delay;
    }

    pub fn get_keyboard_repeat_delay(&self) -> u32 {
        self.settings.keyboard_repeat_delay
    }

    pub fn set_mouse_sensitivity(&mut self, sensitivity: f32) {
        self.settings.mouse_sensitivity = sensitivity;
    }
    
    pub fn get_mouse_sensitivity(&self) -> f32 {
        self.settings.mouse_sensitivity
    }
    
    pub fn set_mouse_left_handed(&mut self, left_handed: bool) {
        self.settings.mouse_left_handed = left_handed;
    }
    
    pub fn get_mouse_left_handed(&self) -> bool {
        self.settings.mouse_left_handed
    }
    
    pub fn set_mouse_scroll_factor(&mut self, factor: f32) {
        self.settings.mouse_scroll_factor = factor;
    }
    
    pub fn get_mouse_scroll_factor(&self) -> f32 {
        self.settings.mouse_scroll_factor
    }
    
    pub fn set_mouse_natural_scroll(&mut self, natural_scroll: bool) {
        self.settings.mouse_natural_scroll = natural_scroll;
    }
    
    pub fn get_mouse_natural_scroll(&self) -> bool {
        self.settings.mouse_natural_scroll
    }

    pub fn set_tablet_orientation(&mut self, orientation: MonitorOrientation) {
        self.settings.tablet_orientation = orientation;
    }

    pub fn get_tablet_orientation(&self) -> MonitorOrientation {
        self.settings.tablet_orientation.clone()
    }

    pub fn set_tablet_monitor(&mut self, monitor: String) {
        self.settings.tablet_monitor = monitor;
    }

    pub fn get_tablet_monitor(&self) -> String {
        self.settings.tablet_monitor.clone()
    }

    pub fn set_tablet_region_x(&mut self, x: u32) {
        self.settings.tablet_region_x = x;
    }

    pub fn get_tablet_region_x(&self) -> u32 {
        self.settings.tablet_region_x
    }

    pub fn set_tablet_region_y(&mut self, y: u32) {
        self.settings.tablet_region_y = y;
    }

    pub fn get_tablet_region_y(&self) -> u32 {
        self.settings.tablet_region_y
    }

    pub fn set_tablet_region_width(&mut self, width: u32) {
        self.settings.tablet_region_width = width;
    }

    pub fn get_tablet_region_width(&self) -> u32 {
        self.settings.tablet_region_width
    }

    pub fn set_tablet_region_height(&mut self, height: u32) {
        self.settings.tablet_region_height = height;
    }

    pub fn get_tablet_region_height(&self) -> u32 {
        self.settings.tablet_region_height
    }

    pub fn set_tablet_relative_input(&mut self, relative: bool) {
        self.settings.tablet_relative_input = relative;
    }

    pub fn get_tablet_relative_input(&self) -> bool {
        self.settings.tablet_relative_input
    }

    pub fn set_tablet_left_handed(&mut self, left_handed: bool) {
        self.settings.tablet_left_handed = left_handed;
    }

    pub fn get_tablet_left_handed(&self) -> bool {
        self.settings.tablet_left_handed
    }

    pub fn set_tablet_active_width(&mut self, width: u32) {
        self.settings.tablet_active_width = width;
    }

    pub fn get_tablet_active_width(&self) -> u32 {
        self.settings.tablet_active_width
    }

    pub fn set_tablet_active_height(&mut self, height: u32) {
        self.settings.tablet_active_height = height;
    }

    pub fn get_tablet_active_height(&self) -> u32 {
        self.settings.tablet_active_height
    }

    pub fn set_tablet_active_x(&mut self, x: u32) {
        self.settings.tablet_active_x = x;
    }

    pub fn get_tablet_active_x(&self) -> u32 {
        self.settings.tablet_active_x.clone()
    }

    pub fn set_tablet_active_y(&mut self, y: u32) {
        self.settings.tablet_active_y = y;
    }

    pub fn get_tablet_active_y(&self) -> u32 {
        self.settings.tablet_active_y.clone()
    }

    pub fn get_settings(&self) -> InputSettings {
        self.settings.clone()
    }
}