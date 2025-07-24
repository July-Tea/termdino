use crate::core::entities::{Dinosaur, Obstacle};
use crate::render::Coordinate;

pub struct CollisionDetector;

impl CollisionDetector {
    pub fn new() -> Self {
        Self
    }
    
    pub fn check_collision(&self, dinosaur: &Dinosaur, obstacle: &Obstacle) -> bool {
        if !obstacle.active || !dinosaur.is_alive() {
            return false;
        }
        
        let (dino_tl, dino_br) = dinosaur.get_bounds();
        let (obs_tl, obs_br) = obstacle.get_bounds();
        
        self.aabb_collision(dino_tl, dino_br, obs_tl, obs_br)
    }
    
    fn aabb_collision(
        &self,
        a_top_left: Coordinate,
        a_bottom_right: Coordinate,
        b_top_left: Coordinate,
        b_bottom_right: Coordinate,
    ) -> bool {
        a_top_left.x < b_bottom_right.x
            && a_bottom_right.x > b_top_left.x
            && a_top_left.y < b_bottom_right.y
            && a_bottom_right.y > b_top_left.y
    }
    
    pub fn check_any_collision(&self, dinosaur: &Dinosaur, obstacles: &[Obstacle]) -> bool {
        obstacles
            .iter()
            .any(|obstacle| self.check_collision(dinosaur, obstacle))
    }
}