pub struct BoardPositionOffset {
    row: isize,
    column: isize,
}

impl BoardPositionOffset {
    pub fn new(row: isize, column: isize) -> BoardPositionOffset {
        BoardPositionOffset { row, column }
    }

    pub fn get_row(&self) -> isize {
        self.row
    }

    pub fn get_column(&self) -> isize {
        self.column
    }
}
