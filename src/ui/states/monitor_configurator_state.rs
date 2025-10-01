use std::collections::HashMap;
use crate::math::vector::Vector;
use crate::providers::monitor_provider::MonitorProvider;
use crate::ui::states::monitor_state::MonitorState;
use crate::utils::RcMut;

#[derive(Clone, Default)]
pub struct MonitorConfiguratorState {
    pub monitor_states: HashMap<String, MonitorState>,
    pub selected_monitor: Option<String>,
}

impl From<RcMut<MonitorProvider>> for MonitorConfiguratorState {
    fn from(value: RcMut<MonitorProvider>) -> Self {
        let display_element_states: HashMap<String, MonitorState> = value.borrow()
            .get_monitor_configurations()
            .into_iter()
            .map(|(monitor_port, configuration)| {
                let offset = configuration.offset;
                let orientation = configuration.orientation.clone();
                let scaled_offset = Vector::new(offset.get_x(), offset.get_y()).mul_by(0.1);
                let scaled_size = orientation.get_size_by_orientation(
                    Vector::new(
                        configuration.video_mode.width_resolution as f64,
                        configuration.video_mode.height_resolution as f64
                    ).mul_by(0.1)
                );

                let state = MonitorState {
                    port_name: monitor_port.clone(),
                    orientation,
                    position: scaled_offset.clone(),
                    previous_position: scaled_offset.clone(),
                    size: scaled_size
                };

                (monitor_port, state)
            })
            .collect();

        MonitorConfiguratorState {
            monitor_states: display_element_states,
            selected_monitor: None
        }
    }
}