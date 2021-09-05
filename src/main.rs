use std::{error::Error, io, time::Duration};
use std::{thread, time};
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
use tui::widgets::{Block, Borders};
use tui::Terminal;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    for i in 1..100 {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(frame.size());
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("World"))
                .paint(|ctx| {
                    ctx.print(i as f64, i as f64, "Hello there", Color::Red);
                })
                .x_bounds([-100.0, 100.0])
                .y_bounds([-100.0, 100.0]);
            frame.render_widget(canvas, chunks[0]);
            thread::sleep(time::Duration::from_millis(100));
        })?;
    }
    terminal.clear()?;
    Ok(())
}
