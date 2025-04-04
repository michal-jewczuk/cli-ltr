use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn get_basic_layout<B: Backend>(f: &mut Frame<B>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(5),
            Constraint::Percentage(85)
            ].as_ref())
        .split(f.size())
}

pub fn create_navigable_list<'a>(items: Vec<&'a str>) -> List<'a> {
    let list_items: Vec<ListItem> = items.iter()
        .map(|&i| ListItem::new(i))
        .collect();
    List::new(list_items)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
            .bg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
            )
        .highlight_symbol(">> ")
}

pub fn get_header(text: Vec<Spans>) -> Paragraph {
    Paragraph::new(text)
        .block(Block::default())
        .style(Style::default().fg(Color::White).bg(Color::DarkGray))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

pub fn get_navbar<'a>(text: Vec<(&'a str, &'a str)>) -> Paragraph<'a> {
    let mut nb = vec![
        get_navbar_element("[Navigation:]", false),
        Span::raw(" "),
    ];
    text.iter().for_each(|t| {
        nb.push(get_navbar_element(t.0, true));
        nb.push(get_navbar_element(t.1, false));
        nb.push(Span::styled(" // ", Style::default().add_modifier(Modifier::BOLD)));
    });
    nb.pop();
    let spans = vec![
        Spans::from(Span::raw("")),
        Spans::from(nb)
    ];
    Paragraph::new(spans)
        .block(Block::default())
        .style(Style::default().bg(Color::Magenta))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true })
}

fn get_navbar_element(text: &str, primary: bool) -> Span {
    if primary {
        Span::styled(text, Style::default().bg(Color::Black).fg(Color::White))
    } else {
        Span::styled(text, Style::default().bg(Color::White).fg(Color::Black))
    }
}

