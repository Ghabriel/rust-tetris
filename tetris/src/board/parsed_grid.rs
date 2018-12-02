use board::{Board, Position};
use piece::PieceGrid;

pub struct ParsedGrid<'a> {
    grid: &'a PieceGrid,
    num_columns: usize,
    size: usize,
}

impl<'a> ParsedGrid<'a> {
    pub fn new(grid: &'a PieceGrid) -> ParsedGrid<'a> {
        let grid_size = grid.0.len();
        let num_columns = (grid_size as f64).sqrt() as usize;

        ParsedGrid {
            grid,
            num_columns,
            size: grid_size,
        }
    }

    pub fn is_touching_board(&self, board: &Board, normalized_position: Position) -> bool {
        let Position(base_line, base_column) = normalized_position;

        for (reverse_index, cell) in self.grid.0.iter().rev().enumerate() {
            if !cell {
                continue;
            }

            let index = self.size - 1 - reverse_index;

            let cell_grid_line = index / self.num_columns;
            let cell_grid_column = index % self.num_columns;

            let cell_line = base_line + cell_grid_line;
            let cell_column = base_column + cell_grid_column;
            let cell_position = Position(cell_line, cell_column);

            let line_below_is_occupied = self.is_occupied(
                cell_grid_line + 1,
                cell_grid_column
            );

            if !line_below_is_occupied && board.cell_touches_board(cell_position) {
                return true;
            }
        }

        false
    }

    fn is_occupied(&self, row: usize, column: usize) -> bool {
        let index = row * self.num_columns + column;

        self.grid.0[index]
    }
}
