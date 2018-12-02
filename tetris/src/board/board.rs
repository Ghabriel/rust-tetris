use board::active_piece::ActivePiece;
use board::Block;
use piece::Piece;
use settings::Settings;

pub struct Board {
    grid: Vec<Option<Block>>,
    num_columns: usize,
    active_piece: Option<ActivePiece>,
}

impl Board {
    pub fn new(num_rows: usize, num_columns: usize) -> Board {
        let grid_size = num_rows * num_columns;
        let mut grid = Vec::with_capacity(grid_size);

        for _ in 0..grid_size {
            grid.push(None);
        }

        Board {
            grid,
            num_columns,
            active_piece: None,
        }
    }

    pub fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    pub fn spawn_piece(&mut self, piece: Piece, position: usize) {
        let active_piece = ActivePiece::new(piece, position);

        self.active_piece = Some(active_piece);
    }

    pub fn is_occupied(&self, row: usize, column: usize) -> bool {
        self.at(row, column).is_some()
    }

    pub fn tick(&mut self, settings: &Settings) {
        if !self.has_active_piece() {
            return;
        }

        if !self.active_piece_touches_board(settings) {
            self.lower_active_piece(settings);
            return;
        }

        self.materialize_active_piece(settings);
        self.check_line_clears(settings);
    }

    fn has_active_piece(&self) -> bool {
        self.active_piece.is_some()
    }

    fn active_piece_touches_board(&self, settings: &Settings) -> bool {
        let active_piece = self.active_piece.as_ref().unwrap();

        active_piece.is_touching_board(self, settings)
    }

    fn at(&self, row: usize, column: usize) -> &Option<Block> {
        let index = row * self.num_columns + column;

        &self.grid[index]
    }

    fn materialize_active_piece(&mut self, _settings: &Settings) {
        // TODO
    }

    fn check_line_clears(&mut self, _settings: &Settings) {
        // TODO
    }

    fn lower_active_piece(&mut self, _settings: &Settings) {
        // TODO
    }
}
