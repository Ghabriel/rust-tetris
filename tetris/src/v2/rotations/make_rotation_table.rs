use super::super::piece::PieceGrid;
use super::RotationTable;

pub fn make_rotation_table(grids: &[&[&str]]) -> RotationTable {
    let mut result = Vec::new();

    for grid in grids {
        let mut parsed_grid = PieceGrid(Vec::new());

        for row in grid.iter() {
            for value in row.chars() {
                parsed_grid.0.push(value == '1');
            }
        }

        result.push(parsed_grid);
    }

    result
}
