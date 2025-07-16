#[derive(Debug, Clone)]
pub struct VideoMode {
    refresh_rate: u32,
    width_resolution: u32,
    height_resolution: u32,
}

impl VideoMode {
    pub fn new(width_resolution: u32, height_resolution: u32, refresh_rate: u32) -> VideoMode {
        VideoMode {
            refresh_rate,
            width_resolution,
            height_resolution,
        }
    }

    pub fn set_width_resolution(&mut self, width: u32) {
        self.width_resolution = width;
    }

    pub fn set_height_resolution(&mut self, height: u32) {
        self.height_resolution = height;
    }

    pub fn set_refresh_rate(&mut self, rate: u32) {
        self.refresh_rate = rate;
    }

    pub fn get_width_resolution(&self) -> u32 {
        self.width_resolution
    }

    pub fn get_height_resolution(&self) -> u32 {
        self.height_resolution
    }

    pub fn get_refresh_rate(&self) -> u32 {
        self.refresh_rate
    }
}