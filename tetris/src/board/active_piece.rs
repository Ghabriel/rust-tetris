use board::{Board, Position};
use board::iteration::{
    NormalizedCellIterationData,
    NormalizedCellIterator,
    NormalizedCellReverseIterator,
};
use piece::Piece;
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
