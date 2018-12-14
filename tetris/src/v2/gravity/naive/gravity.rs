use super::super::super::board::SimpleBoard;
use super::super::super::settings::Settings;

pub struct NaiveGravity {}

impl NaiveGravity {
    pub fn new() -> NaiveGravity {
        NaiveGravity { }
    }

    pub fn clear_rows(&self, board: &mut SimpleBoard, rows: &[usize], settings: &Settings) {
        // TODO
    }
}
