use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VideoMode {
    pub refresh_rate: u32,
    pub width_resolution: u32,
    pub height_resolution: u32,
}