
use crate::screens::home;

use std::io;
use tui::{
    backend::Backend,
    Frame
};
use crossterm::event::{Event, KeyCode, KeyEvent};


pub enum ScreenType {
    Home,
    Tests,
    Scores,
    Rerun,
    Help,
    Quit,
}

pub struct App<'a> {
    is_finished: bool,
    current_screen: ScreenType,
    home: home::Home<'a>,
}

impl App<'_> {
    pub fn new() -> Self {
        App { 
            is_finished: false,
            current_screen: ScreenType::Home,
            home: home::Home::new(),
        }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        match self.current_screen {
            ScreenType::Home => self.home.draw(f),
            _ => {} 
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
            _ => {}
        }
        Ok(())
    }

}



