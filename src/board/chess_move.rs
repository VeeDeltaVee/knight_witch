use std::{convert::TryFrom};

use super::{Square, castling::CastlingDirection};

#[derive(Debug, Clone)]
pub enum ChessMove {
    SimpleMove(Square, Square),
    Castling(CastlingDirection),
}

impl TryFrom<&str> for ChessMove {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "O-O" {
            return Ok(ChessMove::Castling(CastlingDirection::Kingside));
        } else if value == "O-O-O" {
            return Ok(ChessMove::Castling(CastlingDirection::Queenside));
        }

        let mut chars = value.chars();
        let from = Square {
            file: file_from_char(chars.next().ok_or("Invalid value, too short")?)?,
            rank: rank_from_char(chars.next().ok_or("Invalid value, too short")?)?
        };

        let to = Square {
            file: file_from_char(chars.next().ok_or("Invalid value, too short")?)?,
            rank: rank_from_char(chars.next().ok_or("Invalid value, too short")?)?
        };

        Ok(ChessMove::SimpleMove(from, to))
    }
}

fn file_from_char(ch: char) -> Result<usize, &'static str> {
    if !ch.is_ascii_lowercase() {
        Err("Invalid file character")
    } else {
        Ok(ch as usize - 'a' as usize)
    }
}

fn rank_from_char(ch: char) -> Result<usize, &'static str> {
    if !ch.is_ascii_digit() || ch == '0' {
        Err("Invalid rank character")
    } else {
        Ok(ch as usize - '1' as usize)
    }
}
