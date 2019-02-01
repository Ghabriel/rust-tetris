use lazy_static::lazy_static;
use std::collections::HashMap;
use super::super::board::{Board, MaterializationStatus};
use super::super::helpers;
use super::super::piece::{Piece, PieceColor};
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
    piece: Piece,
    position: BoardPosition,
    rotation_system: RotationSystem,
}

impl ActivePiece {
    pub fn new(
        piece: Piece,
        position: BoardPosition,
        rotation_system: RotationSystem,
    ) -> ActivePiece {
        ActivePiece { piece, position, rotation_system }
    }

    pub fn get_block_iterator<'b>(&'b self) -> impl Iterator<Item = BoardPosition> + 'b {
        helpers::get_piece_iterator(&self.piece, &self.position, &self.rotation_system)
    }

    pub fn get_color(&self) -> &PieceColor {
        &self.piece.get_color()
    }

    pub fn get_rotation_system(&self) -> &RotationSystem {
        &self.rotation_system
    }
}

/**
 * Movement-related methods
 */
impl ActivePiece {
    pub fn try_move_towards(
        &mut self,
        direction: Direction,
        board: &dyn Board,
    ) {
        if self.can_move_towards(&direction, board) {
            self.move_towards(&direction);
        }
    }

    pub fn can_move_towards(
        &self,
        direction: &Direction,
        board: &dyn Board,
    ) -> bool {
        let offset = DIRECTION_OFFSETS.get(direction).unwrap();

        self.get_block_iterator()
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
        board: &dyn Board,
    ) {
        self.rotate(&direction);

        if !self.is_valid(board) {
            let reverse_direction = match direction {
                RotationDirection::Clockwise => RotationDirection::Counterclockwise,
                RotationDirection::Counterclockwise => RotationDirection::Clockwise,
            };

            self.rotate(&reverse_direction);
        }
    }

    fn rotate(&mut self, direction: &RotationDirection) {
        self.piece.rotate(direction, &self.rotation_system);
    }

    fn is_valid(&self, board: &dyn Board) -> bool {
        self.get_block_iterator()
            .all(|tile_position| {
                board.is_in_bounds(&tile_position) && !board.is_occupied(&tile_position)
            })
    }
}

/**
 * Materialization
 */
impl ActivePiece {
    pub fn materialize_at(&mut self, board: &mut dyn Board) -> MaterializationStatus {
        board.materialize(
            &self.piece,
            &self.position,
            &self.rotation_system,
        )
    }
}
