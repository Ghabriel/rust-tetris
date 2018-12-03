use board::{Board, Position};
use board::parsed_grid::{NormalizedCellIterator, ParsedGrid};
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

    pub fn is_touching_board(&self, board: &Board, settings: &Settings) -> bool {
        let normalized_position = self.get_normalized_position(board);
        let parsed_grid = self.get_parsed_grid(&settings.rotation_system);

        parsed_grid.is_touching_board(board, normalized_position)
    }

    fn get_normalized_position(&self, board: &Board) -> Position {
        let line = self.position / board.get_num_columns();
        let column = self.position % board.get_num_columns();

        Position(line, column)
    }

    fn get_parsed_grid<'a>(&self, rotation_system: &'a RotationSystem) -> ParsedGrid<'a> {
        let piece_grid = self.piece.get_grid(rotation_system);

        ParsedGrid::new(piece_grid)
    }

    pub fn normalized_cell_iter<'a>(
        &'a self,
        board: &'a Board,
        settings: &'a Settings
    ) -> NormalizedCellIterator<'a> {
        let normalized_position = self.get_normalized_position(board);
        let parsed_grid = self.get_parsed_grid(&settings.rotation_system);

        parsed_grid.normalized_cell_iter(board, normalized_position)
    }
}
