use crate::settings::monitor::monitor_information::MonitorInformation;
use crate::settings::monitor::video_mode::VideoMode;

#[derive(Debug, Clone)]
pub struct MonitorConfiguration {
    pub enabled: bool,
    pub information: MonitorInformation,
    pub video_mode: VideoMode,
}