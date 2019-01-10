use super::super::super::piece::Piece;
use super::super::super::position::Position;
use super::super::super::rotations::RotationSystem;

pub fn piece_to_board_coordinates<'a>(
    board_num_columns: usize,
    piece: &'a Piece,
    position: usize,
    rotation_system: &'a RotationSystem
) -> impl Iterator<Item = usize> + 'a {
    let grid = piece.get_grid(rotation_system);
    let grid_num_columns = (grid.0.len() as f64).sqrt() as usize;

    grid.0.iter()
        .enumerate()
        .filter(|(_, cell)| **cell)
        .map(move |(cell_index, _)| {
            piece_cell_to_board_coordinates(
                board_num_columns,
                cell_index,
                grid_num_columns,
                position
            ).to_index(board_num_columns)
        })
}

pub fn piece_cell_to_board_coordinates(
    board_num_columns: usize,
    cell_index_in_grid: usize,
    piece_grid_num_columns: usize,
    piece_index_in_board: usize
) -> Position {
    let grid_in_board_coordinates = Position::from_index(piece_index_in_board, board_num_columns);
    let cell_in_grid_coordinates = Position::from_index(cell_index_in_grid, piece_grid_num_columns);

    cell_in_grid_coordinates + grid_in_board_coordinates
}
