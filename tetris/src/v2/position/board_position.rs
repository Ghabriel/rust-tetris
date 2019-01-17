use std::ops::{Add, AddAssign};
use super::BoardPositionOffset;

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
            row: (self_row + other.get_row()) as usize,
            column: (self_column + other.get_column()) as usize,
        }
    }
}

impl AddAssign<&BoardPositionOffset> for BoardPosition {
    fn add_assign(&mut self, other: &BoardPositionOffset) {
        let self_row = self.row as isize;
        let self_column = self.column as isize;

        self.row = (self_row + other.get_row()) as usize;
        self.column = (self_column + other.get_column()) as usize;
    }
}
