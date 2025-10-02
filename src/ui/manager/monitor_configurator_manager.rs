use crate::math::vector::Vector;
use crate::ui::controls::monitor_configurator::MonitorConfigurator;
use crate::ui::manager::control_manager::ControlManager;
use crate::utils::RcMut;

#[derive(Clone)]
pub struct MonitorConfiguratorManager {
    display_configurator: RcMut<MonitorConfigurator>
}

impl ControlManager<MonitorConfigurator, DisplayConfiguratorEvent> for MonitorConfiguratorManager {
    fn send_event(&self, event: DisplayConfiguratorEvent) {
        let mut display_configurator = self.display_configurator.borrow_mut();
        match event {
            DisplayConfiguratorEvent::DisplaySelected(monitor_port) => {
                display_configurator.select_monitor(Some(monitor_port));
            },
            DisplayConfiguratorEvent::DisplayMoving(monitor_port, moved_position) => {
                let corrected_moved_position = Self::get_corrected_position(
                    moved_position, display_configurator.get_size()
                );
                display_configurator.move_display_element(
                    monitor_port, corrected_moved_position
                );
            },
            DisplayConfiguratorEvent::DisplayPlaced(
                monitor_port, placed_position
            ) => {
                let corrected_placed_position = Self::get_corrected_position(
                    placed_position, display_configurator.get_size()
                );
                display_configurator.place_monitor(
                    &monitor_port, &corrected_placed_position
                );
            }
        }
    }

    fn get_control(&self) -> RcMut<MonitorConfigurator> {
        self.display_configurator.clone()
    }
}

pub enum DisplayConfiguratorEvent {
    DisplaySelected(String),
    DisplayMoving(String, Vector),
    DisplayPlaced(String, Vector)
}

impl MonitorConfiguratorManager {
    pub fn new(display_configurator: RcMut<MonitorConfigurator>) -> Self {
        Self {
            display_configurator
        }
    }

    fn get_corrected_position(position: Vector, field_size: Vector) -> Vector {
        let mut xposition = position.get_x();
        let mut yposition = position.get_y();
        let field_width = field_size.get_x();
        let field_height = field_size.get_y();

        if xposition < 0.0 {
            xposition = 0.0;
        }

        if xposition > field_width {
            xposition = field_width;
        }

        if yposition < 0.0 {
            yposition = 0.0;
        }

        if yposition > field_height {
            yposition = field_height;
        }

        Vector::new(xposition, yposition)
    }
}