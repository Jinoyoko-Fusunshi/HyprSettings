use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::settings::monitor::monitor_configuration::MonitorConfiguration;
use crate::settings::settings_manager::SettingsManager;

pub struct DisplaySettingsState {
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
}

impl From<&Rc<RefCell<SettingsManager>>> for DisplaySettingsState {
    fn from(value: &Rc<RefCell<SettingsManager>>) -> Self {
        let monitor_configurations = value.borrow()
            .get_monitor_configurations();

        Self {
            monitor_configurations,
        }
    }
}