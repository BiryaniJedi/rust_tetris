pub enum Shape {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}
pub struct Tetromino {
    pub pos: (i32, i32),
    pub shape: Shape,
}

pub enum Direction {
    Left,
    Right,
    Down,
}

impl Tetromino {
    pub fn new(initial_x: i32, initial_y: i32, piece_shape: Shape) -> Self {
        Self {
            pos: (initial_x, initial_y),
            shape: piece_shape,
        }
    }

    pub fn update_pos(&mut self, new_pos: (i32, i32)) {
        self.pos = new_pos;
    }

    pub fn move_piece(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                self.pos.0 -= 1;
            }
            Direction::Right => {
                self.pos.0 += 1;
            }
            Direction::Down => {
                self.pos.1 += 1;
            }
        }
    }

    pub fn get_cords(&self) -> [(i32, i32); 4] {
        get_shape(&self.shape).map(|(x, y)| (x + self.pos.0, y + self.pos.1))
    }
}

pub fn get_shape(piece_shape: &Shape) -> [(i32, i32); 4] {
    match piece_shape {
        Shape::I => [(0, 0), (1, 0), (2, 0), (3, 0)],
        Shape::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
        Shape::T => [(0, 0), (1, 0), (2, 0), (1, 1)],
        Shape::S => [(1, 0), (2, 0), (0, 1), (1, 1)],
        Shape::Z => [(0, 0), (1, 0), (1, 1), (2, 1)],
        Shape::J => [(0, 0), (1, 0), (2, 0), (2, 1)],
        Shape::L => [(0, 0), (1, 0), (2, 0), (0, 1)],
    }
}
