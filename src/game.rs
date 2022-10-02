use crate::{
    game_canvas::Drawable,
    snake::{Direction, Position, Snake, Snakeable},
};
use std::{collections, ops::DerefMut};

#[derive(Debug, Clone, Copy)]
pub struct Fruit {
    position: Position,
}

impl Fruit {
    fn new(position: Position) -> Fruit {
        Fruit { position: position }
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}

// trait DrawSnakeable: Snakeable + Drawable {}

pub struct Game {
    snakes: Vec<Box<dyn Snakeable>>,
    fruits: Vec<Fruit>,
}

impl Game {
    pub fn new(snake: Box<dyn Snakeable>) -> Self {
        Self {
            snakes: vec![snake],
            fruits: vec![],
        }
    }

    pub fn update(&mut self) {
        for snake in self.snakes.iter_mut() {
            snake.get_mut_snake().update();
        }

        let mut indices_to_be_deleted = Vec::new();
        for (idx, snake) in self.snakes.iter().enumerate() {
            if snake.get_snake().self_collision()
                || snake
                    .get_snake()
                    .collision_with_others(&self.snakes.iter().collect::<Vec<&Box<Snakeable>>>())
            {
                indices_to_be_deleted.push(idx);
            }
        }

        for &idx in indices_to_be_deleted.iter().rev() {
            self.snakes.remove(idx);
        }
    }
    pub fn snakes(&self) -> &Vec<Box<Snakeable>> {
        &self.snakes
    }

    pub fn fruits(&self) -> &Vec<Fruit> {
        &self.fruits
    }

    pub fn update_snake(&mut self, i: usize, new_direction: Direction) {
        self.snakes[i]
            .deref_mut()
            .get_mut_snake()
            .set_direction(new_direction)
    }
}
