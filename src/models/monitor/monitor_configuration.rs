use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::models::monitor::monitor_information::MonitorInformation;
use crate::models::monitor::video_mode::VideoMode;
use crate::math::vector::Vector;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub enum MonitorOrientation {
    #[default]
    None,
    Rotation90,
    Rotation180,
    Rotation270,
    Flipped,
    FlippedRotation90,
    FlippedRotation180,
    FlippedRotation270,
}

impl From<String> for MonitorOrientation {
    fn from(enum_string: String) -> Self {
        match enum_string.as_str() {
            "None" => MonitorOrientation::None,
            "90°" => MonitorOrientation::Rotation90,
            "180°" => MonitorOrientation::Rotation180,
            "270°" => MonitorOrientation::Rotation270,
            "Flipped" => MonitorOrientation::Flipped,
            "90° Flipped" => MonitorOrientation::FlippedRotation90,
            "180° Flipped" => MonitorOrientation::FlippedRotation180,
            "270° Flipped" => MonitorOrientation::FlippedRotation270,
            _ => MonitorOrientation::None,
        }
    }
}

impl Display for MonitorOrientation {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let enum_string = match self {
            MonitorOrientation::None => "None",
            MonitorOrientation::Rotation90 => "90°",
            MonitorOrientation::Rotation180 => "180°",
            MonitorOrientation::Rotation270 => "270°",
            MonitorOrientation::Flipped => "Flipped",
            MonitorOrientation::FlippedRotation90 => "90° Flipped",
            MonitorOrientation::FlippedRotation180 => "180° Flipped",
            MonitorOrientation::FlippedRotation270 => "270° Flipped"
        };
        write!(formatter, "{}", enum_string)
    }
}

impl MonitorOrientation {
    pub fn is_landscape(&self) -> bool {
        match self {
            MonitorOrientation::None => true,
            MonitorOrientation::Rotation180 => true,
            MonitorOrientation::Flipped => true,
            MonitorOrientation::FlippedRotation180 => true,
            _ => false
        }
    }

    pub fn is_portrait(&self) -> bool {
        match self {
            MonitorOrientation::Rotation90 => true,
            MonitorOrientation::Rotation270 => true,
            MonitorOrientation::FlippedRotation90 => true,
            MonitorOrientation::FlippedRotation270 => true,
            _ => false
        }
    }
    
    pub fn get_size_by_orientation(&self, size: Vector) -> Vector {
        if self.is_landscape() {
            size
        } else {
            Vector::new(size.get_y(), size.get_x())
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MonitorConfiguration {
    pub enabled: bool,
    pub information: MonitorInformation,
    pub video_mode: VideoMode,
    pub offset: Vector,
    pub resolution_scale: f32,
    pub orientation: MonitorOrientation,
}