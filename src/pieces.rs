use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceSide {
    CurrentlyMoving,
    MovingNext,
}

pub type Piece = Option<(PieceType, PieceSide)>;

// Represents a square on the board
//
// File counts from the left, starts at 0
// Rank counts from the bottom, starts at 0
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square {
    file: usize,
    rank: usize
}

// Represents a Direction on the board
// Represents an offset from a position, used for raycasting
//
// File counts from the left
// Rank counts from the bottom
#[derive(Copy, Clone, Debug)]
pub struct Direction {
    file: isize,
    rank: isize
}

#[derive(Clone, Debug)]
pub struct Board {
    // An array of squares for the board.
    // In a typical chess game, this would be a vector with length 64.
    //
    // Indices work as follows: we start out at the bottom file, go left to
    // right, and then once we reach the end of a file we go up a file.
    squares: Vec<Piece>,
    width: usize,
    en_passant_target: Option<Square>

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Board:\n")?;
        // We want to print rank 0 at the bottom
        for rank in (0..self.squares.len() / self.width).rev() {
            write!(f, "\t")?;
            for file in 0..self.width {
                let square = Square{ rank: rank, file: file};
                if self.en_passant_target == Some(square) {
                    write!(f, "*")?;
                    continue;
                }

                let piece = self.get_piece_at_position(square).unwrap();
                let representation = match piece {
                    None                                                  => ".",
                    Some((PieceType::Pawn, PieceSide::CurrentlyMoving))   => "P",
                    Some((PieceType::Rook, PieceSide::CurrentlyMoving))   => "R",
                    Some((PieceType::Knight, PieceSide::CurrentlyMoving)) => "N",
                    Some((PieceType::Bishop, PieceSide::CurrentlyMoving)) => "B",
                    Some((PieceType::Queen, PieceSide::CurrentlyMoving))  => "Q",
                    Some((PieceType::King, PieceSide::CurrentlyMoving))   => "K",
                    Some((PieceType::Pawn, PieceSide::MovingNext))        => "p",
                    Some((PieceType::Rook, PieceSide::MovingNext))        => "r",
                    Some((PieceType::Knight, PieceSide::MovingNext))      => "n",
                    Some((PieceType::Bishop, PieceSide::MovingNext))      => "b",
                    Some((PieceType::Queen, PieceSide::MovingNext))       => "q",
                    Some((PieceType::King, PieceSide::MovingNext))        => "k"
                };
                write!(f, "{}", representation)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    // Construct a default board
    pub fn default() -> Board {
        let mut white_back_rank = vec![
                Some((PieceType::Rook, PieceSide::CurrentlyMoving)),
                Some((PieceType::Knight, PieceSide::CurrentlyMoving)),
                Some((PieceType::Bishop, PieceSide::CurrentlyMoving)),
                Some((PieceType::Queen, PieceSide::CurrentlyMoving)),
                Some((PieceType::King, PieceSide::CurrentlyMoving)),
                Some((PieceType::Bishop, PieceSide::CurrentlyMoving)),
                Some((PieceType::Knight, PieceSide::CurrentlyMoving)),
                Some((PieceType::Rook, PieceSide::CurrentlyMoving))
            ];
        let mut white_pawn_rank = vec![Some((PieceType::Pawn, PieceSide::CurrentlyMoving)); 8];
        let mut empty_ranks = vec![None; 8*4];
        let mut black_pawn_rank = vec![Some((PieceType::Pawn, PieceSide::MovingNext)); 8];
        let mut black_back_rank = vec![
                Some((PieceType::Rook, PieceSide::MovingNext)),
                Some((PieceType::Knight, PieceSide::MovingNext)),
                Some((PieceType::Bishop, PieceSide::MovingNext)),
                Some((PieceType::Queen, PieceSide::MovingNext)),
                Some((PieceType::King, PieceSide::MovingNext)),
                Some((PieceType::Bishop, PieceSide::MovingNext)),
                Some((PieceType::Knight, PieceSide::MovingNext)),
                Some((PieceType::Rook, PieceSide::MovingNext))
            ];

        let mut squares = vec![];
        squares.append(&mut white_back_rank);
        squares.append(&mut white_pawn_rank);
        squares.append(&mut empty_ranks);
        squares.append(&mut black_pawn_rank);
        squares.append(&mut black_back_rank);

        Board {
            squares: squares,
            width: 8,
            en_passant_target: None
        }
    }

    pub fn with_pieces(pieces: Vec<Piece>, width: usize) -> Self {
        Board {
            squares: pieces,
            width: width,
            en_passant_target: None
        }
    }

    // Generates a list of future board states that are possible from the
    // current board state. Does _not_ flip the piece sides or the board.
    pub fn generate_moves(&self) -> Result<Vec<Board>, &'static str> {
        let mut moves = self.generate_pawn_moves()?;
        moves.append(&mut self.generate_knight_moves()?);
        moves.append(&mut self.generate_bishop_moves());
        moves.append(&mut self.generate_rook_moves());
        moves.append(&mut self.generate_queen_moves());
        moves.append(&mut self.generate_king_moves());

        Ok(moves)
    }

    // Gets the piece that's at the given position.
    //
    // Returns error if position is out of bounds
    pub fn get_piece_at_position(&self, square: Square)
        -> Result<Option<(PieceType, PieceSide)>, &'static str>
    {
        if !self.is_valid_square(square) {
            Err("Position out of bounds")
        } else {
            Ok(self.squares[square.rank * self.width + square.file])
        }
    }

    // Sets the piece at the given position to be the given piece
    //
    // Returns error if position is out of bounds
    pub fn set_piece_at_position(&mut self, piece: Piece, square: Square)
        -> Result<(), &'static str>
    {
        if !self.is_valid_square(square) {
            Err("Position out of bounds")
        } else {
            self.squares[square.rank * self.width + square.file] = piece;
            Ok(())
        }
    }

    // Moves the piece at given old position to given new position
    // Returns a new board with the move made, if you want to make the move in
    // place use the make_move function
    //
    // Returns error if either position is out of bounds, if there's no piece at
    // old position, if there's no piece at the new position, or if the piece
    // to be moved isn't CurrentlyMoving
    pub fn new_board_with_moved_piece(&self, old_pos: Square, new_pos: Square)
        -> Result<Board, &'static str>
    {
        let mut new_board = self.clone();
        new_board.make_move(old_pos, new_pos)?;
        Ok(new_board)
    }

    // Moves the piece at given old position to given new position in place
    //
    // Returns error if either position is out of bounds, if there's no piece at
    // old position, if there's no piece at the new position, or if the piece
    // to be moved isn't CurrentlyMoving
    //
    // TODO: Needs to fail when currently-moving king is in check
    pub fn make_move(&mut self, old_pos: Square, new_pos: Square)
        -> Result<(), &'static str>
    {
        let old_piece = self.get_piece_at_position(old_pos)?;
        let new_piece = self.get_piece_at_position(new_pos)?;
        match (old_piece, new_piece) {
            (None, _) => Err("Can't make move, old_pos doesn't have piece"),
            (Some((_, PieceSide::MovingNext)), _) => Err("Can't make move, piece at old_pos isn't CurrentlyMoving"),
            (_, Some((_, PieceSide::CurrentlyMoving))) => Err("Can't make move, friendly piece exists at new_pos"),
            (Some((_, PieceSide::CurrentlyMoving)), _) => {
                self.set_piece_at_position(old_piece, new_pos)?;
                self.set_piece_at_position(None, old_pos)?;
                self.en_passant_target = None;
                Ok(())
            }
        }
    }

    // Returns the position that is related to the given index
    pub fn index_to_position(&self, index: usize) -> Result<Square, &'static str> {
        if index >= self.squares.len() {
            Err("Index out of bounds")
        } else {
            let rank = index / self.width;
            let file = index - rank * self.width;
            Ok(Square {
                rank: rank,
                file: file
            })
        }
    }

