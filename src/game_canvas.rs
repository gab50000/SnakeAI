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

pub trait Drawable {
    fn draw(&self, ctx: &mut Context);
}

impl Drawable for snake::Snake {
    fn draw(&self, ctx: &mut Context) {
        for pos in self.body() {
            ctx.print(pos.x as f64, pos.y as f64, "X", Color::Green);
        }
    }
}

impl Drawable for game::Fruit {
    fn draw(&self, ctx: &mut Context) {
        ctx.print(
            self.position().x as f64,
            self.position().y as f64,
            "X",
            Color::Green,
        );
    }
}

impl Drawable for game::Game {
    fn draw(&self, ctx: &mut Context) {
        for snake in self.snakes() {
            snake.get_snake().draw(ctx);
        }

        for fruit in self.fruits() {
            fruit.draw(ctx);
        }
    }
}

pub struct GameCanvas {
    width: i64,
    height: i64,
}

impl GameCanvas {
    pub fn new() -> GameCanvas {
        GameCanvas {
            width: 10,
            height: 10,
        }
    }

    pub fn draw(&self, ctx: &mut Context, i: i32) {
        ctx.print(i as f64, 30 as f64, "Booyakalala", Color::Green);
    }
}
