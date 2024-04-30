use std::{ option, thread, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::backend::*;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend}, layout::{self, Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::line, text::{Span, Spans, Text}, widgets::{Block, Borders, List, ListItem, Paragraph, Tabs,ListState}, Frame, Terminal
};

mod backend;
fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let app = App::new();
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

