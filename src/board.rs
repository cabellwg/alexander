use crate::types::Side;

const WHITE_PAWN_START_POS: u64   = 0x000000000000ff00;
const WHITE_KNIGHT_START_POS: u64 = 0x0000000000000042;
const WHITE_BISHOP_START_POS: u64 = 0x0000000000000024;
const WHITE_ROOK_START_POS: u64   = 0x0000000000000081;
const WHITE_QUEEN_START_POS: u64  = 0x0000000000000010;
const WHITE_KING_START_POS: u64   = 0x0000000000000008;
const BLACK_PAWN_START_POS: u64   = 0x00ff000000000000;
const BLACK_KNIGHT_START_POS: u64 = 0x4200000000000000;
const BLACK_BISHOP_START_POS: u64 = 0x2400000000000000;
const BLACK_ROOK_START_POS: u64   = 0x8100000000000000;
const BLACK_QUEEN_START_POS: u64  = 0x1000000000000000;
const BLACK_KING_START_POS: u64   = 0x0800000000000000;


/// Little-endian rank-file bitboard
/// 
/// ```
/// bitIndex = rankIndex * 8 + fileIndex
/// ```
///
pub struct BitBoard {
    position: u64
}

pub struct PieceSet {
    pawns: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks: BitBoard,
    queens: BitBoard,
    king: BitBoard
}

/// Bitboards for all pieces of a color
impl PieceSet {
    fn new(side: Side) -> PieceSet {
        match side {
            Side::White =>  {
                PieceSet {
                    pawns:   BitBoard { position: WHITE_PAWN_START_POS },
                    knights: BitBoard { position: WHITE_KNIGHT_START_POS },
                    bishops: BitBoard { position: WHITE_BISHOP_START_POS },
                    rooks:   BitBoard { position: WHITE_ROOK_START_POS },
                    queens:  BitBoard { position: WHITE_QUEEN_START_POS },
                    king:    BitBoard { position: WHITE_KING_START_POS }
                }
            },
            Side::Black => {
                PieceSet {
                    pawns:   BitBoard { position: BLACK_PAWN_START_POS },
                    knights: BitBoard { position: BLACK_KNIGHT_START_POS },
                    bishops: BitBoard { position: BLACK_BISHOP_START_POS },
                    rooks:   BitBoard { position: BLACK_ROOK_START_POS },
                    queens:  BitBoard { position: BLACK_QUEEN_START_POS },
                    king:    BitBoard { position: BLACK_KING_START_POS }
                }
            }
        }
    }
}

/// Bitboards for all pieces
pub struct PiecewiseBoard {
    white: PieceSet,
    black: PieceSet
}

impl PiecewiseBoard {
    pub fn new() -> PiecewiseBoard {
        PiecewiseBoard {
            white: PieceSet::new(Side::White),
            black: PieceSet::new(Side::Black)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pieceset_initialization() {
        let board = PiecewiseBoard::new();
        
        assert_eq!(WHITE_PAWN_START_POS, board.white.pawns.position);
        assert_eq!(WHITE_KING_START_POS, board.white.king.position);
        assert_eq!(BLACK_ROOK_START_POS, board.black.rooks.position);
        assert_eq!(BLACK_QUEEN_START_POS, board.black.queens.position);
    }
}
