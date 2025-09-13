use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputSettings {
    pub keyboard_layout: String,
    pub numlock_enabled: bool,
    pub keyboard_repeat_rate: u32,
    pub keyboard_repeat_delay: u32,
    pub mouse_sensitivity: f32,
    pub mouse_left_handed: bool,
    pub mouse_scroll_factor: f32,
    pub mouse_natural_scroll: bool
}