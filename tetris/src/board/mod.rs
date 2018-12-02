mod active_piece;
mod board;
mod parsed_grid;

pub use self::board::*;

pub struct Block {
    // color: Color,
}

pub struct Position(usize, usize);
