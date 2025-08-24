use serde::{Deserialize, Serialize};
use crate::settings::monitor::video_mode::VideoMode;
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorInformation {
    pub port_name: String,
    pub brand_name: String,
    pub model_name: String,
    pub serial_number: String,
    pub min_video_mode: VideoMode,
    pub max_video_mode: VideoMode,
}