extern crate tetris;

use tetris::v2::board::SimpleBoard;

fn main() {
    let board = SimpleBoard::from_array(&[
        "00000",
        "00000",
        "11111",
        "11111",
    ]);

    // board.get_filled_rows();
}
