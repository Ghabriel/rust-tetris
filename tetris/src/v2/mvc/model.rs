use lazy_static::lazy_static;
use sfml::window::Key;
use std::collections::HashMap;
use super::super::board::{Block, MaterializationStatus, SimpleBoard};
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::helpers;
use super::super::piece::{Piece, PieceColor, PieceKind};
use super::super::position::{BoardPosition, BoardPositionOffset};
use super::super::rotations::RotationSystem;
use super::super::settings::Settings;
use super::traits::Tick;
use super::InputHandler;

pub struct Model {
    board_gravity_pair: Box<dyn BoardGravityPair>,
    current_piece: Option<CurrentPiece>,
    input_handler: InputHandler,
    settings: Settings,
    running: bool,
}

pub struct CurrentPiece {
    pub piece: Piece,
    pub position: BoardPosition,
}

#[derive(PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Down,
}

lazy_static! {
    static ref DIRECTION_OFFSETS: HashMap<Direction, BoardPositionOffset> = {
        let mut map = HashMap::new();

        map.insert(Direction::Left, BoardPositionOffset::new(0, -1));
        map.insert(Direction::Right, BoardPositionOffset::new(0, 1));
        map.insert(Direction::Down, BoardPositionOffset::new(1, 0));

        map
    };
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
        if !self.running {
            return false;
        }

        if !self.has_active_piece() {
            self.spawn_piece();
            return false;
        }

        self.handle_input();

        // TODO: add an artificial delay to make the game easier

        if self.can_move_active_piece(&Direction::Down) {
            self.move_active_piece(&Direction::Down);
            return false;
        }

        match self.materialize_active_piece() {
            MaterializationStatus::Success => {},
            MaterializationStatus::Failure => {
                self.running = false;
                return false
            }
        }

        self.clear_filled_rows();

        false
    }
}

impl Model {
    pub fn new(settings: Settings) -> Model {
        Model {
            board_gravity_pair: get_boxed_gravity(&settings.gravity, &settings.board_size),
            current_piece: None,
            input_handler: InputHandler::new(),
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
 * handle_input implementation + helpers
 */
impl Model {
    fn handle_input(&mut self) {
        self.input_handler.tick();

        // TODO: find a better solution to this borrow checker issue
        let pressed_keys: Vec<Key> = self.input_handler.get_pressed_keys()
            .cloned()
            .collect();

        pressed_keys.iter().for_each(|key| {
            match key {
                Key::Left => {
                    self.try_move_active_piece(Direction::Left);
                },
                Key::Right => {
                    self.try_move_active_piece(Direction::Right);
                },
                _ => {},
            }
        });
    }

    fn try_move_active_piece(&mut self, direction: Direction) {
        if self.can_move_active_piece(&direction) {
            self.move_active_piece(&direction);
        }
    }

    fn can_move_active_piece(&self, direction: &Direction) -> bool {
        let offset = DIRECTION_OFFSETS.get(direction).unwrap();

        self.get_active_piece_iterator()
            .all(|tile_position| {
                !self.is_touching_wall(&tile_position, direction) &&
                !self.is_occupied(&(tile_position + offset))
            })
    }

    fn get_active_piece_iterator<'a>(&'a self) -> impl Iterator<Item = BoardPosition> + 'a {
        let CurrentPiece { piece, position } = self.current_piece.as_ref().unwrap();

        helpers::get_piece_iterator(piece, position, self.get_rotation_system())
    }

    fn is_touching_wall(&self, position: &BoardPosition, wall_direction: &Direction) -> bool {
        match wall_direction {
            Direction::Left => {
                let position_column = position.get_column();

                position_column == 0
            },
            Direction::Right => {
                let position_column = position.get_column();
                let num_columns = self.board_gravity_pair.board().get_num_columns();

                position_column == num_columns - 1
            },
            Direction::Down => {
                let position_row = position.get_row();
                let num_rows = self.board_gravity_pair.board().get_num_rows();

                position_row == num_rows - 1
            }
        }
    }

    fn is_occupied(&self, position: &BoardPosition) -> bool {
        self.board_gravity_pair.board().is_occupied(position)
    }

    fn move_active_piece(&mut self, direction: &Direction) {
        let current_piece = self.current_piece.as_mut().unwrap();
        let position_offset = DIRECTION_OFFSETS.get(&direction).unwrap();

        current_piece.position += position_offset;
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
    Piece::new(PieceKind::L, PieceColor::Red, 0)
}
