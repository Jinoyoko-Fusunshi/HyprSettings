use crate::monitor::monitor_information::MonitorInformation;
use crate::monitor::video_mode::VideoMode;

#[derive(Debug, Clone)]
pub struct MonitorSetting {
    enabled: bool,
    information: MonitorInformation,
    video_mode: VideoMode,
}

impl MonitorSetting {
    pub fn new(enabled: bool, information: MonitorInformation, video_mode: VideoMode) -> Self {
        MonitorSetting {
            enabled,
            information,
            video_mode,
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_width_resolution(&mut self, width: u32) {
        self.video_mode.set_width_resolution(width);
    }

    pub fn set_height_resolution(&mut self, height: u32) {
        self.video_mode.set_height_resolution(height);
    }

    pub fn set_refresh_rate(&mut self, rate: u32) {
        self.video_mode.set_refresh_rate(rate);
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn get_width_resolution(&self) -> u32 {
        self.video_mode.get_width_resolution()
    }

    pub fn get_height_resolution(&self) -> u32 {
        self.video_mode.get_height_resolution()
    }

    pub fn get_information(&self) -> &MonitorInformation {
        &self.information
    }
}