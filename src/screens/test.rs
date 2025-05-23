use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu, navbar, navbar::NavType};

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::Clear,
    Frame,
};
use crossterm::event::{KeyCode};
use rust_i18n::t;

pub struct Tests {
    pub first_render: bool,
    pub locale: String,
    items: Vec<(String, String)>,
    list: Menu,
}

impl Tests {
    pub fn new(items: Vec<(String, String)>, locale: String) -> Self {
        let names: Vec<String> = items.iter()
            .map(|t| t.1.clone())
            .collect();
        Tests { first_render: true, locale: locale, items: items, list: Menu::new(names) }
    }

    pub fn update_items(&mut self, items: Vec<(String, String)>) {
        let names: Vec<String> = items.iter()
            .map(|t| t.1.clone())
            .collect();

        self.items = items;
        self.list = Menu::new(names);
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }

        let background = layout::get_background();
        f.render_widget(background, f.size());
        
        let layout = layout::get_header_navbar_layout(f.size(), 3, 3);

        self.render_header(f, layout[0]);
        self.render_navbar(f, layout[1]);
        if self.items.len() == 0 {
            self.render_empty_items(f, layout[2]);
        } else {
            self.render_test_items(f, layout[2]);
        }
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
            Some(idx) => (ScreenType::Runner, self.items[idx].0.to_string()),
            None => (ScreenType::Tests, String::from(""))
        }
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled(t!("name.short", locale = &self.locale), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw(t!("title.tests", locale = &self.locale))
            ]),
        ];
        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);
    
        f.render_widget(header, header_area);
    }

    fn render_navbar<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let navbar_e = navbar::get_elements(vec![NavType::Back, NavType::Quit], self.locale.clone());
        let navbar = layout::get_navbar(navbar_e);
        let navbar_area = layout::get_default_column(area);

        f.render_widget(navbar, navbar_area);
    }

    fn render_test_items<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let list = layout::create_navigable_list(self.list.items.clone());
        let list_area = layout::get_adaptative_column(area);

        f.render_stateful_widget(list, list_area, &mut self.list.state);
    }

    fn render_empty_items<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled(t!("test.empty.header", locale = &self.locale), Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(Span::raw("")),
            Spans::from(Span::styled(t!("test.empty.info", locale = &self.locale), Style::default().add_modifier(Modifier::ITALIC))),
            Spans::from(Span::raw("")),
        ];
        let empty = layout::get_par_default(text);
        let empty_area = layout::get_default_column(area);

        f.render_widget(empty, empty_area);
    }
}

