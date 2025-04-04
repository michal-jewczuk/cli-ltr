use crate::app::App;
use crate::ui::menu::Menu;

use std::io;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{KeyCode};


pub fn draw<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(15), 
            Constraint::Percentage(15), 
            Constraint::Percentage(70)
        ].as_ref())
        .split(f.size());

    let mut menu = Menu::home();

    render_header(f, chunks[0]);
    render_menu_instructions(f, chunks[1]);
    render_menu(f, chunks[2], &mut menu);
}

pub fn handle_key_code(app: &mut App, code: KeyCode) -> Result<(), io::Error> {
    match code {
        KeyCode::Up => println!("Up pressed"),
        KeyCode::Down => println!("Down pressed"),
        KeyCode::Enter => println!("Enter pressed"),
        _ => {}
    }
    Ok(())
}

fn render_header<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let header_border = Spans::from(Span::raw("==========================="));
    let text = vec![
        Spans::from(Span::raw("")),
        header_border.clone(),
        Spans::from(Span::raw("")),
        Spans::from(vec![
            Span::raw("WELCOME TO "),
            Span::styled("CLI Language Test Runner", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Spans::from(Span::raw("")),
        header_border.clone(),
    ];
    let header = Paragraph::new(text)
        .block(Block::default())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(header, area);
}

fn render_menu_instructions<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let text = vec![
        Spans::from(Span::raw("")),
        Spans::from(Span::raw("You can use UP or DOWN arrows to navigate through menu items and ENTER to confirm")),
        Spans::from(Span::raw("or you can use the specified key shortcut to instantly confirm")),
    ];
    let instructions = Paragraph::new(text)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(instructions, area);
}

fn render_menu<B: Backend>(f: &mut Frame<B>, area: Rect, menu: &mut Menu) {
    let items: Vec<ListItem> = menu.items.iter()
        .map(|&i| ListItem::new(i))
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .bg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
            )
        .highlight_symbol(">> ");
    let x = area.width / 2 - 10;
    let y = area.y + 2;
    let menu_area = Rect::new(x, y, 20, 7);

    f.render_stateful_widget(list, menu_area, &mut menu.state);
}

