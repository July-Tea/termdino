use crate::render::Coordinate;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Obstacle {
    pub position: Coordinate,
    pub velocity: Coordinate,
    pub sprite: char,
    pub width: u16,
    pub height: u16,
    pub active: bool,
    pub scored: bool,  // 新增：是否已经计分
}

impl Obstacle {
    pub fn new(x: f64, ground_y: f64, sprite: char) -> Self {
        Self {
            position: Coordinate::new(x, ground_y),
            velocity: Coordinate::new(-200.0, 0.0),
            sprite,
            width: 1,
            height: 1,
            active: true,
            scored: false,  // 初始化为未计分
        }
    }
    
    pub fn update(&mut self, delta_time: f64) {
        if !self.active {
            return;
        }
        
        self.position += Coordinate::new(
            self.velocity.x * delta_time,
            self.velocity.y * delta_time,
        );
        
        if self.position.x < -10.0 {
            self.active = false;
        }
    }
    
    pub fn get_bounds(&self) -> (Coordinate, Coordinate) {
        let top_left = self.position;
        let bottom_right = Coordinate::new(
            self.position.x + self.width as f64,
            self.position.y + self.height as f64,
        );
        (top_left, bottom_right)
    }
    
    pub fn is_off_screen(&self) -> bool {
        !self.active || self.position.x < -10.0
    }
}

pub struct ObstacleManager {
    pub obstacles: Vec<Obstacle>,
    spawn_timer: f64,
    spawn_rate: f64,
    obstacle_speed: f64,
    ground_level: f64,
    screen_width: f64,
    sprite: char,
}

impl ObstacleManager {
    pub fn new(spawn_rate: f64, obstacle_speed: f64, ground_level: f64, screen_width: f64, sprite: char) -> Self {
        Self {
            obstacles: Vec::new(),
            spawn_timer: 0.0,
            spawn_rate,
            obstacle_speed,
            ground_level,
            screen_width,
            sprite,
        }
    }
    
    pub fn update(&mut self, delta_time: f64) {
        self.spawn_timer += delta_time;
        
        // 改进障碍物生成逻辑：达到生成间隔且有30%概率生成
        if self.spawn_timer >= self.spawn_rate && rand::rng().random_bool(0.3) {
            self.spawn_obstacle();
            self.spawn_timer = 0.0;
        }
        
        // 更新所有障碍物位置
        for obstacle in &mut self.obstacles {
            obstacle.velocity.x = -self.obstacle_speed;
            obstacle.update(delta_time);
        }
        
        // 清理离开屏幕的障碍物
        self.obstacles.retain(|obstacle| !obstacle.is_off_screen());
    }
    
    fn spawn_obstacle(&mut self) {
        let obstacle = Obstacle::new(self.screen_width + 10.0, self.ground_level, self.sprite);
        self.obstacles.push(obstacle);
    }
    
    pub fn clear(&mut self) {
        self.obstacles.clear();
        self.spawn_timer = 0.0;
    }
    
    pub fn get_active_obstacles(&self) -> impl Iterator<Item = &Obstacle> {
        self.obstacles.iter().filter(|obs| obs.active)
    }
    
    pub fn check_and_score_passed_obstacles(&mut self, player_x: f64) -> u32 {
        let mut bonus_score = 0;
        
        for obstacle in &mut self.obstacles {
            // 如果障碍物还没计分，且已经被玩家超越
            if !obstacle.scored && obstacle.active && obstacle.position.x < player_x - 2.0 {
                obstacle.scored = true;
                bonus_score += 50; // 每个障碍物50分奖励
            }
        }
        
        bonus_score
    }
}