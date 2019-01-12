use super::super::super::helpers;
use super::super::super::piece::{Piece, PieceColor};
use super::super::super::position::BoardPosition;
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

    pub fn tiles(&self) -> impl Iterator<Item = &Option<Block>> {
        self.grid.iter()
    }

    pub fn rows<'a>(&'a self) -> RowIterator<'a> {
        RowIterator::new(self)
    }
}

impl SimpleBoard {
    // fn to_board_coordinates<'a>(
    //     &self,
    //     piece: &'a Piece,
    //     position: usize,
    //     rotation_system: &'a RotationSystem
    // ) -> impl Iterator<Item = usize> + 'a {
    //     helpers::piece_to_board_coordinates(
    //         self.get_num_columns(),
    //         piece,
    //         position,
    //         rotation_system
    //     )
    // }
}

impl Board for SimpleBoard {
    fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    fn get_num_rows(&self) -> usize {
        self.grid.len() / self.num_columns
    }

    fn is_occupied(&self, position: &BoardPosition) -> bool {
        let index = position.to_index(self.num_columns);

        self.grid[index].is_some()
    }

    fn materialize(&mut self, piece: &Piece, position: &BoardPosition, settings: &Settings) {
        let piece_iterator = helpers::get_piece_iterator(
            piece,
            position,
            &settings.rotation_system
        );

        let piece_color = piece.get_color();

        for tile_position in piece_iterator {
            let tile_index = tile_position.to_index(self.get_num_columns());
            let board_tile = &mut self.grid[tile_index];

            match board_tile {
                Some(_) => panic!("tile clash during materialization"),
                None => *board_tile = Some(Block {
                    color: (*piece_color).clone()
                }),
            }
        }
    }

    fn get_filled_rows(&self) -> Vec<usize> {
        self.rows()
            .enumerate()
            .filter(|(_, row)| {
                row.iter().all(|tile| tile.is_some())
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn clear_rows(&mut self, _rows: &[usize], _settings: &Settings) {
        // TODO
    }

    fn for_each_row(&self, callback: &mut FnMut(&Vec<&Option<Block>>)) {
        self.rows()
            .for_each(|row| {
                callback(&row);
            });
    }
}
