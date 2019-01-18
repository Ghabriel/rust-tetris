use super::super::{Board, Block};
use super::SimpleBoard;

pub struct RowIterator<'a> {
    board: &'a SimpleBoard,
    next_row: usize,
}

impl<'a> RowIterator<'a> {
    pub fn new(board: &'a SimpleBoard) -> RowIterator<'a> {
        RowIterator {
            board,
            next_row: 0,
        }
    }
}

impl<'a> Iterator for RowIterator<'a> {
    type Item = Vec<&'a Option<Block>>;

    fn next(&mut self) -> Option<Self::Item> {
        let num_columns = self.board.get_num_columns();
        let next_starting_index = self.next_row * num_columns;

        if next_starting_index >= self.board.len() {
            return None;
        }

        self.next_row += 1;

        Some(self.board.tiles()
            .skip(next_starting_index)
            .take(num_columns)
            .collect())
    }
}
