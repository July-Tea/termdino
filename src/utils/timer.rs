use std::time::{Duration, Instant};

pub struct Timer {
    last_tick: Instant,
    target_fps: u32,
    frame_duration: Duration,
}

impl Timer {
    pub fn new(fps: u32) -> Self {
        Self {
            last_tick: Instant::now(),
            target_fps: fps,
            frame_duration: Duration::from_secs_f64(1.0 / fps as f64),
        }
    }
    
    pub fn tick(&mut self) -> Duration {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_tick);
        self.last_tick = now;
        elapsed
    }
    
    pub fn should_render(&self) -> bool {
        self.last_tick.elapsed() >= self.frame_duration
    }
    
    pub fn sleep_until_next_frame(&self) {
        let elapsed = self.last_tick.elapsed();
        if elapsed < self.frame_duration {
            std::thread::sleep(self.frame_duration - elapsed);
        }
    }
    
    pub fn delta_time(&self) -> f64 {
        self.last_tick.elapsed().as_secs_f64()
    }
}