use std::ops::Add;

pub struct Position(usize, usize);

impl Position {
    pub fn from_index(index: usize, num_columns: usize) -> Position {
        Position(index / num_columns, index % num_columns)
    }

    pub fn to_index(&self, num_columns: usize) -> usize {
        self.0 * num_columns + self.1
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position(
            self.0 + other.0,
            self.1 + other.1,
        )
    }
}
