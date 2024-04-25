use std::{ thread, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{self, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Tabs,Paragraph},
    Frame, Terminal,text::Text
};
struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["day1", "day2", "day3", "day4","day5","day6","day7"],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    // thread::sleep(Duration::from_millis(10000));

    // restore terminal
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next(),
                KeyCode::Left => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &App){
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Rgb((255), (192), (203))).fg(Color::Black));
    f.render_widget(block, size);
    let text = vec![
    Spans::from(vec![
        Span::raw("First"),
        Span::styled("line",Style::default().add_modifier(Modifier::ITALIC)),
        Span::raw("."),
    ])];
    




    let titles = app
    .titles
    .iter()
    .map(|t| {
        let (first, rest) = t.split_at(1);
        Spans::from(vec![
            Span::styled(first, Style::default().fg(Color::Yellow)),
            Span::styled(rest, Style::default().fg(Color::Green)),
        ])
    })
    .collect();
let tabs = Tabs::new(titles)
    .block(Block::default().borders(Borders::ALL).title("day Todos"))
    .select(app.index)
    .style(Style::default().fg(Color::Cyan))
    .highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Black),
    );
f.render_widget(tabs, chunks[0]);
let block = Block::default().title(app.titles[app.index]).borders(Borders::ALL);
f.render_widget(block, chunks[1]);
let chunks=layout::Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            layout::Constraint::Percentage(50),
            layout::Constraint::Percentage(50),
            layout::Constraint::Percentage(50),
        ]
        .as_ref(),
    )
    .split(chunks[1]);
// let block = Block::default().title("Tasks").borders(Borders::ALL);
// f.render_widget(block, chunk[0]);

let block = Block::default().title("doing").borders(Borders::ALL);
f.render_widget(block, chunks[1]);
let block = Block::default().title("done").borders(Borders::ALL);
f.render_widget(block, chunks[0]);



}