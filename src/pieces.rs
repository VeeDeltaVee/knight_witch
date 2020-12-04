#[derive(Clone, Debug)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Debug)]
pub enum PieceSide {
    CurrentlyMoving,
    MovingNext,
}

#[derive(Clone, Debug)]
pub struct Board {
    // An array of squares for the board.
    // In a typical chess game, this would be a vector with length 64.
    //
    // Indices work as follows: we start out at the bottom file, go left to
    // right, and then once we reach the end of a file we go up a file.
    squares: Vec<Option<(PieceType, PieceSide)>>,
    width: usize

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
            ],
            width: 8
        }
    }

    pub fn generate_moves(&self) -> Vec<Board> {
        let mut moves = self.generate_pawn_moves();
        moves.append(&mut self.generate_knight_moves());
        moves.append(&mut self.generate_bishop_moves());
        moves.append(&mut self.generate_rook_moves());
        moves.append(&mut self.generate_queen_moves());
        moves.append(&mut self.generate_king_moves());

        moves
    }

    fn generate_pawn_moves(&self) -> Vec<Board> {
        vec![]
    }

    fn generate_knight_moves(&self) -> Vec<Board> {
        vec![]
    }

    fn generate_bishop_moves(&self) -> Vec<Board> {
        vec![]
    }

    fn generate_rook_moves(&self) -> Vec<Board> {
        vec![]
    }

    fn generate_queen_moves(&self) -> Vec<Board> {
        vec![]
    }

    fn generate_king_moves(&self) -> Vec<Board> {
        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod default {
        use super::*;

        #[test]
        fn has_correct_size() {
            let board = Board::default();

            assert_eq!(board.squares.len(), 64);
            assert_eq!(board.width, 8);
        }

        #[test]
        fn has_pieces_where_it_should() {
            let board = Board::default();

            for i in (0..16).chain(48..64) {
                assert!(board.squares[i].is_some());
            }

        }

        #[test]
        fn has_pawns_where_it_should() {
            let board = Board::default();

            for i in 8..16 {
                assert!(matches!(board.squares[i], Some((PieceType::Pawn, _))));
            }
            for i in 48..56 {
                assert!(matches!(board.squares[i], Some((PieceType::Pawn, _))));
            }
        }

        #[test]
        fn has_no_pieces_where_it_should() {
            let board = Board::default();

            for i in 16..48 {
                assert!(board.squares[i].is_none());
            }
        }

        #[test]
        fn has_rooks_where_it_should() {
            let board = Board::default();
            for i in vec![0, 7, 56, 63] {
                assert!(matches!(board.squares[i], Some((PieceType::Rook, _))));
            }
        }

        #[test]
        fn has_knights_where_it_should() {
            let board = Board::default();
            for i in vec![1, 6, 57, 62] {
                assert!(matches!(board.squares[i], Some((PieceType::Knight, _))));
            }
        }

        #[test]
        fn has_bishop_where_it_should() {
            let board = Board::default();
            for i in vec![2, 5, 58, 61] {
                assert!(matches!(board.squares[i], Some((PieceType::Bishop, _))));
            }
        }

        #[test]
        fn has_queens_where_it_should() {
            let board = Board::default();
            for i in vec![3, 59] {
                assert!(matches!(board.squares[i], Some((PieceType::Queen, _))));
            }
        }

        #[test]
        fn has_kings_where_it_should() {
            let board = Board::default();
            for i in vec![4, 60] {
                assert!(matches!(board.squares[i], Some((PieceType::King, _))));
            }
        }

        #[test]
        fn has_piece_sides_correct() {
            let board = Board::default();

            for i in 0..16 {
                assert!(matches!(board.squares[i], Some((_, PieceSide::CurrentlyMoving))));
            }

            for i in 48..64 {
                assert!(matches!(board.squares[i], Some((_, PieceSide::MovingNext))));
            }
        }
    }

    mod pawn_moves {
        use super::*;

        fn get_test_board_for_simple_pawn_moves() -> Board {
            Board {
                squares: vec![
                    None, None, None,
                    None, None, None,
                    None, None, None,
                    None, Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), None,
                    None, None, None,
                ],
                width: 3
            }
        }

        #[test]
        fn one_square_forward() {
            let board = get_test_board_for_simple_moves();

            let moved_boards = board.generate_moves();

            // At least one of the moves suggested should have the pawn moving
            // up on square
            assert!(
                moved_boards.into_iter()
                .any(|x| matches!(x.squares[7], Some((PieceType::Pawn, _))))
            );
        }

        #[test]
        fn two_squares_forward() {
            let board = get_test_board_for_simple_moves();

            let moved_boards = board.generate_moves();

            // At least one of the moves suggested should have the pawn moving
            // up two squares
            assert!(
                moved_boards.into_iter()
                .any(|x| matches!(x.squares[4], Some((PieceType::Pawn, _))))
            );
        }

        fn get_test_board_for_pawn_captures() -> Board {
            Board {
                squares: vec![
                    None, None, None,
                    Some((PieceType::Bishop, PieceSide::CurrentlyMoving)), None, Some((PieceType::Bishop, PieceSide::MovingNext)),
                    None, Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), None,
                    None, None, None,
                ],
                width: 3
            }
        }

        #[test]
        fn captures_opponents_pieces() {
            let board = get_test_board_for_pawn_captures();

            let moved_boards = board.generate_moves();

            // At least one of the moves suggested should have the pawn
            // take a piece
            assert!(
                moved_boards.into_iter()
                .any(|x| matches!(x.squares[5], Some((PieceType::Pawn, _))))
            );
        }

        #[test]
        fn doesnt_capture_friendly_pieces() {
            let board = get_test_board_for_pawn_captures();

            let moved_boards = board.generate_moves();

            // None of the moves should have a pawn taking the friendly piece
            assert!(
                moved_boards.into_iter()
                .all(|x| !matches!(x.squares[3], Some((PieceType::Pawn, _))))
            );
        }
    }
}
