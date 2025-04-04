use crate::app::ScreenType;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Help {
}

impl Help {
    pub fn new() -> Self {
        Help {}
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ].as_ref())
            .split(f.size());

        self.render_header(f, chunks[0]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return ScreenType::Home,
            _ => {}
        } 
        ScreenType::Help
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled("CLI LTR", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw("Help")
            ]),
        ];
        let header = Paragraph::new(text)
            .block(Block::default())
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
    
        f.render_widget(header, area);
    }
}
