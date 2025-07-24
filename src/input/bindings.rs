use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameAction {
    Jump,
    Pause,
    Restart,
    Quit,
    None,
}

pub struct KeyBindings {
    jump: KeyCode,
    pause: KeyCode,
    restart: KeyCode,
    quit: KeyCode,
    quit_alt: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            jump: KeyCode::Char(' '),
            pause: KeyCode::Char('p'),
            restart: KeyCode::Char('r'),
            quit: KeyCode::Esc,
            quit_alt: KeyCode::Char('q'),
        }
    }
}

impl KeyBindings {
    pub fn get_action(&self, key: KeyCode) -> GameAction {
        match key {
            key if key == self.jump => GameAction::Jump,
            key if key == self.pause => GameAction::Pause,
            key if key == self.restart => GameAction::Restart,
            key if key == self.quit || key == self.quit_alt => GameAction::Quit,
            _ => GameAction::None,
        }
    }
}