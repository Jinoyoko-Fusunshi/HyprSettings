use std::collections::HashMap;
use crate::models::monitor::monitor_configuration::MonitorConfiguration;
use crate::providers::application_provider::ApplicationProvider;

#[derive(Clone)]
pub struct MonitorSettingsState {
    pub enabled: bool,
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
}

impl From<&ApplicationProvider> for MonitorSettingsState {
    fn from(value: &ApplicationProvider) -> Self {
        let monitor_configurations = value.get_monitor_provider()
            .borrow()
            .get_monitor_configurations();

        let enabled = value
            .get_program_provider()
            .borrow()
            .has_dependency_module("wlr-randr".to_string());

        Self {
            monitor_configurations,
            enabled
        }
    }
}