use super::super::board::{Block, SimpleBoard};
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::piece::{Piece, PieceColor, PieceKind};
use super::super::position::{BoardPosition, PiecePosition};
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
}

/**
 * has_active_piece implementation + helpers
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

    fn get_centralized_position_for(&self, piece: &Piece) -> usize {
        let grid_num_columns = self.get_grid_num_columns(piece);
        let board_num_columns = self.get_board_num_columns();

        (board_num_columns - grid_num_columns) / 2
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
            let next_row = tile_position + &next_row_offset;

            if self.is_occupied(&next_row) {
                return true;
            }
        }

        false
    }

    fn get_active_piece_iterator<'a>(&'a self) -> impl Iterator<Item = BoardPosition> + 'a {
        let CurrentPiece { piece, position } = self.current_piece.as_ref().unwrap();

        let piece_position = BoardPosition::from_index(
            *position,
            self.get_board_num_columns()
        );

        self.get_piece_iterator(piece, &piece_position)
    }

    fn get_piece_iterator<'a>(
        &'a self,
        piece: &Piece,
        piece_position: &'a BoardPosition
    ) -> impl Iterator<Item = BoardPosition> + 'a {
        let grid = piece.get_grid(self.get_rotation_system());
        let grid_size = grid.0.len();
        let grid_num_columns = (grid_size as f32).sqrt() as usize;

        grid.0.iter()
            .enumerate()
            .filter(|(_, tile)| **tile)
            .map(|(tile_index, _)| {
                let block_in_piece_coordinates = PiecePosition::from_index(
                    tile_index,
                    grid_num_columns
                );

                BoardPosition::new(
                    block_in_piece_coordinates.get_row() + piece_position.get_row(),
                    block_in_piece_coordinates.get_column() + piece_position.get_column(),
                )
            })
    }

    fn is_occupied(&self, position: &BoardPosition) -> bool {
        self.board_gravity_pair.board().is_occupied(position)
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
