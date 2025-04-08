use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};
use crate::models::test::TestModel;

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Runner<'a> {
    item: Option<TestModel<'a>>,
    current_q_number: u16,
    current_q_text: &'a str,
    current_q_answers: Menu<'a>,
    answers_record: Vec<Option<usize>>,
    question_count: u16,
    show_summary: bool,
}

impl<'a> Runner<'a> {
    pub fn new(item: Option<TestModel<'a>>) -> Self {
         Runner {
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
        if self.is_running() {
            let chunks = layout::get_two_row_layout(f, 20);

            self.render_question(f, chunks[0]);
            self.render_answers(f, chunks[1]);
        } else {
            if self.show_summary {
                let chunks = layout::get_two_row_layout(f, 20);

                self.render_summary_header(f, chunks[0]);
            } else {
                let chunks = layout::get_two_row_layout(f, 20);

                self.render_test_name(f, chunks[0]);
                self.render_start_area(f, chunks[1]);
            }
        }
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if self.is_running() {
                    return ScreenType::Runner
                }
                return ScreenType::Tests;
            }
            // a safeguard
            // should have a confirmation dialog
            KeyCode::Char('P') => {
                if self.is_running() {
                    return ScreenType::Quit
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
            KeyCode::Enter => return self.handle_enter(),
            _ => {}
        } 
        ScreenType::Runner
    }

    fn start_test(&mut self) -> ScreenType {
        self.current_q_number = 1;
        self.current_q_text = "Lets imagine that you see your brother for the first time today and it is 1 pm. How do you greet him?";
        let answers_p: Vec<&str> = vec![
            "Good evening",
            "Good morning",
            "Hi, do we know each other?",
            "Yo bro, shouldn't you be in Buenos Aires right now?"
        ];
        self.current_q_answers = Menu::new(answers_p);
        self.answers_record = vec![];

        ScreenType::Runner
    }

    fn handle_enter(&mut self) -> ScreenType {
        if self.is_running() {
            self.answers_record.push(self.current_q_answers.state.selected());
            //println!("Chosen answer: {:?}", self.current_q_answers.state.selected());
            //println!("Answers record: {:#?}", self.answers_record);

            if self.current_q_number == self.question_count {
                // test over, show summary
                self.current_q_number = 0;
                self.show_summary = true;
            } else {
                // next question
                self.current_q_number += 1;
                self.current_q_text = "This is the ... I am telling you this!";
                self.current_q_answers = Menu::new(vec![
                    "current time",
                    "previous time",
                    "last time",
                    "any timme",
                ]);
            }
        }

        ScreenType::Runner
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
        let cols = layout::get_three_col_layout_rect(area, 60);
        let text = vec![
            Spans::from(vec![
                Span::styled("Test done.", Style::default().add_modifier(Modifier::BOLD)),
            ]),
            Spans::from(Span::raw("Here are your results:")),
        ];
        let header = layout::get_header(text);
        f.render_widget(header, area);
    }
}

