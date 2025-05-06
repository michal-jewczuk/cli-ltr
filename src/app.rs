
use crate::screens::{help, home, rerun, results, runner, test};
use crate::service::testservice;

use std::io;
use tui::{
    backend::Backend,
    Frame
};
use crossterm::event::{Event, KeyCode, KeyEvent};

use rusqlite::Connection;

#[derive(Clone, PartialEq)]
pub enum ScreenType {
    Home,
    Tests,
    Results,
    Rerun,
    Help,
    Quit,
    Runner,
}

pub struct App {
    is_finished: bool,
    locale: String,
    current_screen: ScreenType,
    home: home::Home,
    tests: test::Tests,
    results: results::Results,
    rerun: rerun::Rerun,
    help: help::Help,
    runner: runner::Runner,
    conn: Connection,
}

impl App {
    pub fn new() -> Self {
        let conn = testservice::init_conn_and_populate();
        let tests_to_do = testservice::get_to_do(&conn);
        let tests_finished = testservice::get_finished(&conn);
        // TODO get that from config not to start with en always
        let default_locale = String::from("en");
        // TODO should this be from config as well?
        let all_locales = vec![
            (String::from("en"), String::from("English")), 
            (String::from("pl"), String::from("Polski")), 
        ];
        App { 
            is_finished: false,
            locale: default_locale.clone(),
            current_screen: ScreenType::Home,
            home: home::Home::new(),
            tests: test::Tests::new(tests_to_do, default_locale.clone()),
            results: results::Results::new(None, default_locale.clone()),
            rerun: rerun::Rerun::new(tests_finished, default_locale.clone()),
            help: help::Help::new(default_locale.clone(), all_locales),
            runner: runner::Runner::new(None, default_locale.clone()),
            conn,
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
            // make sure the home screen is always cleared when comming back to it
            KeyCode::Char('b') | KeyCode::Char('B') => {
                self.home.first_render = true;
                self.handle_key_code(KeyCode::Char('b'))?
            }
            other_code => self.handle_key_code(other_code)?
        }
        Ok(())
    }

    fn handle_key_code(&mut self, code: KeyCode) -> Result<(), io::Error> {
        match self.current_screen {
            ScreenType::Home => {
                let screen = self.home.handle_key_code(code);
                match screen {
                    ScreenType::Results => {
                        self.results.results_items = testservice::get_finished(&self.conn);
                        self.current_screen = screen;
                    },
                    _ => self.current_screen = screen

                }
            },
            ScreenType::Tests => {
                let (screen, test_id) = self.tests.handle_key_code(code);
                match screen {
                    ScreenType::Runner => {
                        let test_model = testservice::get_test_by_id(&self.conn, test_id);
                        self.runner = runner::Runner::new(test_model, self.locale.clone());
                        self.runner.origin = ScreenType::Tests;
                        self.current_screen = ScreenType::Runner;
                    },
                    ScreenType::Home => self.current_screen = ScreenType::Home,
                    _ => self.current_screen = screen 
                }
            },
            ScreenType::Results => {
                let (screen, test_o) = self.results.handle_key_code(code);
                match screen {
                    ScreenType::Results => {
                        match test_o {
                            Some(id) => {
                                let result = testservice::get_results_by_id(id);
                                self.results = results::Results::new(result, self.locale.clone());
                                self.results.results_items = testservice::get_finished(&self.conn);
                            },
                            None => ()
                        }
                        self.current_screen = ScreenType::Results
                    },
                    _ => self.current_screen = screen
                }
                //self.current_screen = self.results.handle_key_code(code)}
            },
            ScreenType::Rerun => {
                let (screen, test_id) = self.rerun.handle_key_code(code);
                match screen {
                    ScreenType::Runner => {
                        let test_model = testservice::get_test_by_id(&self.conn, test_id);
                        self.runner = runner::Runner::new(test_model, self.locale.clone());
                        self.runner.origin = ScreenType::Rerun;
                        self.current_screen = ScreenType::Runner;
                    },
                    ScreenType::Home => self.current_screen = ScreenType::Home,
                    _ => self.current_screen = screen
                }
            },
            ScreenType::Runner => {
                let (screen, result) = self.runner.handle_key_code(code);
                match screen {
                    ScreenType::Results => {
                        self.results = results::Results::new(result.clone(), self.locale.clone());
                        self.results.results_items = testservice::get_finished(&self.conn);
                        self.current_screen = ScreenType::Results
                    },
                    _ => self.current_screen = screen
                }
            },
            ScreenType::Help => {
                let (screen, locale) = self.help.handle_key_code(code);
                match screen {
                    ScreenType::Home => {
                        self.update_locale(locale);
                        self.current_screen = screen
                    },
                    _ => self.current_screen = screen
                }
            },
            _ => {}
        }
        Ok(())
    }

    fn update_locale(&mut self, locale: String) {
        self.locale = locale;
        self.home.update_locale(self.locale.clone());
        self.tests.locale = self.locale.clone();
        self.results.locale = self.locale.clone();
        self.rerun.locale = self.locale.clone();
        self.runner.locale = self.locale.clone();
        self.help.locale = self.locale.clone();
    }
}

