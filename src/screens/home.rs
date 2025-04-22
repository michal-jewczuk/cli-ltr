use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};
use crossterm::event::{KeyCode};

pub struct Home {
    pub first_render: bool,
    menu: Menu,
}

impl Home {
    pub fn new() -> Self {
        Home { first_render: true, menu: Menu::home() }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), 
                Constraint::Length(4), 
                Constraint::Min(0)
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
        let text = vec![
            header_border.clone(),
            Spans::from(vec![
                Span::raw("WELCOME TO "),
                Span::styled("CLI Language Test Runner", Style::default().add_modifier(Modifier::BOLD)),
            ]),
            header_border.clone(),
        ];

        let header = layout::get_header(text);
        let header_area = layout::get_column_with_margin(area, 10, 150);
    
        f.render_widget(header, header_area);
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
        let instructions_area = layout::get_column_with_margin(area, 10, 150);
    
        f.render_widget(instructions, instructions_area);
    }

    fn render_menu<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let list = layout::create_navigable_list(self.menu.items.clone());
        let x = area.width / 2 - 10;
        let y = area.y + 2;
        let menu_area = Rect::new(x, y, 20, 7);
    
        f.render_stateful_widget(list, menu_area, &mut self.menu.state);
    }
}

