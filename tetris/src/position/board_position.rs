use std::ops::{Add, AddAssign};
use super::BoardPositionOffset;

// TODO: reduce the redundancy between BoardPosition, PiecePosition and WindowPosition

pub struct BoardPosition {
    pub row: isize,
    pub column: isize,
}

impl BoardPosition {
    pub fn new(row: isize, column: isize) -> BoardPosition {
        BoardPosition { row, column }
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

impl AddAssign for BoardPosition {
    fn add_assign(&mut self, other: BoardPosition) {
        self.row += other.row;
        self.column += other.column;
    }
}


impl Add<&BoardPositionOffset> for BoardPosition {
    type Output = BoardPosition;

    fn add(self, other: &BoardPositionOffset) -> BoardPosition {
        let self_row = self.row as isize;
        let self_column = self.column as isize;

        BoardPosition {
            row: self_row + other.get_row(),
            column: self_column + other.get_column(),
        }
    }
}

impl AddAssign<&BoardPositionOffset> for BoardPosition {
    fn add_assign(&mut self, other: &BoardPositionOffset) {
        let self_row = self.row as isize;
        let self_column = self.column as isize;

        self.row = self_row + other.get_row();
        self.column = self_column + other.get_column();
    }
}
