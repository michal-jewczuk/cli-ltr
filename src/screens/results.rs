use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};
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

pub struct Results {
    pub first_render: bool,
    item: Option<ResultModel>,
    show_details: bool,
    current_q_idx: usize,
    current_q: Option<AnswerModel>,
    count_q: usize,
    results_items: Vec<(String, String)>,
    results_list: Menu,
}

impl Results {
    pub fn new(item: Option<ResultModel>) -> Self {
        let mut show_details = true;
        let mut count_q = 0;
        let results_items = testservice::get_results_list();
        match item {
            None => show_details = false,
            Some(ref r) => count_q = r.answers.len(),
        }
        Results {
            first_render: true,
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
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }

        if self.show_details {
            let chunks = layout::get_two_row_layout_rect(f.size(), 10);

            self.render_details_header(f, chunks[0]);
            self.render_details_body(f, chunks[1]);
        } else {
            self.render_results_list(f);
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return ScreenType::Home,
            KeyCode::Right => self.handle_next(),
            KeyCode::Left => self.handle_previous(),
            _ => {}
        } 
        ScreenType::Results
    }

    fn handle_start(&mut self) {
        self.current_q_idx = 0;
        self.current_q = Some(self.item.clone().unwrap().answers[0].clone());
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

    fn render_results_list<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = layout::get_three_row_layout_rect(f.size(), 15, 10);

        self.render_header(f, chunks[0]);
        self.render_navbar(f, chunks[1]);
        self.render_content(f, chunks[2]);
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
        let content = layout::create_navigable_list(self.results_list.items.clone());

        f.render_widget(content, area);
    }

    fn render_details_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("Results for:")),
            Spans::from(Span::styled(self.item.clone().unwrap().title, Style::default().add_modifier(Modifier::BOLD))),
        ];
        let header = layout::get_header(text);
    
        f.render_widget(header, area);
    }

    fn render_details_body<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let q = self.current_q.clone().unwrap();

        let cols = layout::get_three_col_layout_rect(area, 80);
        let chunks = layout::get_two_row_layout_rect(cols[1], 10);

        let navbar_b = vec![
            ("[RIGHT]", " Next "), ("[LEFT]", " Previous "), ("[b]", " Back to list "), ("[q]", " Quit "),
        ];
        let navbar = layout::get_navbar(navbar_b);
        f.render_widget(navbar, chunks[0]);

        if self.current_q_idx == self.count_q {
            let summary = layout::render_summary_table(self.item.clone().unwrap().answers);
            f.render_widget(summary, chunks[1]);
        } else {
            let mut aidx = 0;
            let answers_spans = q.answers.iter()
                .map(|a| {
                    let mut color = Color::Black;
                    if aidx == q.given.unwrap() {
                        color = Color::Red;
                    }
                    if aidx == q.correct.into() {
                        color = Color::Green;
                    }
                    aidx += 1;
                    Spans::from(Span::styled(
                            a.as_str(), 
                            Style::default().bg(color).fg(Color::White)))
                })
                .collect::<Vec<Spans>>();

            let answers_page = layout::get_results_q_page(self.current_q_idx + 1, self.count_q, q.question, answers_spans, q.time);
            f.render_widget(answers_page, chunks[1]);
        }
    }
}

