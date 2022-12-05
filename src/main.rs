#![windows_subsystem = "windows"]

use rand::Rng;

use raylib::prelude::*;

use std::collections::VecDeque;

const FRAME_TIME: f32 = 0.1;
const TILE_SIZE: i32 = 20;
const SNAKE_COLOR: Color = Color::GREEN;
const TAIL_COLOR: Color = Color::DARKGREEN;
const APPLE_COLOR: Color = Color::RED;
const ROOM_SIZE_X: i32 = 20;
const ROOM_SIZE_Y: i32 = 20;
const WINDOW_WIDTH: i32 = ROOM_SIZE_X * TILE_SIZE;
const WINDOW_HEIGHT: i32 = ROOM_SIZE_Y * TILE_SIZE;


struct Snake {
    x: i32,
    y: i32,
    tail: VecDeque<Tail>,
    input_direction: Option<(i32, i32)>,
    direction: (i32, i32)
}

struct Tail {
    x: i32,
    y: i32,
}

struct Apple {
    x: i32,
    y: i32,
}

struct Game {
    apple: Apple,
    snake: Snake,
    game_over: bool,
}

impl Default for Game {
    fn default() -> Game {
        Game {
            game_over: false,
            apple: Apple::new(1, 2),
            snake: Snake::new(4, 4)
        }
    }
}

impl Apple {
    fn new(x: i32, y: i32) -> Apple {
        Apple { x, y }
    }
}

impl Snake {
    fn new(x: i32, y: i32) -> Snake {
        Snake { 
            x, 
            y, 
            tail: VecDeque::from_iter([ Tail::new(x, y - 1)]),
            input_direction: None,
            direction: (0, 1),
        }
    }
}

impl Tail {
    fn new(x: i32, y: i32) -> Tail {
        Tail { x, y }
    }
}

fn handle_input(game: &mut Game, rl: &mut RaylibHandle) {
    use raylib::consts::KeyboardKey::*;

    // Get player input
    let input = rl.get_key_pressed();

    // Set direction based on input
    if let Some(key) = input {
        let current_direction = game.snake.direction;
        let mut target_direction = (0, 0);

        if key == KEY_W || key == KEY_UP {
            target_direction.1 = -1;
        }
        else if key == KEY_A || key == KEY_LEFT {
            target_direction.0 = -1;
        }
        else if key == KEY_S || key == KEY_DOWN {
            target_direction.1 = 1;
        }
        else if key == KEY_D || key == KEY_RIGHT {
            target_direction.0 = 1;
        }

        // Prevent moving in the opposite direction
        if current_direction.1 != -target_direction.1 || current_direction.0 != -target_direction.0 {
            game.snake.input_direction = Some(target_direction);
        }
    }
}

fn update_game(game: &mut Game) {
    // Set previous and future positions
    let snake_location = (game.snake.x, game.snake.y);
    let snake_next_location: (i32, i32);

    // Determine if the snake is changing its direction
    if let Some(direction) = game.snake.input_direction {
        snake_next_location = (
            snake_location.0 + direction.0,
            snake_location.1 + direction.1
        );
        game.snake.direction = direction;
        game.snake.input_direction = None;
    }
    else {
        snake_next_location = ( 
            snake_location.0 + game.snake.direction.0,
            snake_location.1 + game.snake.direction.1,
        );
    }

    // Check if the snake colides with its tail
    for tail in game.snake.tail.iter() {
        if tail.x == snake_next_location.0 && tail.y == snake_next_location.1 {
            game.game_over = true;
        }
    }

    // Check if snake is out-of-bounds
    if snake_next_location.0 >= ROOM_SIZE_X 
    || snake_next_location.0 < 0 
    || snake_next_location.1 >= ROOM_SIZE_Y 
    || snake_next_location.1 < 0 {
        game.game_over = true;
    }

    // Check if snake eats the apple
    if snake_next_location.0 == game.apple.x && snake_next_location.1 == game.apple.y {
        // Grow the snake
        game.snake.tail.push_front(Tail::new(snake_location.0, snake_location.1));

        // Reset apple position
        game.apple.x = rand::thread_rng().gen_range(0..ROOM_SIZE_X);
        game.apple.y = rand::thread_rng().gen_range(0..ROOM_SIZE_Y);
    }
    else {
        // Move the tail.
        let end = game.snake.tail.pop_back();
        if let Some(mut tail_end) = end {
            tail_end.x = snake_location.0;
            tail_end.y = snake_location.1;
            game.snake.tail.push_front(tail_end);
        }
    }
    
    // Move snake
    game.snake.x = snake_next_location.0;
    game.snake.y = snake_next_location.1;
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::BLACK);

    // Draw game over text
    if game.game_over {
        d.draw_text(
            "Game over!", 
            WINDOW_WIDTH / 2 - 80,
            WINDOW_HEIGHT / 2, 
            32, 
            Color::WHITE
        );
        return;
    }
 
    // Draw snake
    d.draw_rectangle(
        game.snake.x * TILE_SIZE, 
        game.snake.y * TILE_SIZE, 
        TILE_SIZE, 
        TILE_SIZE, 
        SNAKE_COLOR
    );

    // Draw tail
    for tail in game.snake.tail.iter() {
        d.draw_rectangle(
            tail.x * TILE_SIZE, 
            tail.y * TILE_SIZE, 
            TILE_SIZE, 
            TILE_SIZE, 
            TAIL_COLOR
        );
    }

    // Draw apple
    d.draw_rectangle(
        game.apple.x * TILE_SIZE, 
        game.apple.y * TILE_SIZE, 
        TILE_SIZE, 
        TILE_SIZE, 
        APPLE_COLOR
    );
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Snake")
        .build();
    
    rl.set_target_fps(60);

    let mut game = Game::default();
    let mut timer = FRAME_TIME;

    while !rl.window_should_close() {
        timer -= rl.get_frame_time();

        handle_input(&mut game, &mut rl);

        if timer <= 0f32 {
            update_game(&mut game);
            timer = FRAME_TIME;
        }

        draw_game(&game, &mut rl, &thread);
    }
}