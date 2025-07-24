use crate::render::Coordinate;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DinosaurState {
    Running,
    Jumping,
    Falling,
    Dead,
}

pub struct Dinosaur {
    pub position: Coordinate,
    pub velocity: Coordinate,
    pub state: DinosaurState,
    pub sprite: char,
    pub width: u16,
    pub height: u16,
    pub on_ground: bool,
}

impl Dinosaur {
    pub fn new(x: f64, ground_y: f64, sprite: char) -> Self {
        Self {
            position: Coordinate::new(x, ground_y),
            velocity: Coordinate::zero(),
            state: DinosaurState::Running,
            sprite,
            width: 1,
            height: 1,
            on_ground: true,
        }
    }
    
    pub fn jump(&mut self, jump_force: f64) {
        if self.on_ground && self.state != DinosaurState::Dead {
            self.velocity.y = jump_force;
            self.state = DinosaurState::Jumping;
            self.on_ground = false;
        }
    }
    
    pub fn update(&mut self, delta_time: f64, gravity: f64, ground_level: f64) {
        if self.state == DinosaurState::Dead {
            return;
        }
        
        self.velocity.y += gravity * delta_time;
        self.position += Coordinate::new(
            self.velocity.x * delta_time,
            self.velocity.y * delta_time,
        );
        
        if self.position.y <= ground_level {
            self.position.y = ground_level;
            self.velocity.y = 0.0;
            self.on_ground = true;
            self.state = DinosaurState::Running;
        } else {
            self.on_ground = false;
            if self.velocity.y > 0.0 {
                self.state = DinosaurState::Falling;
            }
        }
    }
    
    pub fn kill(&mut self) {
        self.state = DinosaurState::Dead;
        self.velocity = Coordinate::zero();
    }
    
    pub fn is_alive(&self) -> bool {
        self.state != DinosaurState::Dead
    }
    
    pub fn get_bounds(&self) -> (Coordinate, Coordinate) {
        let top_left = self.position;
        let bottom_right = Coordinate::new(
            self.position.x + self.width as f64,
            self.position.y + self.height as f64,
        );
        (top_left, bottom_right)
    }
}