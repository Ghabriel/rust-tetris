#[derive(PartialEq, Eq, Hash)]
pub enum PieceKind {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

pub struct PieceGrid(pub Vec<bool>);

#[derive(Clone)]
pub enum Color {
    Cyan,
    Orange,
    Red,
    Purple,
    Yellow,
    Green,
    Blue,
}
