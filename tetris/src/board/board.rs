use board::active_piece::ActivePiece;
use board::Block;
use board::iteration::NormalizedCell;
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

    pub fn debug(&mut self) {
        self.grid[20] = Some(Block { });
        self.grid[21] = Some(Block { });
        self.grid[22] = Some(Block { });
        self.grid[23] = Some(Block { });
        self.grid[24] = Some(Block { });
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
            self.lower_active_piece();
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

        for cell in normalized_cells {
            let index = cell.board_line * self.num_columns + cell.board_column;
            let board_cell = &mut self.grid[index];

            match board_cell {
                Some(_) => panic!("Cell clash during materialization"),
                None => *board_cell = Some(Block { }),
            }
        }
    }

    pub fn check_line_clears(&mut self, _settings: &Settings) {
        let num_cleared_lines = self.rows()
            .enumerate()
            .filter(|(_, row)| {
                row.iter().all(|cell| cell.is_some())
            })
            .map(|(index, _)| index)
            .inspect(|index| println!("Row {} should be cleared", index))
            .count();

        println!("# cleared rows: {}", num_cleared_lines);
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
