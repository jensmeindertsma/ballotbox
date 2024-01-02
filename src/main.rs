use crossterm::{
    event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Modifier, Style},
    symbols,
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};
use std::{
    io::{self},
    thread,
    time::Duration,
};

fn main() {
    setup_terminal().unwrap();
    let mut terminal = ratatui::Terminal::new(CrosstermBackend::new(io::stdout().lock())).unwrap();

    let mut state = ListState::default().with_selected(Some(0));

    let items = [Item::Votes, Item::People, Item::Settings];

    thread::spawn(|| {});

    loop {
        terminal
            .draw(|frame| {
                let layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Min(25), Constraint::Percentage(100)])
                    .split(frame.size());

                let navigation_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100), Constraint::Min(3)])
                    .split(layout[0]);

                frame.render_stateful_widget(
                    List::new(
                        items
                            .iter()
                            .map(|i| {
                                ListItem::new(match i {
                                    Item::Votes => "Votes",
                                    Item::People => "People",
                                    Item::Settings => "Settings",
                                })
                            })
                            .collect::<Vec<_>>(),
                    )
                    .block(
                        Block::default()
                            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                            .border_set(symbols::border::Set {
                                top_right: symbols::line::HORIZONTAL_DOWN,
                                ..symbols::border::PLAIN
                            })
                            .padding(Padding::horizontal(1)),
                    )
                    .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                    .highlight_symbol(">> "),
                    navigation_layout[0],
                    &mut state,
                );

                frame.render_widget(
                    Paragraph::new("Press `q` to quit")
                        .block(
                            Block::default()
                                .border_set(symbols::border::Set {
                                    top_left: symbols::line::NORMAL.vertical_right,
                                    top_right: symbols::line::NORMAL.vertical_left,
                                    bottom_right: symbols::line::HORIZONTAL_UP,
                                    ..symbols::border::PLAIN
                                })
                                .borders(Borders::ALL),
                        )
                        .alignment(Alignment::Center),
                    navigation_layout[1],
                );

                frame.render_widget(
                    match items[state.selected().unwrap()] {
                        Item::Votes => Paragraph::new("Votes here"),
                        Item::People => Paragraph::new("People here"),
                        Item::Settings => Paragraph::new("Settings here"),
                    }
                    .block(
                        Block::default()
                            .padding(Padding::horizontal(1))
                            .borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT),
                    ),
                    layout[1],
                )
            })
            .unwrap();

        // Spend 16ms waiting for input event, process it, then proceed to next render iteration.
        if crossterm::event::poll(Duration::from_millis(16)).unwrap() {
            if let Event::Key(key) = crossterm::event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match key {
                        KeyEvent {
                            modifiers: KeyModifiers::CONTROL,
                            code: KeyCode::Char('c'),
                            ..
                        }
                        | KeyEvent {
                            code: KeyCode::Char('q'),
                            ..
                        } => {
                            break;
                        }
                        KeyEvent {
                            code: KeyCode::Up, ..
                        } => {
                            let i = state.selected().unwrap();

                            if i == 0 {
                                state.select(Some(items.len() - 1));
                            } else {
                                state.select(Some(i - 1));
                            }
                        }
                        KeyEvent {
                            code: KeyCode::Down,
                            ..
                        } => {
                            let i = state.selected().unwrap();

                            if i >= items.len() - 1 {
                                state.select(Some(0));
                            } else {
                                state.select(Some(i + 1));
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    // Terminal restoration is best-effort.
    let _ = restore_terminal();
}

enum Item {
    Votes,
    People,
    Settings,
}

fn setup_terminal() -> io::Result<()> {
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