    fn generate_pawn_moves(&self) -> Result<Vec<Board>, &'static str> {
        let mut possible_moves = vec![];
        let pawn_positions = self.get_positions_of_pieces_with_given_side_and_type(
            PieceType::Pawn,
            PieceSide::CurrentlyMoving
        )?;

        // Append single square pawn moves
        let single_square_pawn_move_boards = pawn_positions.iter()
            .map(|pos| (pos, Square { file: pos.file, rank: pos.rank + 1 }))

            // The final destination should be free
            .filter(|(_, new_pos)| matches!(self.get_piece_at_position(*new_pos), Ok(None)))

            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)| self.new_board_with_moved_piece(*old_pos, new_pos).ok());
        possible_moves.extend(single_square_pawn_move_boards);

        // Append double square pawn moves
        let double_square_pawn_move_boards = pawn_positions.iter()
            .map(|pos| (pos, Square { file: pos.file, rank: pos.rank + 2 }))

            // Should start from second rank
            .filter(|(old_pos, _)| old_pos.rank == 1)

            // Should have the intervening space be free
            .filter(|(old_pos, new_pos)|
                self.check_ray_for_pieces(**old_pos, Direction {rank: 1, file: 0}, false )
                    .rank >= new_pos.rank)

            // The final destination should be free
            .filter(|(_, new_pos)| matches!(self.get_piece_at_position(*new_pos), Ok(None)))

            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)|
                self.new_board_with_moved_piece(*old_pos, new_pos)
                    .ok()

                    // Should set the en_passant_target
                    .map(|mut board| {
                        board.en_passant_target = Some(Square {
                            rank: new_pos.rank - 1,
                            file: new_pos.file
                        });
                        board
                    }));

        possible_moves.extend(double_square_pawn_move_boards);

        // Append pawn captures
        let pawn_capture_left_moves = pawn_positions.iter()
            .filter(|pos| pos.file > 0)
            .map(|pos| (pos, Square { file: pos.file - 1, rank: pos.rank + 1}));
        let pawn_capture_right_moves = pawn_positions.iter()
            .map(|pos| (pos, Square { file: pos.file + 1, rank: pos.rank + 1}));
        let pawn_capture_boards = pawn_capture_left_moves.clone().chain(pawn_capture_right_moves.clone())

            // The final destination should have an opponent's piece
            .filter(|(_, new_pos)| matches!(self.get_piece_at_position(*new_pos), Ok(Some((_, PieceSide::MovingNext)))))

            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)| self.new_board_with_moved_piece(*old_pos, new_pos).ok());
        possible_moves.extend(pawn_capture_boards);

        // Append en passant captures
        let en_passant_boards = pawn_capture_left_moves.chain(pawn_capture_right_moves)

            // The final destination should be the en passant target, set by the opponent's last move
            .filter(|(_, new_pos)| Some(*new_pos) == self.en_passant_target)

            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)|
                self.new_board_with_moved_piece(*old_pos, new_pos).ok()
                    .map(|mut board| {
                        board.set_piece_at_position(None, Square {
                            rank: new_pos.rank - 1,
                            file: new_pos.file
                        }).unwrap();

                        board
                    }));
        possible_moves.extend(en_passant_boards);

        Ok(possible_moves)
    }

    fn generate_knight_moves(&self) -> Result<Vec<Board>, &'static str> {
        let jumps = vec![(-1, 2), (1, 2),
                     (-2,  1),      (2,  1),
                     (-2, -1),      (2, -1),
                        (-1, -2), (1, -2)];

        let knight_positions = self.get_positions_of_pieces_with_given_side_and_type(
            PieceType::Knight,
            PieceSide::CurrentlyMoving
        )?;

        let mut possible_boards = vec![];
        for old_pos in knight_positions {
            let new_boards = jumps.iter()
                .map(|(file, rank)| Direction {file: *file, rank: *rank})

                // Get target square and check for out-of-bounds moves
                .filter_map(|dir| self.add_direction_to_position(old_pos, dir).ok())

                // Check target square: can't take own pieces
                .filter(|new_pos| match self.get_piece_at_position(*new_pos).unwrap() {
                        None                                  => true,
                        Some((_, PieceSide::MovingNext))      => true,
                        Some((_, PieceSide::CurrentlyMoving)) => false
                    })

                // Should be able to move there without error
                .filter_map(|new_pos| self.new_board_with_moved_piece(old_pos, new_pos).ok());

            possible_boards.extend(new_boards);
        }

        Ok(possible_boards)
    }

    // TODO: Rest of move impls
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

    fn is_valid_square(&self, square: Square) -> bool {
        square.file < self.width &&
            square.rank * self.width + square.file < self.squares.len()
    }

    fn get_positions_of_pieces_with_given_side_and_type(&self, piece_type: PieceType, piece_side: PieceSide)
        -> Result<Vec<Square>, &'static str>
    {
        self.squares.iter()
            .enumerate()
            .filter(|(_, x)| **x == Some((piece_type, piece_side)))
            .map(|(index, _)| self.index_to_position(index))
            .collect()
    }

    // Checks and returns the first piece in the given direction from given position
    //
    // If there are no pieces in the given direction, returns the last square that could be reached
    // If there is a piece in the given direction, returns position of that piece
    fn check_ray_for_pieces(&self, pos: Square, dir: Direction, can_take: bool) -> Square {
        let mut final_pos = pos;
        loop {
            match self.add_direction_to_position(final_pos, dir) {
                Err(_)      => break,
                Ok(new_pos) => {
                    match self.get_piece_at_position(new_pos).unwrap() {
                        Some((_, PieceSide::CurrentlyMoving)) => break,
                        Some((_, PieceSide::MovingNext)) => {
                            if can_take {
                                final_pos = new_pos;
                            }
                            break;
                        },
                        None => final_pos = new_pos,
                    }
                }
            }
        }

        final_pos
    }

    fn add_direction_to_position(&self, pos: Square, dir: Direction) -> Result<Square, &'static str> {
        let new_rank = pos.rank as isize + dir.rank;
        let new_file = pos.file as isize + dir.file;

        if new_rank < 0 {
            Err("Can't add direction to position, new rank is less than 0")
        } else if new_file < 0 {
            Err("Can't add direction to position, new file is less than 0")
        } else if !self.is_valid_square(Square { rank: new_rank as usize, file: new_file as usize }) {
            Err("Can't add direction to position, position is out of bounds")
        } else {
            Ok(Square { rank: new_rank as usize, file: new_file as usize })
        }
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

    fn check_for_moves(boards: Vec<Board>, expected_moves: Vec<Square>, unexpected_moves: Vec<Square>, piece: Piece) {
        for square in expected_moves {
            assert!(
                boards.clone().into_iter()
                    .any(|x| x.get_piece_at_position(square).unwrap() == piece),
                "Didn't find {:?} move at rank {}, file {}", piece.unwrap().0, square.rank, square.file
                );
        }

        for square in unexpected_moves {
            assert!(
                !boards.clone().into_iter()
                    .any(|x| x.get_piece_at_position(square).unwrap() == piece),
                "Found unexpected {:?} move at rank {}, file {}", piece.unwrap().0, square.rank, square.file
                );
        }
    }

    mod pawn_moves {
        use super::*;

        // Returns a board with the setup (FEN piece notation)
        // .......
        // .....p.
        // ....p.p
        // .Pp...P
        // P.P.PP.
        // .......
        fn get_test_board_for_simple_pawn_moves() -> Board {
            let mut squares = vec![None; 7 * 6];
            squares[7] = Some((PieceType::Pawn, PieceSide::CurrentlyMoving));
            squares[9] = Some((PieceType::Pawn, PieceSide::CurrentlyMoving));
            squares[11] = Some((PieceType::Pawn, PieceSide::CurrentlyMoving));
            squares[12] = Some((PieceType::Pawn, PieceSide::CurrentlyMoving));

            squares[15] = Some((PieceType::Pawn, PieceSide::CurrentlyMoving));
            squares[16] = Some((PieceType::Pawn, PieceSide::MovingNext));
            squares[20] = Some((PieceType::Pawn, PieceSide::CurrentlyMoving));

            squares[25] = Some((PieceType::Pawn, PieceSide::MovingNext));
            squares[27] = Some((PieceType::Pawn, PieceSide::MovingNext));

            squares[33] = Some((PieceType::Pawn, PieceSide::MovingNext));

            Board {
                squares: squares,
                width: 7,
                en_passant_target: None
            }
        }

        #[test]
        fn one_square_forward() {
            let board = get_test_board_for_simple_pawn_moves();
            let moved_boards = board.generate_moves().unwrap();

            let expected_single_square_pushes = vec![Square { rank: 2, file: 0 },
                                                 Square { rank: 2, file: 4 },
                                                 Square { rank: 2, file: 5 },
                                                 Square { rank: 3, file: 1 },
                ];
            let unexpected_single_square_pushes = vec![Square { rank: 2, file: 2 },
                                                       Square { rank: 3, file: 6 },
                ];

            check_for_moves(moved_boards,
                            expected_single_square_pushes,
                            unexpected_single_square_pushes,
                            Some((PieceType::Pawn, PieceSide::CurrentlyMoving)));
        }

        #[test]
        fn two_squares_forward() {
            let board = get_test_board_for_simple_pawn_moves();
            let moved_boards = board.generate_moves().unwrap();

            let expected_double_square_pushes = vec![Square { rank: 3, file: 0 },
                                                     Square { rank: 3, file: 5 },
                ];
            let unexpected_double_square_pushes = vec![Square { rank: 4, file: 1 },
                                                       Square { rank: 3, file: 2 },
                                                       Square { rank: 3, file: 4 },
                                                       Square { rank: 4, file: 6 },
                ];

            check_for_moves(moved_boards,
                            expected_double_square_pushes,
                            unexpected_double_square_pushes,
                            Some((PieceType::Pawn, PieceSide::CurrentlyMoving)));
        }

        // Returns a board with the setup
        // .....
        // .....
        // B.ko.
        // .P.pP
        // .....
        // Where o is the en passant target
        fn get_test_board_for_pawn_captures() -> Board {
            let mut board = Board {
                squares: vec![None; 5*5],
                width: 5,
                en_passant_target: Some( Square{rank: 2, file: 3})
            };

            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Square { rank: 1, file: 1 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Bishop, PieceSide::CurrentlyMoving)), Square { rank: 2, file: 0 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Knight, PieceSide::MovingNext)), Square { rank: 2, file: 2 }).unwrap();

            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Square { rank: 1, file: 4 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::MovingNext)), Square { rank: 1, file: 3 }).unwrap();

            board
        }

        #[test]
        fn captures_opponents_pieces() {
            let board = get_test_board_for_pawn_captures();

            let moved_boards = board.generate_moves().unwrap();

            // At least one of the moves suggested should have the pawn
            // take a piece
            assert!(
                moved_boards.into_iter()
                    .any(|x| matches!(x.get_piece_at_position(Square { rank: 2, file: 2 }).unwrap(), Some((PieceType::Pawn, _))))
            );
        }

        #[test]
        fn doesnt_capture_friendly_pieces() {
            let board = get_test_board_for_pawn_captures();

            let moved_boards = board.generate_moves().unwrap();

            // None of the moves should have a pawn taking the friendly piece
            assert!(
                moved_boards.into_iter()
                    .all(|x| !matches!(x.get_piece_at_position(Square { rank: 2, file: 0 }).unwrap(), Some((PieceType::Pawn, _))))
            );
        }

        #[test]
        fn captures_en_passant() {
            let board = get_test_board_for_pawn_captures();

            let moved_boards = board.generate_moves().unwrap();

            // At least one of the moves suggested should have the pawn
            // take the pawn en passant
            assert!(
                moved_boards.iter()
                    .any(|x| matches!(x.get_piece_at_position(Square { rank: 2, file: 3 }).unwrap(), Some((PieceType::Pawn, _)))
                             && matches!(x.get_piece_at_position(Square { rank: 1, file: 3 }).unwrap(), None))
            );
        }
    }

    mod knight_moves {
        use super::*;

        // Returns a board with the setup
        // P.....
        // ...p..
        // .K..p.
        // ...P.p
        // ...K..
        // .....P
        fn get_board_for_simple_knight_moves() -> Board {
            let mut board = Board::with_pieces(vec![None; 6*6], 6);

            board.set_piece_at_position(Some((PieceType::Knight, PieceSide::CurrentlyMoving)), Square { rank: 1, file: 3 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Knight, PieceSide::CurrentlyMoving)), Square { rank: 3, file: 1 }).unwrap();

            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Square { rank: 2, file: 3 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Square { rank: 0, file: 5 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::CurrentlyMoving)), Square { rank: 5, file: 0 }).unwrap();

            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::MovingNext)), Square { rank: 2, file: 5 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::MovingNext)), Square { rank: 3, file: 4 }).unwrap();
            board.set_piece_at_position(Some((PieceType::Pawn, PieceSide::MovingNext)), Square { rank: 4, file: 3 }).unwrap();

            board
        }

        #[test]
        fn moves_like_a_knight() {
            let board = get_board_for_simple_knight_moves();

            let moved_boards = board.generate_moves().unwrap();


            let expected_moves = vec![Square { rank: 5, file: 2 },
                                      Square { rank: 4, file: 3 },
                                      Square { rank: 1, file: 2 },
                                      Square { rank: 1, file: 0 },


                                      Square { rank: 3, file: 2 },
                                      Square { rank: 3, file: 4 },
                                      Square { rank: 2, file: 1 },
                                      Square { rank: 2, file: 5 },
                                      Square { rank: 0, file: 1 },
                ];
            let unexpected_moves = vec![Square { rank: 5, file: 0 },

                                        Square { rank: 0, file: 5 },
                                        Square { rank: 2, file: 3 },
                ];

            check_for_moves(moved_boards,
                            expected_moves,
                            unexpected_moves,
                            Some((PieceType::Knight, PieceSide::CurrentlyMoving)));
        }
    }

    mod king_moves {
        use super::*;

        fn get_board_for_simple_king_moves() -> Board {
            let mut pieces = vec![None; 9];
            pieces[4] = Some((PieceType::King, PieceSide::CurrentlyMoving));

            Board::with_pieces(pieces, 3)
        }

        #[test]
        fn moves_one_step_nearby() {
            let board = get_board_for_simple_king_moves();

            let moved_boards = board.generate_moves().unwrap();

            // every place other than the centre should have a king move
            for rank in 0..2 {
                for file in 0..2 {
                    if (rank, file) != (1, 1) {
                        assert!(
                            moved_boards.iter()
                                .any(|x|
                                    matches!(x.get_piece_at_position( Square {rank: rank, file: file}).unwrap(),
                                        Some((PieceType::King, _))) &&
                                    matches!(x.get_piece_at_position( Square {rank: 1, file: 1}).unwrap(),
                                        None))
                        );
                    }
                }
            }
        }
    }
}
