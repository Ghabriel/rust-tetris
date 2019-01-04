use super::super::board::{Block, SimpleBoard};
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::piece::Piece;
use super::super::rotations::RotationSystem;
use super::super::settings::Settings;
use super::traits::Tick;

pub struct Model {
    board_gravity_pair: Box<dyn BoardGravityPair>,
    current_piece: Option<CurrentPiece>,
    settings: Settings,
}

pub struct CurrentPiece {
    pub piece: Piece,
    pub position: usize,
}

impl Tick for Model {
    fn tick(&mut self, elapsed_time: f64) -> bool {
        // TODO
        false
    }
}

/**
 * Getters used by the view
 */
impl Model {
    pub fn for_each_row(&self, callback: &mut FnMut(&Vec<&Option<Block>>)) {
        self.board_gravity_pair.board().for_each_row(callback);
    }

    pub fn get_active_piece(&self) -> &Option<CurrentPiece> {
        &self.current_piece
    }

    pub fn get_rotation_system(&self) -> &RotationSystem {
        &self.settings.rotation_system
    }
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
