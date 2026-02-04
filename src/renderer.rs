use macroquad::prelude::*;

use crate::board::{BOARD_HEIGHT, BOARD_WIDTH, Board};
use crate::tetromino::{Direction, Shape, Tetromino};

const BLOCK_SIZE: f32 = 30.0;
const BOARD_OFFSET_X: f32 = 50.0;
const BOARD_OFFSET_Y: f32 = 50.0;

pub fn draw_board(board: &Board, current_piece: Option<&Tetromino>) {
    let mut temp_piece: Tetromino;
    clear_background(BLACK);

    draw_rectangle_lines(
        BOARD_OFFSET_X,
        BOARD_OFFSET_Y,
        BOARD_WIDTH as f32 * BLOCK_SIZE,
        BOARD_HEIGHT as f32 * BLOCK_SIZE,
        2.0,
        WHITE,
    );

    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if board.grid[y][x] {
                draw_block(x as i32, y as i32, GRAY);
            }
        }
    }

    if let Some(piece) = current_piece
        && board.can_place(piece)
    {
        temp_piece = Tetromino::new(piece.pos, piece.shape);
        temp_piece.rotation = piece.rotation;
        while board.can_place(&temp_piece) {
            temp_piece.move_piece(Direction::Down);
        }

        if temp_piece.pos != piece.pos {
            temp_piece.move_piece(Direction::Up);
        }

        let color = get_piece_color(&piece.shape);
        for (x, y) in temp_piece.get_cords() {
            draw_block(x, y, BLACK);
        }
        for (x, y) in piece.get_cords() {
            draw_block(x, y, color);
        }
    }
}

fn draw_block(x: i32, y: i32, color: Color) {
    let pixel_x = BOARD_OFFSET_X + x as f32 * BLOCK_SIZE;
    let pixel_y = BOARD_OFFSET_Y + y as f32 * BLOCK_SIZE;
    draw_rectangle(pixel_x, pixel_y, BLOCK_SIZE, BLOCK_SIZE, color);
    draw_rectangle_lines(pixel_x, pixel_y, BLOCK_SIZE, BLOCK_SIZE, 2.0, WHITE);
}

fn get_piece_color(shape: &Shape) -> Color {
    match shape {
        Shape::I => SKYBLUE,
        Shape::O => YELLOW,
        Shape::T => PURPLE,
        Shape::S => GREEN,
        Shape::Z => RED,
        Shape::J => BLUE,
        Shape::L => ORANGE,
    }
}

pub fn draw_ui(score: u32, level: u32) {
    let ui_x = BOARD_OFFSET_X + (BOARD_WIDTH as f32 * BLOCK_SIZE) + 30.0;

    draw_text("TETRIS", ui_x, 80.0, 40.0, WHITE);
    draw_text(&format!("Score: {}", score), ui_x, 140.0, 30.0, WHITE);
    draw_text(&format!("Level: {}", level), ui_x, 180.0, 30.0, WHITE);
}
