use board::Board;

#[test]
fn filled_rows_are_detected() {
    let board = Board::from_array(&[
        "00000",
        "10000",
        "11000",
        "11111",
        "11111",
        "10111",
        "11111",
    ]);

    assert_eq!(board.get_filled_rows(), vec![3, 4, 6]);
}
