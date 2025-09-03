use std::collections::HashMap;
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::providers::application_provider::ApplicationProvider;

#[derive(Clone)]
pub struct DisplaySettingsState {
    pub enabled: bool,
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
}

impl From<&ApplicationProvider> for DisplaySettingsState {
    fn from(value: &ApplicationProvider) -> Self {
        let monitor_configurations = value
            .get_settings_provider()
            .borrow()
            .get_monitor_configurations();

        let enabled = value.get_module_provider()
            .borrow()
            .get_module("wlr-randr".to_string())
            .is_some();

        Self {
            monitor_configurations,
            enabled
        }
    }
}