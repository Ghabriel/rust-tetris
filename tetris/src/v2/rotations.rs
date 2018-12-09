use v2::piece::{PieceGrid, PieceKind};
use std::collections::HashMap;

pub type RotationSystem = HashMap<PieceKind, RotationTable>;
pub type RotationTable = Vec<PieceGrid>;

pub fn build_nintendo_rotation_system() -> RotationSystem {
    let mut result = HashMap::new();

    result.insert(PieceKind::I, nintendo_rotation_i());
    result.insert(PieceKind::J, nintendo_rotation_j());
    result.insert(PieceKind::L, nintendo_rotation_l());
    result.insert(PieceKind::O, nintendo_rotation_o());
    result.insert(PieceKind::S, nintendo_rotation_s());
    result.insert(PieceKind::T, nintendo_rotation_t());
    result.insert(PieceKind::Z, nintendo_rotation_z());

    return result;
}

fn nintendo_rotation_i() -> RotationTable {
    return make_rotation_table(&[
        &["0000", "0000", "1111", "0000"],
        &["0010", "0010", "0010", "0010"],
    ]);
}

fn nintendo_rotation_j() -> RotationTable {
    return make_rotation_table(&[
        &["000", "111", "001"],
        &["010", "010", "110"],
        &["100", "111", "000"],
        &["011", "010", "010"],
    ]);
}

fn nintendo_rotation_l() -> RotationTable {
    return make_rotation_table(&[
        &["000", "111", "100"],
        &["110", "010", "010"],
        &["001", "111", "000"],
        &["010", "010", "011"],
    ]);
}

fn nintendo_rotation_o() -> RotationTable {
    return make_rotation_table(&[
        &["11", "11"],
    ]);
}

fn nintendo_rotation_s() -> RotationTable {
    return make_rotation_table(&[
        &["000", "011", "110"],
        &["010", "011", "001"],
    ]);
}

fn nintendo_rotation_t() -> RotationTable {
    return make_rotation_table(&[
        &["000", "111", "010"],
        &["010", "110", "010"],
        &["010", "111", "000"],
        &["010", "011", "010"],
    ]);
}

fn nintendo_rotation_z() -> RotationTable {
    return make_rotation_table(&[
        &["000", "110", "011"],
        &["001", "011", "010"],
    ]);
}

fn make_rotation_table(grids: &[&[&str]]) -> RotationTable {
    let mut result = Vec::new();

    for grid in grids {
        let mut parsed_grid = PieceGrid(Vec::new());

        for row in grid.iter() {
            for value in row.chars() {
                parsed_grid.0.push(value == '1');
            }
        }

        result.push(parsed_grid);
    }

    result
}
