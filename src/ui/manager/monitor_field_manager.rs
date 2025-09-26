use crate::ui::controls::activable_control::ActivableControl;
use crate::ui::controls::monitor_field::MonitorField;
use crate::ui::manager::monitor_field_manager::MonitorFieldEvent::VisibilityChanged;
use crate::utils::RcMut;

pub struct MonitorFieldManager {
    monitor_field: RcMut<MonitorField>
}

pub enum MonitorFieldEvent {
    VisibilityChanged(bool),
}

impl MonitorFieldManager {
    pub fn new(monitor_field: RcMut<MonitorField>) -> Self {
        Self {
            monitor_field
        }
    }

    pub fn send_event(&self, event: MonitorFieldEvent) {
        match event {
            VisibilityChanged(visible) => {
                if visible {
                    self.monitor_field.borrow_mut().enable_control();
                } else {
                    self.monitor_field.borrow_mut().disable_control();
                }
            }
        }
    }
}