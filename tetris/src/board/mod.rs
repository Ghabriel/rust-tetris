mod active_piece;
mod board;
mod iteration;

pub use self::board::*;

use piece::Color;

pub struct Block {
    color: Color,
}

pub struct Position(usize, usize);

#[cfg(test)]
mod tests;
