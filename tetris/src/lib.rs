pub mod piece;
pub mod rotations;

#[cfg(test)]
mod tests {
    use super::*;
    use piece::{Color, Piece, PieceGrid, PieceKind};
    use rotations::RotationSystem;

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

    #[test]
    fn nintendo_rotation_j() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::J, 0);
        assert_grid_eq(&piece, &rotation_system, "000/111/001");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/010/110");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "100/111/000");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "011/010/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "000/111/001");
    }

    #[test]
    fn nintendo_rotation_l() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::L, 0);
        assert_grid_eq(&piece, &rotation_system, "000/111/100");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "110/010/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "001/111/000");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/010/011");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "000/111/100");
    }

    #[test]
    fn nintendo_rotation_o() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::O, 0);
        assert_grid_eq(&piece, &rotation_system, "11/11");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "11/11");
    }

    #[test]
    fn nintendo_rotation_s() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::S, 0);
        assert_grid_eq(&piece, &rotation_system, "000/011/110");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/011/001");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "000/011/110");
    }

    #[test]
    fn nintendo_rotation_t() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::T, 0);
        assert_grid_eq(&piece, &rotation_system, "000/111/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/110/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/111/000");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/011/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "000/111/010");
    }

    #[test]
    fn nintendo_rotation_z() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::Z, 0);
        assert_grid_eq(&piece, &rotation_system, "000/110/011");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "001/011/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "000/110/011");
    }

    #[test]
    fn nintendo_rotation_t_different_initial_state() {
        let rotation_system = rotations::build_nintendo_rotation_system();

        let mut piece = new_piece(PieceKind::T, 1);
        assert_grid_eq(&piece, &rotation_system, "010/110/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/111/000");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/011/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "000/111/010");

        piece.rotate(&rotation_system);
        assert_grid_eq(&piece, &rotation_system, "010/110/010");
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
