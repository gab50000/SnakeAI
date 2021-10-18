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

fn wait_for_input(tx: mpsc::Sender<Option<char>>) {
    let stdin = io::stdin();
    for event in stdin.events() {
        match event {
            Ok(Event::Key(Key::Char(x))) => tx.send(Some(x)),
            _ => tx.send(None),
        };
    }
}

fn handle_input(rx: &mpsc::Receiver<Option<char>>) -> Option<char> {
    let key = rx.try_recv();
    match key {
        Ok(x) => x,
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
    let stdin = termion::async_stdin().keys();
    let mut i = 0;

    let mut game = Game::new(snake::Position { x: 0, y: 0 });

    terminal.hide_cursor()?;
    terminal.clear()?;
    let game_canvas = GameCanvas::new();

    let (tx, rx) = mpsc::channel();
    thread::spawn(|| wait_for_input(tx));

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
        if let Some(input) = handle_input(&rx) {
            match input {
                'q' => {
                    run = false;
                }
                _ => {}
            }
        };
        thread::sleep(time::Duration::from_millis(100));
        i = (i + 1) % 100;
        game.update();
    }
    Ok(())
}
