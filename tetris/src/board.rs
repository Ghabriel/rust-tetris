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

struct Position(usize, usize);

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
        self.active_piece.is_some()
    }

    fn active_piece_touches_board(&self, settings: &Settings) -> bool {
        let active_piece = self.active_piece.as_ref().unwrap();
        let Position(base_line, base_column) = self.get_active_piece_position();

        let piece = &active_piece.piece;
        let piece_grid = piece.get_grid(&settings.rotation_system);
        let piece_grid_size = piece_grid.0.len();
        let piece_grid_columns = (piece_grid_size as f64).sqrt() as usize;

        for (reverse_index, cell) in piece_grid.0.iter().rev().enumerate() {
            if !cell {
                continue;
            }

            let index = piece_grid_size - 1 - reverse_index;

            let cell_grid_line = index / piece_grid_columns;
            let cell_grid_column = index % piece_grid_columns;

            let cell_line = base_line + cell_grid_line;
            let cell_column = base_column + cell_grid_column;
            let cell_position = Position(cell_line, cell_column);

            let line_below = self.grid_index(
                cell_grid_line + 1,
                cell_grid_column,
                piece_grid_columns,
            );

            if !piece_grid.0[line_below] && self.cell_touches_board(cell_position) {
                return true;
            }
        }

        false
    }

    fn get_active_piece_position(&self) -> Position {
        match self.active_piece {
            Some(ref active_piece) => {
                let position = active_piece.position;
                let line = position / self.num_columns;
                let column = position % self.num_columns;

                Position(line, column)
            },
            None => panic!("No active piece"),
        }
    }

    fn cell_touches_board(&self, cell_position: Position) -> bool {
        let Position(cell_row, cell_column) = cell_position;

        self.is_occupied(cell_row + 1, cell_column)
    }

    fn is_occupied(&self, row: usize, column: usize) -> bool {
        self.at(row, column).is_some()
    }

    fn grid_index(&self, row: usize, column: usize, num_columns: usize) -> usize {
        row * num_columns + column
    }

    fn at(&self, row: usize, column: usize) -> &Option<Block> {
        let index = self.grid_index(row, column, self.num_columns);

        &self.grid[index]
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
