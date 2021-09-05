use snake_ai::snake;

#[test]
fn test_snake() {
    let pos = snake::Position { x: 0, y: 0 };
    let snake = snake::Snake::new(pos);
}
