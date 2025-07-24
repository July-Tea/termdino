#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Initialize,
    Running,
    Paused,
    GameOver,
}

impl GameState {
    pub fn can_transition_to(&self, new_state: GameState) -> bool {
        use GameState::*;
        match (self, new_state) {
            (Initialize, Running) => true,
            (Running, Paused) => true,
            (Running, GameOver) => true,
            (Paused, Running) => true,
            (Paused, GameOver) => true,
            (GameOver, Initialize) => true,
            (GameOver, Running) => true,
            _ => false,
        }
    }
    
    pub fn is_playing(&self) -> bool {
        matches!(self, GameState::Running)
    }
    
    pub fn is_paused(&self) -> bool {
        matches!(self, GameState::Paused)
    }
    
    pub fn is_game_over(&self) -> bool {
        matches!(self, GameState::GameOver)
    }
}