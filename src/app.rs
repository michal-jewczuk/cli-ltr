
use crate::screens::{help, home, rerun, results, runner, test};
use crate::service::testservice;

use std::io;
use tui::{
    backend::Backend,
    Frame
};
use crossterm::event::{Event, KeyCode, KeyEvent};


pub enum ScreenType {
    Home,
    Tests,
    Results,
    Rerun,
    Help,
    Quit,
    Runner,
}

pub struct App<'a> {
    is_finished: bool,
    current_screen: ScreenType,
    home: home::Home<'a>,
    tests: test::Tests<'a>,
    results: results::Results<'a>,
    rerun: rerun::Rerun,
    help: help::Help,
    runner: runner::Runner<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
	let tests_to_do = testservice::get_to_do();
        App { 
            is_finished: false,
            current_screen: ScreenType::Home,
            home: home::Home::new(),
            tests: test::Tests::new(tests_to_do),
            results: results::Results::new(None),
            rerun: rerun::Rerun::new(),
            help: help::Help::new(),
            runner: runner::Runner::new(None),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        match self.current_screen {
            ScreenType::Home => self.home.draw(f),
            ScreenType::Tests => self.tests.draw(f),
            ScreenType::Results => self.results.draw(f),
            ScreenType::Rerun => self.rerun.draw(f),
            ScreenType::Help => self.help.draw(f),
            ScreenType::Runner => self.runner.draw(f),
            ScreenType::Quit => self.is_finished = true,
        }
    }

    pub fn is_done(&self) -> bool {
        self.is_finished
    }

    pub fn on_terminal_event(&mut self, event: Event) -> Result<(), io::Error> {
        match event {
            Event::Key(ev) => self.on_key_event(ev)?,
            _ => {}
        }
        Ok(())
    }

    fn on_key_event(&mut self, event: KeyEvent) -> Result<(), io::Error> {
        match event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                // will not work while test is on
                if !self.runner.is_running() {
                    self.is_finished = true
                }
            },
            other_code => self.handle_key_code(other_code)?
        }
        Ok(())
    }

    fn handle_key_code(&mut self, code: KeyCode) -> Result<(), io::Error> {
        match self.current_screen {
            ScreenType::Home => self.current_screen = self.home.handle_key_code(code),
            ScreenType::Tests => {
                let (screen, test_id) = self.tests.handle_key_code(code);
                match screen {
                    ScreenType::Runner => {
                        let test_model = testservice::get_by_id(test_id);
                        self.runner = runner::Runner::new(test_model);
                        self.current_screen = ScreenType::Runner;
                    },
                    _ => self.current_screen = screen 
                }
            },
            ScreenType::Results => self.current_screen = self.results.handle_key_code(code),
            ScreenType::Rerun => self.current_screen = self.rerun.handle_key_code(code),
            ScreenType::Runner => {
                let (screen, result) = self.runner.handle_key_code(code);
                match screen {
                    ScreenType::Results => {
                        self.results = results::Results::new(result.clone());
                        self.current_screen = ScreenType::Results
                    },
                    _ => self.current_screen = screen
                }
            },
            ScreenType::Help => self.current_screen = self.help.handle_key_code(code),
            _ => {}
        }
        Ok(())
    }
}

