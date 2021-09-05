use crate::snake::{Position, Snake};

#[derive(Debug, Clone, Copy)]
struct Fruit {
    position: Position,
}

impl Fruit {
    fn new(position: Position) -> Fruit {
        Fruit { position: position }
    }
}

struct Game {
    snakes: Vec<Snake>,
    fruits: Vec<Fruit>,
}
