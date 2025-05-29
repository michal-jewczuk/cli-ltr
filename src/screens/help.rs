use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu, navbar, navbar::NavType};

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

// state: 0 - default, 1 - lang switch, 2 - import switch, 3 - import progress, 4 - import done
pub struct Help {
    pub first_render: bool,
    pub locale: String,
    pub state: u8,
    pub import_results: Vec<String>,
    all_locales: Vec<(String, String)>,
    lang_name: String,
    langs: Menu,
}

impl Help {
    pub fn new(locale: String, all_locales: Vec<(String, String)>) -> Self {
        let lang_name = all_locales.iter()
            .find(|t| t.0 == locale).unwrap().1.clone();
        let names: Vec<String> = all_locales.iter()
            .map(|t| t.1.clone())
            .collect();
        // TODO set initial menu selected lang based on app lang
        Help {
            first_render: true, 
            locale, 
            state: 0,
            all_locales, 
            lang_name, 
            langs: Menu::new(names),
            import_results: vec![],
        }
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
            KeyCode::Char('b') | KeyCode::Char('B') => {
                self.state = 0;
                self.import_results = vec![];
                return (ScreenType::Home, self.locale.clone());
            },
            KeyCode::Char('c') | KeyCode::Char('C') => self.handle_lang_switch(),
            KeyCode::Char('i') | KeyCode::Char('I') => self.handle_import_switch(),
            KeyCode::Char('s') | KeyCode::Char('S') => return self.handle_import(),
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Up => {
                if self.state == 1 {
                    self.langs.previous();
                }
            },
            KeyCode::Down => {
                if self.state == 1 {
                    self.langs.next();
                }
            },
            KeyCode::Esc => {
                // TODO refactor once lang is saved on enter
                if self.state == 1 {
                    self.state = 0;
                    let loc_idx = self.get_locale_idx();
                    self.langs.state.select(loc_idx);
                }

                if self.state == 2 || self.state == 4 {
                    self.state = 0;
                }
            }
            _ => {}
        } 
        (ScreenType::Help, self.locale.clone())
    }

    fn handle_lang_switch(&mut self) {
        if self.state == 0 {
            self.state = 1;
        } else if self.state == 1 {
            self.state = 0;
        } else {
            return;
        }
    }

    fn handle_import_switch(&mut self) {
        if self.state == 0 {
            self.state = 2
        } else if self.state == 2 || self.state == 4 {
            self.state = 0;
        } else {
            return;
        }
    }

    fn handle_import(&mut self) -> (ScreenType, String) {
        if self.state != 2 {
            return (ScreenType::Help, String::from(""))
        }

        self.state = 3;
        (ScreenType::Importer, String::from(""))
    }

    // TODO have a similar solution as import to save config on Enter not on Back
    fn handle_enter(&mut self) {
        if self.state != 1 {
            return;
        }

        let idx = self.langs.state.selected().unwrap();
        let (code, name) = self.all_locales[idx].clone();
        self.locale = code;
        self.lang_name = name;
        self.state = 0;
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
        let navbar_e = navbar::get_elements(
            vec![NavType::Language, NavType::Import, NavType::Back, NavType::Quit], 
            self.locale.clone());
        let navbar = layout::get_navbar(navbar_e);
        let navbar_area = layout::get_default_column(area);

        f.render_widget(navbar, navbar_area);
    }

    fn render_content<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let content_area = layout::get_column_with_margin(area, 40, 120);
        let layout = layout::get_header_body_layout(content_area, 5);
        let selected_lang_text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::raw(t!("help.selected", locale = &self.locale)),
                Span::styled(&self.lang_name, Style::default().bg(Color::Blue).fg(Color::White).add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(Span::raw("")),
        ];
        let selected_lang = layout::get_par_with_colors(selected_lang_text, Color::White, Color::Black);
        f.render_widget(selected_lang, layout[0]);

        if self.state > 1 {
            self.render_import_area(f, layout[1]);
        } else {
            self.render_switch_area(f, layout[1]);
        }
    }

    fn render_import_area<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        if self.state == 2 {
            let import_start_text = vec![
                Spans::from(Span::raw("---------")),
                Spans::from(Span::raw("")),
                Spans::from(vec![
                    Span::raw(t!("import.welcome", locale = &self.locale)),
                ]),
                Spans::from(Span::raw("")),
            ];
            let import_start = layout::get_par_with_colors(import_start_text, Color::White, Color::Black);

            f.render_widget(import_start, area);
        } else if self.state == 4 {
            let mut import_finish_text = vec![
                Spans::from(Span::raw("---------")),
                Spans::from(Span::raw("")),
            ];
            self.import_results.iter()
                .map(|r| Spans::from(Span::styled(r, Style::default().add_modifier(Modifier::ITALIC))))
                .for_each(|l| import_finish_text.push(l));

            let import_finish = layout::get_par_with_colors(import_finish_text, Color::White, Color::Black);

            f.render_widget(import_finish, area);
        } else if self.state == 3 {
            let loading_text = vec![Spans::from(Span::raw("importing..."))];
            let loading = layout::get_par_with_colors(loading_text, Color::White, Color::Black);

            f.render_widget(loading, area);
        }
    }

    fn render_switch_area<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        if self.state == 0 {
            f.render_widget(Clear, area);
        } else if self.state == 1 {
            let lang_list = layout::create_navigable_list(self.langs.items.clone());
            
            f.render_stateful_widget(lang_list, area, &mut self.langs.state);
        }
    }

    fn get_locale_idx(&self) -> Option<usize> {
        let mut idx = 0;
        for locale in self.all_locales.iter() {
            if locale.0 == self.locale {
                return Some(idx);
            }
            idx += 1;
        }
        return None;
    }
}

