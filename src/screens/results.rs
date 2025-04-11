use crate::app::ScreenType;
use crate::ui::layout;

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Clear,
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Results {
    pub first_render: bool,
}

impl Results {
    pub fn new() -> Self {
        Results {first_render: true}
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }

        let chunks = layout::get_three_row_layout_rect(f.size(), 15, 10);

        self.render_header(f, chunks[0]);
        self.render_navbar(f, chunks[1]);
        self.render_content(f, chunks[2]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return ScreenType::Home,
            _ => {}
        } 
        ScreenType::Results
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled("CLI LTR", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw("Display results")
            ]),
        ];
        let header = layout::get_header(text);
    
        f.render_widget(header, area);
    }

    fn render_navbar<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![("[b]", " Home "), ("[q]", " Quit ")];
        let navbar = layout::get_navbar(text);

        f.render_widget(navbar, area);
    }

    fn render_content<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("Content")),
        ];
        let content = layout::get_header(text);

        f.render_widget(content, area);
    }

}

