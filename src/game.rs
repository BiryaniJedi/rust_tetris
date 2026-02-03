use crate::board::Board;
use crate::tetromino::{Shape, Tetromino};

pub enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    board: Board,
    current_piece: Option<Tetromino>,
    state: GameState,
    score: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_piece: None,
            state: GameState::Playing,
            score: 0,
        }
    }

    pub fn set_random_piece(&mut self) {
        let rand_piece_index = fastrand::u8(0..7); //7 different shapes
        let mut counter: u8 = 0;
        self.current_piece = Some(Tetromino::new((3, 0), match_shape(rand_piece_index)));
        while counter < 7 {
            if let Some(temp_piece) = &self.current_piece
                && self.board.can_place(temp_piece)
            {
                return;
            }
            counter += 1;
            self.current_piece = Some(Tetromino::new(
                (3, 0),
                match_shape((rand_piece_index + counter) % 7),
            ));
        }
        self.state = GameState::GameOver;
    }
}

pub fn match_shape(shape_index: u8) -> Shape {
    match shape_index {
        0 => Shape::I,
        1 => Shape::O,
        2 => Shape::T,
        3 => Shape::S,
        4 => Shape::Z,
        5 => Shape::J,
        6 => Shape::L,
        _ => Shape::T,
    }
}
