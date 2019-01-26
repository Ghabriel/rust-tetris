use super::super::rotations::{RotationDirection, RotationSystem, RotationTable};
use super::{PieceColor, PieceGrid, PieceKind};

pub struct Piece {
    kind: PieceKind,
    color: PieceColor,
    rotation_index: usize,
}

impl Piece {
    pub fn new(kind: PieceKind, color: PieceColor, rotation_index: usize) -> Piece {
        Piece { kind, color, rotation_index }
    }

    pub fn rotate(&mut self, direction: &RotationDirection, rotation_system: &RotationSystem) {
        let rotation_table = self.get_rotation_table(rotation_system);
        let num_rotations = rotation_table.len();

        match direction {
            RotationDirection::Clockwise => {
                self.rotation_index += 1;
                self.rotation_index %= num_rotations;
            },
            RotationDirection::Counterclockwise => {
                self.rotation_index += num_rotations - 1;
                self.rotation_index %= num_rotations;
            }
        }
    }

    pub fn get_grid<'a>(&self, rotation_system: &'a RotationSystem) -> &'a PieceGrid {
        let rotation_table = self.get_rotation_table(rotation_system);

        &rotation_table[self.rotation_index]
    }

    pub fn get_color(&self) -> &PieceColor {
        &self.color
    }

    fn get_rotation_table<'a>(&self, rotation_system: &'a RotationSystem) -> &'a RotationTable {
        rotation_system.get(&self.kind).expect("Incomplete rotation system")
    }
}
