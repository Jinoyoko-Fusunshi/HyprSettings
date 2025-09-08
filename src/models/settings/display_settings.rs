use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::monitor::monitor_configuration::MonitorConfiguration;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DisplaySettings {
    pub monitor_configurations: HashMap<String, MonitorConfiguration>,
}

impl DisplaySettings {
    pub fn new(monitor_configurations: HashMap<String, MonitorConfiguration>,) -> Self {
        Self {
            monitor_configurations
        }
    }
}