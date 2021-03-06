use super::super::super::core::Direction;
use super::super::super::helpers;
use super::super::super::piece::{Piece, PieceColor};
use super::super::super::position::BoardPosition;
use super::super::super::rotations::RotationSystem;
use super::super::super::settings::Settings;
use super::super::{Block, Board, MaterializationStatus};
use super::row_iterator::RowIterator;

pub struct SimpleBoard {
    grid: Vec<Option<Block>>,
    num_columns: usize,
}

struct PositionOutOfBounds;

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

    fn at(&self, position: &BoardPosition) -> Result<&Option<Block>, PositionOutOfBounds> {
        let index_opt = self.position_to_index(position);

        if let Some(index) = index_opt {
            Ok(&self.grid[index])
        } else {
            Err(PositionOutOfBounds)
        }
    }

    fn at_mut(&mut self, position: &BoardPosition) -> Result<&mut Option<Block>, PositionOutOfBounds> {
        let index_opt = self.position_to_index(position);

        if let Some(index) = index_opt {
            Ok(&mut self.grid[index])
        } else {
            Err(PositionOutOfBounds)
        }
    }

    fn position_to_index(&self, position: &BoardPosition) -> Option<usize> {
        let num_rows = self.get_num_rows();
        let num_columns = self.get_num_columns();

        if position.is_inside_grid(num_rows, num_columns) {
            let &BoardPosition { row, column } = position;
            let index = row * num_columns as isize + column;

            Some(index as usize)
        } else {
            None
        }
    }
}

impl Board for SimpleBoard {
    fn get_num_columns(&self) -> usize {
        self.num_columns
    }

    fn get_num_rows(&self) -> usize {
        self.grid.len() / self.num_columns
    }

    /**
     * Checks if a given position is occupied by a block. Returns false if
     * the position is out of bounds.
     */
    fn is_occupied(&self, position: &BoardPosition) -> bool {
        match self.at(position) {
            Ok(tile) => tile.is_some(),
            Err(_) => false,
        }
    }

    fn is_touching_wall(
        &self,
        position: &BoardPosition,
        wall_direction: &Direction,
    ) -> bool {
        match wall_direction {
            Direction::Left => {
                position.column == 0
            },
            Direction::Right => {
                let num_columns = self.get_num_columns() as isize;

                position.column == num_columns - 1
            },
            Direction::Down => {
                let num_rows = self.get_num_rows() as isize;

                position.row == num_rows - 1
            }
        }
    }

    fn is_in_bounds(&self, position: &BoardPosition) -> bool {
        position.is_inside_grid(self.get_num_rows(), self.get_num_columns())
    }

    fn materialize(
        &mut self,
        piece: &Piece,
        position: &BoardPosition,
        rotation_system: &RotationSystem
    ) -> MaterializationStatus {
        let piece_iterator = helpers::get_piece_iterator(piece, position, &rotation_system);
        let piece_color = piece.get_color();

        for tile_position in piece_iterator {
            let board_tile = self.at_mut(&tile_position);

            match board_tile {
                Ok(Some(_)) | Err(_) => return MaterializationStatus::Failure,
                Ok(empty_tile) => *empty_tile = Some(Block { color: (*piece_color).clone() }),
            }
        }

        MaterializationStatus::Success
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
