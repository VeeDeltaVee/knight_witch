mod bishop;
mod castling;
pub mod chess_move;
pub mod game;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
pub mod square;

mod errors;
pub mod piece;
mod straight_moving_piece;
mod test_utils;

use crate::board::pawn::PawnMovement;
use std::convert::TryFrom;
use std::fmt;

use piece::*;

use self::bishop::BishopMovement;
use self::castling::{CastlingMovement, CastlingState};
use self::chess_move::ChessMove;
use self::errors::*;
use self::game::ChessResult;
use self::king::KingMovement;
use self::knight::KnightMovement;
use self::pawn::PawnState;
use self::queen::QueenMovement;
use self::rook::RookMovement;
use self::square::{Square, UncheckedSquare};

// Represents a Offset on the board
// Represents an offset from a position, used for raycasting
//
// File counts from the left
// Rank counts from the bottom
#[derive(Copy, Clone, Debug)]
pub struct Offset {
    file: isize,
    rank: isize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Board {
    // An array of squares for the board.
    // In a typical chess game, this would be a vector with length 64.
    //
    // Indices work as follows: we start out at the bottom file, go left to
    // right, and then once we reach the end of a file we go up a file.
    squares: Vec<Option<Piece>>,
    width: usize,
    en_passant_target: Option<Square>,

    // Which side has to make a move next
    current_move: Side,

    /// Store 4 booleans, representing which side can castle where,
    /// if at all. Shouldn't be accessed directly
    /// The state is stored in top to bottom, left
    /// to right:
    /// black queenside, black kingside, white queenside,
    /// white kingside
    castling_availability: [bool; 4],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Board:")?;
        // We want to print rank 0 at the bottom
        for rank in (0..self.squares.len() / self.width).rev() {
            write!(f, "\t")?;
            for file in 0..self.width {
                let square = Square { rank, file };
                if self.en_passant_target == Some(square) {
                    write!(f, "*")?;
                    continue;
                }

                let piece = self.get_piece_at_position(square).unwrap();
                let character = piece.as_ref().map_or('.', char::from);
                write!(f, "{}", character)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Board {
    // Construct a default board
    pub fn default() -> Board {
        use {PieceType::*, Side::*};
        let mut white_back_rank = vec![
            Some(Piece::new(White, Rook)),
            Some(Piece::new(White, Knight)),
            Some(Piece::new(White, Bishop)),
            Some(Piece::new(White, Queen)),
            Some(Piece::new(White, King)),
            Some(Piece::new(White, Bishop)),
            Some(Piece::new(White, Knight)),
            Some(Piece::new(White, Rook)),
        ];
        let mut white_pawn_rank = vec![Some(Piece::new(White, Pawn)); 8];
        let mut empty_ranks = vec![None; 8 * 4];
        let mut black_pawn_rank = vec![Some(Piece::new(Black, Pawn)); 8];
        let mut black_back_rank = vec![
            Some(Piece::new(Black, Rook)),
            Some(Piece::new(Black, Knight)),
            Some(Piece::new(Black, Bishop)),
            Some(Piece::new(Black, Queen)),
            Some(Piece::new(Black, King)),
            Some(Piece::new(Black, Bishop)),
            Some(Piece::new(Black, Knight)),
            Some(Piece::new(Black, Rook)),
        ];

        let mut squares = vec![];
        squares.append(&mut white_back_rank);
        squares.append(&mut white_pawn_rank);
        squares.append(&mut empty_ranks);
        squares.append(&mut black_pawn_rank);
        squares.append(&mut black_back_rank);

        Board {
            squares,
            width: 8,
            en_passant_target: None,
            current_move: Side::White,
            castling_availability: [true, true, true, true],
        }
    }

    /// Generates a board state from ascii art of the board
    /// Pieces are denoted like FEN notation, just in a grid
    /// instead of compressed. Makes for easier reading of tests etc.
    ///
    /// Note: This doesn't add any castling availability
    /// Since in the arbitrary case there's no way to know
    /// if castling is available, we don't try, and say it's not
    /// available at all
    pub fn from_art(art: &str) -> Result<Self, &'static str> {
        let pieces = art
            .lines()
            .map(|line| line.chars().map(Piece::try_from).map(Result::ok))
            .rev();

        let mut widths = pieces.clone().map(|rank| rank.count());
        let first_width =
            widths.next().ok_or("Can't create board with no height")?;
        widths.all(|w| w == first_width);

        Ok(Board::with_pieces(pieces.flatten().collect(), first_width))
    }

    pub fn with_pieces(pieces: Vec<Option<Piece>>, width: usize) -> Self {
        Board {
            squares: pieces,
            width,
            en_passant_target: None,
            current_move: Side::White,

            // There's no real way to get the castling availability
            // while constructing a board from pieces, so we set all to false
            castling_availability: [false, false, false, false],
        }
    }

    /// A read only view into the squares of board.
    pub fn get_squares(&self) -> &[Option<Piece>] {
        &self.squares
    }

    /// A view into the currently moving side
    ///
    /// This doesn't need to return a reference, because a Side is just a binary
    /// enum and will clone cheaply. It implements Copy.
    pub fn get_current_side(&self) -> Side {
        self.current_move
    }

    pub fn flip_current_side(&mut self) -> &mut Self {
        self.current_move = self.current_move.flip();
        self
    }

    // Generates a list of moves that are possible from the
    // current board state.
    pub fn generate_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str> {
        let mut moves = self.generate_pawn_moves(checked)?;
        moves.append(&mut self.generate_knight_moves(checked)?);
        moves.append(&mut self.generate_bishop_moves(checked)?);
        moves.append(&mut self.generate_rook_moves(checked)?);
        moves.append(&mut self.generate_queen_moves(checked)?);
        moves.append(&mut self.generate_king_moves(checked)?);
        moves.append(&mut self.generate_castling_moves(checked)?);

        Ok(moves)
    }

    // Generates a list of future board states that are possible from the
    // current board state.
    pub fn generate_moved_boards(
        &self,
        checked: bool,
    ) -> Result<Vec<Board>, &'static str> {
        let moves = self.generate_moves(checked)?;

        moves
            .into_iter()
            .map(|chess_move| {
                let mut clone = self.clone();
                clone.make_move(chess_move, checked)?;

                Ok(clone)
            })
            .collect()
    }

    // Gets the piece that's at the given position.
    //
    // Returns error if position is out of bounds
    pub fn get_piece_at_position(
        &self,
        square: Square,
    ) -> Result<Option<Piece>, InvalidSquareError> {
        Ok(self.squares[square.rank * self.width + square.file])
    }

    // Sets the piece at the given position to be the given piece
    //
    // Returns error if position is out of bounds
    pub fn set_piece_at_position(
        &mut self,
        piece: Option<Piece>,
        square: Square,
    ) -> Result<(), &'static str> {
        self.squares[square.rank * self.width + square.file] = piece;
        Ok(())
    }

    // Moves the piece at given old position to given new position
    // Returns a new board with the move made, if you want to make the move in
    // place use the make_move function
    //
    // Returns error if either position is out of bounds, if there's no piece at
    // old position, if there's no piece at the new position, or if the piece
    // to be moved isn't CurrentlyMoving
    pub fn new_board_with_moved_piece(
        &self,
        old_pos: Square,
        new_pos: Square,
        checked: bool,
    ) -> Result<Board, &'static str> {
        let mut new_board = self.clone();
        new_board
            .make_move(ChessMove::SimpleMove(old_pos, new_pos), checked)?;
        Ok(new_board)
    }

