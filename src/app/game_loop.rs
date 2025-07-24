use crate::app::state_manager::StateManager;
use crate::core::entities::{Dinosaur, ObstacleManager};
use crate::core::physics::{CollisionDetector, PhysicsEngine};
use crate::core::GameState;
use crate::error::GameResult;
use crate::input::{GameAction, InputHandler};
use crate::render::Renderer;
use crate::utils::{Config, Timer};
use std::time::Duration;

pub struct App {
    config: Config,
    state_manager: StateManager,
    input_handler: InputHandler,
    renderer: Renderer,
    dinosaur: Dinosaur,
    obstacle_manager: ObstacleManager,
    physics_engine: PhysicsEngine,
    collision_detector: CollisionDetector,
    timer: Timer,
    score: u32,
    running: bool,
}

impl App {
    pub fn new() -> GameResult<Self> {
        let config = Config::new();
        let state_manager = StateManager::new();
        let input_handler = InputHandler::new();
        let renderer = Renderer::new()?;
        
        let (width, height) = renderer.get_size()?;
        crate::terminal::utils::check_terminal_size(width, height)?;
        
        let dinosaur = Dinosaur::new(10.0, config.ground_level, config.dino_char);
        let obstacle_manager = ObstacleManager::new(
            config.spawn_rate,
            config.obstacle_speed,
            config.ground_level,
            width as f64,
            config.obstacle_char,
        );
        
        let physics_engine = PhysicsEngine::new(config.gravity, config.ground_level);
        let collision_detector = CollisionDetector::new();
        let timer = Timer::new(config.fps);
        
        Ok(Self {
            config,
            state_manager,
            input_handler,
            renderer,
            dinosaur,
            obstacle_manager,
            physics_engine,
            collision_detector,
            timer,
            score: 0,
            running: true,
        })
    }
    
    pub fn run(&mut self) -> GameResult<()> {
        while self.running {
            let delta_time = self.timer.tick();
            
            self.handle_input()?;
            self.update(delta_time.as_secs_f64())?;
            self.render()?;
            
            self.timer.sleep_until_next_frame();
        }
        Ok(())
    }
    
    fn handle_input(&mut self) -> GameResult<()> {
        let action = self.input_handler.poll_events(Duration::from_millis(1))?;
        
        if action == GameAction::Quit {
            self.running = false;
            return Ok(());
        }
        
        match (self.state_manager.current_state(), action) {
            (GameState::Running, GameAction::Jump) => {
                self.dinosaur.jump(self.config.jump_force);
            }
            (GameState::Initialize, GameAction::Jump) => {
                self.start_new_game();
            }
            (GameState::GameOver, GameAction::Restart) => {
                self.start_new_game();
            }
            _ => {}
        }
        
        self.state_manager.handle_input(action);
        Ok(())
    }
    
    fn update(&mut self, delta_time: f64) -> GameResult<()> {
        match self.state_manager.current_state() {
            GameState::Running => {
                self.physics_engine.update_dinosaur(&mut self.dinosaur, delta_time);
                self.obstacle_manager.update(delta_time);
                
                // 检查碰撞
                if self.collision_detector.check_any_collision(
                    &self.dinosaur,
                    &self.obstacle_manager.obstacles,
                ) {
                    self.dinosaur.kill();
                    self.state_manager.game_over();
                } else {
                    // 只有在活着的时候才计分
                    self.update_score(delta_time);
                }
            }
            GameState::Initialize => {
                // 确保分数在初始化时为0
                self.score = 0;
            }
            GameState::Paused => {
                // 暂停时不更新分数
            }
            GameState::GameOver => {
                // 游戏结束时不更新分数
            }
        }
        
        Ok(())
    }
    
    fn update_score(&mut self, delta_time: f64) {
        // 基础分数：基于时间的持续计分 (每秒30分)
        self.score += (delta_time * 30.0) as u32;
        
        // 障碍物躲避奖励分数
        let bonus_score = self.obstacle_manager.check_and_score_passed_obstacles(self.dinosaur.position.x);
        self.score += bonus_score;
        
        // 跳跃奖励：鼓励玩家跳跃 (每秒额外10分)
        if self.dinosaur.state == crate::core::entities::dinosaur::DinosaurState::Jumping {
            self.score += (delta_time * 10.0) as u32;
        }
    }
    
    fn render(&mut self) -> GameResult<()> {
        self.renderer.render(
            &self.dinosaur,
            &self.obstacle_manager.obstacles,
            self.score,
            self.state_manager.current_state(),
        )?;
        Ok(())
    }
    
    fn start_new_game(&mut self) {
        let (width, _) = self.renderer.get_size().unwrap_or((80, 24));
        self.dinosaur = Dinosaur::new(10.0, self.config.ground_level, self.config.dino_char);
        self.obstacle_manager = ObstacleManager::new(
            self.config.spawn_rate,
            self.config.obstacle_speed,
            self.config.ground_level,
            width as f64,
            self.config.obstacle_char,
        );
        self.obstacle_manager.clear();
        self.score = 0;
        self.state_manager.start_game();
    }
}