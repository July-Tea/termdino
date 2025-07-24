use crate::core::entities::Dinosaur;

pub struct PhysicsEngine {
    gravity: f64,
    ground_level: f64,
}

impl PhysicsEngine {
    pub fn new(gravity: f64, ground_level: f64) -> Self {
        Self {
            gravity,
            ground_level,
        }
    }
    
    pub fn update_dinosaur(&self, dinosaur: &mut Dinosaur, delta_time: f64) {
        dinosaur.update(delta_time, self.gravity, self.ground_level);
    }
    
    pub fn apply_gravity(&self, velocity_y: &mut f64, delta_time: f64) {
        *velocity_y += self.gravity * delta_time;
    }
    
    pub fn check_ground_collision(&self, position_y: f64) -> bool {
        position_y <= self.ground_level
    }
}