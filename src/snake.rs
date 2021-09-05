use std::collections;
#[derive(Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
}
pub struct Snake {
    body: collections::VecDeque<Position>,
    max_length: i64,
}

impl Snake {
    pub fn new(pos: Position) -> Snake {
        Snake {
            body: collections::VecDeque::from(vec![pos]),
            max_length: 1,
        }
    }
}
