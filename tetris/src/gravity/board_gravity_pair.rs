use super::super::board::Board;
use super::super::settings::Settings;

pub trait BoardGravityPair {
    fn board(&self) -> &Board;
    fn board_mut(&mut self) -> &mut Board;
    fn clear_rows(&mut self, rows: &[usize], settings: &Settings);
}
