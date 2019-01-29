use super::super::board::Board;

pub trait BoardGravityPair {
    fn board(&self) -> &Board;
    fn board_mut(&mut self) -> &mut Board;
    fn clear_rows(&mut self, rows: &[usize]);
}
