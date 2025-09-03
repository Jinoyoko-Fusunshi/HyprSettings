use crate::models::monitor::monitor_configuration::MonitorConfiguration;

#[derive(Clone)]
pub struct DisplayFieldState {
    pub monitor_port: String,
    pub monitor_configuration: MonitorConfiguration,
}