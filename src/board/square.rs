use super::{Board, errors::InvalidSquareError, Orientation};

/// Represents a square on the board
///
/// File counts from the left, starts at 0
/// Rank counts from the bottom, starts at 0
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square {
    file: usize,
    rank: usize,
}

impl Square {
    pub fn get_file(&self) -> usize {
        return self.file;
    }

    pub fn get_rank(&self) -> usize {
        return self.rank;
    }
}

#[cfg(test)]
impl Square {
    /// Create a square that is assumed to be valid
    ///
    /// This is usefull for tests when we know we have
    /// a valid square and saves some boilerplate
    pub fn new(rank: usize, file: usize) -> Square {
        Square {
            rank,
            file,
        }
    }
}

/// Represents a possibly invalid location of a square
/// on the board.
///
/// File counts from the left, starts at 0
/// Rank counts from the bottom, starts at 0
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UncheckedSquare {
    pub file: usize,
    pub rank: usize,
}

impl UncheckedSquare {
    /// Checks that the square is valid on the given board
    ///
    // TODO: validate through the type system that it's valid for only the board
    // that it's been tested on. This may require some lifetime shenanigans
    pub fn validate(self, board: &Board) -> Result<Square, InvalidSquareError> {
        if self.file >= board.width {
            if self.rank * board.width + self.file >= board.squares.len(){
                Err(InvalidSquareError::OutOfBounds(Orientation::Both, self))
            } else {
                Err(InvalidSquareError::OutOfBounds(Orientation::File, self))
            }
        } else if self.rank * board.width + self.file >= board.squares.len() {
            Err(InvalidSquareError::OutOfBounds(Orientation::Rank, self))
        } else {
            Ok(Square {rank: self.rank, file: self.file})
        }
    }
}

impl From<Square> for UncheckedSquare {
    fn from(item: Square) -> UncheckedSquare {
        UncheckedSquare {
            file: item.file,
            rank: item.rank,
        }
    }
}
