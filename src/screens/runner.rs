use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};
use crate::models::test::TestModel;

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Runner<'a> {
    item: Option<TestModel<'a>>,
}

impl<'a> Runner<'a> {
    pub fn new(item: Option<TestModel<'a>>) -> Self {
         Runner {item}
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = layout::get_basic_layout(f);

        self.render_header(f, chunks[0]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return ScreenType::Home,
            _ => {}
        } 
        ScreenType::Runner
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled("CLI LTR", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw("TMP")
            ]),
        ];
        let header = layout::get_header(text);
    
        f.render_widget(header, area);
    }
}

