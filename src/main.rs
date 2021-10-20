use snake_ai::game::{self, Game};
use snake_ai::game_canvas::{Drawable, GameCanvas};
use snake_ai::snake;
use std::sync::mpsc;
use std::{error::Error, io, time::Duration};
use std::{thread, time};
use termion::event::{Event, Key, MouseEvent};
use termion::input::{Events, TermRead};
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
use tui::widgets::{Block, Borders};
use tui::Terminal;

enum Action {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

fn wait_for_input(tx: mpsc::SyncSender<Option<termion::event::Key>>) {
    let stdin = io::stdin();
    let events = stdin.events();
    for event in events {
        match event {
            Ok(Event::Key(key)) => tx.send(Some(key)),
            _ => break,
        };
    }
}

fn handle_input(rx: &mpsc::Receiver<Option<termion::event::Key>>) -> Option<termion::event::Key> {
    let key = rx.try_recv();
    match key {
        Ok(x) => x,
        _ => None,
    }
}

fn determine_action(rx: &mpsc::Receiver<Option<termion::event::Key>>) -> Option<Action> {
    match handle_input(rx) {
        Some(c) => match c {
            Key::Char('q') => Some(Action::Quit),
            Key::Left => Some(Action::Left),
            Key::Right => Some(Action::Right),
            Key::Up => Some(Action::Up),
            Key::Down => Some(Action::Down),
            _ => None,
        },
        _ => None,
    }
}

fn set_layout(area: tui::layout::Rect) -> Vec<tui::layout::Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area)
}

fn main() -> Result<(), io::Error> {
    let mut run = true;
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut i = 0;

    let mut game = Game::new(snake::Position { x: 0, y: 0 });

    terminal.hide_cursor()?;
    terminal.clear()?;
    let game_canvas = GameCanvas::new();

    let (tx, rx) = mpsc::sync_channel(1);
    let handle = thread::spawn(|| wait_for_input(tx));

    while run {
        terminal.draw(|frame| {
            let chunks = set_layout(frame.size());
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("World"))
                .paint(|ctx| {
                    // ctx.print(5 as f64, 5 as f64, "Hello there", Color::Red);
                    game.draw(ctx);
                })
                .x_bounds([-100.0, 100.0])
                .y_bounds([-100.0, 100.0]);
            frame.render_widget(canvas, chunks[0]);
        })?;
        if let Some(action) = determine_action(&rx) {
            match action {
                Action::Down => game.update_snake(0, snake::Direction::Down),
                Action::Up => game.update_snake(0, snake::Direction::Up),
                Action::Left => game.update_snake(0, snake::Direction::Left),
                Action::Right => game.update_snake(0, snake::Direction::Right),
                Action::Quit => break,
            }
        };
        thread::sleep(time::Duration::from_millis(16));
        i = (i + 1) % 100;
        game.update();
    }
    Ok(())
}
