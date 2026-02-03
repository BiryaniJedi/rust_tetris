mod board;
mod game;
mod tetromino;

//use crate::fastrand;

use board::Board;
use game::Game;
use tetromino::{Direction, Tetromino};

use std::{thread, time::Duration};

fn main() {
    let mut game = Game::new();
    println!("Starting Tetris!");
    println!("Pieces will fall automatically...\n");
    for tick in 0..100 {
        println!("\n========= Tick {} =========", tick);
        game.display();
        println!("Score: {}", game.get_score());

        // Optional: add some random moves for testing
        if tick % 7 == 0 {
            let _ = game.try_move(Direction::Left);
        }
        if tick % 11 == 0 {
            let _ = game.try_rotate_clock();
        }

        // Wait a bit so you can see it
        thread::sleep(Duration::from_millis(500));

        // Move piece down (or lock it)
        game.tick_down();

        // Check for game over
        if matches!(game.get_state(), game::GameState::GameOver) {
            println!("\n╔════════════════════╗");
            println!("║   GAME OVER!       ║");
            println!("║   Final Score: {:4} ║", game.get_score());
            println!("╚════════════════════╝");
            break;
        }
    }
}
