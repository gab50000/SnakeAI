use snake_ai::game::{self, Game};
use snake_ai::game_canvas::{Drawable, GameCanvas};
use snake_ai::input_handler::{Action, InputHandler};
use snake_ai::snake;
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

fn set_layout(area: tui::layout::Rect) -> Vec<tui::layout::Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(area)
}

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut i = 0;

    let mut game = Game::new(snake::Position { x: 0, y: 0 });

    terminal.hide_cursor()?;
    terminal.clear()?;

    let input_handler = InputHandler::new();

    loop {
        terminal.draw(|frame| {
            let chunks = set_layout(frame.size());
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("World"))
                .paint(|ctx| {
                    // ctx.print(5 as f64, 5 as f64, "Hello there", Color::Red);
                    game.draw(ctx);
                })
                .x_bounds([-100.0, 100.0])
                .y_bounds([-30.0, 30.0]);
            frame.render_widget(canvas, chunks[0]);
        })?;
        if let Some(action) = input_handler.determine_action() {
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
    terminal.clear()?;
    Ok(())
}
