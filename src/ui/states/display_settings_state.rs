use std::collections::HashMap;
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::providers::application_provider::ApplicationProvider;

pub struct DisplaySettingsState {
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
}

impl From<&ApplicationProvider> for DisplaySettingsState {
    fn from(value: &ApplicationProvider) -> Self {
        let monitor_configurations = value
            .get_settings_provider()
            .borrow()
            .get_monitor_configurations();

        Self {
            monitor_configurations,
        }
    }
}