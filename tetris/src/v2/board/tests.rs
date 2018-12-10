use super::{Board, SimpleBoard};

#[test]
fn simple_board_construction_new() {
    let board = SimpleBoard::new(30, 15);

    assert_eq!(board.get_num_columns(), 30);
    assert_eq!(board.get_num_rows(), 15);
    assert_eq!(board.len(), 450);
}

#[test]
fn simple_board_construction_from_array() {
    let board = SimpleBoard::from_array(&[
        "00000",
        "10000",
        "11000",
        "11111",
        "11111",
        "10111",
        "11111",
    ]);

    assert_eq!(board.get_num_columns(), 5);
    assert_eq!(board.get_num_rows(), 7);
    assert_eq!(board.len(), 35);
}

#[test]
fn filled_rows_are_detected() {
    let board = SimpleBoard::from_array(&[
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
