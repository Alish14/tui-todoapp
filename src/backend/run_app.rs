use std::{ option, thread, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend}, layout::{self, Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::line, text::{Span, Spans, Text}, widgets::{Block, Borders, List, ListItem, Paragraph, Tabs,ListState}, Frame, Terminal
};
use super::ui;
use super::structures::*;

pub fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut is_done_list = false;
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('D')=>{
                            if is_done_list {
                                app.items_done.delete_task();
                            } else {
                                app.items.delete_task();
                            }
                        }
                        KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                        KeyCode::Down => {
                            if is_done_list {
                                app.items_done.next();
                            } else {
                                app.items.next();
                            }
                        },
                        KeyCode::Right => app.next(),
                        KeyCode::Left => app.previous(),
                        KeyCode::Up => {
                            if is_done_list {
                                app.items_done.previous();
                            } else {
                                app.items.previous();
                            }
                        },
                        KeyCode::Char('s') => 
                        {  
                            if is_done_list {
                            app.items_done.unselect();
                        } else {
                            app.items.unselect();
                        }
                    },
                        KeyCode::Tab=> {
                            if is_done_list {
                                app.items_done.unselect();
                            } else {
                                app.items.unselect();
                            }
                            is_done_list = !is_done_list;
                        },
                        KeyCode::Enter => if let Some(task) = app.items.task_done(&mut app.items_done) {
                            app.items_done.items_done_arr.push(task);
                        },
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        KeyCode::Enter => {
                            app.items.items.push(app.input.drain(..).collect());
                        }
                        KeyCode::Backspace => {
                            app.input.pop();
                        }
                        KeyCode::Char(c) => {
                            app.input.push(c);
                        }
                        _ => {}
                    },
                }
            }
        }
    }
}
