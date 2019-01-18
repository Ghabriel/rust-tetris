use super::gravity::Gravity;
use super::rotations::RotationSystem;

pub struct Settings {
    pub board_size: (usize, usize),
    pub gravity: Gravity,
    pub rotation_system: RotationSystem,
}
