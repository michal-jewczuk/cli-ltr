use crate::app::ScreenType;
use crate::ui::{layout, navbar, navbar::NavType};

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Clear,
    Frame,
};
use crossterm::event::{KeyCode};
use rust_i18n::t;

pub struct Help {
    pub first_render: bool,
    pub locale: String,
    all_locales: Vec<(String, String)>,
    lang_name: String,
}

impl Help {
    pub fn new(locale: String, all_locales: Vec<(String, String)>) -> Self {
        let lang_name = all_locales.iter()
            .find(|t| t.0 == locale).unwrap().1.clone();
        Help {first_render: true, locale, all_locales, lang_name}
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
        self.render_content(f, layout[2]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> (ScreenType, String) {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return (ScreenType::Home, self.locale.clone()),
            KeyCode::Char('c') | KeyCode::Char('C') => self.handle_lang_switch(),
            _ => {}
        } 
        (ScreenType::Help, self.locale.clone())
    }

    fn handle_lang_switch(&mut self) {
        let (code, name) = self.get_new_lang();
        self.locale = code;
        self.lang_name = name;
    }

    fn get_new_lang(&self) -> (String, String) {
        // TODO update logic to use available langs and menu selection
        match self.locale.as_str() {
            "en" => (String::from("pl"), String::from("Polski")),
            "pl" => (String::from("en"), String::from("English")),
            _ => (String::from("en"), String::from("English")),
        }
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled(t!("name.short", locale = &self.locale), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw(t!("title.help", locale = &self.locale))
            ]),
        ];
        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);
    
        f.render_widget(header, header_area);
    }

    fn render_navbar<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let navbar_e = navbar::get_elements(vec![NavType::Language, NavType::Back, NavType::Quit], self.locale.clone());
        let navbar = layout::get_navbar(navbar_e);
        let navbar_area = layout::get_default_column(area);

        f.render_widget(navbar, navbar_area);
    }

    fn render_content<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let content_area = layout::get_default_column(area);
        let selected_lang_text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::raw(t!("help.selected", locale = &self.locale)),
                Span::styled(&self.lang_name, Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(Span::raw("")),
        ];
        let selected_lang = layout::get_par_with_colors(selected_lang_text, Color::White, Color::Black);
        f.render_widget(selected_lang, content_area);
    }
}

