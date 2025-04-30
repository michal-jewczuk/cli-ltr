use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu, navbar, navbar::NavType};
use crate::models::test::{TestModel, ResultModel, AnswerModel};

use std::time::{Instant};

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

pub struct Runner<'a> {
    pub first_render: bool,
    pub locale: String,
    pub origin: ScreenType,
    item: Option<TestModel<'a>>,
    current_q_number: usize,
    current_q_text: &'a str,
    current_q_answers: Menu,
    result: ResultModel,
    question_count: usize,
    show_summary: bool,
    timer_t: Instant,
    timer_q: Instant,
}

impl<'a> Runner<'a> {
    pub fn new(item: Option<TestModel<'a>>, locale: String) -> Self {
         let mut question_count = 0; 
         if !item.is_none() {
             question_count = item.clone().unwrap().questions.len();
         }
         Runner {
             first_render: true,
             locale: locale,
             item, 
             current_q_number: 0,
             current_q_text: "",
             current_q_answers: Menu::new(vec![]),
             result: ResultModel::new(String::from(""), String::from(""), vec![], 0),
             question_count, 
             show_summary: false,
             timer_t: Instant::now(),
             timer_q: Instant::now(),
             origin: ScreenType::Tests,
         }
    }

    pub fn is_running(&self) -> bool {
        if self.current_q_number == 0 {
            false
        } else {
            true
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

        if self.is_running() {
            self.render_question_page(f);
        } else {
            if self.show_summary {
                let layout = layout::get_header_navbar_layout(f.size(), 3, 3);

                self.render_summary_header(f, layout[0]);
                self.render_summary_navbar(f, layout[1]);
		self.render_summary_body(f, layout[2]);
            } else {
                let layout = layout::get_header_body_layout(f.size(), 3);

                self.render_test_name(f, layout[0]);
                self.render_start_area(f, layout[1]);
            }
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> (ScreenType, Option<ResultModel>) {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if self.is_running() {
                    return (ScreenType::Runner, None)
                }
                return (self.origin.clone(), None);
            }
            // a safeguard
            // should have a confirmation dialog
            KeyCode::Char('P') => {
                if self.is_running() {
                    return (ScreenType::Quit, None);
                }
            },
            KeyCode::Char('s') | KeyCode::Char('S') => {
                if !self.is_running() && !self.show_summary {
                    return self.start_test()
                }
            },
            KeyCode::Up => {
                if self.is_running() {
                    self.current_q_answers.previous()
                }
            },
            KeyCode::Down => {
                if self.is_running() {
                    self.current_q_answers.next()
                }
            },
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if self.show_summary {
                    self.show_summary = false;
                    return (ScreenType::Results, Some(self.result.clone()));
                }
            },
            KeyCode::Enter => return self.handle_enter(),
            _ => {}
        } 
        (ScreenType::Runner, None)
    }

    fn start_test(&mut self) -> (ScreenType, Option<ResultModel>) {
        // TODO check when test has 0 questions
        // how to handle that so the user can see?
        // not allow to have that test shown on list?

        let test_m = self.item.clone().unwrap();

        self.timer_t = Instant::now();
        self.timer_q = Instant::now();
        self.current_q_number = 1;
        self.show_summary = false;
        self.result = ResultModel::new(
            String::from(test_m.id),
            String::from(test_m.title),
            vec![], 
            0
            );

        // should here be a None check?
        let tmp_test = self.item.clone().unwrap();
        self.current_q_text = tmp_test.questions[0].question;
        let answers_list: Vec<String> = tmp_test.questions[0].answers.clone().iter()
            .map(|&i| String::from(i)).collect();
        self.current_q_answers = Menu::new(answers_list);

        (ScreenType::Runner, None)
    }

    fn handle_enter(&mut self) -> (ScreenType, Option<ResultModel>) {
        if self.is_running() {
            let q = self.item.clone().unwrap().questions[self.current_q_number - 1].clone();
            let answers = q.answers.iter()
                .map(|&a| String::from(a))
                .collect::<Vec<String>>();
            let answer = AnswerModel::new(
                    String::from(self.current_q_text),
                    answers,
                    q.correct,
                    self.current_q_answers.state.selected(),
                    q.is_correct(self.current_q_answers.state.selected()),
                    self.timer_q.elapsed().as_secs() 
                ); 
            self.result.answers.push(answer);

            if self.current_q_number == self.question_count {
                self.current_q_number = 0;
                self.show_summary = true;
                self.result.total_time = self.timer_t.elapsed().as_secs();

            } else {
                let tmp_test = self.item.clone().unwrap();
                self.current_q_text = tmp_test.questions[self.current_q_number].question;
                let answers_list: Vec<String> = tmp_test.questions[self.current_q_number].answers.clone().iter()
                    .map(|&i| String::from(i)).collect();
                self.current_q_answers = Menu::new(answers_list);
                self.current_q_number += 1;
                self.timer_q = Instant::now();
            }
        }
        (ScreenType::Runner, None)
    }

    fn render_question_page<B: Backend>(&mut self, f: &mut Frame<B>) {
        let layout = layout::get_two_row_layout(f.size(), 40);

        self.render_question(f, layout[0]);
        self.render_answers(f, layout[1]);
    }

    fn render_test_name<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let title = self.item.as_ref().map_or("42", |i| i.title);
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
            ]),
        ];
        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);
    
        f.render_widget(header, header_area);
    }

    fn render_start_area<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let start_area = layout::get_default_column(area);
        let layout = layout::get_header_navbar_layout(start_area, 4, 3);

        let instruction = vec![
            Spans::from(Span::raw("")),
            Spans::from(Span::raw(t!("runner.start", locale = &self.locale))),
            Spans::from(vec![
                Span::styled(t!("runner.note.l1", locale = &self.locale), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(t!("runner.note.l2", locale = &self.locale)),
            ])
        ];
        let instruction_p = layout::get_par_with_colors(instruction, Color::White, Color::Blue);
        f.render_widget(instruction_p, layout[0]);

        let start_e = navbar::get_elements(vec![NavType::Start, NavType::Back, NavType::Quit], self.locale.clone());
        let start_buttons = layout::get_test_start_row(start_e);
        f.render_widget(start_buttons, layout[1]);
    }

    fn render_question<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let q_time = self.timer_q.elapsed().as_secs();
        let t_time = self.timer_t.elapsed().as_secs();
        let question_l = layout::get_question_area(
            self.current_q_text, self.current_q_number, self.question_count, q_time, t_time,
            ); 

        let q_area = layout::get_column_with_margin(area, 30, 150);

        f.render_widget(question_l, q_area);
    }

    fn render_answers<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let answers_l = layout::create_navigable_list(self.current_q_answers.items.clone());
        let a_area = layout::get_column_with_margin(area, 30, 150);

        f.render_stateful_widget(answers_l, a_area, &mut self.current_q_answers.state);
    }

    fn render_summary_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(vec![
                Span::styled("Test summary", Style::default().add_modifier(Modifier::BOLD)),
            ]),
        ];
        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);

        f.render_widget(header, header_area);
    }

    fn render_summary_navbar<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let navbar_e = navbar::get_elements(vec![NavType::Details, NavType::Back, NavType::Quit], self.locale.clone());
        let navbar = layout::get_navbar(navbar_e);
        let navbar_area = layout::get_default_column(area);

        f.render_widget(navbar, navbar_area);
    }

    fn render_summary_body<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let table = layout::render_summary_table(self.result.clone().answers);
        let table_area = layout::get_default_column(area);

	f.render_widget(table, table_area);
    }
}

