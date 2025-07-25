use crate::monitor::monitor_information::MonitorInformation;
use crate::monitor::video_mode::VideoMode;

#[derive(Debug, Clone)]
pub struct MonitorSetting {
    pub enabled: bool,
    pub information: MonitorInformation,
    pub video_mode: VideoMode,
}