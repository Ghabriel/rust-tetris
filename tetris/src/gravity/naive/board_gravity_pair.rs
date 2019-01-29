use super::super::super::board::{Board, SimpleBoard};
use super::super::BoardGravityPair;
use super::NaiveGravity;

pub struct NaiveGravityPair {
    board: SimpleBoard,
    gravity: NaiveGravity,
}

impl NaiveGravityPair {
    pub fn new(board: SimpleBoard, gravity: NaiveGravity) -> NaiveGravityPair {
        NaiveGravityPair {
            board,
            gravity,
        }
    }
}

impl BoardGravityPair for NaiveGravityPair {
    fn board(&self) -> &Board {
        &self.board
    }

    fn board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    fn clear_rows(&mut self, rows: &[usize]) {
        self.gravity.clear_rows(&mut self.board, rows);
    }
}
