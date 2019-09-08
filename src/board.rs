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


/// Bitboards for all pieces
pub struct PiecewiseBoard {
    pawns: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks: BitBoard,
    queens: BitBoard,
    king: BitBoard
}

impl PiecewiseBoard {
    fn new(side: Side) -> PiecewiseBoard {
        match side {
            Side::White =>  {
                PiecewiseBoard {
                    pawns:   BitBoard { position: WHITE_PAWN_START_POS },
                    knights: BitBoard { position: WHITE_KNIGHT_START_POS },
                    bishops: BitBoard { position: WHITE_BISHOP_START_POS },
                    rooks:   BitBoard { position: WHITE_ROOK_START_POS },
                    queens:  BitBoard { position: WHITE_QUEEN_START_POS },
                    king:    BitBoard { position: WHITE_KING_START_POS }
                }
            },
            Side::Black => {
                PiecewiseBoard {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piecewise_board_initialization() {
        let board = PiecewiseBoard::new(Side::White);
        
        assert_eq!(WHITE_PAWN_START_POS, board.pawns.position);
        assert_eq!(WHITE_KING_START_POS, board.king.position);

        let board = PiecewiseBoard::new(Side::Black);

        assert_eq!(BLACK_ROOK_START_POS, board.rooks.position);
        assert_eq!(BLACK_QUEEN_START_POS, board.queens.position);
    }
}
