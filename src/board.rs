use std::convert::TryFrom;
use std::fmt;
use std::ops::BitXor;
use std::str::FromStr;

use regex::Regex;

use crate::types::*;

const WHITE_PAWN_START_POS: u64 = 0x000000000000ff00;
const WHITE_KNIGHT_START_POS: u64 = 0x0000000000000042;
const WHITE_BISHOP_START_POS: u64 = 0x0000000000000024;
const WHITE_ROOK_START_POS: u64 = 0x0000000000000081;
const WHITE_QUEEN_START_POS: u64 = 0x0000000000000010;
const WHITE_KING_START_POS: u64 = 0x0000000000000008;
const BLACK_PAWN_START_POS: u64 = 0x00ff000000000000;
const BLACK_KNIGHT_START_POS: u64 = 0x4200000000000000;
const BLACK_BISHOP_START_POS: u64 = 0x2400000000000000;
const BLACK_ROOK_START_POS: u64 = 0x8100000000000000;
const BLACK_QUEEN_START_POS: u64 = 0x1000000000000000;
const BLACK_KING_START_POS: u64 = 0x0800000000000000;

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
        let bit_index = lerf_index_for(square).unwrap();

        BitBoard(1u64 << bit_index)
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
        let bits = self.0.reverse_bits();
        write!(
            f,
            "BitBoard:
               8 {:08b}
               7 {:08b}
               6 {:08b}
               5 {:08b}
               4 {:08b}
               3 {:08b}
               2 {:08b}
               1 {:08b}
                 abcdefgh",
            (bits & 0x00000000000000ff),
            (bits & 0x000000000000ff00) >> 8,
            (bits & 0x0000000000ff0000) >> 2 * 8,
            (bits & 0x00000000ff000000) >> 3 * 8,
            (bits & 0x000000ff00000000) >> 4 * 8,
            (bits & 0x0000ff0000000000) >> 5 * 8,
            (bits & 0x00ff000000000000) >> 6 * 8,
            (bits & 0xff00000000000000) >> 7 * 8
        )
    }
}

/// 8x8 board to store pieces by square
///
/// Indexed by file then rank
#[derive(Copy, Clone)]
pub struct _8x8Board([[(Option<Piece>); 8]; 8]);

impl _8x8Board {
    /// Creates a new 8x8 board with the default piece configuration
    pub fn new() -> _8x8Board {
        _8x8Board::try_from([
            ["♜", "♞", "♝", "♛", "♚", "♝", "♞", "♜"],
            ["♟", "♟", "♟", "♟", "♟", "♟", "♟", "♟"],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["", "", "", "", "", "", "", ""],
            ["♙", "♙", "♙", "♙", "♙", "♙", "♙", "♙"],
            ["♖", "♘", "♗", "♕", "♔", "♗", "♘", "♖"],
        ])
        .unwrap()
    }

    /// Creates a new, empty 8x8 board
    pub fn empty() -> _8x8Board {
        _8x8Board([[None; 8]; 8])
    }

    /// Sets the value of a square on the board
    pub fn set_square(
        &mut self,
        square: &str,
        value: Option<Piece>,
    ) -> Result<(), InvalidSquareError> {
        let bit_index = lerf_index_for(square)? as usize;
        self.0[bit_index / 8][bit_index % 8] = value;
        Ok(())
    }

    /// Gets the value of a square on the board
    pub fn get_square(&self, square: &str) -> Option<Piece> {
        let bit_index = lerf_index_for(square).unwrap() as usize;
        self.0[bit_index / 8][bit_index % 8]
    }
}

impl TryFrom<[[&str; 8]; 8]> for _8x8Board {
    type Error = InvalidPieceError;

    fn try_from(board: [[&str; 8]; 8]) -> Result<Self, Self::Error> {
        let mut new_board = Self::empty();

        for (rank_index, rank) in board.iter().enumerate() {
            for (file_index, piece) in rank.iter().enumerate() {
                new_board.0[7 - rank_index][file_index] = match piece {
                    &"" => None,
                    _ => Some(Piece::try_from(board[rank_index][file_index])?),
                }
            }
        }

        Ok(new_board)
    }
}

