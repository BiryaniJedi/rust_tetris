//! Board module for managing the Tetris game grid.
//!
//! This module provides the [`Board`] struct and associated functionality
//! for representing and manipulating the Tetris playing field, including
//! piece placement, collision detection, and row completion checks.

use crate::tetromino::Tetromino;
use std::mem;

/// The width of the Tetris board in cells.
pub const BOARD_WIDTH: usize = 10;

/// The height of the Tetris board in cells.
pub const BOARD_HEIGHT: usize = 20;

/// Represents the Tetris game board.
///
/// The board is a 2D grid where `true` indicates an occupied cell
/// and `false` indicates an empty cell. The coordinate system uses
/// (0, 0) as the top-left corner.
///
/// # Examples
///
/// ```
/// let board = Board::new();
/// assert!(!board.grid[0][0]); // Initially all cells are empty
/// ```
pub struct Board {
    /// The internal grid representation. Each cell is `true` if occupied, `false` if empty.
    pub grid: [[bool; BOARD_WIDTH]; BOARD_HEIGHT],
}

impl Board {
    /// Creates a new empty board.
    ///
    /// All cells in the grid are initialized to `false` (empty).
    ///
    /// # Returns
    ///
    /// A new `Board` instance with all cells empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::new();
    /// assert!(board.pos_free(0, 0));
    /// ```
    pub fn new() -> Self {
        Board {
            grid: [[false; 10]; 20],
        }
    }

    /// Checks if a position is within the board boundaries.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate to check (column index)
    /// * `y` - The y-coordinate to check (row index)
    ///
    /// # Returns
    ///
    /// `true` if the position is within bounds, `false` otherwise.
    /// Negative coordinates or coordinates exceeding the board dimensions return `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// assert!(Board::pos_in_bounds(0, 0));
    /// assert!(Board::pos_in_bounds(9, 19));
    /// assert!(!Board::pos_in_bounds(-1, 0));
    /// assert!(!Board::pos_in_bounds(10, 20));
    /// ```
    pub fn pos_in_bounds(x: i32, y: i32) -> bool {
        if x < 0 || y < 0 {
            return false;
        }
        let x_usize = x as usize;
        let y_usize = y as usize;
        x_usize < BOARD_WIDTH && y_usize < BOARD_HEIGHT
    }

    /// Checks if a position is free (empty and within bounds).
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate to check (column index)
    /// * `y` - The y-coordinate to check (row index)
    ///
    /// # Returns
    ///
    /// `true` if the position is within bounds and unoccupied, `false` otherwise.
    /// Returns `false` for out-of-bounds positions or occupied cells.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// assert!(board.pos_free(5, 5));
    /// board.fill_pos(5, 5);
    /// assert!(!board.pos_free(5, 5));
    /// ```
    pub fn pos_free(&self, x: i32, y: i32) -> bool {
        if !(Self::pos_in_bounds(x, y)) {
            return false;
        }
        !self.grid[y as usize][x as usize]
    }

    /// Checks if a row is completely filled.
    ///
    /// A row is considered full when all cells in that row are occupied (`true`).
    ///
    /// # Arguments
    ///
    /// * `y` - The row index to check
    ///
    /// # Returns
    ///
    /// `true` if all cells in the row are occupied, `false` otherwise.
    /// Returns `false` if the row index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// // Fill the bottom row
    /// for x in 0..BOARD_WIDTH {
    ///     board.fill_pos(x as i32, 19);
    /// }
    /// assert!(board.is_row_full(19));
    /// assert!(!board.is_row_full(18));
    /// ```
    pub fn is_row_full(&self, y: i32) -> bool {
        self.grid[y as usize].iter().all(|&pos| pos)
    }

