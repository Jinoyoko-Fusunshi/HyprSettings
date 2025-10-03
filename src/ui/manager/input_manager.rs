use crate::math::vector::Vector;
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::ui::manager::input_manager::InputManagerEvent::MonitorChanged;
use crate::ui::pages::input::Input;
use crate::ui::updatable_control::UpdatableControl;
use crate::utils::RcMut;

#[derive(Clone)]
pub struct InputManager {
    input: RcMut<Input>,   
}

pub enum InputManagerEvent {
    MonitorChanged(String, Option<MonitorConfiguration>),
}

impl InputManager {
    pub fn new(input: RcMut<Input>) -> Self {
        Self {
            input
        }
    }

    pub fn send_event(&self, event: InputManagerEvent) {
        match event {
            MonitorChanged(selected_monitor, selected_configuration) => {
                let empty_size = Vector::new(0.0, 0.0);
                let mut state = self.input.borrow().get_current_state();
                state.tablet_monitor = selected_monitor.clone();

                let mut input_mut = self.input.borrow_mut();
                if let Some(selected_configuration) = selected_configuration {
                    let tablet_screen_size = Vector::new(
                        selected_configuration.video_mode.width_resolution as f64,
                        selected_configuration.video_mode.height_resolution as f64
                    );

                    state.tablet_region_size = tablet_screen_size.clone();
                    state.tablet_active_size = tablet_screen_size.clone();
                    input_mut.toggle_tablet_region(true);
                } else {
                    
                    state.tablet_region_size = empty_size.clone();
                    
                    input_mut.toggle_tablet_region(false);
                }

                state.tablet_region_position = empty_size.clone();
                state.tablet_active_size = empty_size.clone();
                state.tablet_active_position = empty_size.clone();
                input_mut.update_tablet_region(state.clone());
            }
        }
    }
}