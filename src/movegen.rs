use std::error::Error;

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
    origin: String,
    target: String,
    move_type: MoveType,
}

impl Move {
    fn is_capture(&self) -> bool {
        self.move_type.is_capture()
    }

    fn apply(&self, board: &mut Board) -> Result<(), Box<dyn Error>> {
        match self.move_type {
            MoveType::Quiet | MoveType::DoublePawnPush => {
                let piece_bb = board.bit_board_for(self.piece);
                let move_bb = BitBoard::from(self.origin.as_str())
                    ^ BitBoard::from(self.target.as_str());
                board.set_bit_board(piece_bb ^ move_bb, self.piece);
                board.set_square(self.origin.as_str(), None)?;
                board.set_square(self.target.as_str(), Some(self.piece))?;
            },
            MoveType::KingsideCastle => {
            },
            _ => (),
        };
        Ok(())
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
            origin: "b1".to_string(),
            target: "c3".to_string(),
            move_type: MoveType::Quiet,
        };

        assert!(quiet_move.apply(&mut board).is_ok());

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
        assert_eq!(Some(piece), board.get_square("c3"));
        assert_eq!(None, board.get_square("b1"));
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
            origin: "c7".to_string(),
            target: "c5".to_string(),
            move_type: MoveType::DoublePawnPush,
        };

        assert!(double_pawn_push.apply(&mut board).is_ok());

        assert_eq!(
            BitBoard::from("b1") ^ BitBoard::from("g1"),
            board.bit_board_for(Piece {
                side: Side::White,
                ptype: PieceType::Knight
            })
        );
        assert_eq!(BitBoard(0x00fb000400000000), board.bit_board_for(piece));
        assert_eq!(Some(piece), board.get_square("c5"));
        assert_eq!(None, board.get_square("c7"));
    }
}
