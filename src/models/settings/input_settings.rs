use serde::{Deserialize, Serialize};
use crate::models::monitor::monitor_configuration::MonitorOrientation;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct InputSettings {
    pub keyboard_layout: String,
    pub numlock_enabled: bool,
    pub keyboard_repeat_rate: u32,
    pub keyboard_repeat_delay: u32,
    pub mouse_sensitivity: f32,
    pub mouse_left_handed: bool,
    pub mouse_scroll_factor: f32,
    pub mouse_natural_scroll: bool,
    pub tablet_orientation: MonitorOrientation,
    pub tablet_monitor: String,
    pub tablet_region_x: u32,
    pub tablet_region_y: u32,
    pub tablet_region_width: u32,
    pub tablet_region_height: u32,
    pub tablet_relative_input: bool,
    pub tablet_left_handed: bool,
    pub tablet_active_width: u32,
    pub tablet_active_height: u32,
    pub tablet_active_x: u32,
    pub tablet_active_y: u32
}