    /// Clears a row (sets all positions to free) and propogates every above row down by 1
    ///
    /// # Arguments
    ///
    /// * `y` - The row index to clear
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// // Fill the bottom row
    /// for x in 0..BOARD_WIDTH {
    ///     board.fill_pos(x as i32, 19);
    /// }
    /// board.clear_row_and_prop(10);
    /// ```
    pub fn clear_row(&mut self, y: i32) {
        let mut temp_row = [false; 10];
        if !(Self::pos_in_bounds(0, y)) {
            return;
        }
        self.grid
            .iter_mut()
            .take(y as usize + 1)
            .enumerate()
            .for_each(|(i, row)| match i {
                0 => {
                    row.iter_mut().for_each(|pos| {
                        *pos = false;
                    });
                    temp_row = *row;
                }
                _ => mem::swap(row, &mut temp_row),
            });
    }

    pub fn clear_until_not_full(&mut self, y: i32) -> u32 {
        let mut cleared_count: u32 = 0;
        while self.is_row_full(y) {
            self.clear_row(y);
            cleared_count += 1;
        }
        cleared_count
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut cleared_count: u32 = 0;
        for i in (0..BOARD_HEIGHT as i32).rev() {
            cleared_count += self.clear_until_not_full(i);
            if cleared_count as usize >= BOARD_HEIGHT {
                return cleared_count;
            }
        }
        cleared_count
    }

    /// Fills (occupies) a position on the board.
    ///
    /// Sets the specified cell to `true`, marking it as occupied.
    ///
    /// # Arguments
    ///
    /// * `x` - The x-coordinate of the cell to fill (column index)
    /// * `y` - The y-coordinate of the cell to fill (row index)
    ///
    /// # Returns
    ///
    /// `true` if the position was successfully filled, `false` if the position
    /// is out of bounds and the operation failed.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// assert!(board.fill_pos(0, 0));
    /// assert!(!board.pos_free(0, 0));
    /// assert!(!board.fill_pos(-1, 0)); // Out of bounds
    /// ```
    pub fn fill_pos(&mut self, x: i32, y: i32) -> bool {
        if !Self::pos_in_bounds(x, y) {
            return false;
        }
        self.grid[y as usize][x as usize] = true;
        true
    }

    /// Checks if a tetromino piece can be placed at its current position.
    ///
    /// A piece can be placed if all of its cells are within bounds and unoccupied.
    ///
    /// # Arguments
    ///
    /// * `piece` - A reference to the tetromino to check
    ///
    /// # Returns
    ///
    /// `true` if all cells of the piece can be placed without collision,
    /// `false` if any cell would be out of bounds or overlap with an occupied cell.
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::new();
    /// let piece = Tetromino::new(/* ... */);
    /// if board.can_place(&piece) {
    ///     // Safe to place the piece
    /// }
    /// ```
    pub fn can_place(&self, piece: &Tetromino) -> bool {
        piece.get_cords().iter().all(|&(x, y)| self.pos_free(x, y))
    }

    /// Locks a tetromino piece onto the board.
    ///
    /// Fills all cells occupied by the piece, permanently adding it to the board.
    /// This should typically be called after verifying placement with [`can_place`].
    ///
    /// # Arguments
    ///
    /// * `piece` - A reference to the tetromino to lock onto the board
    ///
    /// # Examples
    ///
    /// ```
    /// let mut board = Board::new();
    /// let piece = Tetromino::new(/* ... */);
    /// if board.can_place(&piece) {
    ///     board.lock_piece(&piece);
    /// }
    /// ```
    ///
    /// [`can_place`]: Board::can_place
    pub fn lock_piece(&mut self, piece: &Tetromino) {
        piece.get_cords().iter().for_each(|&(x, y)| {
            self.fill_pos(x, y);
        })
    }

    /// Prints the board to the console with the current piece displayed.
    ///
    /// This is a debug/visualization function that displays:
    /// - Empty cells as "0"
    /// - Locked pieces as "1"
    /// - The active piece as "X"
    ///
    /// Each row is indented with a tab character.
    ///
    /// # Arguments
    ///
    /// * `piece` - A reference to the tetromino to display on the board
    ///
    /// # Examples
    ///
    /// ```
    /// let board = Board::new();
    /// let piece = Tetromino::new(/* ... */);
    /// board.print_with_piece(&piece); // Prints visual representation to console
    /// ```
    pub fn print_with_piece(&self, piece: &Tetromino) {
        let mut board_str = [[" "; BOARD_WIDTH]; BOARD_HEIGHT];

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
