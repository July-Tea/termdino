#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

impl Coordinate {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
    
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    
    pub fn to_terminal_coords(&self, terminal_height: u16) -> (u16, u16) {
        let term_x = self.x.max(0.0) as u16;
        // 修复坐标转换，确保在有效范围内
        let term_y = if terminal_height > 0 {
            ((terminal_height as f64 - self.y).max(0.0).min(terminal_height as f64 - 1.0)) as u16
        } else {
            0
        };
        (term_x, term_y)
    }
    
    pub fn from_terminal_coords(x: u16, y: u16, terminal_height: u16) -> Self {
        Self {
            x: x as f64,
            y: terminal_height as f64 - y as f64,
        }
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}