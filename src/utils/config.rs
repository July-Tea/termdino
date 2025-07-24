pub struct Config {
    pub fps: u32,
    pub gravity: f64,
    pub jump_force: f64,
    pub ground_level: f64,
    pub obstacle_speed: f64,
    pub spawn_rate: f64,
    pub dino_char: char,
    pub obstacle_char: char,
    pub use_unicode: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fps: 60,
            gravity: -1000.0,         // 适中的重力
            jump_force: 180.0,        // 适中的跳跃力度
            ground_level: 3.0,        // 地面高度
            obstacle_speed: 40.0,     // 稍微降低障碍物速度，让游戏更容易
            spawn_rate: 3.0,          // 增加生成间隔，减少难度
            dino_char: '🦕',
            obstacle_char: '🌵',
            use_unicode: true,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let mut config = Self::default();
        
        // 强制使用Unicode emoji，大部分现代终端都支持
        config.use_unicode = true;
        config.dino_char = '🦕';
        config.obstacle_char = '🌵';
        
        config
    }
}