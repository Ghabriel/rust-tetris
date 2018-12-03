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

    fn is_occupied(&self, row: usize, column: usize) -> bool {
        let index = row * self.num_columns + column;

        self.grid.0[index]
    }

    pub fn normalized_cell_iter(
        &'a self,
        board: &'a Board,
        normalized_position: Position
    ) -> NormalizedCellIterator<'a> {
        NormalizedCellIterator::new(
            NormalizedCellIterationData::new(
                board,
                normalized_position,
                self.grid,
                self.num_columns
            )
        )
    }

    pub fn normalized_cell_iter_rev(
        &'a self,
        board: &'a Board,
        normalized_position: Position
    ) -> NormalizedCellReverseIterator<'a> {
        NormalizedCellReverseIterator::new(
            NormalizedCellIterationData::new(
                board,
                normalized_position,
                self.grid,
                self.num_columns
            )
        )
    }
}

pub struct NormalizedCell {
    grid_line: usize,
    grid_column: usize,
    board_line: usize,
    board_column: usize,
}

struct NormalizedCellIterationData<'a> {
    board: &'a Board,
    normalized_position: Position,
    grid: &'a PieceGrid,
    num_columns: usize,
}

impl<'a> NormalizedCellIterationData<'a> {
    fn new(
        board: &'a Board,
        normalized_position: Position,
        grid: &'a PieceGrid,
        num_columns: usize,
    ) -> NormalizedCellIterationData<'a> {
        NormalizedCellIterationData {
            board,
            normalized_position,
            grid,
            num_columns,
        }
    }

    fn get_normalized_cell(&self, index: usize) -> NormalizedCell {
        let Position(base_line, base_column) = self.normalized_position;

        let cell_grid_line = index / self.num_columns;
        let cell_grid_column = index % self.num_columns;

        NormalizedCell {
            grid_line: cell_grid_line,
            grid_column: cell_grid_column,
            board_line: base_line + cell_grid_line,
            board_column: base_column + cell_grid_column,
        }
    }
}

pub struct NormalizedCellIterator<'a> {
    data: NormalizedCellIterationData<'a>,
    index: usize,
}

impl<'a> NormalizedCellIterator<'a> {
    fn new(data: NormalizedCellIterationData<'a>) -> NormalizedCellIterator<'a> {
        NormalizedCellIterator { data, index: 0 }
    }
}

impl<'a> Iterator for NormalizedCellIterator<'a> {
    type Item = NormalizedCell;

    fn next(&mut self) -> Option<Self::Item> {
        let grid = &self.data.grid.0;

        while self.index < grid.len() && !grid[self.index] {
            self.index += 1;
        }

        if self.index >= grid.len() {
            return None;
        }

        let normalized_cell = self.data.get_normalized_cell(self.index);
        self.index += 1;

        Some(normalized_cell)
    }
}


pub struct NormalizedCellReverseIterator<'a> {
    data: NormalizedCellIterationData<'a>,
    reverse_index: usize,
}

impl<'a> NormalizedCellReverseIterator<'a> {
    fn new(data: NormalizedCellIterationData<'a>) -> NormalizedCellReverseIterator<'a> {
        NormalizedCellReverseIterator { data, reverse_index: 0 }
    }

    fn index(&self) -> usize {
        self.data.grid.0.len() - 1 - self.reverse_index
    }
}

impl<'a> Iterator for NormalizedCellReverseIterator<'a> {
    type Item = NormalizedCell;

    fn next(&mut self) -> Option<Self::Item> {
        let grid = &self.data.grid.0;

        while self.reverse_index < grid.len() && !grid[self.index()] {
            self.reverse_index += 1;
        }

        if self.reverse_index >= grid.len() {
            return None;
        }

        let normalized_cell = self.data.get_normalized_cell(self.index());
        self.reverse_index += 1;

        Some(normalized_cell)
    }
}
