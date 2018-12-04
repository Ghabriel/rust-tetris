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
        for (reverse_index, cell) in self.grid.0.iter().rev().enumerate() {
            if !cell {
                continue;
            }

            let index = self.size - 1 - reverse_index;

            if self.is_cell_touching_board(index, board, &normalized_position) {
                return true;
            }
        }

        false
    }

    fn is_cell_touching_board(
        &self,
        cell_index: usize,
        board: &Board,
        normalized_position: &Position
    ) -> bool {
        let Position(base_line, base_column) = normalized_position;

        let cell_grid_line = cell_index / self.num_columns;
        let cell_grid_column = cell_index % self.num_columns;

        let cell_board_line = base_line + cell_grid_line;
        let cell_board_column = base_column + cell_grid_column;

        let grid_below_is_occupied = self.is_occupied(cell_grid_line + 1, cell_grid_column);
        let board_below_is_occupied = board.is_occupied(cell_board_line + 1, cell_board_column);

        !grid_below_is_occupied && board_below_is_occupied
    }

    pub fn is_occupied(&self, row: usize, column: usize) -> bool {
        let index = row * self.num_columns + column;

        self.grid.0[index]
    }
}
