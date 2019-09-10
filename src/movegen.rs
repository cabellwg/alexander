use crate::types::*;
use crate::board::{ BitBoard, Board };

const CAPTURE_FLAG: u8 = 0x04;

#[derive(Copy, Clone)]
pub enum MoveType {
    Quiet = 0,
    DoublePawnPush,
    KingsideCastle,
    QueensideCastle,
    Capture,
    EnPassant,
    KnightPromote,
    BishopPromote,
    RookPromote,
    QueenPromote,
    KnightPromoteCapture,
    BishopPromoteCapture,
    RookPromoteCapture,
    QueenPromoteCapture
}

impl MoveType {
    fn move_type_flags(&self) -> u8 {
        let flags = *self as u8;
        if flags > 5 { flags + 2 } else { flags }
    }

    fn is_capture(&self) -> bool {
        self.move_type_flags() & CAPTURE_FLAG != 0
    }
}

pub struct Move {
    side: Side,
    piece: Piece,
    origin: BitBoard,
    target: BitBoard,
    move_type: MoveType
}

impl Move {
    fn is_capture(&self) -> bool {
        self.move_type.is_capture()
    }

    fn apply(&self, board: &Board) -> Board {
        match self.move_type {
            MoveType::Quiet | MoveType::DoublePawnPush => {
                let piece_bb = board.bit_board_for(self.side, self.piece);
                let move_bb = self.origin ^ self.target;
                board.with_bit_board(piece_bb ^ move_bb, self.side, self.piece)
            },
            _ => Board::new()
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_type_flags() {
        assert_eq!(0x01, MoveType::DoublePawnPush.move_type_flags());
        assert_eq!(0x05, MoveType::EnPassant.move_type_flags());
        assert_eq!(0x08, MoveType::KnightPromote.move_type_flags());
        assert_eq!(0x0f, MoveType::QueenPromoteCapture.move_type_flags());
    }

    #[test]
    fn test_is_capture() {
        assert_eq!(false, MoveType::Quiet.is_capture());
        assert_eq!(false, MoveType::DoublePawnPush.is_capture());
        assert_eq!(true, MoveType::Capture.is_capture());
        assert_eq!(true, MoveType::KnightPromoteCapture.is_capture());
    }

    #[test]
    fn test_quiet_move_apply() {
        let board = Board::new();
        let quiet_move = Move {
            side: Side::White,
            piece: Piece::Knight,
            origin: BitBoard::from("b1"),
            target: BitBoard::from("c3"),
            move_type: MoveType::Quiet
        };

        let new_board = quiet_move.apply(&board);

        assert_eq!(BitBoard::from("c3") ^ BitBoard::from("g1"),
                   new_board.bit_board_for(Side::White, Piece::Knight));
        assert_eq!(BitBoard(0x000000000000ff00), new_board.bit_board_for(Side::White, Piece::Pawn));
    }

    #[test]
    fn test_double_pawn_push() {
        let board = Board::new();
        let double_pawn_push = Move {
            side: Side::Black,
            piece: Piece::Pawn,
            origin: BitBoard::from("c7"),
            target: BitBoard::from("c5"),
            move_type: MoveType::DoublePawnPush
        };

        let new_board = double_pawn_push.apply(&board);

        assert_eq!(BitBoard::from("b1") ^ BitBoard::from("g1"),
                   new_board.bit_board_for(Side::White, Piece::Knight));
        assert_eq!(BitBoard(0x00fb000400000000), new_board.bit_board_for(Side::Black, Piece::Pawn));

    }
}
