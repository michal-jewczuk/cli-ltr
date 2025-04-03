
use crate::ui::menu::Menu;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};


pub fn ui<B: Backend>(f: &mut Frame<B>, menu: &mut Menu) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());

    render_header(f, chunks[0]);
    render_menu(f, chunks[1], menu);
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

fn render_menu<B: Backend>(f: &mut Frame<B>, area: Rect, menu: &mut Menu) {
    let items: Vec<ListItem> = menu.items.iter()
        .map(|&i| ListItem::new(i))
        .collect();
    let list = List::new(items)
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .bg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
            )
        .highlight_symbol(">> ");
    let x = area.width / 2 - 10;
    let y = area.y + 2;
    let menu_area = Rect::new(x, y, 20, 10);

    f.render_stateful_widget(list, menu_area, &mut menu.state);
}

