use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Home<'a> {
    menu: Menu<'a>,
}

impl<'a> Home<'a> {
    pub fn new() -> Self {
        Home { menu: Menu::home() }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(15), 
                Constraint::Percentage(15), 
                Constraint::Percentage(70)
            ].as_ref())
            .split(f.size());
    
        self.render_header(f, chunks[0]);
        self.render_menu_instructions(f, chunks[1]);
        self.render_menu(f, chunks[2]);
    }

    pub fn handle_key_code(&mut self, code: KeyCode) -> ScreenType {
        match code {
            KeyCode::Up => self.menu.previous(),
            KeyCode::Down => self.menu.next(),
            KeyCode::Enter => return self.handle_enter(),
            _ => {} 
        }
        ScreenType::Home
    }

    fn handle_enter(&mut self) -> ScreenType {
        match self.menu.state.selected() {
            Some(screen) => {
                match screen {
                    0 => ScreenType::Tests,
                    1 => ScreenType::Results,
                    2 => ScreenType::Rerun,
                    3 => ScreenType::Help,
                    4 => ScreenType::Quit,
                    _ => ScreenType::Home
                }
            },
            None => ScreenType::Home
        }
    }

    fn render_header<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let header_border = Spans::from(Span::raw("==========================="));
        let mut text = vec![
            Spans::from(Span::raw("")),
            header_border.clone(),
            Spans::from(vec![
                Span::raw("WELCOME TO "),
                Span::styled("CLI Language Test Runner", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!("{}", area.height)),
            ]),
            header_border.clone(),
            Spans::from(Span::raw("")),
        ];
        if area.height < 5 {
            text = text[1..4].to_vec();
        }

        let header = Paragraph::new(text)
            .block(Block::default())
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
    
        f.render_widget(header, area);
    }

    fn render_menu_instructions<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let mut text = vec![
            Spans::from(Span::raw("")),
            Spans::from(Span::raw("You can use UP or DOWN arrows to navigate through menu items and ENTER to confirm")),
            Spans::from(Span::raw("or you can use the specified key shortcut to instantly confirm")),
        ];
        if area.height < 6 {
            text = text[1..3].to_vec();
        }
        let instructions = Paragraph::new(text)
            .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
            .style(Style::default().fg(Color::White).bg(Color::DarkGray))
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
    
        f.render_widget(instructions, area);
    }

    fn render_menu<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let list = layout::create_navigable_list(self.menu.items.clone());
        let x = area.width / 2 - 10;
        let y = area.y + 2;
        let menu_area = Rect::new(x, y, 20, 7);
    
        f.render_stateful_widget(list, menu_area, &mut self.menu.state);
    }
}

