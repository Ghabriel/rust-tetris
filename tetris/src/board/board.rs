use super::super::core::Direction;
use super::super::piece::Piece;
use super::super::position::BoardPosition;
use super::super::rotations::RotationSystem;
use super::super::settings::Settings;
use super::{Block, MaterializationStatus};

pub trait Board {
    /**
     * Required methods
     */
    fn get_num_columns(&self) -> usize;
    fn get_num_rows(&self) -> usize;
    fn is_occupied(&self, position: &BoardPosition) -> bool;
    fn is_touching_wall(&self, position: &BoardPosition, wall_direction: &Direction) -> bool;
    fn is_in_bounds(&self, position: &BoardPosition) -> bool;
    fn materialize(
        &mut self,
        piece: &Piece,
        position: &BoardPosition,
        rotation_system: &RotationSystem
    ) -> MaterializationStatus;
    fn get_filled_rows(&self) -> Vec<usize>;
    fn clear_rows(&mut self, rows: &[usize], settings: &Settings);

    fn for_each_row(&self, callback: &mut FnMut(&Vec<&Option<Block>>));

    /**
     * Provided methods
     */
    fn clear_filled_rows(&mut self, settings: &Settings) {
        let filled_rows = self.get_filled_rows();

        self.clear_rows(&filled_rows, settings);
    }
}
