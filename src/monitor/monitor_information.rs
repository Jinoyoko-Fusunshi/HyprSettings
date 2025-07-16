use crate::monitor::video_mode::VideoMode;

#[derive(Debug, Clone)]
pub struct MonitorInformation {
    port_name: String,
    brand_name: String,
    model_name: String,
    serial_number: String,
    min_video_mode: VideoMode,
    max_video_mode: VideoMode,
}

impl MonitorInformation {
    pub fn new(
        port_name: String,
        brand_name: String,
        model_name: String,
        serial_number: String,
        min_video_mode: VideoMode,
        max_video_mode: VideoMode,
    ) -> Self{
        MonitorInformation {
            port_name,
            brand_name,
            model_name,
            serial_number,
            min_video_mode,
            max_video_mode,
        }
    }

    pub fn get_port_name(&self) -> &String {
        &self.port_name
    }

    pub fn get_brand_name(&self) -> &String {
        &self.brand_name
    }

    pub fn get_model_name(&self) -> &String {
        &self.model_name
    }

    pub fn get_serial_number(&self) -> &String {
        &self.serial_number
    }

    pub fn get_min_video_mode(&self) -> &VideoMode {
        &self.min_video_mode
    }

    pub fn get_max_video_mode(&self) -> &VideoMode {
        &self.max_video_mode
    }
}