use std::{
    io::{self},
    time::Duration,
};

use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::widgets::ListState;

fn main() {
    let mut stdout = io::stdout().lock();

    setup_terminal(&mut stdout);

    let state = ListState::default();

    loop {
        // Spend 16ms waiting for input event, process it, then proceed to next render iteration.
        if crossterm::event::poll(Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = crossterm::event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            restore_terminal();
                            break;
                        }
                        KeyCode::Up => { /* NEXT LIST ITEM */},
                        KeyCode::Down => 
                    }
                }
            }
        }
    }
}

fn setup_terminal(stdout: &mut io::StdoutLock<'static>) -> io::Result<()> {
    // Extend the default panic handler so we always try to restore the terminal, even when we panic.
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        restore_terminal().unwrap();
        original_hook(panic_info);
    }));

    crossterm::execute!(io::stdout(), EnterAlternateScreen)?;
    terminal::enable_raw_mode()
}

fn restore_terminal() -> io::Result<()> {
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;
    terminal::disable_raw_mode()
}
