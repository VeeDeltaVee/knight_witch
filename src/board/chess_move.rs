use std::convert::TryFrom;

use super::{castling::CastlingDirection, Square};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChessMove {
    SimpleMove(Square, Square),

    /// A move from a square, to different square, capturing a third piece
    /// that's the target for en passant capture
    EnPassant(Square, Square, Square),

    Castling(CastlingDirection),

    /// A move that does nothing, but flips the side.
    NullMove,
}

impl TryFrom<&str> for ChessMove {
    type Error = &'static str;

    /// Note that this doesn't work with en_passent moves at the moment. That
    /// conversion would require knowing the context of the board, which this
    /// function can't know.
    ///
    /// For en passant moves, this function just returns as if it were a simple
    /// capture move
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "O-O" {
            return Ok(ChessMove::Castling(CastlingDirection::Kingside));
        } else if value == "O-O-O" {
            return Ok(ChessMove::Castling(CastlingDirection::Queenside));
        }

        let mut chars = value.chars();
        let from = Square {
            file: file_from_char(
                chars.next().ok_or("Invalid value, too short")?,
            )?,
            rank: rank_from_char(
                chars.next().ok_or("Invalid value, too short")?,
            )?,
        };

        let to = Square {
            file: file_from_char(
                chars.next().ok_or("Invalid value, too short")?,
            )?,
            rank: rank_from_char(
                chars.next().ok_or("Invalid value, too short")?,
            )?,
        };

        Ok(ChessMove::SimpleMove(from, to))
    }
}

fn file_from_char(ch: char) -> Result<u8, &'static str> {
    if !ch.is_ascii_lowercase() {
        Err("Invalid file character")
    } else {
        Ok(ch as u8 - b'a')
    }
}

fn rank_from_char(ch: char) -> Result<u8, &'static str> {
    if !ch.is_ascii_digit() || ch == '0' {
        Err("Invalid rank character")
    } else {
        Ok(ch as u8 - b'1')
    }
}
