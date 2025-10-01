use crate::models::monitor::monitor_configuration::MonitorConfiguration;

#[derive(Clone, Default)]
pub struct MonitorFieldState {
    pub monitor_port: String,
    pub monitor_configuration: MonitorConfiguration,
}