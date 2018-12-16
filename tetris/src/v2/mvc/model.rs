use super::super::board::SimpleBoard;
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::piece::Piece;
use super::super::settings::Settings;

pub struct Model {
    board_gravity_pair: Box<dyn BoardGravityPair>,
    current_piece: Option<CurrentPiece>,
    settings: Settings,
}

pub struct CurrentPiece {
    piece: Piece,
    position: usize,
}

impl Model {
    pub fn new(settings: Settings) -> Model {
        Model {
            board_gravity_pair: get_boxed_gravity(&settings.gravity, &settings.board_size),
            current_piece: None,
            settings: settings,
        }
    }

    pub fn change_gravity(&mut self, gravity: Gravity) {
        self.settings.gravity = gravity;

        self.board_gravity_pair = get_boxed_gravity(
            &self.settings.gravity,
            &self.settings.board_size
        );
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

fn get_boxed_gravity(
    gravity: &Gravity,
    board_size: &(usize, usize),
) -> Box<dyn BoardGravityPair> {
    match gravity {
        Gravity::Naive => {
            let (num_columns, num_rows) = *board_size;
            let board = SimpleBoard::new(num_columns, num_rows);
            let gravity_instance = NaiveGravity::new();

            Box::new(
                NaiveGravityPair::new(board, gravity_instance)
            )
        }
    }
}
