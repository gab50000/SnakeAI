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

struct GameCanvas {
    width: i64,
    height: i64,
    terminal: Terminal<TermionBackend<RawTerminal<Stdout>>>,
}

impl GameCanvas {
    fn new() -> Result<GameCanvas, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(GameCanvas {
            width: 1,
            height: 1,
            terminal,
        })
    }

    fn draw(&self, game: game::Game) {
        // self.terminal.draw(f)
    }
}
