use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};
use crate::models::test::{
	   TestModel, QuestionModel,
	   ResultModel, AnswerModel,
        };

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::Clear,
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Runner<'a> {
    pub first_render: bool,
    item: Option<TestModel<'a>>,
    current_q_number: usize,
    current_q_text: &'a str,
    current_q_answers: Menu<'a>,
    answers_record: Vec<Option<usize>>,
    question_count: usize,
    show_summary: bool,
}

impl<'a> Runner<'a> {
    pub fn new(item: Option<TestModel<'a>>) -> Self {
         Runner {
             first_render: true,
             item, 
             current_q_number: 0,
             current_q_text: "",
             current_q_answers: Menu::new(vec![]),
             answers_record: vec![],
             question_count: 2,
             show_summary: false,
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

        if self.is_running() {
            let chunks = layout::get_two_row_layout(f, 20);

            self.render_question(f, chunks[0]);
            self.render_answers(f, chunks[1]);
        } else {
            if self.show_summary {
                let chunks = layout::get_three_row_layout_rect(f.size(), 10, 10);

                self.render_summary_header(f, chunks[0]);
                self.render_summary_navbar(f, chunks[1]);
		self.render_summary_body(f, chunks[2]);
            } else {
                let chunks = layout::get_two_row_layout(f, 20);

                self.render_test_name(f, chunks[0]);
                self.render_start_area(f, chunks[1]);
            }
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> (ScreenType, Option<String>) {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if self.is_running() {
                    return (ScreenType::Runner, None)
                }
                return (ScreenType::Tests, None);
            }
            // a safeguard
            // should have a confirmation dialog
            KeyCode::Char('P') => {
                if self.is_running() {
                    return (ScreenType::Quit, None);
                }
            },
            KeyCode::Char('s') | KeyCode::Char('S') => return self.start_test(),
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
                    return (ScreenType::Results, None);
                }
            },
            KeyCode::Enter => return self.handle_enter(),
            _ => {}
        } 
        (ScreenType::Runner, None)
    }

    fn start_test(&mut self) -> (ScreenType, Option<String>) {
        // TODO check when test has 0 questions
        // how to handle that so the user can see?
        // not allow to have that test shown on list?


        self.current_q_number = 1;
        self.show_summary = false;

        // should here be a None check?
        let tmp_test = self.item.clone().unwrap();
        self.current_q_text = tmp_test.questions[0].question;
        self.current_q_answers = Menu::new(tmp_test.questions[0].answers.clone());
        self.answers_record = vec![];

        (ScreenType::Runner, None)
    }

    fn handle_enter(&mut self) -> (ScreenType, Option<String>) {
        if self.is_running() {
            self.answers_record.push(self.current_q_answers.state.selected());

            if self.current_q_number == self.question_count {
                // test over, show summary
                self.current_q_number = 0;
                self.show_summary = true;
            } else {
                // next question
                let tmp_test = self.item.clone().unwrap();
                self.current_q_text = tmp_test.questions[self.current_q_number].question;
                self.current_q_answers = Menu::new(tmp_test.questions[self.current_q_number].answers.clone());
                self.current_q_number += 1;
            }
        }
        (ScreenType::Runner, None)
    }

    fn render_test_name<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let title = self.item.as_ref().map_or("42", |i| i.title);
        let text = vec![
            Spans::from(Span::raw("")),
            Spans::from(Span::raw("---=====---")),
            Spans::from(vec![
                Span::styled(title, Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(Span::raw("---=====---")),
        ];
        let header = layout::get_header(text);
    
        f.render_widget(header, area);
    }

    fn render_start_area<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let cols = layout::get_three_col_layout_rect(area, 60);
        let chunks = layout::get_three_row_layout_rect(cols[1], 15,15);

        let instruction = vec![
            Spans::from(Span::raw("Do you want to start the test?")),
            Spans::from(vec![
                Span::styled("Please note: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("once started, the test must be finished!"),
            ])
        ];
        let instruction_a = layout::get_par_with_colors(instruction, Color::White, Color::Blue);
        f.render_widget(instruction_a, chunks[0]);

        let text = vec![("[s]", " Start "), ("[b]", " Back "), ("[q]", " Quit ")];
        let start_buttons = layout::get_test_start_row(text);
        f.render_widget(start_buttons, chunks[1]);
    }

    fn render_question<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let cols = layout::get_three_col_layout_rect(area, 60);
        let question_l = layout::render_question(self.current_q_text); 

        f.render_widget(question_l, cols[1]);
    }

    fn render_answers<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let cols = layout::get_three_col_layout_rect(area, 60);
        let answers_l = layout::create_navigable_list(self.current_q_answers.items.clone());

        f.render_stateful_widget(answers_l, cols[1], &mut self.current_q_answers.state);
    }

    fn render_summary_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(vec![
                Span::styled("Test summary", Style::default().add_modifier(Modifier::BOLD)),
            ]),
        ];
        let header = layout::get_header(text);
        f.render_widget(header, area);
    }

    fn render_summary_navbar<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![("[d]", " Detailed results "), ("[b]", " Back to tests "), ("[q]", " Quit ")];
        let navbar = layout::get_navbar(text);

        f.render_widget(navbar, area);
    }

    fn render_summary_body<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
	// will be created after every question answered
	let q_list = self.item.clone().unwrap().questions;
	let mut answer_list: Vec<AnswerModel> = vec![];
	for i in 0..self.question_count {
	    let q_text = q_list[i].question;
	    let correct_a = q_list[i].correct;
	    let given = self.answers_record[i];
	    let is_correct = q_list[i].is_correct(self.answers_record[i]);
	    let time = 10;
	    let answer = AnswerModel::new(q_text, correct_a, given,
		is_correct, time);
	    answer_list.push(answer);
	}		

        let cols = layout::get_three_col_layout_rect(area, 60);
        let table = layout::render_summary_table(answer_list.clone());
	f.render_widget(table, cols[1]);
    }
}

