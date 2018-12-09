mod make_rotation_table;
mod nintendo_rotations;
mod rotation_system;
mod rotation_table;

#[cfg(test)]
mod tests;

pub use self::nintendo_rotations::build_nintendo_rotation_system;
pub use self::rotation_system::RotationSystem;
pub use self::rotation_table::RotationTable;

use self::make_rotation_table::make_rotation_table;
