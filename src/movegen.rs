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
    origin: u8,
    target: u8,
    move_type: MoveType
}

impl Move {
    fn is_capture(&self) -> bool {
        self.move_type.is_capture()
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
}
