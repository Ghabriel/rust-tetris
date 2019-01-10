use super::super::super::piece::Piece;
use super::super::super::rotations::RotationSystem;

pub fn to_board_coordinates<'a>(
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
            let (row, column) = piece_to_board_coordinates(
                board_num_columns,
                cell_index,
                grid_num_columns,
                position
            );

            row * board_num_columns + column
        })
}

pub fn piece_to_board_coordinates(
    board_num_columns: usize,
    piece_cell_index: usize,
    piece_grid_num_columns: usize,
    piece_position: usize
) -> (usize, usize) {
    let grid_in_board_coordinates = index_to_coordinates(piece_position, board_num_columns);
    let cell_in_grid_coordinates = index_to_coordinates(piece_cell_index, piece_grid_num_columns);

    (
        cell_in_grid_coordinates.0 + grid_in_board_coordinates.0,
        cell_in_grid_coordinates.1 + grid_in_board_coordinates.1,
    )
}

pub fn index_to_coordinates(index: usize, num_columns: usize) -> (usize, usize) {
    (index / num_columns, index % num_columns)
}
