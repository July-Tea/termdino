use crate::core::GameState;
use crate::input::GameAction;

pub struct StateManager {
    current_state: GameState,
    previous_state: GameState,
}

impl StateManager {
    pub fn new() -> Self {
        Self {
            current_state: GameState::Initialize,
            previous_state: GameState::Initialize,
        }
    }
    
    pub fn current_state(&self) -> GameState {
        self.current_state
    }
    
    pub fn previous_state(&self) -> GameState {
        self.previous_state
    }
    
    pub fn transition_to(&mut self, new_state: GameState) -> bool {
        if self.current_state.can_transition_to(new_state) {
            self.previous_state = self.current_state;
            self.current_state = new_state;
            true
        } else {
            false
        }
    }
    
    pub fn handle_input(&mut self, action: GameAction) {
        match (self.current_state, action) {
            (GameState::Initialize, GameAction::Jump) => {
                self.transition_to(GameState::Running);
            }
            (GameState::Running, GameAction::Pause) => {
                self.transition_to(GameState::Paused);
            }
            (GameState::Paused, GameAction::Pause) => {
                self.transition_to(GameState::Running);
            }
            (GameState::Running, _) if matches!(action, GameAction::Quit) => {
                // Handle quit in main loop
            }
            (GameState::GameOver, GameAction::Restart) => {
                self.transition_to(GameState::Initialize);
            }
            _ => {}
        }
    }
    
    pub fn game_over(&mut self) {
        self.transition_to(GameState::GameOver);
    }
    
    pub fn start_game(&mut self) {
        self.transition_to(GameState::Running);
    }
}