extern crate tetris;

use tetris::board::Board;

fn main() {
    let board = Board::from_array(&[
        "00000",
        "00000",
        "11111",
        "11111",
    ]);

    board.get_filled_rows();
}
