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
    
    pub fn get_settings(&self) -> InputSettings {
        self.settings.clone()
    }
}