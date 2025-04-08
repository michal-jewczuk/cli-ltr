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
    current_question: u16,
}

impl<'a> Runner<'a> {
    pub fn new(item: Option<TestModel<'a>>) -> Self {
         Runner {item, current_question: 0}
    }

    pub fn is_running(&self) -> bool {
        if self.current_question == 0 {
            false
        } else {
            true
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.is_running() {
            println!("I am running...");
        } else {
            let chunks = layout::get_two_row_layout(f, 20);

            self.render_test_name(f, chunks[0]);
            self.render_start_area(f, chunks[1]);
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
            KeyCode::Enter => return self.handle_enter(),
            _ => {}
        } 
        ScreenType::Runner
    }

    fn start_test(&mut self) -> ScreenType {
        self.current_question = 1;

        ScreenType::Runner
    }

    fn handle_enter(&mut self) -> ScreenType {
        // 2) if running - show nex question
        // 3) if last question - show summary

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
        //let chunks = layout::get_two_row_layout_rect(area, 15);
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
}

