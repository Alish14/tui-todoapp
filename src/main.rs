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
enum InputMode {
    Normal,
    Editing,
}

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}
struct StatefulListDone<T> {
    state: ListState,
    items_done_arr: Vec<T>,
}
impl <T>StatefulListDone<T>{
    fn with_items(items: Vec<T>) -> StatefulListDone<T> {
        StatefulListDone {
            state: ListState::default(),
            items_done_arr:items,
        }
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items_done_arr.len() - 1 {
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
                    self.items_done_arr.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn add_task(&mut self,task: T){
        self.items_done_arr.push(task);
    }
    fn unselect(&mut self) {
        self.state.select(None);
    }
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
    fn task_done(&mut self,other: &mut StatefulListDone<T>){
            other.items_done_arr.push(self.items.remove(self.state.selected().unwrap()));

       
        let i: usize = match self.state.selected(){
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
    items: StatefulList<(&'a str)>,
    items_done: StatefulListDone<(&'a str)>,
    input_mode: InputMode,
    input: String,
    messages: Vec<String>,
}
impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            titles: vec!["day1", "day2", "day3", "day4","day5","day6","day7"],
            index: 0,
            messages: Vec::new(),
            input: String::new(),
            input_mode: InputMode::Normal,
            items: StatefulList::with_items(vec![
                "task1","task2","task3"
            ]),
            items_done: StatefulListDone::with_items(vec![]),
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
                match app.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('e') => app.input_mode = InputMode::Editing,
                        KeyCode::Down => app.items.next(),
                        KeyCode::Right => app.next(),
                        KeyCode::Left => app.previous(),
                        KeyCode::Up => app.items.previous(),
                        KeyCode::Char('s') => app.items.unselect(),
                        KeyCode::Enter => app.items.task_done(&mut app.items_done),
                        _ => {}
                    },
                    InputMode::Editing => match key.code {
                        KeyCode::Esc => app.input_mode = InputMode::Normal,
                        KeyCode::Enter => {
                            app.messages.push(app.input.drain(..).collect());
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
fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(size);
    
    let block = Block::default().style(Style::default().bg(Color::Rgb((255), (192), (203))).fg(Color::Black));
    f.render_widget(block, size);
    let text = vec![
        Spans::from(Span::styled(
            "enter q for quit and use arrow key for change tabs",
            Style::default().fg(Color::Red),
        )),
        ];
        
        let chunks_down = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(120)].as_ref())
        .split(chunks[1]);
        
        let block = Block::default()
        .style(Style::default().bg(Color::Rgb(255, 192, 203)).fg(Color::Black));
        f.render_widget(block, chunks[0]);
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
    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record the message"),
            ],
            Style::default(),
        ),
    };
let mut text = Text::from(Spans::from(msg));
text.patch_style(style);
let help_message = Paragraph::new(text).alignment(Alignment::Center);
f.render_widget(help_message, chunks[1]);
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

    let item_done: Vec<ListItem> = app
        .items_done
        .items_done_arr
        .iter()
        .map(|i| {
            let mut lines: Vec<Spans<'_>> = vec![Spans::from()];
                lines.push(Spans::from(Span::styled(
                    app.messages.clone(),
                    Style::default().add_modifier(Modifier::ITALIC),
                )));
            
            ListItem::new(lines).style(Style::default().fg(Color::Black))
        })
        .collect();
    let items_done = List::new(item_done)
    .block(Block::default().borders(Borders::ALL).title("Done"))
    .highlight_style(
        Style::default()
        .bg(Color::LightGreen)
        .add_modifier(Modifier::BOLD),

    )
    .highlight_symbol(">> ");
    let items: Vec<ListItem> = app
    .items
        .items
        .iter()
        .map(|i| {
            let mut lines = vec![Spans::from(*i.to_string())];
                // lines.push(Spans::from(Span::styled(
                //     "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                //     Style::default().add_modifier(Modifier::ITALIC),
                // )));
            
            ListItem::new(lines).style(Style::default().fg(Color::Black))
        })
        .collect();
    let input = Paragraph::new(app.input.as_ref())
    .style(match app.input_mode {
        InputMode::Normal => Style::default(),
        InputMode::Editing => Style::default().fg(Color::Yellow),
    })
    .block(Block::default().borders(Borders::ALL).title("Input"));
f.render_widget(input, chunks[0]);
// match app.input_mode {
//     InputMode::Normal =>
//         {}

//     InputMode::Editing => {
//         f.set_cursor(
//             chunks[1].x + app.input.width() as u16 + 1,e
//             chunks[1].y + 1,
//         )
//     }
    let items = List::new(items)
    .block(Block::default().borders(Borders::ALL).title("List"))
    .highlight_style(
        Style::default()
        .bg(Color::LightGreen)
        .add_modifier(Modifier::BOLD),
    )
    .highlight_symbol(">> ");

let message = vec![
    Spans::from(Span::styled(
        "This is simple Tui Todo app",
        Style::default().fg(Color::Green),
    )),
    ];
    f.render_stateful_widget(items, chunks_list[0], &mut app.items.state);
    let paragraph = Paragraph::new(message).alignment(Alignment::Center);
    f.render_widget(paragraph, chunks[0]);
    let block = Block::default().title("done").borders(Borders::ALL);
    f.render_widget(block, chunks_list[1]);
    let block = Block::default().title("doing").borders(Borders::ALL);
    f.render_stateful_widget(items_done, chunks_list[1], &mut app.items_done.state);
    f.render_widget(block, chunks_list[0]);



}