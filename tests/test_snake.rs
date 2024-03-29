use snake_ai::snake::{self, Position, Snakeable};

#[test]
fn test_snake_update() {
    let pos = snake::Position { x: 0, y: 0 };
    let mut snake = snake::Snake::new(pos, snake::Direction::Right);

    for _i in 0..3 {
        snake.update();
    }

    assert_eq!(snake.body()[0], snake::Position { x: 3, y: 0 });
}

#[test]
fn test_snake_error_on_self_collision() {
    let mut snake = snake::Snake::new(Position { x: 0, y: 0 }, snake::Direction::Right);
    snake.set_max_length(5);
    snake.update();
    assert!(!snake.self_collision());
    snake.set_direction(snake::Direction::Down);
    snake.update();
    assert!(!snake.self_collision());
    snake.set_direction(snake::Direction::Left);
    snake.update();
    assert!(!snake.self_collision());
    snake.set_direction(snake::Direction::Up);
    snake.update();
    assert!(snake.self_collision());
}

#[test]
fn test_body_length_does_not_exceed_max_length() {
    let mut snake = snake::Snake::new(Position { x: 0, y: 0 }, snake::Direction::Down);
    let max_length = 3;
    snake.set_max_length(max_length);

    for _ in 0..5 {
        snake.update();
    }
    assert_eq!(snake.body().len(), max_length);
}

#[test]
fn test_multi_collision() {
    let snake1 = Box::new(snake::Snake::new_body(
        vec![
            Position { x: 0, y: 0 },
            Position { x: 1, y: 0 },
            Position { x: 2, y: 0 },
            Position { x: 3, y: 0 },
        ],
        snake::Direction::Right,
    ));
    let snake2 = Box::new(snake::Snake::new_body(
        vec![
            Position { x: 3, y: -3 },
            Position { x: 3, y: -2 },
            Position { x: 3, y: -1 },
            Position { x: 3, y: 0 },
        ],
        snake::Direction::Right,
    ));
    let snake3 = Box::new(snake::Snake::new_body(
        vec![Position { x: 4, y: -3 }, Position { x: 3, y: -3 }],
        snake::Direction::Right,
    ));

    assert!(snake1.collision_with_others(&vec![&snake2, &snake3]));
    assert!(!snake1.collision_with_others(&vec![&snake1, &snake3]));

    assert!(snake2.collision_with_others(&vec![&snake1]));
    assert!(snake2.collision_with_others(&vec![&snake3]));
    assert!(!snake2.collision_with_others(&vec![&snake2]));

    assert!(snake3.collision_with_others(&vec![&snake1, &snake2]));
    assert!(!snake3.collision_with_others(&vec![&snake1, &snake3]));
}
