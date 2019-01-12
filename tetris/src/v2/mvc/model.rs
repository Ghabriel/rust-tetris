use super::super::board::{Block, MaterializationStatus, SimpleBoard};
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::helpers;
use super::super::piece::{Piece, PieceColor, PieceKind};
use super::super::position::BoardPosition;
use super::super::rotations::RotationSystem;
use super::super::settings::Settings;
use super::traits::Tick;

pub struct Model {
    board_gravity_pair: Box<dyn BoardGravityPair>,
    current_piece: Option<CurrentPiece>,
    settings: Settings,
    running: bool,
}

pub struct CurrentPiece {
    pub piece: Piece,
    pub position: BoardPosition,
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

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Model {
    fn stop_if_not_running(&mut self) -> bool {
        self.running
    }

    fn spawn_piece_if_needed(&mut self) -> bool {
        if !self.has_active_piece() {
            self.spawn_piece();
            return false;
        }

        true
    }

    fn lower_active_piece_if_possible(&mut self) -> bool {
        if !self.active_piece_touches_board() {
            self.lower_active_piece();
            return false;
        }

        true
    }

    fn materialization_process(&mut self) -> bool {
        match self.materialize_active_piece() {
            MaterializationStatus::Success => {},
            MaterializationStatus::Failure => {
                self.running = false;
                return false;
            }
        }

        self.clear_filled_rows();
        true
    }

    fn run_until_false(&mut self, steps: &[fn(&mut Self) -> bool]) {
        for step in steps.iter() {
            if !step(self) {
                break;
            }
        }
    }
}

impl Tick for Model {
    fn tick(&mut self, elapsed_time: f64) -> bool {
        // TODO: add an artificial delay to make the game easier

        self.run_until_false(&[
            Model::stop_if_not_running,
            Model::spawn_piece_if_needed,
            Model::lower_active_piece_if_possible,
            Model::materialization_process
        ]);

        false
    }
}

impl Model {
    pub fn new(settings: Settings) -> Model {
        Model {
            board_gravity_pair: get_boxed_gravity(&settings.gravity, &settings.board_size),
            current_piece: None,
            settings: settings,
            running: true, // TODO: change to false later
        }
    }

    pub fn change_gravity(&mut self, gravity: Gravity) {
        self.settings.gravity = gravity;

        self.board_gravity_pair = get_boxed_gravity(
            &self.settings.gravity,
            &self.settings.board_size
        );
    }
}

/**
 * has_active_piece implementation
 */
impl Model {
    fn has_active_piece(&self) -> bool {
        self.current_piece.is_some()
    }
}

/**
 * spawn_piece implementation + helpers
 */
impl Model {
    fn spawn_piece(&mut self) {
        let piece = random_piece();
        let position = self.get_centralized_position_for(&piece);

        self.current_piece = Some(CurrentPiece { piece, position });
    }

    fn get_centralized_position_for(&self, piece: &Piece) -> BoardPosition {
        let grid_num_columns = self.get_grid_num_columns(piece);
        let board_num_columns = self.get_board_num_columns();

        BoardPosition::new(
            0,
            (board_num_columns - grid_num_columns) / 2,
        )
    }

    fn get_grid_num_columns(&self, piece: &Piece) -> usize {
        let grid = piece.get_grid(self.get_rotation_system());
        let grid_size = grid.0.len();

        (grid_size as f32).sqrt() as usize
    }
}

/**
 * active_piece_touches_board implementation + helpers
 */
impl Model {
    fn active_piece_touches_board(&self) -> bool {
        let next_row_offset = BoardPosition::new(1, 0);

        for tile_position in self.get_active_piece_iterator() {
            let tile_below = tile_position + &next_row_offset;

            if self.is_below_floor(&tile_below) || self.is_occupied(&tile_below) {
                return true;
            }
        }

        false
    }

    fn get_active_piece_iterator<'a>(&'a self) -> impl Iterator<Item = BoardPosition> + 'a {
        let CurrentPiece { piece, position } = self.current_piece.as_ref().unwrap();

        helpers::get_piece_iterator(piece, position, self.get_rotation_system())
    }

    fn is_occupied(&self, position: &BoardPosition) -> bool {
        self.board_gravity_pair.board().is_occupied(position)
    }

    fn is_below_floor(&self, position: &BoardPosition) -> bool {
        let num_rows = self.board_gravity_pair.board().get_num_rows();

        position.get_row() >= num_rows
    }
}

/**
 * lower_active_piece implementation
 */
impl Model {
    fn lower_active_piece(&mut self) {
        let current_piece = self.current_piece.as_mut().unwrap();

        current_piece.position += BoardPosition::new(1, 0);
    }
}

/**
 * materialize_active_piece implementation
 */
impl Model {
    fn materialize_active_piece(&mut self) -> MaterializationStatus {
        let CurrentPiece { piece, position } = self.current_piece.take().unwrap();

        self.board_gravity_pair.board_mut().materialize(
            &piece,
            &position,
            &self.settings.rotation_system,
        )
    }
}

/**
 * clear_filled_rows implementation
 */
impl Model {
    fn clear_filled_rows(&mut self) {
        let filled_rows = self.board_gravity_pair.board().get_filled_rows();

        self.board_gravity_pair.clear_rows(&filled_rows, &self.settings);
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
