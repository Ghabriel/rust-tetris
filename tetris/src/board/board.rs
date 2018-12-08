use board::active_piece::ActivePiece;
use board::Block;
use board::iteration::NormalizedCell;
use piece::{Color, Piece};
use settings::Settings;

pub struct Board {
    grid: Vec<Option<Block>>,
    num_columns: usize,
    active_piece: Option<ActivePiece>,
}

/**
 * Board constructors
 */
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

    pub fn from_array(rows: &[&str]) -> Board {
        let num_rows = rows.len();

        assert!(num_rows > 0, "empty board data array");

        let num_columns = rows[0].len();
        let grid_size = num_rows * num_columns;
        let mut grid = Vec::with_capacity(grid_size);

        for row in rows {
            for column in row.chars() {
                match column {
                    '0' => grid.push(None),
                    '1' => grid.push(Some(Block {
                        color: Color::Blue,
                    })),
                    _ => panic!("Invalid board construction data"),
                }
            }
        }

        Board {
            grid,
            num_columns,
            active_piece: None,
        }
    }
}

/**
 * Board getters/checkers
 */
impl Board {
    pub fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    pub fn is_occupied(&self, row: usize, column: usize) -> bool {
        self.at(row, column).is_some()
    }

    pub fn get_filled_rows(&self) -> Vec<usize> {
        self.rows()
            .enumerate()
            .filter(|(_, row)| {
                row.iter().all(|cell| cell.is_some())
            })
            .map(|(index, _)| index)
            .collect()
    }
}

impl Board {
    pub fn spawn_piece(&mut self, piece: Piece, position: usize) {
        let active_piece = ActivePiece::new(piece, position);

        self.active_piece = Some(active_piece);
    }

    pub fn tick(&mut self, settings: &Settings) {
        if !self.has_active_piece() {
            return;
        }

        if !self.active_piece_touches_board(settings) {
            self.lower_active_piece();
            return;
        }

        self.materialize_active_piece(settings);
        self.clear_filled_rows(settings);
    }
}

/**
 * Private methods
 */
impl Board {
    fn has_active_piece(&self) -> bool {
        self.active_piece.is_some()
    }

    fn active_piece_touches_board(&self, settings: &Settings) -> bool {
        let active_piece = self.active_piece.as_ref().unwrap();

        active_piece.normalized_cell_iter_rev(self, settings)
            .any(|cell| {
                let grid_below_is_occupied = active_piece.is_grid_cell_occupied(
                    cell.grid_line + 1,
                    cell.grid_column,
                    &settings.rotation_system,
                );

                let board_below_is_occupied = self.is_occupied(
                    cell.board_line + 1,
                    cell.board_column,
                );

                !grid_below_is_occupied && board_below_is_occupied
            })
    }

    fn lower_active_piece(&mut self) {
        if let Some(ref mut active_piece) = self.active_piece {
            active_piece.shift(self.num_columns as isize);
        }
    }

    fn materialize_active_piece(&mut self, settings: &Settings) {
        let active_piece = self.active_piece.as_ref().unwrap();
        let normalized_cells: Vec<NormalizedCell> = active_piece
            .normalized_cell_iter(self, settings)
            .collect();

        let piece_color = active_piece.get_color();

        for cell in normalized_cells {
            let index = cell.board_line * self.num_columns + cell.board_column;
            let board_cell = &mut self.grid[index];

            match board_cell {
                Some(_) => panic!("Cell clash during materialization"),
                None => *board_cell = Some(Block {
                    color: (*piece_color).clone()
                }),
            }
        }
    }

    fn clear_filled_rows(&mut self, settings: &Settings) {
        let filled_rows = self.get_filled_rows();

        self.clear_rows(&filled_rows, settings);
    }

    fn clear_rows(&mut self, rows: &[usize], settings: &Settings) {
        // TODO
    }

    fn at(&self, row: usize, column: usize) -> &Option<Block> {
        let index = row * self.num_columns + column;

        &self.grid[index]
    }

    fn rows<'a>(&'a self) -> BoardRowIterator<'a> {
        BoardRowIterator::new(self)
    }
}

struct BoardRowIterator<'a> {
    board: &'a Board,
    next_row: usize,
}

impl<'a> BoardRowIterator<'a> {
    fn new(board: &'a Board) -> BoardRowIterator<'a> {
        BoardRowIterator {
            board,
            next_row: 0,
        }
    }
}

impl<'a> Iterator for BoardRowIterator<'a> {
    type Item = Vec<&'a Option<Block>>;

    fn next(&mut self) -> Option<Self::Item> {
        let grid = &self.board.grid;
        let num_columns = self.board.num_columns;
        let next_starting_index = self.next_row * num_columns;

        if next_starting_index >= grid.len() {
            return None;
        }

        self.next_row += 1;

        Some(grid.iter()
            .skip(next_starting_index)
            .take(num_columns)
            .collect())
    }
}
