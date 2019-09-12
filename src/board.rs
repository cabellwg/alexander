use std::fmt;
use std::error::Error;
use std::ops::BitXor;
use std::str::FromStr;

use regex::Regex;

use crate::types::*;

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
#[derive(PartialEq, Copy, Clone)]
pub struct BitBoard(pub u64);

impl From<&str> for BitBoard {
    /// Maps a coordinate to a square on a bitboard
    fn from(square: &str) -> Self {
        let bit_index = lerf_index_for(square).unwrap_or(u8::max_value());

        if bit_index < u8::max_value() {
            return BitBoard(1u64 << bit_index)
        }

        BitBoard(0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 ^ rhs.0) 
    }
}

impl fmt::Debug for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BitBoard:
               8 {:08b}
               7 {:08b}
               6 {:08b}
               5 {:08b}
               4 {:08b}
               3 {:08b}
               2 {:08b}
               1 {:08b}
                 abcdefgh",
               (self.0.reverse_bits() & 0x00000000000000ff),
               (self.0.reverse_bits() & 0x000000000000ff00) >> 8,
               (self.0.reverse_bits() & 0x0000000000ff0000) >> 2 * 8,
               (self.0.reverse_bits() & 0x00000000ff000000) >> 3 * 8,
               (self.0.reverse_bits() & 0x000000ff00000000) >> 4 * 8,
               (self.0.reverse_bits() & 0x0000ff0000000000) >> 5 * 8,
               (self.0.reverse_bits() & 0x00ff000000000000) >> 6 * 8,
               (self.0.reverse_bits() & 0xff00000000000000) >> 7 * 8
               )
    }
}


/// 8x8 board to store pieces by square
///
/// Indexed by file then rank
#[derive(Copy, Clone)]
pub struct _8x8Board(pub [[(Side, Piece); 8]; 8]);

impl _8x8Board {
    /// Creates a new 8x8 board with the default piece configuration
    // pub fn new() -> _8x8Board {
    //     _8x8Board()
    // }

    pub fn add_piece(side: Side, piece: Piece, square: &str) {
    }
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
                    pawns:   BitBoard(WHITE_PAWN_START_POS),
                    knights: BitBoard(WHITE_KNIGHT_START_POS),
                    bishops: BitBoard(WHITE_BISHOP_START_POS),
                    rooks:   BitBoard(WHITE_ROOK_START_POS),
                    queens:  BitBoard(WHITE_QUEEN_START_POS),
                    king:    BitBoard(WHITE_KING_START_POS)
                }
            },
            Side::Black => {
                PieceSet {
                    pawns:   BitBoard(BLACK_PAWN_START_POS),
                    knights: BitBoard(BLACK_KNIGHT_START_POS),
                    bishops: BitBoard(BLACK_BISHOP_START_POS),
                    rooks:   BitBoard(BLACK_ROOK_START_POS),
                    queens:  BitBoard(BLACK_QUEEN_START_POS),
                    king:    BitBoard(BLACK_KING_START_POS)
                }
            }
        }
    }

    fn bit_board_for(&self, piece: Piece) -> BitBoard {
        match piece {
            Piece::Pawn => self.pawns,
            Piece::Knight => self.knights,
            Piece::Bishop => self.bishops,
            Piece::Rook => self.rooks,
            Piece::Queen => self.queens,
            Piece::King => self.king
        }
    }

    fn set_bit_board(&mut self, bit_board: BitBoard, piece: Piece) {
        match piece {
            Piece::Pawn => { self.pawns = bit_board; },
            Piece::Knight => { self.knights = bit_board; },
            Piece::Bishop => { self.bishops = bit_board; },
            Piece::Rook => { self.rooks = bit_board; },
            Piece::Queen => { self.queens = bit_board; },
            Piece::King => { self.king = bit_board; }
        }
    }
}

/// Bitboards for all pieces on the board
pub struct Board {
    white: PieceSet,
    black: PieceSet
}

impl Board {
    pub fn new() -> Board {
        Board {
            white: PieceSet::new(Side::White),
            black: PieceSet::new(Side::Black)
        }
    }

