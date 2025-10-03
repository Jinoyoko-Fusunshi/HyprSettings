use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::monitor::monitor_configuration::MonitorConfiguration;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct MonitorSettings {
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
}

impl MonitorSettings {
    pub fn new(monitor_configurations: HashMap<String, MonitorConfiguration>,) -> Self {
        Self {
            monitor_configurations
        }
    }
}