    pub fn check_king_threat(&self) -> Result<bool, &'static str> {
        use PieceType::King;
        let kings = self.get_positions_of_matching_pieces(Piece::new(
            self.current_move,
            King,
        ))?;
        let num_kings = kings.len();

        let mut skipped_move_board = self.clone();
        skipped_move_board.current_move =
            skipped_move_board.current_move.flip();

        let other_sides_potential_moves =
            skipped_move_board.generate_moved_boards(false)?;

        let is_king_in_threat = other_sides_potential_moves
            .iter()
            .map(|b| {
                b.get_positions_of_matching_pieces(Piece::new(
                    self.current_move,
                    King,
                ))
                .map(|ks| ks.len())
                .unwrap_or(0)
            })
            .any(|n| num_kings != n);

        Ok(is_king_in_threat)
    }

    /// Returns whether the game is in progress or has ended, along with the
    /// result.
    ///
    /// Don't call in really performance intensive situations, because it has to
    /// execute another make_move to figure out king threat.
    pub fn get_game_result(&self) -> Result<Option<ChessResult>, &'static str> {
        if self.generate_moves(true)?.is_empty() {
            if self.check_king_threat()? {
                Ok(Some(ChessResult::Checkmate(self.current_move)))
            } else {
                Ok(Some(ChessResult::Draw))
            }
        } else {
            Ok(None)
        }
    }

    // Executes the given `chess_move` in place on self
    //
    // Returns error if the move can't be performed
    pub fn make_move(
        &mut self,
        chess_move: ChessMove,
        checked: bool,
    ) -> Result<(), &'static str> {
        match chess_move {
            ChessMove::SimpleMove(from, to) => {
                self.make_simple_move(from, to)?
            }
            ChessMove::EnPassant(from, to, capturing) => {
                self.capture_en_passant(from, to, capturing)?
            }
            ChessMove::Castling(dir) => self.castle(dir, checked)?,
        };

        if checked && self.check_king_threat()? {
            return Err("Can't make move, there's King in check");
        }

        self.update_en_passant_target(&chess_move)?;
        self.update_castling_state(&chess_move);

        self.current_move = self.current_move.flip();

        Ok(())
    }

    fn make_simple_move(
        &mut self,
        from: Square,
        to: Square,
    ) -> Result<(), &'static str> {
        let old_piece = self.get_piece_at_position(from)?;
        let new_piece = self.get_piece_at_position(to)?;

        if old_piece
            .ok_or("Can't make move, old_pos doesn't have piece")?
            .side
            != self.current_move
        {
            Err("Can't make move, piece at old_pos isn't currently moving")
        } else if new_piece.is_some_and(|s| s.side == self.current_move) {
            Err("Can't make move, friendly piece exists at new_pos")
        } else {
            self.set_piece_at_position(None, from)?;
            self.set_piece_at_position(old_piece, to)
        }
    }

    // Returns the position that is related to the given index
    pub fn index_to_position(
        &self,
        index: usize,
    ) -> Result<Square, &'static str> {
        if index >= self.squares.len() {
            Err("Index out of bounds")
        } else {
            let rank = index / self.width;
            let file = index - rank * self.width;
            Ok(Square { rank, file })
        }
    }

    // Checks that the square is a valid square on the board
    //
    // If the square is out of bounds in either or both directions an error is returned
    fn check_square(
        &self,
        square: UncheckedSquare,
    ) -> Result<Square, InvalidSquareError> {
        square.check_with_board(self)
    }

    fn get_positions_of_matching_pieces(
        &self,
        piece: Piece,
    ) -> Result<Vec<Square>, &'static str> {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == piece.into())
            .map(|(index, _)| self.index_to_position(index))
            .collect()
    }

    // Checks and returns the first piece in the given offset from given position
    //
    // If there are no pieces in the given offset, returns the last square that could be reached
    // If there is a piece in the given offset, returns position of that piece
    fn check_ray_for_pieces(
        &self,
        pos: Square,
        offset: Offset,
        can_take: bool,
    ) -> Square {
        let mut final_pos = pos;
        loop {
            match self.add_offset_to_position(final_pos, offset) {
                Err(_) => break,
                Ok(new_pos) => match self
                    .get_piece_at_position(new_pos)
                    .unwrap()
                {
                    Some(Piece { side, .. }) if side == self.current_move => {
                        break
                    }
                    Some(Piece { .. }) => {
                        if can_take {
                            final_pos = new_pos;
                        }
                        break;
                    }
                    None => final_pos = new_pos,
                },
            }
        }

        final_pos
    }

    fn get_all_squares_between(
        &self,
        start: Square,
        dest: Square,
        offset: Offset,
    ) -> Result<Vec<Square>, InvalidOffsetError> {
        let mut squares = vec![];
        let mut current = start;
        while current != dest {
            current = self.add_offset_to_position(current, offset)?;
            squares.push(current);
        }

        Ok(squares)
    }

    // add an offset to a square and check that it is on the board
    fn add_offset_to_position(
        &self,
        pos: Square,
        offset: Offset,
    ) -> Result<Square, InvalidOffsetError> {
        let new_rank = pos.rank as isize + offset.rank;
        let new_file = pos.file as isize + offset.file;

        if new_rank < 0 {
            if new_file < 0 {
                Err(InvalidOffsetError::LessThanZero(Orientation::Both, offset))
            } else {
                Err(InvalidOffsetError::LessThanZero(Orientation::Rank, offset))
            }
        } else if new_file < 0 {
            Err(InvalidOffsetError::LessThanZero(Orientation::File, offset))
        } else {
            Ok(self.check_square(UncheckedSquare {
                rank: new_rank as usize,
                file: new_file as usize,
            })?)
        }
    }

    // Calculate an offset between two squares on the board
    fn get_offset_of_move(&self, old: Square, new: Square) -> Offset {
        Offset {
            rank: new.rank as isize - old.rank as isize,
            file: new.file as isize - old.file as isize,
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

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
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::Pawn,
                        ..
                    })
                ));
            }
            for i in 48..56 {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::Pawn,
                        ..
                    })
                ));
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
            for i in [0, 7, 56, 63] {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::Rook,
                        ..
                    })
                ));
            }
        }

        #[test]
        fn has_knights_where_it_should() {
            let board = Board::default();
            for i in [1, 6, 57, 62] {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::Knight,
                        ..
                    })
                ));
            }
        }

        #[test]
        fn has_bishop_where_it_should() {
            let board = Board::default();
            for i in [2, 5, 58, 61] {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::Bishop,
                        ..
                    })
                ));
            }
        }

        #[test]
        fn has_queens_where_it_should() {
            let board = Board::default();
            for i in [3, 59] {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::Queen,
                        ..
                    })
                ));
            }
        }

        #[test]
        fn has_kings_where_it_should() {
            let board = Board::default();
            for i in [4, 60] {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        piece_type: PieceType::King,
                        ..
                    })
                ));
            }
        }

        #[test]
        fn has_piece_sides_correct() {
            let board = Board::default();

            for i in 0..16 {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        side: Side::White,
                        ..
                    })
                ));
            }

            for i in 48..64 {
                assert!(matches!(
                    board.squares[i],
                    Some(Piece {
                        side: Side::Black,
                        ..
                    })
                ));
            }
        }
    }

    /// See https://en.wikipedia.org/wiki/Shannon_number
    ///
    /// This test just compares the move generation of `board` against the
    /// numbers found online. This is a pretty difficult test. Currently, at 5
    /// ply, it takes 426 seconds to pass. I'm setting this to run at 3 ply for
    /// now so that the test runs relatively quickly.
    #[test]
    fn generates_expected_move_counts() {
        let expected_move_counts = [20, 400, 8902];
        // let expected_move_counts = [20, 400, 8902, 197281, 4865609];

        let mut current_boards = vec![Board::default()];
        let mut generated_moves: HashMap<Board, Vec<ChessMove>> =
            HashMap::new();

        for &expected_move_count in expected_move_counts.iter() {
            if !generated_moves.is_empty() && !current_boards.is_empty() {
                current_boards.clear();
                for (board, chess_moves) in generated_moves {
                    for chess_move in chess_moves {
                        let mut new_board = board.clone();
                        new_board.make_move(chess_move, false).unwrap();
                        current_boards.push(new_board);
                    }
                }
            }

            generated_moves = HashMap::new();
            for board in current_boards.iter() {
                let moves = board.generate_moves(true).unwrap();
                generated_moves.insert(board.clone(), moves);
            }

            let actual_move_count = generated_moves
                .iter()
                .fold(0, |count, (_b, ms)| count + ms.len());
            assert_eq!(expected_move_count, actual_move_count);
        }
    }

    #[test]
    fn from_art_works_as_expected() {
        let art = "rnbqkbnr\n\
             pppppppp\n\
             ........\n\
             ........\n\
             ........\n\
             ........\n\
             PPPPPPPP\n\
             RNBQKBNR";

        let mut board = Board::from_art(art).unwrap();

        // The default board constructed by Board::default()
        // has castling as true, but art returns it as false
        // So just to make testing equality easier, set
        // it to true here too
        board.castling_availability = [true, true, true, true];

        assert_eq!(board, Board::default());
    }
}
