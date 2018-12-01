pub mod piece;
use piece::{Color, PieceGrid, PieceKind};

pub mod rotations;
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nintendo_rotation_i() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::I, 0);
        assert_grid_eq(&piece, &rotation_system, "0000/0000/1111/0000");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "0010/0010/0010/0010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "0000/0000/1111/0000");
    }

    fn new_piece(kind: PieceKind, rotation_index: usize) -> Piece {
        Piece::new(kind, Color::Blue, rotation_index)
    }

    fn assert_grid_eq(piece: &Piece, rotation_system: &RotationSystem, expected: &str) {
        assert_eq!(
            readable_grid(piece.get_grid(rotation_system)),
            expected.replace("/", "")
        );
    }

    fn readable_grid(grid: &PieceGrid) -> String {
        let mut result = String::new();

        for &cell in &grid.0 {
            result += if cell { "1" } else { "0" };
        }

        result
    }
}
