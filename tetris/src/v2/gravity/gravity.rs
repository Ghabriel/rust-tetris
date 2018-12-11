use super::super::board::Board;

pub trait Gravity {
    fn apply(board: &mut impl Board);
}
