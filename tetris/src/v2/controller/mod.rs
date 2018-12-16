use super::board::SimpleBoard;
use super::gravity::{BoardGravityPair, Gravity};
use super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::piece::Piece;

pub struct Controller {
    board_gravity_pair: Box<dyn BoardGravityPair>,
    current_piece: Option<CurrentPiece>,
    board_size: (usize, usize),
}

pub struct CurrentPiece {
    piece: Piece,
    position: usize,
}

impl Controller {
    pub fn change_gravity(&mut self, gravity: Gravity) {
        match gravity {
            Gravity::Naive => {
                let (num_columns, num_rows) = self.board_size;
                let board = SimpleBoard::new(num_columns, num_rows);
                let gravity_instance = NaiveGravity::new();

                self.board_gravity_pair = Box::new(
                    NaiveGravityPair::new(board, gravity_instance)
                );
            }
        }
    }

//     pub fn has_active_piece(&self) -> bool {
//         self.current_piece.is_some()
//     }

//     // pub fn active_piece_touches_board

//     pub fn tick(&mut self, settings: &Settings) {
//         if !self.has_active_piece() {
//             return;
//         }

//         if !self.active_piece_touches_board(settings) {
//             self.lower_active_piece();
//             return;
//         }

//         self.materialize_active_piece(settings);
//         self.clear_filled_rows(settings);
//     }
}
