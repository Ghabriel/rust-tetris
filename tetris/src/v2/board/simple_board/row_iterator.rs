use super::super::Block;
use super::SimpleBoard;

pub struct SimpleBoardRowIterator<'a> {
    board: &'a SimpleBoard,
    next_row: usize,
}

impl<'a> SimpleBoardRowIterator<'a> {
    pub fn new(board: &'a SimpleBoard) -> SimpleBoardRowIterator<'a> {
        SimpleBoardRowIterator {
            board,
            next_row: 0,
        }
    }
}

// impl<'a> Iterator for SimpleBoardRowIterator<'a> {
//     type Item = Vec<&'a Option<Block>>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let grid = &self.board.grid;
//         let num_columns = self.board.num_columns;
//         let next_starting_index = self.next_row * num_columns;

//         if next_starting_index >= grid.len() {
//             return None;
//         }

//         self.next_row += 1;

//         Some(grid.iter()
//             .skip(next_starting_index)
//             .take(num_columns)
//             .collect())
//     }
// }