impl fmt::Display for _8x8Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut display_str = "".to_string();
        for (rank_index, rank) in self.0.iter().enumerate() {
            display_str.push_str(&format!("{} ", 8 - rank_index));
            for (_file_index, piece) in rank.iter().enumerate() {
                if let Some(piece) = piece {
                    display_str += &piece.to_string();
                } else {
                    display_str += &".".to_string();
                }
            }
            display_str += &"\n".to_string();
        }

        display_str += &"\n  abcdefgh".to_string();

        write!(f, "{}", display_str)
    }
}

pub struct PieceSet {
    pawns: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks: BitBoard,
    queens: BitBoard,
    king: BitBoard,
}

/// Bitboards for all pieces of a color
impl PieceSet {
    fn new(side: Side) -> PieceSet {
        match side {
            Side::White => PieceSet {
                pawns: BitBoard(WHITE_PAWN_START_POS),
                knights: BitBoard(WHITE_KNIGHT_START_POS),
                bishops: BitBoard(WHITE_BISHOP_START_POS),
                rooks: BitBoard(WHITE_ROOK_START_POS),
                queens: BitBoard(WHITE_QUEEN_START_POS),
                king: BitBoard(WHITE_KING_START_POS),
            },
            Side::Black => PieceSet {
                pawns: BitBoard(BLACK_PAWN_START_POS),
                knights: BitBoard(BLACK_KNIGHT_START_POS),
                bishops: BitBoard(BLACK_BISHOP_START_POS),
                rooks: BitBoard(BLACK_ROOK_START_POS),
                queens: BitBoard(BLACK_QUEEN_START_POS),
                king: BitBoard(BLACK_KING_START_POS),
            },
        }
    }

    fn bit_board_for(&self, piece: PieceType) -> BitBoard {
        match piece {
            PieceType::Pawn => self.pawns,
            PieceType::Knight => self.knights,
            PieceType::Bishop => self.bishops,
            PieceType::Rook => self.rooks,
            PieceType::Queen => self.queens,
            PieceType::King => self.king,
        }
    }

    fn set_bit_board(&mut self, bit_board: BitBoard, piece: PieceType) {
        match piece {
            PieceType::Pawn => {
                self.pawns = bit_board;
            }
            PieceType::Knight => {
                self.knights = bit_board;
            }
            PieceType::Bishop => {
                self.bishops = bit_board;
            }
            PieceType::Rook => {
                self.rooks = bit_board;
            }
            PieceType::Queen => {
                self.queens = bit_board;
            }
            PieceType::King => {
                self.king = bit_board;
            }
        }
    }
}

/// Bitboards for all pieces on the board
pub struct Board {
    white: PieceSet,
    black: PieceSet,
    squares: _8x8Board,
}

impl Board {
    pub fn new() -> Board {
        Board {
            white: PieceSet::new(Side::White),
            black: PieceSet::new(Side::Black),
            squares: _8x8Board::new(),
        }
    }

    pub fn bit_board_for(&self, piece: Piece) -> BitBoard {
        match piece.side {
            Side::White => self.white.bit_board_for(piece.ptype),
            Side::Black => self.black.bit_board_for(piece.ptype),
        }
    }

    pub fn set_bit_board(&mut self, bit_board: BitBoard, piece: Piece) {
        match piece.side {
            Side::White => {
                self.white.set_bit_board(bit_board, piece.ptype);
            }
            Side::Black => {
                self.black.set_bit_board(bit_board, piece.ptype);
            }
        }
    }

    pub fn get_square(&self, square: &str) -> Option<Piece> {
        self.squares.get_square(square)
    }

    pub fn set_square(
        &mut self,
        square: &str,
        piece: Option<Piece>,
    ) -> Result<(), InvalidSquareError> {
        self.squares.set_square(square, piece)
    }
}

