use super::gravity::Gravity;
use super::rotations::{self, RotationSystem};

pub const TILE_SIZE: usize = 18;
pub const TILE_SCALING: f32 = 1.5;

pub const BOARD_VIEW_POSITION_X: f32 = 10.;
pub const BOARD_VIEW_POSITION_Y: f32 = 20.;

pub struct Settings {
    pub board_size: (usize, usize),
    pub gravity: Gravity,
    pub rotation_system: RotationSystem,
}

pub fn make_default_settings() -> Settings {
    Settings {
        board_size: (15, 20),
        gravity: Gravity::Naive,
        rotation_system: rotations::build_nintendo_rotation_system(),
    }
}
