use crate::core::entities::{Dinosaur, Obstacle};
use crate::core::GameState;
use crate::error::GameResult;
use crate::render::widgets::GameWidget;
use crate::terminal::TerminalManager;

pub struct Renderer {
    terminal: TerminalManager,
}

impl Renderer {
    pub fn new() -> GameResult<Self> {
        let terminal = TerminalManager::new()?;
        Ok(Self { terminal })
    }
    
    pub fn render(
        &mut self,
        dinosaur: &Dinosaur,
        obstacles: &[Obstacle],
        score: u32,
        state: GameState,
    ) -> GameResult<()> {
        self.terminal.draw(|frame| {
            let widget = GameWidget::new(dinosaur, obstacles, score, state);
            frame.render_widget(widget, frame.area());
        })?;
        
        Ok(())
    }
    
    pub fn clear(&mut self) -> GameResult<()> {
        self.terminal.clear()
    }
    
    pub fn get_size(&self) -> GameResult<(u16, u16)> {
        self.terminal.size()
    }
}