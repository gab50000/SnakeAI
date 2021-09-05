use std::collections;

#[derive(Debug, Clone)]
pub struct SnakeDeadError;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}

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

impl Snake {
    pub fn new(pos: Position, direction: Direction) -> Snake {
        Snake {
            body: collections::VecDeque::from(vec![pos]),
            direction: direction,
            max_length: 1,
        }
    }

    pub fn body(&self) -> &collections::VecDeque<Position> {
        &self.body
    }

    pub fn set_max_length(&mut self, new_length: usize) {
        self.max_length = new_length;
    }

    pub fn set_direction(&mut self, new_direction: Direction) {
        self.direction = new_direction;
    }

    pub fn update(&mut self) -> Result<(), SnakeDeadError> {
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

        if self.body.contains(&new_pos) {
            return Err(SnakeDeadError);
        }

        self.body.push_back(new_pos);

        if self.body.len() > self.max_length {
            self.body.pop_front();
        }

        Ok(())
    }
}
