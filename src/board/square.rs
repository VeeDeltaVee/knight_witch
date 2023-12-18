use super::{Board, Orientation};
use super::errors::InvalidSquareError;

/// Represents a square on the board
///
/// File counts from the left, starts at 0
/// Rank counts from the bottom, starts at 0
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square {
    pub file: usize,
    pub rank: usize,
}

/// Represents a square that may or may not be on the board
///
/// File counts from the left, starting at 0
/// Rank counts from the bottom, starting at 0
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UncheckedSquare {
    pub file: usize,
    pub rank: usize,
}

impl UncheckedSquare {
    /// Verify that the square is on the given board
    pub fn check_with_board(self, board: &Board) -> Result<Square, InvalidSquareError> {
        if self.file >= board.width {
            if self.rank * board.width + self.file >= board.squares.len(){
                Err(InvalidSquareError::OutOfBounds(Orientation::Both, self))
            } else {
                Err(InvalidSquareError::OutOfBounds(Orientation::File, self))
            }
        } else if self.rank * board.width + self.file >= board.squares.len() {
            Err(InvalidSquareError::OutOfBounds(Orientation::Rank, self))
        } else {
            Ok(Square { rank: self.rank, file: self.file })
        }
    }
}

impl From<Square> for UncheckedSquare {
    fn from(item: Square) -> UncheckedSquare {
        UncheckedSquare { rank: item.rank, file: item.file }
    }
}
