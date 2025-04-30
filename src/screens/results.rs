use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu, navbar, navbar::NavType};
use crate::models::test::{AnswerModel, ResultModel};
use crate::service::testservice;

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

pub struct Results {
    pub first_render: bool,
    pub locale: String,
    item: Option<ResultModel>,
    show_details: bool,
    current_q_idx: usize,
    current_q: Option<AnswerModel>,
    count_q: usize,
    results_items: Vec<(String, String)>,
    results_list: Menu,
}

impl Results {
    pub fn new(item: Option<ResultModel>, locale: String) -> Self {
        let mut show_details = true;
        let mut count_q = 0;
        let results_items = testservice::get_results_list();
        match item {
            None => show_details = false,
            Some(ref r) => count_q = r.answers.len(),
        }
        Results {
            first_render: true,
            locale,
            item,
            show_details,
            current_q_idx: 0,
            current_q: None,
            count_q,
            results_items,
            results_list: Menu::new(vec![]),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            if self.show_details {
                self.handle_start();
            }

            self.results_list = Menu::new(self.results_items.clone().into_iter().map(|t| t.1).collect());
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }
        
        let background = layout::get_background();
        f.render_widget(background, f.size());

        if self.show_details {
            let layout = layout::get_header_body_layout(f.size(), 3);

            self.render_details_header(f, layout[0]);
            self.render_details_body(f, layout[1]);
        } else {
            self.render_list(f);
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if self.show_details {
                    self.show_details = false;
                    return ScreenType::Results;
                }
                return ScreenType::Home;
            },
            KeyCode::Right => self.handle_next(),
            KeyCode::Left => self.handle_previous(),
            KeyCode::Up => self.results_list.previous(),
            KeyCode::Down => self.results_list.next(),
            KeyCode::Enter => self.handle_enter(),
            _ => {}
        } 
        ScreenType::Results
    }

    fn handle_enter(&mut self) {
        if self.show_details {
            return;
        }

        match self.results_list.state.selected() {
            Some(idx) => {
                self.item = testservice::get_results_by_id(self.results_items[idx].0.clone());
                if self.item.is_some() {
                    self.handle_start();
                }
            },
            None => ()
        }
    }

    fn handle_start(&mut self) {
        self.current_q_idx = 0;
        self.current_q = Some(self.item.clone().unwrap().answers[0].clone());
        self.count_q = self.item.as_ref().unwrap().answers.len();
        self.show_details = true;
    }

    fn handle_next(&mut self) {
        if !self.show_details {
            return;
        }

        self.current_q_idx += 1;
        if self.current_q_idx > self.count_q {
            self.current_q_idx = self.count_q;
        }
        if self.current_q_idx < self.count_q {
            self.current_q = Some(self.item.clone().unwrap().answers[self.current_q_idx].clone());
        }
    }

    fn handle_previous(&mut self) {
        if !self.show_details {
            return;
        }

        if self.current_q_idx == 0 {
            self.current_q_idx = 0; 
        } else {
            self.current_q_idx -= 1;
        }
        self.current_q = Some(self.item.clone().unwrap().answers[self.current_q_idx].clone());
    }

    fn render_list<B: Backend>(&mut self, f: &mut Frame<B>) {
        let layout = layout::get_header_navbar_layout(f.size(), 3, 3);

        self.render_list_header(f, layout[0]);
        self.render_list_navbar(f, layout[1]);
        self.render_list_content(f, layout[2]);
    }

    fn render_list_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled(t!("name.short", locale = &self.locale), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" - "),
                Span::raw(t!("title.results", locale = &self.locale))
            ]),
        ];

        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);
    
        f.render_widget(header, header_area);
    }

    fn render_list_navbar<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let navbar_e = navbar::get_elements(vec![NavType::Back, NavType::Quit], self.locale.clone());
        let navbar = layout::get_navbar(navbar_e);
        let navbar_area = layout::get_default_column(area);

        f.render_widget(navbar, navbar_area);
    }

    fn render_list_content<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let content = layout::create_navigable_list(self.results_list.items.clone());
        let content_area = layout::get_adaptative_column(area);

        f.render_stateful_widget(content, content_area, &mut self.results_list.state);
    }

    fn render_details_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw(t!("results.for", locale = &self.locale))),
            Spans::from(Span::styled(self.item.clone().unwrap().title, Style::default().add_modifier(Modifier::BOLD))),
        ];
        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);
    
        f.render_widget(header, header_area);
    }

    fn render_details_body<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let q = self.current_q.clone().unwrap();

        let layout = layout::get_header_body_layout(area, 3);

        let navbar_b = navbar::get_elements(
            vec![NavType::Next, NavType::Previous, NavType::Back, NavType::Quit],
            self.locale.clone()
        );
        let navbar = layout::get_navbar(navbar_b);
        let navbar_area = layout::get_default_column(layout[0]);
        f.render_widget(navbar, navbar_area);

        let content_area = layout::get_column_with_margin(layout[1], 20, 150);
        if self.current_q_idx == self.count_q {
            let summary = layout::render_summary_table(self.item.clone().unwrap().answers, &self.locale);
            f.render_widget(summary, content_area);
        } else {
            let mut aidx: usize = 0;
            let answers_spans = q.answers.iter()
                .map(|a| {
                    let mut color = Color::Black;
                    if aidx == q.given.unwrap() {
                        color = Color::Red;
                    }
                    //if aidx == q.correct.into() {
                    if aidx == usize::from(q.correct) {
                        color = Color::Green;
                    }
                    aidx += 1;
                    Spans::from(Span::styled(
                            a.as_str(), 
                            Style::default().bg(color).fg(Color::White)))
                })
                .collect::<Vec<Spans>>();

            let answers_page = layout::get_results_q_page(self.current_q_idx + 1, 
                self.count_q, q.question, answers_spans, q.time, &self.locale);
            f.render_widget(answers_page, content_area);
        }
    }
}

