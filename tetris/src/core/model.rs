use lazy_static::lazy_static;
use sfml::window::Key;
use std::collections::HashMap;
use super::super::board::{Block, Board, MaterializationStatus, SimpleBoard};
use super::super::gravity::{BoardGravityPair, Gravity};
use super::super::gravity::naive::{NaiveGravity, NaiveGravityPair};
use super::super::piece::{Piece, PieceColor, PieceKind};
use super::super::position::{BoardPosition, BoardPositionOffset};
use super::super::rotations::{RotationDirection, RotationSystem};
use super::super::settings::Settings;
use super::traits::Tick;
use super::{ActivePiece, Delay, InputHandler};

pub struct Model {
    board_gravity_pair: Box<dyn BoardGravityPair>,
    active_piece: Option<ActivePiece>,
    input_handler: InputHandler,
    settings: Settings,
    running: bool,
    delay: Delay,
}

#[derive(PartialEq, Eq, Hash)]
pub enum Direction {
    Left,
    Right,
    Down,
}

lazy_static! {
    pub static ref DIRECTION_OFFSETS: HashMap<Direction, BoardPositionOffset> = {
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
        self.get_board().for_each_row(callback);
    }

    pub fn get_active_piece(&self) -> &Option<ActivePiece> {
        &self.active_piece
    }

    pub fn get_rotation_system(&self) -> &RotationSystem {
        &self.settings.rotation_system
    }

    pub fn get_board_num_rows(&self) -> usize {
        self.get_board().get_num_rows()
    }

    pub fn get_board_num_columns(&self) -> usize {
        self.get_board().get_num_columns()
    }
}

/**
 * Other getters
 */
impl Model {
    fn get_board(&self) -> &dyn Board {
        self.board_gravity_pair.board()
    }
}

impl Tick for Model {
    fn tick(&mut self, elapsed_time_nanos: u64) -> bool {
        if !self.running {
            return false;
        }

        if !self.has_active_piece() {
            self.spawn_piece();
            return false;
        }

        self.handle_input();

        self.delay.tick();
        if self.delay.block_for_frames(15) {
            return false;
        }

        let can_move_down = self.active_piece
            .as_mut()
            .unwrap()
            .can_move_towards(
                &Direction::Down,
                &self.settings.rotation_system,
                self.board_gravity_pair.board()
            );

        if can_move_down {
            self.get_active_piece_mut().move_towards(&Direction::Down);
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
            active_piece: None,
            input_handler: InputHandler::new(),
            settings: settings,
            running: true, // TODO: change to false later
            delay: Delay::new(),
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
        self.active_piece.is_some()
    }
}

/**
 * spawn_piece implementation + helpers
 */
impl Model {
    fn spawn_piece(&mut self) {
        let piece = random_piece();
        let position = self.get_centralized_position_for(&piece);

        self.active_piece = Some(ActivePiece { piece, position });
    }

    fn get_centralized_position_for(&self, piece: &Piece) -> BoardPosition {
        let grid_num_columns = self.get_grid_num_columns(piece);
        let board_num_columns = self.get_board_num_columns();

        BoardPosition::new(
            0,
            ((board_num_columns - grid_num_columns) / 2) as isize,
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
                Key::Left | Key::Right => {
                    let direction = match key {
                        Key::Left => Direction::Left,
                        Key::Right => Direction::Right,
                        _ => unreachable!(),
                    };

                    self.active_piece
                        .as_mut()
                        .unwrap()
                        .try_move_towards(
                            direction,
                            &self.settings.rotation_system,
                            self.board_gravity_pair.board(),
                        );
                },
                Key::A | Key::S => {
                    let direction = match key {
                        Key::A => RotationDirection::Counterclockwise,
                        Key::S => RotationDirection::Clockwise,
                        _ => unreachable!(),
                    };

                    self.active_piece
                        .as_mut()
                        .unwrap()
                        .try_rotate(
                            direction,
                            &self.settings.rotation_system,
                            self.board_gravity_pair.board(),
                        );
                },
                _ => {},
            }
        });
    }

    fn get_active_piece_mut(&mut self) -> &mut ActivePiece {
        self.active_piece.as_mut().unwrap()
    }
}

/**
 * materialize_active_piece implementation
 */
impl Model {
    fn materialize_active_piece(&mut self) -> MaterializationStatus {
        let ActivePiece { piece, position } = self.active_piece.take().unwrap();

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
        let filled_rows = self.get_board().get_filled_rows();

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
