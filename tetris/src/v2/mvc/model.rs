use super::super::board::{Block, SimpleBoard};
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::piece::{Piece, PieceColor, PieceKind};
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

    pub fn get_board_num_columns(&self) -> usize {
        self.board_gravity_pair.board().get_num_columns()
    }
}

impl Tick for Model {
    fn tick(&mut self, elapsed_time: f64) -> bool {
        if !self.has_active_piece() {
            self.spawn_piece();
            return false;
        }

        // TODO: add an artificial delay to make the game easier

        if !self.active_piece_touches_board() {
            self.lower_active_piece();
            return false;
        }

        self.materialize_active_piece();
        self.clear_filled_rows();

        false
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

    pub fn has_active_piece(&self) -> bool {
        self.current_piece.is_some()
    }

    pub fn spawn_piece(&mut self) {
        let piece = random_piece();
        let position = self.get_centralized_position_for(&piece);

        self.current_piece = Some(CurrentPiece { piece, position });
    }

    pub fn get_centralized_position_for(&self, piece: &Piece) -> usize {
        let grid_num_columns = self.get_grid_num_columns(piece);
        let board_num_columns = self.get_board_num_columns();

        (board_num_columns - grid_num_columns) / 2
    }

    pub fn get_grid_num_columns(&self, piece: &Piece) -> usize {
        let grid = piece.get_grid(self.get_rotation_system());
        let grid_size = grid.0.len();

        (grid_size as f32).sqrt() as usize
    }
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

fn random_piece() -> Piece {
    // TODO
    Piece::new(PieceKind::L, PieceColor::Blue, 0)
}
