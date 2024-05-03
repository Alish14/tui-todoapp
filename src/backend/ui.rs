use tui::{
    backend::Backend,
    layout::{self, Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame,
};
use super::structures::*;

pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().bg(Color::Rgb(40, 42, 54)).fg(Color::White));
    f.render_widget(block, size);

    let chunks_down = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::LightYellow)),
                Span::styled(rest, Style::default().fg(Color::LightMagenta)),
            ])
        })
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title(" Day Todos "))
        .select(app.index)
        .style(Style::default().fg(Color::LightCyan))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Rgb(61, 61, 61)),
        );

    let (msg, style) = match app.input_mode {
        InputMode::Normal => (
            vec![
                Span::styled("  ", Style::default().fg(Color::LightYellow)),
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to start editing, "),
                Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to switch lists, "),
                Span::styled("D", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to delete, "),
                Span::styled("S", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to save."),
                Span::styled(" R", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to reset."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        InputMode::Editing => (
            vec![
                Span::styled("  ", Style::default().fg(Color::LightYellow)),
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to record."),
            ],
            Style::default(),
        ),
    };

    let mut text = Text::from(Spans::from(msg));
    text.patch_style(style);
    let help_message = Paragraph::new(text).alignment(Alignment::Left);
    f.render_widget(help_message, chunks[0]);
    f.render_widget(tabs, chunks_down[0]);

    let block = Block::default()
        .title(Spans::from(vec![
            Span::styled("  ", Style::default().fg(Color::LightYellow)),
            Span::styled(app.titles[app.index], Style::default().fg(Color::LightCyan)),
        ]))
        .borders(Borders::ALL);
    f.render_widget(block, chunks_down[1]);

    let chunks_list = layout::Layout::default()
        .direction(Direction::Horizontal)
        .constraints([layout::Constraint::Percentage(50), layout::Constraint::Percentage(50)].as_ref())
        .split(chunks_down[1]);

    let days_tasks_ref = app.days_tasks.get_mut(&app.titles[app.index]).unwrap();

    let item_done: Vec<ListItem> = days_tasks_ref
        .1
        .items_done_arr
        .iter()
        .map(|i| {
            let lines: Vec<Spans<'_>> = vec![Spans::from(i.as_str())];
            ListItem::new(lines).style(Style::default().fg(Color::LightGreen))
        })
        .collect();

    let items_done = List::new(item_done)
        .block(Block::default().borders(Borders::ALL).title(" Done "))
        .highlight_style(Style::default().bg(Color::Rgb(61, 61, 61)).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let items: Vec<ListItem> = days_tasks_ref
        .0
        .items
        .iter()
        .map(|i| {
            let lines: Vec<Spans<'_>> = vec![Spans::from(i.as_str())];
            ListItem::new(lines).style(Style::default().fg(Color::LightYellow))
        })
        .collect();

    let input: Paragraph = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::LightYellow),
        })
        .block(Block::default().borders(Borders::ALL).title(" Input "));

    if InputMode::Editing == app.input_mode {
        f.render_widget(input, chunks[0]);
    }
    match app.input_mode {
        InputMode::Normal =>
            {}

        InputMode::Editing => {
            f.set_cursor(
                chunks[0].x + app.input.len() as u16 + 1,
                chunks[0].y + 1,
            )
        }
    }

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" To Do "))
        .highlight_style(Style::default().bg(Color::Rgb(61, 61, 61)).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let test_0 = &mut days_tasks_ref.0.state;
    let test_1 = &mut days_tasks_ref.1.state;
    f.render_stateful_widget(items, chunks_list[0], test_0);

    let block = Block::default().title(" Done ").borders(Borders::ALL);
    f.render_widget(block, chunks_list[1]);

    let block = Block::default().title(" To Do ").borders(Borders::ALL);
    f.render_stateful_widget(items_done, chunks_list[1], test_1);

    f.render_widget(block, chunks_list[0]);
}