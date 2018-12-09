use std::collections::HashMap;
use super::super::piece::PieceKind;
use super::RotationTable;

pub type RotationSystem = HashMap<PieceKind, RotationTable>;
