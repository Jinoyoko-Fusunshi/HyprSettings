use crate::math::geometry::aabb::AABB;
use crate::models::monitor::monitor_configuration::MonitorOrientation;
use crate::math::vector::Vector;

#[derive(Clone, Default)]
pub struct MonitorState {
    pub port_name: String,
    pub orientation: MonitorOrientation,
    pub previous_position: Vector,
    pub position: Vector,
    pub size: Vector,
}

impl MonitorState {
    pub fn get_orientated_size(&self) -> Vector {
        self.orientation.get_size_by_orientation(self.size.clone())
    }

    pub fn get_aabb(&self) -> AABB {
        AABB::new(
            self.position.get_x(),
            self.position.get_x() + self.size.get_x(),
            self.position.get_y(),
            self.position.get_y() + self.size.get_y()
        )
    }
}