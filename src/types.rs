/// Pick a side
#[derive(Copy, Clone)]
pub enum Side {
    White,
    Black
}

/// Piece types
#[derive(Copy, Clone)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}
