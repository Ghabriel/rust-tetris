use board::Position;
use piece::PieceGrid;

pub struct NormalizedCell {
    pub grid_line: usize,
    pub grid_column: usize,
    pub board_line: usize,
    pub board_column: usize,
}

pub struct NormalizedCellIterationData<'a> {
    normalized_position: Position,
    grid: &'a PieceGrid,
    num_columns: usize,
}

impl<'a> NormalizedCellIterationData<'a> {
    pub fn new(
        normalized_position: Position,
        grid: &'a PieceGrid,
        num_columns: usize,
    ) -> NormalizedCellIterationData<'a> {
        NormalizedCellIterationData {
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
    pub fn new(data: NormalizedCellIterationData<'a>) -> NormalizedCellIterator<'a> {
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
    pub fn new(data: NormalizedCellIterationData<'a>) -> NormalizedCellReverseIterator<'a> {
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