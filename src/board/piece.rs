use std::{fmt, convert::TryFrom};

/// Represents a type of chess piece.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

/// Represents the color of a chess piece.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    pub fn flip(self: Self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

/// Represents a tile on the chessboard and the piece optionally on it.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Piece {
    pub side: Side,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(side: Side, piece_type: PieceType) -> Self {
        Self { side, piece_type }
    }
}

impl From<(PieceType, Side)> for Piece {
    fn from((piece_type, side): (PieceType, Side)) -> Self {
        Self { side, piece_type }
    }
}

impl TryFrom<char> for Piece {
    type Error = String;
    fn try_from(value: char) -> Result<Piece, Self::Error> {
        match value {
            'K' => Ok(Self{ piece_type: PieceType::King, side: Side::White}),
            'Q' => Ok(Self{ piece_type: PieceType::Queen, side: Side::White}),
            'R' => Ok(Self{ piece_type: PieceType::Rook, side: Side::White}),
            'B' => Ok(Self{ piece_type: PieceType::Bishop, side: Side::White}),
            'N' => Ok(Self{ piece_type: PieceType::Knight, side: Side::White}),
            'P' => Ok(Self{ piece_type: PieceType::Pawn, side: Side::White}),
            'k' => Ok(Self{ piece_type: PieceType::King, side: Side::Black}),
            'q' => Ok(Self{ piece_type: PieceType::Queen, side: Side::Black}),
            'r' => Ok(Self{ piece_type: PieceType::Rook, side: Side::Black}),
            'b' => Ok(Self{ piece_type: PieceType::Bishop, side: Side::Black}),
            'n' => Ok(Self{ piece_type: PieceType::Knight, side: Side::Black}),
            'p' => Ok(Self{ piece_type: PieceType::Pawn, side: Side::Black}),
            _ => Err(format!("Invalid piece '{value}'.")),
        }
    }
}

impl From<&Piece> for char {
    fn from(value: &Piece) -> Self {
        match value {
            Piece { piece_type: PieceType::Pawn, side: Side::White } => 'P',
            Piece { piece_type: PieceType::Rook, side: Side::White } => 'R',
            Piece { piece_type: PieceType::Knight, side: Side::White } => 'N',
            Piece { piece_type: PieceType::Bishop, side: Side::White } => 'B',
            Piece { piece_type: PieceType::Queen, side: Side::White } => 'Q',
            Piece { piece_type: PieceType::King, side: Side::White } => 'K',
            Piece { piece_type: PieceType::Pawn, side: Side::Black } => 'p',
            Piece { piece_type: PieceType::Rook, side: Side::Black } => 'r',
            Piece { piece_type: PieceType::Knight, side: Side::Black } => 'n',
            Piece { piece_type: PieceType::Bishop, side: Side::Black } => 'b',
            Piece { piece_type: PieceType::Queen, side: Side::Black } => 'q',
            Piece { piece_type: PieceType::King, side: Side::Black } => 'k',
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<char>::into(self))
    }
}

