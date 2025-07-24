use crate::core::entities::{Dinosaur, Obstacle};
use crate::core::GameState;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct GameWidget<'a> {
    dinosaur: &'a Dinosaur,
    obstacles: &'a [Obstacle],
    score: u32,
    state: GameState,
}

impl<'a> GameWidget<'a> {
    pub fn new(
        dinosaur: &'a Dinosaur,
        obstacles: &'a [Obstacle],
        score: u32,
        state: GameState,
    ) -> Self {
        Self {
            dinosaur,
            obstacles,
            score,
            state,
        }
    }
}

impl<'a> Widget for GameWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL);
        let inner = block.inner(area);
        block.render(area, buf);
        
        self.render_game_elements(inner, buf);
        self.render_ui(inner, buf);
    }
}

impl<'a> GameWidget<'a> {
    fn render_game_elements(&self, area: Rect, buf: &mut Buffer) {
        // 确保游戏区域足够大
        if area.height < 5 || area.width < 10 {
            return;
        }
        
        // 使用游戏区域的高度进行坐标转换
        let game_area_height = area.height;
        
        // 渲染地面线，位置与game逻辑中的ground_level对应
        let ground_line = area.height.saturating_sub(3);
        for x in 0..area.width {
            if x < area.width && ground_line < area.height {
                let cell = &mut buf[(area.x + x, area.y + ground_line)];
                cell.set_char('─');
                cell.set_fg(Color::Yellow);
            }
        }
        
        // 渲染恐龙
        let (dino_x, dino_y) = self
            .dinosaur
            .position
            .to_terminal_coords(game_area_height);
        
        if dino_x < area.width && dino_y < area.height {
            let cell = &mut buf[(area.x + dino_x, area.y + dino_y)];
            cell.set_char(self.dinosaur.sprite);
            
            // 根据恐龙状态设置不同颜色
            match self.dinosaur.state {
                crate::core::entities::dinosaur::DinosaurState::Dead => {
                    cell.set_fg(Color::Red);
                }
                crate::core::entities::dinosaur::DinosaurState::Jumping => {
                    cell.set_fg(Color::Cyan);
                }
                crate::core::entities::dinosaur::DinosaurState::Falling => {
                    cell.set_fg(Color::Blue);
                }
                _ => {
                    cell.set_fg(Color::Green);
                }
            }
        }
        
        // 渲染障碍物
        for obstacle in self.obstacles.iter().filter(|obs| obs.active) {
            let (obs_x, obs_y) = obstacle.position.to_terminal_coords(game_area_height);
            
            if obs_x < area.width && obs_y < area.height {
                let cell = &mut buf[(area.x + obs_x, area.y + obs_y)];
                cell.set_char(obstacle.sprite);
                cell.set_fg(Color::Red);
            }
        }
    }
    
    fn render_ui(&self, area: Rect, buf: &mut Buffer) {
        // 左上角显示游戏名
        let game_title = "Termdino";
        let title_para = Paragraph::new(game_title)
            .style(Style::default().fg(Color::Green))
            .alignment(Alignment::Left);
        title_para.render(
            Rect::new(area.x + 2, area.y, game_title.len() as u16, 1),
            buf,
        );
        
        // 右上角显示分数
        let score_text = format!("Score: {}", self.score);
        let score_len = score_text.len() as u16;
        let score_x = area.width.saturating_sub(score_len + 2);
        let score_para = Paragraph::new(score_text)
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Right);
        score_para.render(
            Rect::new(area.x + score_x, area.y, score_len, 1),
            buf,
        );
        
        match self.state {
            GameState::Initialize => {
                let start_text = "🎮 Press SPACE to start the Terminal Dino Game! 🎮";
                let start_para = Paragraph::new(start_text)
                    .style(Style::default().fg(Color::Green))
                    .alignment(Alignment::Center);
                start_para.render(
                    Rect::new(
                        area.x,
                        area.y + area.height / 2,
                        area.width,
                        1,
                    ),
                    buf,
                );
            }
            GameState::Paused => {
                let pause_text = "⏸️  PAUSED - Press P to resume  ⏸️";
                let pause_para = Paragraph::new(pause_text)
                    .style(Style::default().fg(Color::Yellow))
                    .alignment(Alignment::Center);
                pause_para.render(
                    Rect::new(
                        area.x,
                        area.y + area.height / 2,
                        area.width,
                        1,
                    ),
                    buf,
                );
            }
            GameState::GameOver => {
                let game_over_text = format!("💀 GAME OVER - Final Score: {} 💀", self.score);
                let game_over_para = Paragraph::new(game_over_text)
                    .style(Style::default().fg(Color::Red))
                    .alignment(Alignment::Center);
                game_over_para.render(
                    Rect::new(
                        area.x,
                        area.y + area.height / 2,
                        area.width,
                        1,
                    ),
                    buf,
                );
                
                let restart_text = "Press R to restart or Q to quit";
                let restart_para = Paragraph::new(restart_text)
                    .style(Style::default().fg(Color::White))
                    .alignment(Alignment::Center);
                restart_para.render(
                    Rect::new(
                        area.x,
                        area.y + area.height / 2 + 1,
                        area.width,
                        1,
                    ),
                    buf,
                );
            }
            _ => {}
        }
    }
}