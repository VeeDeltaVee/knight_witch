#[derive(Debug)]
pub enum Orientation {
    Rank,
    File,
    Both,
}

#[derive(Debug)]
pub enum InvalidOffsetError {
    LessThanZero(Orientation, Offset),
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
