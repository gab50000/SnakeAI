use std::{collections, ptr};

#[derive(Debug, Clone)]
pub struct SnakeDeadError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    body: collections::VecDeque<Position>,
    direction: Direction,
    max_length: usize,
}

pub trait Snakeable {
    fn get_snake(&self) -> &Snake;
    fn get_mut_snake(&mut self) -> &mut Snake;
    fn update_direction(&mut self);
}

impl Snake {
    pub fn new(pos: Position, direction: Direction) -> Snake {
        Snake {
            body: collections::VecDeque::from(vec![pos]),
            direction: direction,
            max_length: 1,
        }
    }

    pub fn new_body(positions: Vec<Position>, direction: Direction) -> Snake {
        let max_length = positions.len();
        Snake {
            body: collections::VecDeque::from(positions),
            direction,
            max_length,
        }
    }

    pub fn body(&self) -> &collections::VecDeque<Position> {
        &self.body
    }

    pub fn set_max_length(&mut self, new_length: usize) {
        self.max_length = new_length;
    }

    pub fn collision_with_other(&self, other: &dyn Snakeable) -> bool {
        if ptr::eq(self, other.get_snake()) {
            return false;
        }

        let own_head = self.body.back().unwrap();
        let other_head = other.get_snake().body.back().unwrap();
        other.get_snake().body.contains(own_head) || self.body.contains(other_head)
    }

    pub fn update(&mut self) {
        let head_pos = self.body.back().unwrap();
        let new_pos = match self.direction {
            Direction::Down => Position {
                x: head_pos.x,
                y: head_pos.y - 1,
            },
            Direction::Up => Position {
                x: head_pos.x,
                y: head_pos.y + 1,
            },
            Direction::Left => Position {
                x: head_pos.x - 1,
                y: head_pos.y,
            },
            Direction::Right => Position {
                x: head_pos.x + 1,
                y: head_pos.y,
            },
        };

        self.body.push_back(new_pos);
        if self.body.len() > self.max_length {
            self.body.pop_front();
        }
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn self_collision(&self) -> bool {
        let current_length = self.body.len();
        let body_without_head: Vec<&Position> = self.body.range(0..current_length - 1).collect();
        let head = self.body.back().unwrap();
        return body_without_head.contains(&head);
    }

    pub fn collision_with_others<T: Snakeable + ?Sized>(&self, snakes: &Vec<&Box<T>>) -> bool {
        for snake in snakes.iter() {
            if self.collision_with_other(snake.get_snake()) {
                return true;
            }
        }
        false
    }
}

impl Snakeable for Snake {
    fn get_snake(&self) -> &Snake {
        return self;
    }

    fn get_mut_snake(&mut self) -> &mut Snake {
        return self;
    }

    fn update_direction(&mut self) {
        self.update_direction();
    }
}

impl Snakeable for Box<Snake> {
    fn get_snake(&self) -> &Snake {
        return self.as_ref().get_snake();
    }
    fn get_mut_snake(&mut self) -> &mut Snake {
        return self.as_mut().get_mut_snake();
    }
    fn update_direction(&mut self) {
        self.as_mut().update_direction();
    }
}
