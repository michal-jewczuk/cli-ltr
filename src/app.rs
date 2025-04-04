
use crate::screens::{help, home, rerun, results, test};

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
}

pub struct App<'a> {
    is_finished: bool,
    current_screen: ScreenType,
    home: home::Home<'a>,
    tests: test::Tests<'a>,
    results: results::Results,
    rerun: rerun::Rerun,
    help: help::Help,
}

impl App<'_> {
    pub fn new() -> Self {
        let to_do = vec![
            ("xyz", "[2025-03-07]: English idioms with twist"),
            ("abc", "[2025-02-28]: Verbs and stuff"),
            ("cde", "[2025-02-27]: Week exam #2"),
        ];
        App { 
            is_finished: false,
            current_screen: ScreenType::Home,
            home: home::Home::new(),
            tests: test::Tests::new(to_do),
            results: results::Results::new(),
            rerun: rerun::Rerun::new(),
            help: help::Help::new(),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        match self.current_screen {
            ScreenType::Home => self.home.draw(f),
            ScreenType::Tests => self.tests.draw(f),
            ScreenType::Results => self.results.draw(f),
            ScreenType::Rerun => self.rerun.draw(f),
            ScreenType::Help => self.help.draw(f),
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
            KeyCode::Char('q') | KeyCode::Char('Q') => self.is_finished = true,
            other_code => self.handle_key_code(other_code)?
        }
        Ok(())
    }

    fn handle_key_code(&mut self, code: KeyCode) -> Result<(), io::Error> {
        match self.current_screen {
            ScreenType::Home => self.current_screen = self.home.handle_key_code(code),
            ScreenType::Tests => self.current_screen = self.tests.handle_key_code(code),
            ScreenType::Results => self.current_screen = self.results.handle_key_code(code),
            ScreenType::Rerun => self.current_screen = self.rerun.handle_key_code(code),
            ScreenType::Help => self.current_screen = self.help.handle_key_code(code),
            _ => {}
        }
        Ok(())
    }

}



