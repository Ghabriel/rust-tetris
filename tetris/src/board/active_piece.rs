use board::{Board, Position};
use piece::{Piece, PieceGrid};
use rotations::RotationSystem;
use settings::Settings;

pub struct ActivePiece {
    piece: Piece,
    position: usize,
}

impl ActivePiece {
    pub fn new(piece: Piece, position: usize) -> ActivePiece {
        ActivePiece {
            piece,
            position,
        }
    }

    pub fn shift(&mut self, offset: isize) {
        let unsigned_offset = if offset >= 0 {
            offset as usize
        } else {
            -offset as usize
        };

        if offset >= 0 {
            self.position += unsigned_offset;
        } else if unsigned_offset <= self.position {
            self.position -= unsigned_offset;
        } else {
            self.position = 0;
        }
    }

    fn get_normalized_position(&self, board: &Board) -> Position {
        let line = self.position / board.get_num_columns();
        let column = self.position % board.get_num_columns();

        Position(line, column)
    }

    pub fn is_grid_cell_occupied(
        &self,
        row: usize,
        column: usize,
        rotation_system: &RotationSystem
    ) -> bool {
        let piece_grid = self.piece.get_grid(rotation_system);
        let grid_num_columns = (piece_grid.0.len() as f64).sqrt() as usize;
        let index = row * grid_num_columns + column;

        piece_grid.0[index]
    }

    pub fn normalized_cell_iter<'a>(
        &'a self,
        board: &'a Board,
        settings: &'a Settings
    ) -> NormalizedCellIterator<'a> {
        let normalized_position = self.get_normalized_position(board);
        let piece_grid = self.piece.get_grid(&settings.rotation_system);
        let grid_size = piece_grid.0.len();
        let num_columns = (grid_size as f64).sqrt() as usize;

        NormalizedCellIterator::new(
            NormalizedCellIterationData::new(
                normalized_position,
                piece_grid,
                num_columns
            )
        )
    }

    pub fn normalized_cell_iter_rev<'a>(
        &'a self,
        board: &'a Board,
        settings: &'a Settings
    ) -> NormalizedCellReverseIterator<'a> {
        let normalized_position = self.get_normalized_position(board);
        let piece_grid = self.piece.get_grid(&settings.rotation_system);
        let grid_size = piece_grid.0.len();
        let num_columns = (grid_size as f64).sqrt() as usize;

        NormalizedCellReverseIterator::new(
            NormalizedCellIterationData::new(
                normalized_position,
                piece_grid,
                num_columns
            )
        )
    }
}


pub struct NormalizedCell {
    pub grid_line: usize,
    pub grid_column: usize,
    pub board_line: usize,
    pub board_column: usize,
}

struct NormalizedCellIterationData<'a> {
    normalized_position: Position,
    grid: &'a PieceGrid,
    num_columns: usize,
}

impl<'a> NormalizedCellIterationData<'a> {
    fn new(
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
