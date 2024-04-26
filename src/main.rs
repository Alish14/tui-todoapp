use std::{ thread, time::Duration,time::Instant};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend}, layout::{self, Alignment, Constraint, Direction, Layout}, style::{Color, Modifier, Style}, symbols::line, text::{Span, Spans, Text}, widgets::{Block, Borders, List, ListItem, Paragraph, Tabs,ListState}, Frame, Terminal
};

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn unselect(&mut self) {
        self.state.select(None);
    }
}

struct App<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
    items: StatefulList<(&'a str, usize)>
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["day1", "day2", "day3", "day4","day5","day6","day7"],
            index: 0,
            items: StatefulList::with_items(vec![
                ("Item0", 1),
                ("Item1", 2),
                ("Item2", 1),
                ("Item3", 3),
                ("Item4", 1),
                ("Item5", 4),
                ("Item6", 1),
                ("Item7", 3),
                ("Item8", 1),
                ("Item9", 6),
                ("Item10", 1),
                ("Item11", 3),
                ("Item12", 1),
                ("Item13", 2),
                ("Item14", 1),
                ("Item15", 1),
                ("Item16", 4),
                ("Item17", 1),
                ("Item18", 5),
                ("Item19", 4),
                ("Item20", 1),
                ("Item21", 2),
                ("Item22", 1),
                ("Item23", 3),
                ("Item24", 1),
            ]),
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
    let mut terminal: Terminal<CrosstermBackend<io::Stdout>> = Terminal::new(backend)?;

    let app = App::new();
    let tick_rate = Duration::from_millis(250);
    let res = run_app(&mut terminal, app,tick_rate);

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


fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('s') => app.items.unselect(),
                    KeyCode::Down => app.items.next(),
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    KeyCode::Up => app.items.previous(),
                    _ => {}
                }
            }
        }
    }
}
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(size);

    let text = vec![
        Spans::from(Span::styled(
            "enter q for quit and use arrow key for change tabs",
            Style::default().fg(Color::Red),
        )),
    ];
    let paragraph = Paragraph::new(text.clone()).alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[1]);

    let block = Block::default()
        .style(Style::default().bg(Color::Rgb(255, 192, 203)).fg(Color::Black));
    f.render_widget(block, chunks[0]);

    let chunks_down = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
        .split(chunks[1]);


    let block = Block::default().style(Style::default().bg(Color::Rgb((255), (192), (203))).fg(Color::Black));
    f.render_widget(block, size);
    let text = vec![
        Spans::from(Span::styled(
            "enter q for quit and use arrow key for change tabs",
            Style::default().fg(Color::Red),
        )),
    ];

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
f.render_widget(tabs, chunks_down[0]);
let block = Block::default().title(app.titles[app.index]).borders(Borders::ALL);
f.render_widget(block, chunks_down[1]);
let chunks_list=layout::Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
        [
            layout::Constraint::Percentage(50),
            layout::Constraint::Percentage(50),        ]
        .as_ref(),
    )
    .split(chunks_down[1]);

    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(i.0)];
            for _ in 0..i.1 {
                lines.push(Spans::from(Span::styled(
                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            }
            ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let items = List::new(items)
    .block(Block::default().borders(Borders::ALL).title("List"))
    .highlight_style(
        Style::default()
            .bg(Color::LightGreen)
            .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");
f.render_stateful_widget(items, chunks_list[0], &mut app.items.state);

let message = vec![
    Spans::from(Span::styled(
        "This is simple Tui Todo app",
        Style::default().fg(Color::Green),
    )),
];
let paragraph = Paragraph::new(message).alignment(Alignment::Center);
f.render_widget(paragraph, chunks[0]);
let block = Block::default().title("done").borders(Borders::ALL);
f.render_widget(block, chunks_list[1]);
let block = Block::default().title("doing").borders(Borders::ALL);
f.render_widget(block, chunks_list[0]);



}