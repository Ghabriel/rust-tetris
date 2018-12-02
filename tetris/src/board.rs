use piece::{Color, Piece};
use settings::Settings;

pub struct Board {
    grid: Vec<Option<Block>>,
    num_rows: usize,
    num_columns: usize,
    active_piece: Option<ActivePiece>,
}

pub struct Block {
    color: Color,
}

pub struct ActivePiece {
    piece: Piece,
    position: usize,
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
            num_rows,
            num_columns,
            active_piece: None,
        }
    }

    pub fn spawn_piece(&mut self, piece: Piece, position: usize) {
        let active_piece = ActivePiece {
            piece,
            position,
        };

        self.active_piece = Some(active_piece);
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
        match self.active_piece {
            Some(_) => true,
            None => false,
        }
    }

    fn active_piece_touches_board(&self, settings: &Settings) -> bool {
        if let Some(ref active_piece) = self.active_piece {
            let piece = &active_piece.piece;

            let base_position = active_piece.position;
            let base_line = base_position / self.num_columns;
            let base_column = base_position % self.num_columns;

            let piece_grid = piece.get_grid(&settings.rotation_system);
            let piece_grid_size = piece_grid.0.len();
            let piece_grid_columns   = (piece_grid_size as f64).sqrt() as usize;

            for (reverse_index, cell) in piece_grid.0.iter().rev().enumerate() {
                let index = piece_grid_size - 1 - reverse_index;

                let cell_grid_line = index / piece_grid_columns;
                let cell_grid_column = index % piece_grid_columns;
                let cell_position_offset = cell_grid_line * self.num_columns + cell_grid_column;

                let cell_line = base_line + cell_grid_line;
                let cell_column = base_column + cell_grid_column;

                if let Some(_) = self.at(cell_line, cell_column) {
                    return true;
                }
            }
        }

        false
    }

    fn at(&self, row: usize, column: usize) -> &Option<Block> {
        &self.grid[row * self.num_columns + column]
    }

    fn materialize_active_piece(&mut self, settings: &Settings) {
        // TODO
    }

    fn check_line_clears(&mut self, settings: &Settings) {
        // TODO
    }

    fn lower_active_piece(&mut self, settings: &Settings) {
        // TODO
    }
}
