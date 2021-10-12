use std::boxed::Box;
use std::io;
use std::io::Stdout;

use termion::raw::{IntoRawMode, RawTerminal};
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Color;
use tui::widgets::canvas::{Canvas, Context, Line, Map, MapResolution, Rectangle};
use tui::widgets::{Block, Borders};
use tui::Terminal;

use crate::game;
use crate::snake;
use std::{thread, time};

trait Drawable {
    fn draw(&self, ctx: &mut Context);
}

impl Drawable for snake::Snake {
    fn draw(&self, ctx: &mut Context) {
        for pos in self.body() {
            ctx.print(pos.x as f64, pos.y as f64, "X", Color::Green);
        }
    }
}

struct GameCanvas<'a> {
    width: i64,
    height: i64,
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
    canvas: Box<Canvas<'a, Box<dyn Fn(&mut Context) -> ()>>>,
}

impl<'a> GameCanvas<'a> {
    fn new() -> Result<GameCanvas<'a>, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        let canvas = Box::new(Canvas::default());
        Ok(GameCanvas {
            width: 1,
            height: 1,
            terminal,
            canvas,
        })
    }

    fn draw(&mut self, game: game::Game) -> Result<(), io::Error> {
        for i in 1..100 {
            self.terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(frame.size());
                //TODO: use member canvas
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
        Ok(())
    }
}
