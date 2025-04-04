use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Tests<'a> {
    items: Vec<(&'a str, &'a str)>,
    list: Menu<'a>,
}

impl<'a> Tests<'a> {
    pub fn new(items: Vec<(&'a str, &'a str)>) -> Self {
        let names: Vec<&str> = items.iter()
            .map(|t| t.1)
            .collect();
        Tests { items: items, list: Menu::new(names) }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = layout::get_basic_layout(f);

        self.render_header(f, chunks[0]);
        self.render_navbar(f, chunks[1]);
        self.render_test_items(f, chunks[2]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return ScreenType::Home,
            KeyCode::Up => self.list.previous(),
            KeyCode::Down => self.list.next(),
            _ => {}
        } 
        ScreenType::Tests
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled("CLI LTR", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw("Run tests")
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

    fn render_test_items<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let list = layout::create_navigable_list(self.list.items.clone());
        let mut x = 0;
        let mut x_w = area.width;
        if area.width > 100 {
            x = area.width / 2 - 50;
            x_w = 100; 
        }
        let list_area = Rect::new(x, area.y, x_w, area.height);
        f.render_stateful_widget(list, list_area, &mut self.list.state);
    }
}

