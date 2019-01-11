use std::ops::Add;

// TODO: reduce the redundancy between BoardPosition, PiecePosition and WindowPosition

pub struct PiecePosition {
    row: usize,
    column: usize,
}

impl PiecePosition {
    pub fn new(row: usize, column: usize) -> PiecePosition {
        PiecePosition { row, column }
    }

    pub fn from_index(index: usize, num_columns: usize) -> PiecePosition {
        PiecePosition {
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

impl Add<&PiecePosition> for PiecePosition {
    type Output = PiecePosition;

    fn add(self, other: &PiecePosition) -> PiecePosition {
        PiecePosition {
            row: self.row + other.row,
            column: self.column + other.column,
        }
    }
}
