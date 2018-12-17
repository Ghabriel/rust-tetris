use super::super::super::piece::{Piece, PieceColor};
use super::super::super::rotations::RotationSystem;
use super::super::super::settings::Settings;
use super::super::{Block, Board};
use super::row_iterator::RowIterator;

pub struct SimpleBoard {
    grid: Vec<Option<Block>>,
    num_columns: usize,
}

impl SimpleBoard {
    pub fn new(num_columns: usize, num_rows: usize) -> SimpleBoard {
        let grid_size = num_rows * num_columns;
        let mut grid = Vec::with_capacity(grid_size);

        for _ in 0..grid_size {
            grid.push(None);
        }

        SimpleBoard {
            grid,
            num_columns,
        }
    }

    pub fn from_array(rows: &[&str]) -> SimpleBoard {
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
                        color: PieceColor::Blue,
                    })),
                    _ => panic!("Invalid board construction data"),
                }
            }
        }

        SimpleBoard {
            grid,
            num_columns,
        }
    }

    pub fn len(&self) -> usize {
        self.grid.len()
    }

    pub fn cells(&self) -> impl Iterator<Item = &Option<Block>> {
        self.grid.iter()
    }

    pub fn rows<'a>(&'a self) -> RowIterator<'a> {
        RowIterator::new(self)
    }
}

impl SimpleBoard {
    fn to_board_coordinates<'a>(
        &self,
        piece: &'a Piece,
        position: usize,
        rotation_system: &'a RotationSystem
    ) -> impl Iterator<Item = usize> + 'a {
        let grid = piece.get_grid(rotation_system);
        let grid_num_columns = (grid.0.len() as f64).sqrt() as usize;
        let board_num_columns = self.get_num_columns();

        grid.0.iter()
            .enumerate()
            .filter(|(_, cell)| **cell)
            .map(move |(index, _)| {
                // the grid position, in board coordinates: pos(grid, board)
                let base_row = position / board_num_columns;
                let base_column = position % board_num_columns;

                // the cell position, in grid coordinates: pos(cell, grid)
                let cell_grid_row = index / grid_num_columns;
                let cell_grid_column = index % grid_num_columns;

                // the cell position, in board coordinates:
                // pos(cell, board) = pos(cell, grid) + pos(grid, board)
                let row = base_row + cell_grid_row;
                let column = base_column + cell_grid_column;

                row * board_num_columns + column
            })
    }
}

impl Board for SimpleBoard {
    fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    fn get_num_rows(&self) -> usize {
        self.grid.len() / self.num_columns
    }

    fn is_occupied(&self, row: usize, column: usize) -> bool {
        let index = row * self.num_columns + column;

        self.grid[index].is_some()
    }

    fn materialize(&mut self, piece: Piece, position: usize, settings: &Settings) {
        let piece_color = piece.get_color();

        self.to_board_coordinates(&piece, position, &settings.rotation_system)
            .for_each(|index| {
                let board_cell = &mut self.grid[index];

                match board_cell {
                    Some(_) => panic!("Cell clash during materialization"),
                    None => *board_cell = Some(Block {
                        color: (*piece_color).clone()
                    }),
                }
            });
    }

    fn get_filled_rows(&self) -> Vec<usize> {
        self.rows()
            .enumerate()
            .filter(|(_, row)| {
                row.iter().all(|cell| cell.is_some())
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn clear_rows(&mut self, _rows: &[usize], _settings: &Settings) {
        // TODO
    }
}