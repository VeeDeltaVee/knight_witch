use super::{Offset, UncheckedSquare};

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
    InvalidSquare(InvalidSquareError),
}

impl From<InvalidOffsetError> for &'static str {
    fn from(item: InvalidOffsetError) -> &'static str {
        match item {
            InvalidOffsetError::LessThanZero(Orientation::File, _) => {
                "Invalid offset, resulting file is less than zero"
            }
            InvalidOffsetError::LessThanZero(_, _) => {
                "Invalid offset, resulting rank is less than zero"
            }
            InvalidOffsetError::InvalidSquare(error) => error.into(),
        }
    }
}

#[derive(Debug)]
pub enum InvalidSquareError {
    /// Indicates that the given square would be off the top or right side of the board
    OutOfBounds(Orientation, UncheckedSquare),
}

impl From<InvalidSquareError> for &'static str {
    fn from(_: InvalidSquareError) -> &'static str {
        "Square is out of bounds"
    }
}

impl From<InvalidSquareError> for InvalidOffsetError {
    fn from(item: InvalidSquareError) -> InvalidOffsetError {
        InvalidOffsetError::InvalidSquare(item)
    }
}
