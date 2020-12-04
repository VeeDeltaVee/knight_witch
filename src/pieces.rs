enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

enum PieceSide {
    CurrentlyMoving,
    MovingNext,
}

struct Board {
    // An array of squares for the board.
    // In a typical chess game, this would be a vector with length 64.
    //
    // Indices work as follows: we start out at the bottom file, go left to
    // right, and then once we reach the end of a file we go up a file.
    squares: Vec<Option<(PieceType, PieceSide)>>;
}

impl Board {
    // Construct a default board
    fn default() -> Board {
        Board {
            squares: vec![
                Some((Rook, CurrentlyMoving)), Some((Knight, CurrentlyMoving)), Some((Bishop, CurrentlyMoving)), Some((Queen, CurrentlyMoving)), Some((King, CurrentlyMoving)), Some((Bishop, CurrentlyMoving)), Some((Knight, CurrentlyMoving)), Some((Rook, CurrentlyMoving)),
                Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)), Some((Pawn, CurrentlyMoving)),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some((Pawn, MovingNext)), Some((Pawn, MovingNext)), Some((Pawn, MovingNext)), Some((Pawn, MovingNext)), Some((Pawn, MovingNext)), Some((Pawn, MovingNext)), Some((Pawn, MovingNext)), Some((Pawn, MovingNext)),
                Some((Rook, MovingNext)), Some((Knight, MovingNext)), Some((Bishop, MovingNext)), Some((Queen, MovingNext)), Some((King, MovingNext)), Some((Bishop, MovingNext)), Some((Knight, MovingNext)), Some((Rook, MovingNext)), 
            ];
        }
    }
}
