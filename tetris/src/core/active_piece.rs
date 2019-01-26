use lazy_static::lazy_static;
use std::collections::HashMap;
use super::super::board::Board;
use super::super::helpers;
use super::super::piece::Piece;
use super::super::position::{BoardPosition, BoardPositionOffset};
use super::super::rotations::{RotationDirection, RotationSystem};
use super::Direction;

lazy_static! {
    static ref DIRECTION_OFFSETS: HashMap<Direction, BoardPositionOffset> = {
        let mut map = HashMap::new();

        map.insert(Direction::Left, BoardPositionOffset::new(0, -1));
        map.insert(Direction::Right, BoardPositionOffset::new(0, 1));
        map.insert(Direction::Down, BoardPositionOffset::new(1, 0));

        map
    };
}

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
                !board.is_touching_wall(&tile_position, direction) &&
                !board.is_occupied(&(tile_position + offset))
            })
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
                board.is_in_bounds(&tile_position) && !board.is_occupied(&tile_position)
            })
    }
}
