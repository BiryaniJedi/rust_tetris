mod board;
mod game;
mod tetromino;

//use crate::fastrand;

use board::Board;
use tetromino::{Shape, Tetromino};
fn main() {
    let board = Board::new();
    let mut piece = Tetromino::new((3, 5), Shape::T);

    println!("Rotation 0:");
    board.print_with_piece(&piece);

    piece.rotate_cw();
    println!("\nRotation 1:");
    board.print_with_piece(&piece);

    piece.rotate_cw();
    println!("\nRotation 2:");
    board.print_with_piece(&piece);

    piece.rotate_cw();
    println!("\nRotation 3:");
    board.print_with_piece(&piece);
}
