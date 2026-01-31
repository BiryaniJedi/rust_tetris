use crate::tetromino::Tetromino;

pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub struct Board {
    pub grid: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        Board {
            grid: [[false; 10]; 20],
        }
    }

    pub fn pos_in_bounds(x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let x_usize = x as usize;
        let y_usize = y as usize;
        x_usize < BOARD_WIDTH && y_usize < BOARD_HEIGHT
    }

    pub fn pos_free(&self, x: i32, y: i32) -> bool {
        if !(Self::pos_in_bounds(x, y)) {
            return false;
        }
        !self.grid[y as usize][x as usize]
    }

    pub fn fill_pos(&mut self, x: i32, y: i32) -> bool {
        if !Self::pos_in_bounds(x, y) {
            return false;
        }
        self.grid[y as usize][x as usize] = true;
        true
    }

    pub fn can_place(&self, piece: &Tetromino) -> bool {
        piece.get_cords().iter().all(|&(x, y)| self.pos_free(x, y))
    }

    pub fn lock_piece(&mut self, piece: &Tetromino) {
        piece.get_cords().iter().for_each(|&(x, y)| {
            self.fill_pos(x, y);
        })
    }

    pub fn print_with_piece(&self, piece: &Tetromino) {
        let mut board_str = [["0"; BOARD_WIDTH]; BOARD_HEIGHT];

        piece.get_cords().iter().for_each(|&(x, y)| {
            if Board::pos_in_bounds(x, y) {
                board_str[y as usize][x as usize] = "X";
            }
        });

        self.grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &pos)| {
                if pos {
                    board_str[i][j] = "1";
                }
            });
        });

        board_str.iter().for_each(|row| {
            print!("\t");
            row.iter().for_each(|&cell| {
                print!("{} ", cell);
            });
            println!();
        })
    }
}
