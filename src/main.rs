mod board;
mod tetromino;

use board::Board;
use tetromino::{Shape, Tetromino};
fn main() {
    let mut board = Board::new();
    let piece = Tetromino::new(3, 17, Shape::I);

    println!("Before locking:");
    board.print_with_piece(&piece);

    board.lock_piece(&piece);

    println!("\nAfter locking (no piece overlay):");
    let dummy = Tetromino::new(-10, -10, Shape::I); // Off-screen piece
    board.print_with_piece(&dummy);
}
