use crate::input_handler::{Action, InputHandler};
use crate::snake;
use snake::Snakeable;
pub struct InputSnake {
    snake: snake::Snake,
    input_handler: InputHandler,
}

impl InputSnake {
    pub fn new(pos: snake::Position, direction: snake::Direction) -> Self {
        let snake = snake::Snake::new(pos, direction);
        let input_handler = InputHandler::new();
        Self {
            snake,
            input_handler,
        }
    }

    pub fn update_direction(&mut self) {
        let action = self.input_handler.determine_action();
        if let Some(action) = action {
            match action {
                Action::Down => self.snake.set_direction(snake::Direction::Down),
                Action::Up => self.snake.set_direction(snake::Direction::Up),
                Action::Left => self.snake.set_direction(snake::Direction::Left),
                Action::Right => self.snake.set_direction(snake::Direction::Right),
                _ => {}
            }
        }
    }
}

impl Snakeable for InputSnake {
    fn get_snake(&self) -> &snake::Snake {
        return &self.snake;
    }
    fn get_mut_snake(&mut self) -> &mut snake::Snake {
        return &mut self.snake;
    }
}
