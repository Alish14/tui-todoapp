
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
use super::structures::*;

// use crate::structures::{App, InputMode};
pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(5)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(size);
        
        let block = Block::default().style(Style::default().bg(Color::Rgb(255, 192, 203)).fg(Color::Black));
        f.render_widget(block, size);        
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
                    Span::raw("Press "),
                    Span::styled("Tab", Style::default().add_modifier(Modifier::BOLD)),
                    Span::raw(" to switch between lists."),
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
                let mut lines: Vec<Spans<'_>> = vec![Spans::from(i.as_str())];
                
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
                let mut lines: Vec<Spans<'_>> = vec![Spans::from(i.as_str())];
                ListItem::new(lines).style(Style::default().fg(Color::Black))
            })
            .collect();
        let input = Paragraph::new(app.input.as_ref())
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Input"));
    if InputMode::Editing == app.input_mode {
        f.render_widget(input, chunks[0]);
    }
    
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