    pub fn bit_board_for(&self, side: Side, piece: Piece) -> BitBoard {
        match side {
            Side::White => self.white.bit_board_for(piece),
            Side::Black => self.black.bit_board_for(piece)
        }
    }

    pub fn set_bit_board(&mut self, bit_board: BitBoard, side: Side, piece: Piece) {
        match side {
            Side::White => {
                self.white.set_bit_board(bit_board, piece);
            },
            Side::Black => {
                self.black.set_bit_board(bit_board, piece);
            }
        }
    }
}

/// Error type for square parse errors
#[derive(Debug)]
struct InvalidSquareError {
    msg: String
}

impl fmt::Display for InvalidSquareError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid square: {}", self.msg)
    }
}

/// Little-endian rank-file index of a square
/// 
/// ```
/// lerf_index = rank_index * 8 + file_index
/// ```
fn lerf_index_for(square: &str) -> Result<u8, InvalidSquareError> {
    let filtre = Regex::new(r"(?i)[a-h][1-8]").unwrap();
    if !filtre.is_match(square) { return Err(InvalidSquareError { msg: square.to_string() }) }

    let sqre = Regex::new(r"(?P<file>[a-h])(?P<rank>[1-8])").unwrap();
    let captures = sqre.captures(square).unwrap();
    let rank_str = &captures["rank"];
    let file_str = &captures["file"];

    let file_index = file_index_of(file_str);
    let rank_index = u8::from_str(rank_str).unwrap() - 1;

    Ok(rank_index * 8 + file_index)
}

/// Maps a file to its numerical index
/// 
/// ```
/// a = 0,
/// b = 1,
/// ...
/// h = 7
/// ```
fn file_index_of(file: &str) -> u8 {
    match file {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => u8::max_value()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_index_of() {
        assert_eq!(0, file_index_of("a"));
        assert_eq!(4, file_index_of("e"));
        assert_eq!(7, file_index_of("h"));
        assert_eq!(u8::max_value(), file_index_of("j"));
    }

    #[test]
    fn test_bit_for_square() {
        assert_eq!(BitBoard(0x0100000000000000), BitBoard::from("a8"));
        assert_eq!(BitBoard(0x0000000000000001), BitBoard::from("a1"));
        assert_eq!(BitBoard(0x8000000000000000), BitBoard::from("h8"));
        assert_eq!(BitBoard(0x0004000000000000), BitBoard::from("c7"));
        assert_eq!(BitBoard(0), BitBoard::from("bad input"));
    }

    #[test]
    fn test_bit_board_xor() {
        assert_eq!(0x3fba81 ^ 0xfbfbab, (BitBoard(0x3fba81) ^ BitBoard(0xfbfbab)).0);
        assert_eq!(0xf1f1f1 ^ 0xf1f1f1, BitBoard(0).0);
    }

    #[test]
    fn test_pieceset_initialization() {
        let board = Board::new();
        
        assert_eq!(WHITE_PAWN_START_POS, board.white.pawns.0);
        assert_eq!(WHITE_KING_START_POS, board.white.king.0);
        assert_eq!(BLACK_ROOK_START_POS, board.black.rooks.0);
        assert_eq!(BLACK_QUEEN_START_POS, board.black.queens.0);
    }

    #[test]
    fn test_pieceset_set_bit_board() {
        let mut pieces = PieceSet::new(Side::White);

        pieces.set_bit_board(BitBoard(0xf), Piece::Knight);

        assert_eq!(BitBoard(0xf), pieces.knights);

        pieces.set_bit_board(BitBoard(0xd), Piece::King);

        assert_eq!(BitBoard(0xd), pieces.king);
    }

    #[test]
    fn test_board_set_bit_board() {
        let mut board = Board::new();

        board.set_bit_board(BitBoard(0xe), Side::Black, Piece::Queen);

        assert_eq!(BitBoard(0xe), board.black.queens);
        assert_eq!(BitBoard(WHITE_QUEEN_START_POS), board.white.queens);
    }
}
