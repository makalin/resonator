use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use resonator::App;
use std::io;

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let terminal = ratatui::Terminal::new(backend)?;

    // Create and run app
    let mut app = App::new()?;
    let res = app.run(terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

