use crate::error::GameResult;

pub fn check_terminal_size(width: u16, height: u16) -> GameResult<()> {
    const MIN_WIDTH: u16 = 80;
    const MIN_HEIGHT: u16 = 24;
    
    if width < MIN_WIDTH || height < MIN_HEIGHT {
        return Err(crate::error::GameError::TerminalError(
            format!("Terminal too small. Minimum size: {}x{}, current: {}x{}", 
                MIN_WIDTH, MIN_HEIGHT, width, height)
        ));
    }
    
    Ok(())
}

pub fn supports_unicode() -> bool {
    std::env::var("TERM").map_or(false, |term| {
        !term.contains("xterm-256color") || term.contains("utf")
    })
}