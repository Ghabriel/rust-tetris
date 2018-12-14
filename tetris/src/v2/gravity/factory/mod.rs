use super::super::super::board::SimpleBoard;
use super::naive::NaiveGravity;

pub struct NaiveFactory {}

impl NaiveFactory {
    pub fn make_board(num_columns: usize, num_rows: usize) -> SimpleBoard {
        SimpleBoard::new(num_columns, num_rows)
    }

    pub fn make_gravity() -> NaiveGravity {
        NaiveGravity::new()
    }
}
