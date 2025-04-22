use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Clear,
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Tests {
    pub first_render: bool,
    items: Vec<(String, String)>,
    list: Menu,
}

impl Tests {
    pub fn new(items: Vec<(String, String)>) -> Self {
        let names: Vec<String> = items.iter()
            .map(|t| t.1.clone())
            .collect();
        Tests { first_render: true, items: items, list: Menu::new(names) }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }
        
        let chunks = layout::get_basic_layout(f);

        self.render_header(f, chunks[0]);
        self.render_navbar(f, chunks[1]);
        self.render_test_items(f, chunks[2]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> (ScreenType, String) {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return (ScreenType::Home, String::from("")),
            KeyCode::Up => self.list.previous(),
            KeyCode::Down => self.list.next(),
            KeyCode::Enter => return self.handle_enter(),
            _ => {}
        } 
        (ScreenType::Tests, String::from("")) 
    }

    fn handle_enter(&mut self) -> (ScreenType, String) {
        match self.list.state.selected() {
            Some(idx) => {
                (ScreenType::Runner, self.items[idx].0.to_string())
            },
            None => (ScreenType::Tests, String::from(""))
        }
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
        let list_area = layout::get_adaptative_column(area);

        f.render_stateful_widget(list, list_area, &mut self.list.state);
    }
}

