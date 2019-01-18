use std::ops::Add;

// TODO: reduce the redundancy between BoardPosition, PiecePosition and WindowPosition

pub struct WindowPosition {
    row: f32,
    column: f32,
}

impl WindowPosition {
    pub fn new(row: f32, column: f32) -> WindowPosition {
        WindowPosition { row, column }
    }

    pub fn as_xy(&self) -> (f32, f32) {
        (self.column, self.row)
    }

    pub fn get_row(&self) -> f32 {
        self.row
    }

    pub fn get_column(&self) -> f32 {
        self.column
    }
}

impl Add<&WindowPosition> for WindowPosition {
    type Output = WindowPosition;

    fn add(self, other: &WindowPosition) -> WindowPosition {
        WindowPosition {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}
