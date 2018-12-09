use super::super::{Block, Board};
use v2::piece::{Piece, PieceColor};
use v2::settings::Settings;

pub struct SimpleBoard {
    grid: Vec<Option<Block>>,
    num_columns: usize,
}

impl SimpleBoard {
    pub fn new(num_rows: usize, num_columns: usize) -> SimpleBoard {
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

    // fn rows<'a>(&'a self) -> SimpleBoardRowIterator<'a> {
    //     SimpleBoardRowIterator::new(self)
    // }
}

impl Board for SimpleBoard {
    fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    fn is_occupied(&self, row: usize, column: usize) -> bool {
        let index = row * self.num_columns + column;

        self.grid[index].is_some()
    }

    fn materialize(&mut self, piece: Piece, position: usize, settings: &Settings) {
        // let normalized_cells: Vec<NormalizedCell> = piece
        //     .normalized_cell_iter(self, settings)
        //     .collect();

        // let piece_color = piece.get_color();

        // for cell in normalized_cells {
        //     let index = cell.board_line * self.num_columns + cell.board_column;
        //     let board_cell = &mut self.grid[index];

        //     match board_cell {
        //         Some(_) => panic!("Cell clash during materialization"),
        //         None => *board_cell = Some(Block {
        //             color: (*piece_color).clone()
        //         }),
        //     }
        // }
    }

    fn get_filled_rows(&self) -> Vec<usize> {
        // self.rows()
        //     .enumerate()
        //     .filter(|(_, row)| {
        //         row.iter().all(|cell| cell.is_some())
        //     })
        //     .map(|(index, _)| index)
        //     .collect()

        // TODO
        vec![]
    }

    fn clear_rows(&mut self, _rows: &[usize], _settings: &Settings) {
        // TODO
    }
}
