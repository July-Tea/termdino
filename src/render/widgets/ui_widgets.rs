use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

pub struct UIWidget;

impl UIWidget {
    pub fn render_instructions(area: Rect, buf: &mut Buffer) {
        let instructions = vec![
            "Controls:",
            "SPACE - Jump",
            "P - Pause/Resume",
            "R - Restart (when game over)",
            "Q/ESC - Quit",
        ];
        
        for (i, instruction) in instructions.iter().enumerate() {
            if i < area.height as usize {
                let para = Paragraph::new(*instruction)
                    .style(Style::default().fg(Color::Cyan));
                para.render(
                    Rect::new(area.x, area.y + i as u16, area.width, 1),
                    buf,
                );
            }
        }
    }
}