use crate::snake::{Direction, Position, Snake};
use std::collections;

#[derive(Debug, Clone, Copy)]
struct Fruit {
    position: Position,
}

impl Fruit {
    fn new(position: Position) -> Fruit {
        Fruit { position: position }
    }
}

pub struct Game {
    snakes: Vec<Snake>,
    fruits: Vec<Fruit>,
}

impl Game {
    pub fn new(pos: Position) -> Self {
        let snake = Snake::new(pos, Direction::Right);

        Self {
            snakes: vec![snake],
            fruits: vec![],
        }
    }

    pub fn update(&mut self) {
        for snake in self.snakes.iter_mut() {
            snake.update();
        }

        let mut indices_to_be_deleted = Vec::new();
        for (idx, snake) in self.snakes.iter().enumerate() {
            if snake.self_collision() || snake.collision_with_others(&self.snakes.iter().collect())
            {
                indices_to_be_deleted.push(idx);
            }
        }

        for &idx in indices_to_be_deleted.iter().rev() {
            self.snakes.remove(idx);
        }
    }
    pub fn snakes(&self) -> &Vec<Snake> {
        &self.snakes
    }
}
