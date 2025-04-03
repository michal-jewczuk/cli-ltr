
use crate::ui::menu;
use crate::screens::home;

use std::{io, time::{Duration, Instant}};
use tui::{
    backend::{Backend,CrosstermBackend},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


pub fn run() -> Result<(), io::Error> {
    
    //setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // tmp
    let menu_items = vec![
        "Tests [t]",
        "Results [r]",
        "Redo [d]",
        "Help [h]",
        "Exit [q]",
    ];
    let mut menu = menu::Menu::new(menu_items);

    // TODO add refresh rate as parameter
    let result = run_app(&mut terminal, &mut menu, Duration::from_millis(250));

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    //Ok(())
    result
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    menu: &mut menu::Menu,
    tick_rate: Duration,
    ) -> Result<(), io::Error> {

    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| home::ui(f, menu))?;

        //let timeout = tick_rate
        //    .checked_sub(last_tick.elapsed())
        //    .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => menu.previous(),
                    KeyCode::Down => menu.next(),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

