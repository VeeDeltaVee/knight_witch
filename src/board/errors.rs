use super::{Square, Offset};

/// Helper enum to indicate whether a given error is referencing a row or column on the board
#[derive(Debug)]
pub enum Orientation {
    Rank,
    File,
    Both,
}

/// Gives an error when a offset calculation gives an unrepresentable result
#[derive(Debug)]
pub enum InvalidOffsetError {
    /// Indicates that the resulting offset would be off the left or bottom side of the board
    LessThanZero(Orientation, Offset),
    /// Indicates that the given square would be off the top or right side of the board
    InvalidSquare(InvalidSquareError)
}

impl Into<&'static str> for InvalidOffsetError  {
    fn into(self) -> &'static str {
        match self {
            InvalidOffsetError::LessThanZero(Orientation::File, _) => "Invalid offset, resulting file is less than zero",
            InvalidOffsetError::LessThanZero(_, _) => "Invalid offset, resulting rank is less than zero",
            InvalidOffsetError::InvalidSquare(error) => error.into(),
        }
    }
}

#[derive(Debug)]
pub enum InvalidSquareError {
    /// Indicates that the given square would be off the top or right side of the board
    OutOfBounds(Orientation, Square),
}

impl Into<&'static str> for InvalidSquareError {
    fn into(self) -> &'static str {
        "Square is out of bounds"
    }
}

impl Into<InvalidOffsetError> for InvalidSquareError {
    fn into(self) -> InvalidOffsetError {
        InvalidOffsetError::InvalidSquare(self)
    }
}
