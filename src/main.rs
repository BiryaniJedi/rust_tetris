mod board;
mod game;
mod renderer;
mod tetromino;

//use crate::fastrand;

use game::{Game, GameState};
use renderer::{draw_board, draw_ui};
use tetromino::Direction;

use macroquad::prelude::*;

const INITIAL_FALL_DELAY: f64 = 0.5;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Tetris".to_owned(),
        window_width: 600,
        window_height: 800,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    let mut last_fall_time = get_time();
    let mut last_level_up_time = get_time();
    let mut fall_delay = 0.5; // Seconds between automatic falls
    let level_up_delay = 30.0; // Seconds between automatic falls
    let mut level = 1;
    loop {
        // Handle input
        if is_key_pressed(KeyCode::Left) {
            let _ = game.try_move(Direction::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            let _ = game.try_move(Direction::Right);
        }
        if is_key_pressed(KeyCode::Down) {
            fall_delay *= 0.5;
        }
        if is_key_released(KeyCode::Down) {
            fall_delay *= 2.0;
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
            let _ = game.try_rotate_clock();
        }
        if is_key_pressed(KeyCode::Z) {
            let _ = game.try_rotate_counter();
        }
        if is_key_pressed(KeyCode::Space) {
            game.hard_lock();
        }

        let current_time = get_time();
        if current_time - last_level_up_time > level_up_delay {
            level += 1;
            fall_delay *= 0.8;
            last_level_up_time = current_time;
        }
        if current_time - last_fall_time > fall_delay {
            game.tick_down();
            last_fall_time = current_time;
        }

        draw_board(&game.board, game.current_piece.as_ref());
        draw_ui(game.score, level);

        // Check game over
        if matches!(game.state, GameState::GameOver) {
            draw_text("GAME OVER", 200.0, 300.0, 50.0, RED);
            draw_text(
                &format!("Final Score: {}", game.score),
                200.0,
                360.0,
                30.0,
                WHITE,
            );
            draw_text("Press R to restart", 200.0, 400.0, 20.0, GRAY);

            if is_key_pressed(KeyCode::R) {
                game = Game::new();
                last_fall_time = get_time();
                level = 1;
                last_level_up_time = get_time();
                fall_delay = INITIAL_FALL_DELAY;
            }
        }

        next_frame().await;
    }
}
