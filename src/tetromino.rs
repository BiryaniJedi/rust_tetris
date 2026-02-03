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
    pub rotation: u8,
}

pub enum Direction {
    Left,
    Down,
    Right,
    Up,
}

pub enum Rotation {
    Clock,
    Counter,
}

impl Tetromino {
    pub fn new(initial_cords: (i32, i32), piece_shape: Shape) -> Self {
        Self {
            pos: initial_cords,
            shape: piece_shape,
            rotation: 0,
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
            //reversal purposes
            Direction::Up => {
                self.pos.1 -= 1;
            }
        }
    }

    pub fn rotate_cw(&mut self) {
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn rotate_ccw(&mut self) {
        self.rotation = (self.rotation + 3) % 4;
    }
    pub fn get_cords(&self) -> [(i32, i32); 4] {
        get_shape(&self.shape, self.rotation).map(|(x, y)| (x + self.pos.0, y + self.pos.1))
    }
}

pub fn get_shape(piece_shape: &Shape, rot: u8) -> [(i32, i32); 4] {
    match (piece_shape, rot) {
        //I shapes

        //####
        (Shape::I, 0) => [(0, 0), (1, 0), (2, 0), (3, 0)],

        // #
        // #
        // #
        // #
        (Shape::I, 1) => [(1, 0), (1, 1), (1, 2), (1, 3)],

        //####
        (Shape::I, 2) => [(0, 0), (1, 0), (2, 0), (3, 0)],

        // #
        // #
        // #
        // #
        (Shape::I, 3) => [(1, 0), (1, 1), (1, 2), (1, 3)],

        //O Shapes

        //## (all)
        //##
        (Shape::O, 0) => [(0, 0), (1, 0), (0, 1), (1, 1)],
        (Shape::O, 1) => [(0, 0), (1, 0), (0, 1), (1, 1)],
        (Shape::O, 2) => [(0, 0), (1, 0), (0, 1), (1, 1)],
        (Shape::O, 3) => [(0, 0), (1, 0), (0, 1), (1, 1)],

        //T Shapes

        //###
        // #
        (Shape::T, 0) => [(0, 0), (1, 0), (2, 0), (1, 1)],

        // #
        //##
        // #
        (Shape::T, 1) => [(1, 0), (0, 1), (1, 1), (1, 2)],

        // #
        //###
        (Shape::T, 2) => [(1, 0), (0, 1), (1, 1), (2, 1)],

        //#
        //##
        //#
        (Shape::T, 3) => [(0, 0), (0, 1), (1, 1), (0, 2)],

        //S Shapes

        // ##
        //##
        (Shape::S, 0) => [(1, 0), (2, 0), (0, 1), (1, 1)],

        //#
        //##
        // #
        (Shape::S, 1) => [(0, 0), (0, 1), (1, 1), (1, 2)],

        // ##
        //##
        (Shape::S, 2) => [(1, 0), (2, 0), (0, 1), (1, 1)],

        //#
        //##
        // #
        (Shape::S, 3) => [(0, 0), (0, 1), (1, 1), (1, 2)],

        //Z Shapes

        //##
        // ##
        (Shape::Z, 0) => [(0, 0), (1, 0), (1, 1), (2, 1)],

        // #
        //##
        //#
        (Shape::Z, 1) => [(1, 0), (0, 1), (1, 1), (0, 2)],

        //##
        // ##
        (Shape::Z, 2) => [(0, 0), (1, 0), (1, 1), (2, 1)],

        // #
        //##
        //#
        (Shape::Z, 3) => [(1, 0), (0, 1), (1, 1), (0, 2)],

        //J Shapes

        //###
        //  #
        (Shape::J, 0) => [(0, 0), (1, 0), (2, 0), (2, 1)],

        // #
        // #
        //##
        (Shape::J, 1) => [(1, 0), (1, 1), (0, 2), (1, 2)],

        //#
        //###
        (Shape::J, 2) => [(0, 0), (0, 1), (1, 1), (2, 1)],

        //##
        //#
        //#
        (Shape::J, 3) => [(0, 0), (1, 0), (0, 1), (0, 2)],

        //L Shapes

        //###
        //#
        (Shape::L, 0) => [(0, 0), (1, 0), (2, 0), (0, 1)],

        //##
        // #
        // #
        (Shape::L, 1) => [(0, 0), (1, 0), (1, 1), (1, 2)],

        //  #
        //###
        (Shape::L, 2) => [(2, 0), (0, 1), (1, 1), (2, 1)],

        //#
        //#
        //##
        (Shape::L, 3) => [(0, 0), (0, 1), (0, 2), (1, 2)],

        //Default impossible if used correctly
        (_, _) => panic!("Invalid rotation state"),
    }
}
