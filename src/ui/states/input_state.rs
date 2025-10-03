use crate::math::vector::Vector;
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::providers::application_provider::ApplicationProvider;
use crate::providers::monitor_provider::MonitorProvider;
use crate::utils::RcMut;

pub const CURRENT_MONITOR: &str = "current";

#[derive(Clone, Default)]
pub struct InputState {
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
    pub tablet_region_position: Vector,
    pub tablet_region_size: Vector,
    pub tablet_relative_input: bool,
    pub tablet_left_handed: bool,
    pub tablet_active_size: Vector,
    pub tablet_active_position: Vector,
}

impl From<&ApplicationProvider> for InputState {
    fn from(application_provider: &ApplicationProvider) -> Self {
        let input_provider = application_provider.get_input_provider();
        let input_provider_ref = input_provider.borrow();

        let keyboard_layout = input_provider_ref.get_keyboard_layout();
        let numlock_enabled = input_provider_ref.get_numlock_enabled();
        let keyboard_repeat_rate = input_provider_ref.get_keyboard_repeat_rate();
        let keyboard_repeat_delay = input_provider_ref.get_keyboard_repeat_delay();
        let mouse_sensitivity = input_provider_ref.get_mouse_sensitivity();
        let mouse_left_handed = input_provider_ref.get_mouse_left_handed();
        let mouse_scroll_factor = input_provider_ref.get_mouse_scroll_factor();
        let mouse_natural_scroll = input_provider_ref.get_mouse_natural_scroll();
        let tablet_orientation = input_provider_ref.get_tablet_orientation();
        let tablet_monitor = input_provider_ref.get_tablet_monitor();
        let tablet_region_x = input_provider_ref.get_tablet_region_x();
        let tablet_region_y = input_provider_ref.get_tablet_region_y();
        let tablet_region_width = input_provider_ref.get_tablet_region_width();
        let tablet_region_height = input_provider_ref.get_tablet_region_height();
        let tablet_relative_input = input_provider_ref.get_tablet_relative_input();
        let tablet_left_handed = input_provider_ref.get_tablet_left_handed();
        let tablet_active_width = input_provider_ref.get_tablet_active_width();
        let tablet_active_height = input_provider_ref.get_tablet_active_height();
        let tablet_active_x = input_provider_ref.get_tablet_active_x();
        let tablet_active_y = input_provider_ref.get_tablet_active_y();

        Self {
            keyboard_layout,
            numlock_enabled,
            keyboard_repeat_rate,
            keyboard_repeat_delay,
            mouse_sensitivity,
            mouse_left_handed,
            mouse_scroll_factor,
            mouse_natural_scroll,
            tablet_orientation,
            tablet_monitor,
            tablet_region_position: Vector::new(tablet_region_x as f64, tablet_region_y as f64),
            tablet_region_size: Vector::new(tablet_region_width as f64, tablet_region_height as f64),
            tablet_relative_input,
            tablet_left_handed,
            tablet_active_size: Vector::new(tablet_active_width as f64, tablet_active_height as f64),
            tablet_active_position: Vector::new(tablet_active_x as f64, tablet_active_y as f64),
        }
    }
}

impl InputState {
    pub fn get_tablet_max_region_size(&self, monitor_provider: RcMut<MonitorProvider>) -> Vector {
        let max_region_width = if let Some(configuration) = monitor_provider
            .borrow()
            .get_monitor_configuration(self.tablet_monitor.clone())
        {
            configuration.video_mode.width_resolution.clone() as f64
        } else {
            0.0
        };

        let max_region_height = if let Some(configuration) = monitor_provider
            .borrow()
            .get_monitor_configuration(self.tablet_monitor.clone())
        {
            configuration.video_mode.height_resolution.clone() as f64
        } else {
            0.0
        };

        Vector::new(max_region_width, max_region_height)
    }
}