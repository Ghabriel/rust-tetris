use std::ops::Add;

// TODO: reduce the redundancy between BoardPosition, PiecePosition and WindowPosition

pub struct BoardPosition {
    row: usize,
    column: usize,
}

impl BoardPosition {
    pub fn new(row: usize, column: usize) -> BoardPosition {
        BoardPosition { row, column }
    }

    pub fn from_index(index: usize, num_columns: usize) -> BoardPosition {
        BoardPosition {
            row: index / num_columns,
            column: index % num_columns,
        }
    }

    pub fn to_index(&self, num_columns: usize) -> usize {
        self.row * num_columns + self.column
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_column(&self) -> usize {
        self.column
    }
}

impl Add<&BoardPosition> for BoardPosition {
    type Output = BoardPosition;

    fn add(self, other: &BoardPosition) -> BoardPosition {
        BoardPosition {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}
