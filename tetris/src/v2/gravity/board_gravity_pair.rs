use super::super::board::Board;
use super::super::settings::Settings;

pub trait BoardGravityPair {
    fn board(&mut self) -> &mut Board;
    fn clear_rows(&mut self, rows: &[usize], settings: &Settings);
}