/// Little-endian rank-file index of a square
///
/// ```
/// lerf_index = rank_index * 8 + file_index
/// ```
fn lerf_index_for(square: &str) -> Result<u8, InvalidSquareError> {
    let filtre = Regex::new(r"(?i)[a-h][1-8]").unwrap();
    if !filtre.is_match(square) {
        return Err(InvalidSquareError {
            msg: square.to_string(),
        });
    }

    let sqre = Regex::new(r"(?P<file>[a-h])(?P<rank>[1-8])").unwrap();
    let captures = sqre.captures(square).unwrap();
    let rank_str = &captures["rank"];
    let file_str = &captures["file"];

    let file_chr = file_str.chars().next().unwrap();

    let file_index = file_index_of(file_chr).unwrap();
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
pub fn file_index_of(file: char) -> Result<u8, InvalidFileError> {
    let chrindex = file.to_digit(18);
    if chrindex.is_none() {
        return Err(InvalidFileError {
            msg: format!("File out of range: {}", file),
        });
    }

    let index = chrindex.unwrap() - 'a'.to_digit(18).unwrap();

    let index = index as u8;
    if index < 8 {
        return Ok(index);
    }

    Err(InvalidFileError {
        msg: file.to_string(),
    })
}

/// Maps a numerical index to a file
///
/// Inverse of `file_index_of`
pub fn file_for_index(index: u8) -> Result<String, InvalidFileError> {
    if index > 7 {
        return Err(InvalidFileError {
            msg: format!("{}", index),
        });
    }
    let files = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    Ok(files[index as usize].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_index_of() {
        assert_eq!(Ok(0), file_index_of('a'));
        assert_eq!(Ok(4), file_index_of('e'));
        assert_eq!(Ok(7), file_index_of('h'));
        assert!(file_index_of('j').is_err());
    }

    #[test]
    fn test_bit_for_square() {
        assert_eq!(BitBoard(0x0100000000000000), BitBoard::from("a8"));
        assert_eq!(BitBoard(0x0000000000000001), BitBoard::from("a1"));
        assert_eq!(BitBoard(0x8000000000000000), BitBoard::from("h8"));
        assert_eq!(BitBoard(0x0004000000000000), BitBoard::from("c7"));
    }

    #[test]
    #[should_panic]
    fn test_bit_for_square_with_bad_input() {
        BitBoard::from("bad input");
    }

    #[test]
    fn test_bit_board_xor() {
        assert_eq!(
            0x3fba81 ^ 0xfbfbab,
            (BitBoard(0x3fba81) ^ BitBoard(0xfbfbab)).0
        );
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

        pieces.set_bit_board(BitBoard(0xf), PieceType::Knight);

        assert_eq!(BitBoard(0xf), pieces.knights);

        pieces.set_bit_board(BitBoard(0xd), PieceType::King);

        assert_eq!(BitBoard(0xd), pieces.king);
    }

    #[test]
    fn test_8x8_board_initialization() {
        let board = _8x8Board::new();

        assert_eq!(None, board.get_square(&"a4"));
        assert_eq!(None, board.get_square(&"d5"));
        assert_eq!(
            Some(Piece {
                side: Side::White,
                ptype: PieceType::Queen
            }),
            board.get_square(&"d1")
        );
        assert_eq!(
            Some(Piece {
                side: Side::White,
                ptype: PieceType::King
            }),
            board.get_square(&"e1")
        );
        assert_eq!(
            Some(Piece {
                side: Side::Black,
                ptype: PieceType::Queen
            }),
            board.get_square(&"d8")
        );
        assert_eq!(
            Some(Piece {
                side: Side::Black,
                ptype: PieceType::Bishop
            }),
            board.get_square(&"f8")
        );
    }

    #[test]
    fn test_8x8_board_set_square() {
        let mut board = _8x8Board::empty();
        let piece = Piece {
            side: Side::White,
            ptype: PieceType::Pawn,
        };

        assert!(!board.set_square("a1", Some(piece)).is_err());

        assert_eq!(Some(piece), board.0[0][0]);
    }

    #[test]
    fn test_board_set_bit_board() {
        let mut board = Board::new();

        board.set_bit_board(
            BitBoard(0xe),
            Piece {
                side: Side::Black,
                ptype: PieceType::Queen,
            },
        );

        assert_eq!(BitBoard(0xe), board.black.queens);
        assert_eq!(BitBoard(WHITE_QUEEN_START_POS), board.white.queens);
    }
}
