use crate::math::vector::Vector;

/// Axis Aligned Bounding Box
///
pub struct AABB {
    start_xposition: f64,
    end_xposition: f64, 
    start_yposition: f64, 
    end_yposition: f64
}

impl AABB {
    pub fn new(start_xposition: f64, end_xposition: f64, start_yposition: f64, end_yposition: f64) -> Self {
        Self {
            start_xposition,
            end_yposition,
            start_yposition,
            end_xposition,
        }
    }
    
    pub fn get_center_position(&self) -> Vector {
        let half_width = (self.end_xposition - self.start_xposition) / 2.0;
        let half_height = (self.end_yposition - self.start_yposition) / 2.0;
        Vector::new(self.start_xposition + half_width, self.start_yposition + half_height)
    }
    
    pub fn intersects_with(&self, other: &Self) -> bool {
        let top_left_point = Vector::new(self.start_xposition, self.start_yposition);
        let top_right_point = Vector::new(self.end_xposition, self.start_yposition);
        let bottom_left_point = Vector::new(self.start_xposition, self.end_yposition);
        let bottom_right_point = Vector::new(self.end_xposition, self.end_yposition);
        
        self.point_intersects_with(top_left_point, other)
            || self.point_intersects_with(top_right_point, other)
            || self.point_intersects_with(bottom_left_point, other)
            || self.point_intersects_with(bottom_right_point, other)
    }
    
    pub fn point_intersects_with(&self, point: Vector, other: &Self) -> bool {
        point.get_x() >= other.start_xposition && point.get_x() <= other.end_xposition 
            && point.get_y() >= other.start_yposition && point.get_y() <= other.end_yposition
    }
    
    pub fn get_start_xposition(&self) -> f64 {
        self.start_xposition
    }
    
    pub fn get_end_xposition(&self) -> f64 {
        self.end_xposition
    }
    
    pub fn get_start_yposition(&self) -> f64 {
        self.start_yposition
    }
    
    pub fn get_end_yposition(&self) -> f64 {
        self.end_yposition
    }
    
    pub fn get_width(&self) -> f64 {
        self.end_xposition - self.start_xposition
    }
    
    pub fn get_height(&self) -> f64 {
        self.end_yposition - self.start_yposition
    }
}