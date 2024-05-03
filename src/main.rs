use std::time::Duration;
use crossterm::{
    event::{ DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::backend::*;
use std::io;
use tui::{
    backend::CrosstermBackend, Terminal
};
use std::{fs::File, io::Read};
mod backend;
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;
    let file_path = "todo_state.json";
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let app= serde_json::from_str(&contents)?;


    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app,tick_rate);


    if let Err(err) = res {
        println!("{:?}", err)
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

