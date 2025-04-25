use crate::app::ScreenType;
use crate::ui::{layout, menu::Menu};

use tui::{
    backend::Backend,
    layout::{Rect},
    style::{Modifier, Style},
    text::{Span, Spans},
    widgets::{Clear},
    Frame,
};
use crossterm::event::{KeyCode};
use rust_i18n::t;

pub struct Home {
    pub first_render: bool,
    menu: Menu,
}

impl Home {
    pub fn new() -> Self {
        let menu_items = vec![
            format!("[t] {}", t!("menu.tests", locale = "pl")),
            format!("[r] {}", t!("menu.results", locale = "pl")),
            format!("[d] {}", t!("menu.redo", locale = "pl")),
            format!("[h] {}", t!("menu.help", locale = "pl")),
            format!("[q] {}", t!("menu.exit", locale = "pl")),
        ];
        Home { first_render: true, menu: Menu::new(menu_items) }
    }

    pub fn draw<B: Backend>(&mut self, f: &mut Frame<B>) {
        if self.first_render {
            self.first_render = false;
            f.render_widget(Clear, f.size());
            return;
        }

        let background = layout::get_background();
        f.render_widget(background, f.size());
    
        let layout = layout::get_header_navbar_layout(f.size(), 3, 4);
        self.render_header(f, layout[0]);
        self.render_menu_instructions(f, layout[1]);
        self.render_menu(f, layout[2]);
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
                Span::raw(t!("title.home", locale = "pl")),
                Span::raw(" "),
                Span::styled(t!("name.full", locale = "pl"), Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(format!(" [{}]", t!("name.short"))),
            ]),
            header_border.clone(),
        ];

        let header = layout::get_header(text);
        let header_area = layout::get_default_column(area);
    
        f.render_widget(header, header_area);
    }

    fn render_menu_instructions<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let text = vec![
            Spans::from(Span::raw(t!("home.instruction.l1", locale = "pl"))),
            Spans::from(Span::raw(t!("home.instruction.l2", locale = "pl"))),
        ];
        let instructions = layout::get_par_with_borders(text);
        let instructions_area = layout::get_default_column(area);
    
        f.render_widget(instructions, instructions_area);
    }

    fn render_menu<B: Backend>(&mut self, f: &mut Frame<B>, area: Rect) {
        let menu = layout::create_navigable_list(self.menu.items.clone());
        let menu_area = layout::get_adaptative_column(area);
    
        f.render_stateful_widget(menu, menu_area, &mut self.menu.state);
    }
}

