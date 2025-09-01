use serde::{Deserialize, Serialize};
use crate::models::monitor::monitor_information::MonitorInformation;
use crate::models::monitor::video_mode::VideoMode;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorConfiguration {
    pub enabled: bool,
    pub information: MonitorInformation,
    pub video_mode: VideoMode,
}