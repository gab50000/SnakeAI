use snake_ai::snake::{self, Position};

#[test]
fn test_snake_update() {
    let pos = snake::Position { x: 0, y: 0 };
    let mut snake = snake::Snake::new(pos, snake::Direction::Right);

    for _i in 0..3 {
        snake.update().unwrap_or_else(|_| assert!(false));
    }

    assert_eq!(snake.body()[0], snake::Position { x: 3, y: 0 });
}

#[test]
fn test_snake_error_on_self_collision() {
    let mut snake = snake::Snake::new(Position { x: 0, y: 0 }, snake::Direction::Right);
    snake.set_max_length(4);
    snake.update().unwrap_or_else(|_| assert!(false));
    snake.set_direction(snake::Direction::Down);
    snake.update().unwrap_or_else(|_| assert!(false));
    snake.set_direction(snake::Direction::Left);
    snake.update().unwrap_or_else(|_| assert!(false));
    snake.set_direction(snake::Direction::Up);
    match snake.update() {
        Ok(()) => assert!(false),
        Err(snake::SnakeDeadError) => assert!(true),
    };
}
