pub mod properties;

pub use self::properties::{Color, PieceGrid, PieceKind};

use rotations::{RotationSystem, RotationTable};

pub struct Piece {
    kind: PieceKind,
    color: Color,
    rotation_index: usize,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color, rotation_index: usize) -> Piece {
        Piece { kind, color, rotation_index }
    }

    pub fn rotate(&mut self, rotation_system: &RotationSystem) {
        let rotation_table = self.get_rotation_table(rotation_system);

        self.rotation_index += 1;
        self.rotation_index %= rotation_table.len();
    }

    pub fn get_grid<'a>(&self, rotation_system: &'a RotationSystem) -> &'a PieceGrid {
        let rotation_table = self.get_rotation_table(rotation_system);

        &rotation_table[self.rotation_index]
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    fn get_rotation_table<'a>(&self, rotation_system: &'a RotationSystem) -> &'a RotationTable {
        rotation_system.get(&self.kind).expect("Incomplete rotation system")
    }
}