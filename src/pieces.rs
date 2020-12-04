pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub enum PieceSide {
    CurrentlyMoving,
    MovingNext,
}

pub struct Board {
    // An array of squares for the board.
    // In a typical chess game, this would be a vector with length 64.
    //
    // Indices work as follows: we start out at the bottom file, go left to
    // right, and then once we reach the end of a file we go up a file.
    squares: Vec<Option<(PieceType, PieceSide)>>
}

impl Board {
    // Construct a default board
    pub fn default() -> Board {
        Board {
            squares: vec![
                Some((PieceType::Rook, PieceSide::CurrentlyMoving)), Some((PieceType::Knight, PieceSide::CurrentlyMoving)), Some((PieceType::Bishop, PieceSide::CurrentlyMoving)), Some((PieceType::Queen, PieceSide::CurrentlyMoving)), Some((PieceType::King, PieceSide::CurrentlyMoving)), Some((PieceType::Bishop, PieceSide::CurrentlyMoving)), Some((PieceType::Knight, PieceSide::CurrentlyMoving)), Some((PieceType::Rook, PieceSide::CurrentlyMoving)),
                Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Some((PieceType::Pawn, PieceSide::CurrentlyMoving)),
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)), Some((PieceType::Pawn, PieceSide::MovingNext)),
                Some((PieceType::Rook, PieceSide::MovingNext)), Some((PieceType::Knight, PieceSide::MovingNext)), Some((PieceType::Bishop, PieceSide::MovingNext)), Some((PieceType::Queen, PieceSide::MovingNext)), Some((PieceType::King, PieceSide::MovingNext)), Some((PieceType::Bishop, PieceSide::MovingNext)), Some((PieceType::Knight, PieceSide::MovingNext)), Some((PieceType::Rook, PieceSide::MovingNext)),
            ]
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_has_pieces_where_it_should() {
        let board = Board::default();

        for i in (0..16).chain(48..64) {
            assert!(board.squares[i].is_some());
        }

    }

    #[test]
    fn default_has_pawns_where_it_should() {
        let board = Board::default();

        for i in 8..16 {
            assert!(matches!(board.squares[i], Some((PieceType::Pawn, _))));
        }
        for i in 48..56 {
            assert!(matches!(board.squares[i], Some((PieceType::Pawn, _))));
        }
    }

    #[test]
    fn default_has_no_pieces_where_it_should() {
        let board = Board::default();

        for i in 16..48 {
            assert!(board.squares[i].is_none());
        }
    }

    #[test]
    fn default_has_rooks_where_it_should() {
        let board = Board::default();
        for i in vec![0, 7, 56, 63] {
            assert!(matches!(board.squares[i], Some((PieceType::Rook, _))));
        }
    }

    #[test]
    fn default_has_knights_where_it_should() {
        let board = Board::default();
        for i in vec![1, 6, 57, 62] {
            assert!(matches!(board.squares[i], Some((PieceType::Knight, _))));
        }
    }

    #[test]
    fn default_has_bishop_where_it_should() {
        let board = Board::default();
        for i in vec![2, 5, 58, 61] {
            assert!(matches!(board.squares[i], Some((PieceType::Bishop, _))));
        }
    }

    #[test]
    fn default_has_queens_where_it_should() {
        let board = Board::default();
        for i in vec![3, 59] {
            assert!(matches!(board.squares[i], Some((PieceType::Queen, _))));
        }
    }

    #[test]
    fn default_has_kings_where_it_should() {
        let board = Board::default();
        for i in vec![4, 60] {
            assert!(matches!(board.squares[i], Some((PieceType::King, _))));
        }
    }

    #[test]
    fn default_has_piece_sides_correct() {
        let board = Board::default();

        for i in 0..16 {
            assert!(matches!(board.squares[i], Some((_, PieceSide::CurrentlyMoving))));
        }

        for i in 48..64 {
            assert!(matches!(board.squares[i], Some((_, PieceSide::MovingNext))));
        }
    }
}
