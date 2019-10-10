use std::error::Error;
use std::convert::TryFrom;
use std::fmt;

/// Pick a side
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Side {
    White,
    Black,
}

/// Piece types
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub ptype: PieceType,
    pub side: Side,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.side {
            Side::White => match self.ptype {
                PieceType::Pawn => write!(f, "♙"),
                PieceType::Knight => write!(f, "♘"),
                PieceType::Bishop => write!(f, "♗"),
                PieceType::Rook => write!(f, "♖"),
                PieceType::Queen => write!(f, "♕"),
                PieceType::King => write!(f, "♔"),
            },
            Side::Black => match self.ptype {
                PieceType::Pawn => write!(f, "♟"),
                PieceType::Knight => write!(f, "♞"),
                PieceType::Bishop => write!(f, "♝"),
                PieceType::Rook => write!(f, "♜"),
                PieceType::Queen => write!(f, "♛"),
                PieceType::King => write!(f, "♚"),
            },
        }
    }
}

impl TryFrom<&str> for Piece {
    type Error = InvalidPieceError;

    fn try_from(piece: &str) -> Result<Self, Self::Error> {
        match piece {
            "♙" => Ok(Piece {
                side: Side::White,
                ptype: PieceType::Pawn,
            }),
            "♘" => Ok(Piece {
                side: Side::White,
                ptype: PieceType::Knight,
            }),
            "♗" => Ok(Piece {
                side: Side::White,
                ptype: PieceType::Bishop,
            }),
            "♖" => Ok(Piece {
                side: Side::White,
                ptype: PieceType::Rook,
            }),
            "♕" => Ok(Piece {
                side: Side::White,
                ptype: PieceType::Queen,
            }),
            "♔" => Ok(Piece {
                side: Side::White,
                ptype: PieceType::King,
            }),
            "♟" => Ok(Piece {
                side: Side::Black,
                ptype: PieceType::Pawn,
            }),
            "♞" => Ok(Piece {
                side: Side::Black,
                ptype: PieceType::Knight,
            }),
            "♝" => Ok(Piece {
                side: Side::Black,
                ptype: PieceType::Bishop,
            }),
            "♜" => Ok(Piece {
                side: Side::Black,
                ptype: PieceType::Rook,
            }),
            "♛" => Ok(Piece {
                side: Side::Black,
                ptype: PieceType::Queen,
            }),
            "♚" => Ok(Piece {
                side: Side::Black,
                ptype: PieceType::King,
            }),
            _ => Err(InvalidPieceError {
                msg: String::from(piece),
            }),
        }
    }
}

// Error types

/// Error type for piece parse errors
#[derive(Debug)]
pub struct InvalidPieceError {
    pub msg: String,
}

impl Error for InvalidPieceError {}

impl fmt::Display for InvalidPieceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid piece: {}", self.msg)
    }
}

/// Error type for square parse errors
#[derive(Debug)]
pub struct InvalidSquareError {
    pub msg: String,
}

impl Error for InvalidSquareError {}

impl fmt::Display for InvalidSquareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid square: {}", self.msg)
    }
}

/// Error type for file parse errors
#[derive(Debug, PartialEq)]
pub struct InvalidFileError {
    pub msg: String,
}

impl Error for InvalidFileError {}

impl fmt::Display for InvalidFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid file: {}", self.msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_try_from() {
        assert_eq!(
            Piece {
                side: Side::White,
                ptype: PieceType::Queen
            },
            Piece::try_from("♕").unwrap()
        );
        assert!(Piece::try_from("bad input").is_err());
    }
}
