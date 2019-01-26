use super::super::piece::Piece;
use super::super::position::BoardPosition;

pub struct ActivePiece {
    pub piece: Piece,
    pub position: BoardPosition,
}
