use crate::app::ScreenType;
use crate::ui::layout;
use crate::models::test::ResultModel;

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
}

impl Results {
    pub fn new(item: Option<ResultModel>) -> Self {
        let mut show_details = true;
        match item {
            None => show_details = false,
            Some(_) => show_details = true,
        }
        Results {
            first_render: true,
            item,
            show_details,
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }

        let mut chunks = vec![]; 
        if self.show_details {
            chunks = layout::get_two_row_layout_rect(f.size(), 10);

            self.render_details_header(f, chunks[0]);
            self.render_details_body(f, chunks[1]);
        } else {
            chunks = layout::get_three_row_layout_rect(f.size(), 15, 10);

            self.render_header(f, chunks[0]);
            self.render_navbar(f, chunks[1]);
            self.render_content(f, chunks[2]);
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => return ScreenType::Home,
            _ => {}
        } 
        ScreenType::Results
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let mut text = vec![
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
            Spans::from(Span::raw("List with finished tests")),
        ];
        let content = layout::get_header(text);

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
        let q = self.item.clone().unwrap().answers[0].clone();
        let content = layout::render_result_step_q(q.question.as_str());

        let cols = layout::get_three_col_layout_rect(area, 80);
        let chunks = layout::get_three_row_layout_rect(cols[1], 10, 30);

        f.render_widget(content, chunks[1]);

        let answers_spans = q.answers.iter()
            .map(|a| Spans::from(Span::from(a.as_str())))
            .collect::<Vec<Spans>>();

        let answers_p = layout::get_header(answers_spans);

        f.render_widget(answers_p, chunks[2]);
    }
}

