use super::super::board::Board;
use super::super::helpers;
use super::super::piece::Piece;
use super::super::position::BoardPosition;
use super::super::rotations::{RotationDirection, RotationSystem};
use super::model::{Direction, DIRECTION_OFFSETS};

pub struct ActivePiece {
    pub piece: Piece,
    pub position: BoardPosition,
}

impl ActivePiece {
    pub fn get_block_iterator<'a>(
        &'a self,
        rotation_system: &'a RotationSystem
    ) -> impl Iterator<Item = BoardPosition> + 'a {
        helpers::get_piece_iterator(&self.piece, &self.position, rotation_system)
    }
}

/**
 * Movement-related methods
 */
impl ActivePiece {
    pub fn try_move_towards(
        &mut self,
        direction: Direction,
        rotation_system: &RotationSystem,
        board: &dyn Board,
    ) {
        if self.can_move_towards(&direction, rotation_system, board) {
            self.move_towards(&direction);
        }
    }

    pub fn can_move_towards(
        &self,
        direction: &Direction,
        rotation_system: &RotationSystem,
        board: &dyn Board,
    ) -> bool {
        let offset = DIRECTION_OFFSETS.get(direction).unwrap();

        self.get_block_iterator(rotation_system)
            .all(|tile_position| {
                !self.is_touching_wall(&tile_position, direction, board) &&
                !board.is_occupied(&(tile_position + offset))
            })
    }

    // TODO: move to Board itself
    fn is_touching_wall(
        &self,
        position: &BoardPosition,
        wall_direction: &Direction,
        board: &dyn Board
    ) -> bool {
        match wall_direction {
            Direction::Left => {
                position.column == 0
            },
            Direction::Right => {
                let num_columns = board.get_num_columns() as isize;

                position.column == num_columns - 1
            },
            Direction::Down => {
                let num_rows = board.get_num_rows() as isize;

                position.row == num_rows - 1
            }
        }
    }

    pub fn move_towards(&mut self, direction: &Direction) {
        let position_offset = DIRECTION_OFFSETS.get(&direction).unwrap();

        self.position += position_offset;
    }
}

/**
 * Rotation-related methods
 */
impl ActivePiece {
    pub fn try_rotate(
        &mut self,
        direction: RotationDirection,
        rotation_system: &RotationSystem,
        board: &dyn Board,
    ) {
        self.rotate(&direction, rotation_system);

        if !self.is_valid(rotation_system, board) {
            let reverse_direction = match direction {
                RotationDirection::Clockwise => RotationDirection::Counterclockwise,
                RotationDirection::Counterclockwise => RotationDirection::Clockwise,
            };

            self.rotate(&reverse_direction, rotation_system);
        }
    }

    fn rotate(&mut self, direction: &RotationDirection, rotation_system: &RotationSystem) {
        self.piece.rotate(direction, rotation_system);
    }

    fn is_valid(&self, rotation_system: &RotationSystem, board: &dyn Board) -> bool {
        self.get_block_iterator(rotation_system)
            .all(|tile_position| {
                self.is_inside_board(&tile_position, board) && !board.is_occupied(&tile_position)
            })
    }

    // TODO: move to Board itself
    fn is_inside_board(&self, position: &BoardPosition, board: &dyn Board) -> bool {
        let board_num_rows = board.get_num_rows();
        let board_num_columns = board.get_num_columns();

        position.is_inside_grid(board_num_rows, board_num_columns)
    }
}
