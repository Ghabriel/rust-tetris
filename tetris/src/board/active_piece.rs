use board::{Board, Position};
use board::parsed_grid::ParsedGrid;
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
}
