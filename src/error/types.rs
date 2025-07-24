use std::fmt;

#[derive(Debug)]
pub enum GameError {
    TerminalError(String),
    InputError(String),
    RenderError(String),
    PhysicsError(String),
    ConfigError(String),
    IoError(std::io::Error),
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::TerminalError(msg) => write!(f, "Terminal error: {}", msg),
            GameError::InputError(msg) => write!(f, "Input error: {}", msg),
            GameError::RenderError(msg) => write!(f, "Render error: {}", msg),
            GameError::PhysicsError(msg) => write!(f, "Physics error: {}", msg),
            GameError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            GameError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for GameError {}

impl From<std::io::Error> for GameError {
    fn from(err: std::io::Error) -> Self {
        GameError::IoError(err)
    }
}

pub type GameResult<T> = Result<T, GameError>;