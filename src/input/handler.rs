use crate::input::bindings::{GameAction, KeyBindings};
use crate::error::GameResult;
use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use std::time::Duration;

pub struct InputHandler {
    bindings: KeyBindings,
    last_action: GameAction,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            bindings: KeyBindings::default(),
            last_action: GameAction::None,
        }
    }
    
    pub fn poll_events(&mut self, timeout: Duration) -> GameResult<GameAction> {
        if event::poll(timeout)? {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                let action = self.bindings.get_action(code);
                self.last_action = action;
                return Ok(action);
            }
        }
        Ok(GameAction::None)
    }
    
    pub fn last_action(&self) -> GameAction {
        self.last_action
    }
    
    pub fn clear_last_action(&mut self) {
        self.last_action = GameAction::None;
    }
}