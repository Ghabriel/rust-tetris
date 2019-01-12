use super::super::piece::Piece;
use super::super::position::{BoardPosition, PiecePosition};
use super::super::rotations::RotationSystem;

pub fn get_piece_iterator<'a>(
    piece: &Piece,
    piece_position: &'a BoardPosition,
    rotation_system: &'a RotationSystem
) -> impl Iterator<Item = BoardPosition> + 'a {
    let grid = piece.get_grid(rotation_system);
    let grid_size = grid.0.len();
    let grid_num_columns = (grid_size as f32).sqrt() as usize;

    grid.0.iter()
        .enumerate()
        .filter(|(_, tile)| **tile)
        .map(|(tile_index, _)| {
            let block_in_piece_coordinates = PiecePosition::from_index(
                tile_index,
                grid_num_columns
            );

            BoardPosition::new(
                block_in_piece_coordinates.get_row() + piece_position.get_row(),
                block_in_piece_coordinates.get_column() + piece_position.get_column(),
            )
        })
}
