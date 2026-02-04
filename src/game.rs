use crate::board::Board;
use crate::tetromino::{Direction, Shape, Tetromino};

#[derive(Debug, Copy, Clone)]
pub enum GameState {
    Playing,
    GameOver,
}

pub struct Game {
    pub board: Board,
    pub current_piece: Option<Tetromino>,
    pub state: GameState,
    pub score: u32,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Self {
            board: Board::new(),
            current_piece: None,
            state: GameState::Playing,
            score: 0,
        };
        game.set_random_piece();
        game
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

    pub fn try_move(&mut self, direction: Direction) -> Result<(), String> {
        if let Some(piece) = &mut self.current_piece {
            //compute new position after movement
            let old_direction = match &direction {
                Direction::Left => Direction::Right,
                Direction::Down => Direction::Up,
                Direction::Right => Direction::Left,
                Direction::Up => Direction::Down,
            };
            piece.move_piece(direction);
            if self.board.can_place(piece) {
                return Ok(());
            }
            piece.move_piece(old_direction);
            return Err("Move invalid".to_string());
        }
        Err("No current piece".to_string())
    }

    pub fn try_rotate_clock(&mut self) -> Result<(), String> {
        if let Some(piece) = &mut self.current_piece {
            //compute new position after movement
            piece.rotate_cw();
            if self.board.can_place(piece) {
                return Ok(());
            }
            piece.rotate_ccw();
            return Err("Rotation invalid".to_string());
        }
        Err("No current piece".to_string())
    }
    pub fn try_rotate_counter(&mut self) -> Result<(), String> {
        if let Some(piece) = &mut self.current_piece {
            //compute new position after movement
            piece.rotate_ccw();
            if self.board.can_place(piece) {
                return Ok(());
            }
            piece.rotate_cw();
            return Err("Rotation invalid".to_string());
        }
        Err("No current piece".to_string())
    }
    pub fn lock_current_piece(&mut self) -> Result<(), String> {
        if let Some(piece) = &self.current_piece {
            self.board.lock_piece(piece);
            let lines_cleared = self.board.clear_lines();
            self.score += match lines_cleared {
                1 => 100,
                2 => 300,
                3 => 500,
                4 => 800,
                _ => 0,
            };
            self.set_random_piece();
            return Ok(());
        }
        Err("No current piece or place to put it!".to_string())
    }
    pub fn tick_down(&mut self) {
        if self.try_move(Direction::Down).is_ok() {
            return;
        }
        let _ = self.lock_current_piece();
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
