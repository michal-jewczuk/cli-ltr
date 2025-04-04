use crate::app::App;

use std::{io, time::{Duration, Instant}};
use tui::{
    backend::{Backend,CrosstermBackend},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
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

    // TODO add refresh rate as parameter
    let result = run_app(&mut terminal, Duration::from_millis(250));

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    result
}

fn run_app<B: Backend>(
        terminal: &mut Terminal<B>,
        tick_rate: Duration,
    ) -> Result<(), io::Error> {

    let mut last_tick = Instant::now();
    let mut app = App::new();
    loop {
        terminal.draw(|f| app.draw(f))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            app.on_terminal_event(event::read()?)?;
            if app.is_done() {
                return Ok(());
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

