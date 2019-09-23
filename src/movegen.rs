use crate::board::{BitBoard, Board};
use crate::types::*;

const CAPTURE_FLAG: u8 = 0x04;

#[derive(Copy, Clone)]
pub enum MoveType {
    Quiet = 0,
    DoublePawnPush,
    KingsideCastle,
    QueensideCastle,
    Capture,
    EnPassant,
    KnightPromote = 8,
    BishopPromote,
    RookPromote,
    QueenPromote,
    KnightPromoteCapture,
    BishopPromoteCapture,
    RookPromoteCapture,
    QueenPromoteCapture,
}

impl MoveType {
    fn is_capture(&self) -> bool {
        *self as u8 & CAPTURE_FLAG != 0
    }
}

pub struct Move {
    piece: Piece,
    origin: BitBoard,
    target: BitBoard,
    move_type: MoveType,
}

impl Move {
    fn is_capture(&self) -> bool {
        self.move_type.is_capture()
    }

    fn apply(&self, board: &mut Board) {
        match self.move_type {
            MoveType::Quiet | MoveType::DoublePawnPush => {
                let piece_bb = board.bit_board_for(self.piece);
                let move_bb = self.origin ^ self.target;
                board.set_bit_board(piece_bb ^ move_bb, self.piece)
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_capture() {
        assert_eq!(false, MoveType::Quiet.is_capture());
        assert_eq!(false, MoveType::DoublePawnPush.is_capture());
        assert_eq!(true, MoveType::Capture.is_capture());
        assert_eq!(true, MoveType::KnightPromoteCapture.is_capture());
    }

    #[test]
    fn test_quiet_move_apply() {
        let mut board = Board::new();
        let piece = Piece {
            side: Side::White,
            ptype: PieceType::Knight,
        };
        let quiet_move = Move {
            piece: piece,
            origin: BitBoard::from("b1"),
            target: BitBoard::from("c3"),
            move_type: MoveType::Quiet,
        };

        quiet_move.apply(&mut board);

        assert_eq!(
            BitBoard::from("c3") ^ BitBoard::from("g1"),
            board.bit_board_for(piece)
        );
        assert_eq!(
            BitBoard(0x000000000000ff00),
            board.bit_board_for(Piece {
                side: Side::White,
                ptype: PieceType::Pawn
            })
        );
    }

    #[test]
    fn test_double_pawn_push() {
        let mut board = Board::new();
        let piece = Piece {
            side: Side::Black,
            ptype: PieceType::Pawn,
        };
        let double_pawn_push = Move {
            piece: piece,
            origin: BitBoard::from("c7"),
            target: BitBoard::from("c5"),
            move_type: MoveType::DoublePawnPush,
        };

        double_pawn_push.apply(&mut board);

        assert_eq!(
            BitBoard::from("b1") ^ BitBoard::from("g1"),
            board.bit_board_for(Piece {
                side: Side::White,
                ptype: PieceType::Knight
            })
        );
        assert_eq!(BitBoard(0x00fb000400000000), board.bit_board_for(piece));
    }
}
