use super::super::super::{
    board::SimpleBoard,
    gravity::NaiveGravity,
    settings::Settings,
};
use super::BoardGravityPair;

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
    fn clear_rows(&mut self, rows: &[usize], settings: &Settings) {
        self.gravity.clear_rows(&mut self.board, rows, settings);
    }
}
