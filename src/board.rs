pub mod knight;
pub mod pawn;
pub mod rook;
pub mod bishop;
pub mod queen;
pub mod king;
pub mod castling;
pub mod square;

mod straight_moving_piece;
mod test_utils;
mod errors;

use crate::board::pawn::PawnMovement;
use std::fmt;

use self::knight::KnightMovement;
use self::rook::RookMovement;
use self::bishop::BishopMovement;
use self::queen::QueenMovement;
use self::king::KingMovement;
use self::errors::*;
use self::square::{Square, UncheckedSquare};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    White,
    Black,
}

impl Side {
    fn flip(self: Self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

pub type Piece = Option<(PieceType, Side)>;

pub fn get_piece_side(piece: Piece) -> Option<Side> {
    piece.map(|(_, side)| side)
}

pub fn get_piece_type(piece: Piece) -> Option<PieceType> {
    piece.map(|(t, _)| t)
}

pub fn get_piece_from_char(ch: char) -> Piece {
    match ch {
         'K' => Some((PieceType::King, Side::White)),
         'Q' => Some((PieceType::Queen, Side::White)),
         'R' => Some((PieceType::Rook, Side::White)),
         'B' => Some((PieceType::Bishop, Side::White)),
         'N' => Some((PieceType::Knight, Side::White)),
         'P' => Some((PieceType::Pawn, Side::White)),
         'k' => Some((PieceType::King, Side::Black)),
         'q' => Some((PieceType::Queen, Side::Black)),
         'r' => Some((PieceType::Rook, Side::Black)),
         'b' => Some((PieceType::Bishop, Side::Black)),
         'n' => Some((PieceType::Knight, Side::Black)),
         'p' => Some((PieceType::Pawn, Side::Black)),
          _  => None,
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board {
    // An array of squares for the board.
    // In a typical chess game, this would be a vector with length 64.
    //
    // Indices work as follows: we start out at the bottom file, go left to
    // right, and then once we reach the end of a file we go up a file.
    squares: Vec<Piece>,
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
        write!(f, "Board:\n")?;
        // We want to print rank 0 at the bottom
        for rank in (0..self.squares.len() / self.width).rev() {
            write!(f, "\t")?;
            for file in 0..self.width {
                let square = UncheckedSquare {
                    rank: rank,
                    file: file,
                }.validate(self)
                .expect("We are iterating over the squares in the board, this should always be a valid square, someone fucked up");

                if self.en_passant_target == Some(square) {
                    write!(f, "*")?;
                    continue;
                }

                let piece = self.get_piece_at_position(square);
                let representation = match piece {
                    None => ".",
                    Some((PieceType::Pawn, Side::White)) => "P",
                    Some((PieceType::Rook, Side::White)) => "R",
                    Some((PieceType::Knight, Side::White)) => "N",
                    Some((PieceType::Bishop, Side::White)) => "B",
                    Some((PieceType::Queen, Side::White)) => "Q",
                    Some((PieceType::King, Side::White)) => "K",
                    Some((PieceType::Pawn, Side::Black)) => "p",
                    Some((PieceType::Rook, Side::Black)) => "r",
                    Some((PieceType::Knight, Side::Black)) => "n",
                    Some((PieceType::Bishop, Side::Black)) => "b",
                    Some((PieceType::Queen, Side::Black)) => "q",
                    Some((PieceType::King, Side::Black)) => "k",
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
            Some((PieceType::Rook, Side::White)),
            Some((PieceType::Knight, Side::White)),
            Some((PieceType::Bishop, Side::White)),
            Some((PieceType::Queen, Side::White)),
            Some((PieceType::King, Side::White)),
            Some((PieceType::Bishop, Side::White)),
            Some((PieceType::Knight, Side::White)),
            Some((PieceType::Rook, Side::White)),
        ];
        let mut white_pawn_rank = vec![Some((PieceType::Pawn, Side::White)); 8];
        let mut empty_ranks = vec![None; 8 * 4];
        let mut black_pawn_rank = vec![Some((PieceType::Pawn, Side::Black)); 8];
        let mut black_back_rank = vec![
            Some((PieceType::Rook, Side::Black)),
            Some((PieceType::Knight, Side::Black)),
            Some((PieceType::Bishop, Side::Black)),
            Some((PieceType::Queen, Side::Black)),
            Some((PieceType::King, Side::Black)),
            Some((PieceType::Bishop, Side::Black)),
            Some((PieceType::Knight, Side::Black)),
            Some((PieceType::Rook, Side::Black)),
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
        let pieces = art.lines()
            .map(|line| line.chars().map(get_piece_from_char)).rev();

        let mut widths = pieces.clone().map(|rank| rank.count());
        let first_width = widths.next().ok_or("Can't create board with no height")?;
        widths.all(|w| w == first_width);

        Ok(Board::with_pieces(pieces.flatten().collect(), first_width))
    }

    pub fn with_pieces(pieces: Vec<Piece>, width: usize) -> Self {
        Board {
            squares: pieces,
            width: width,
            en_passant_target: None,
            current_move: Side::White,

            // There's no real way to get the castling availability 
            // while constructing a board from pieces, so we set all to false
            castling_availability: [false, false, false, false],
        }
    }

    // Generates a list of future board states that are possible from the
    // current board state.
    pub fn generate_moves(&self, checked: bool) -> Result<Vec<Board>, &'static str> {
        let mut moves = self.generate_pawn_moves(checked)?;
        moves.append(&mut self.generate_knight_moves(checked)?);
        moves.append(&mut self.generate_bishop_moves(checked)?);
        moves.append(&mut self.generate_rook_moves(checked)?);
        moves.append(&mut self.generate_queen_moves(checked)?);
        moves.append(&mut self.generate_king_moves(checked)?);

        Ok(moves)
    }

    // Gets the piece that's at the given position.
    //
    // Returns error if position is out of bounds
    pub fn get_piece_at_position(
        &self,
        square: Square,
    ) -> Option<(PieceType, Side)> { 
        self.squares[square.get_rank() * self.width + square.get_file()]
    }

    // Sets the piece at the given position to be the given piece
    //
    // Returns error if position is out of bounds
    pub fn set_piece_at_position(
        &mut self,
        piece: Piece,
        square: Square,
    ){
        self.squares[square.get_rank() * self.width + square.get_file()] = piece;
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
        new_board.make_move(old_pos, new_pos, checked)?;
        Ok(new_board)
    }

    pub fn check_king_threat(&self) -> Result<bool, &'static str> {
        let kings = self.get_positions_of_pieces_with_given_side_and_type(PieceType::King, self.current_move)?;
        let num_kings = kings.len();

        let mut skipped_move_board = self.clone();
        skipped_move_board.current_move = skipped_move_board.current_move.flip();

        let other_sides_potential_moves = skipped_move_board.generate_moves(false)?;

        let is_king_in_threat = other_sides_potential_moves
            .iter()
            .map(|b| b.get_positions_of_pieces_with_given_side_and_type(PieceType::King, self.current_move).map(|ks| ks.len()).unwrap_or(0))
            .any(|n| {
                num_kings != n
            });

        Ok(is_king_in_threat)
    }

    // Moves the piece at given old position to given new position in place
    //
    // Returns error if either position is out of bounds, if there's no piece at
    // old position, if there's no piece at the new position, or if the piece
    // to be moved isn't CurrentlyMoving
    //
    // TODO: Needs to fail when currently-moving king is in check
    pub fn make_move(&mut self, old_pos: Square, new_pos: Square, checked: bool) -> Result<(), &'static str> {
        let old_piece = self.get_piece_at_position(old_pos);
        let new_piece = self.get_piece_at_position(new_pos);

        let is_old_piece_currently_moving = get_piece_side(old_piece)
            .ok_or("Can't make move, old_pos doesn't have piece")?
            == self.current_move;
        let is_new_piece_ours = get_piece_side(new_piece).is_some_and(|s| s == self.current_move);

        match (is_old_piece_currently_moving, is_new_piece_ours) {
            (true, false) => {
                self.set_piece_at_position(old_piece, new_pos);
                self.set_piece_at_position(None, old_pos);

                let offset = self.get_offset_of_move(old_pos, new_pos);

                if old_piece.is_some_and(|(t, _)| t == PieceType::Pawn) &&
                    offset.rank.abs() == 2 &&
                    old_pos.get_file() == new_pos.get_file()
                {
                    let en_passent_target_dir = Offset {
                        rank: offset.rank / 2,
                        file: 0
                    };

                    self.en_passant_target = Some(self.add_offset_to_position(old_pos, en_passent_target_dir)?);
                } else {
                    self.en_passant_target = None;
                }

                if checked && self.check_king_threat()? {
                    Err("Can't make move, there's King in check")
                } else {
                    self.current_move = self.current_move.flip();
                    Ok(())
                }
            }
            (false, _) => Err("Can't make move, piece at old_pos isn't CurrentlyMoving"),
            (_, true) => Err("Can't make move, friendly piece exists at new_pos"),
        }
    }

    // Returns the position that is related to the given index
    pub fn index_to_position(&self, index: usize) -> Result<Square, &'static str> {
        if index >= self.squares.len() {
            Err("Index out of bounds")
        } else {
            let rank = index / self.width;
            let file = index - rank * self.width;
            Ok(UncheckedSquare {
                rank: rank,
                file: file,
            }.validate(self)?)
        }
    }

    fn get_positions_of_pieces_with_given_side_and_type(
        &self,
        piece_type: PieceType,
        piece_side: Side,
    ) -> Result<Vec<Square>, &'static str> {
        self.squares
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == Some((piece_type, piece_side)))
            .map(|(index, _)| self.index_to_position(index))
            .collect()
    }

    // Checks and returns the first piece in the given offset from given position
    //
    // If there are no pieces in the given offset, returns the last square that could be reached
    // If there is a piece in the given offset, returns position of that piece
    fn check_ray_for_pieces(&self, pos: Square, offset: Offset, can_take: bool) -> Square {
        let mut final_pos = pos;
        loop {
            match self.add_offset_to_position(final_pos, offset) {
                Err(_) => break,
                Ok(new_pos) => match self.get_piece_at_position(new_pos) {
                    Some((_, Side::White)) => break,
                    Some((_, Side::Black)) => {
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

    fn get_all_squares_between(&self, start: Square, dest: Square, offset: Offset) -> Result<Vec<Square>, InvalidOffsetError> {
        let mut squares = vec![];
        let mut current = start;
        while current != dest {
            current = self.add_offset_to_position(current, offset)?;
            squares.push(current);
        }

        Ok(squares)
    }

    // add an offset to a square and validate that it is on the board
    fn add_offset_to_position(
        &self,
        pos: Square,
        offset: Offset,
    ) -> Result<Square, InvalidOffsetError> {
        let new_rank = pos.get_rank() as isize + offset.rank;
        let new_file = pos.get_file() as isize + offset.file;

        if new_rank < 0 {
            if new_file < 0 {
                Err(InvalidOffsetError::LessThanZero(Orientation::Both, offset))
            } else {
                Err(InvalidOffsetError::LessThanZero(Orientation::Rank, offset))
            }
        } else if new_file < 0 {
            Err(InvalidOffsetError::LessThanZero(Orientation::File, offset))
        } else {
            Ok(UncheckedSquare {
                rank: new_rank as usize,
                file: new_file as usize,
            }.validate(self)?)
        }
    }

    // Calculate an offset between two squares on the board
    fn get_offset_of_move(&self, old: Square, new: Square) -> Offset {
        Offset {
            rank: new.get_rank() as isize - old.get_rank() as isize,
            file: new.get_file() as isize - old.get_file() as isize,
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
                assert!(matches!(board.squares[i], Some((_, Side::White))));
            }

            for i in 48..64 {
                assert!(matches!(board.squares[i], Some((_, Side::Black))));
            }
        }
    }

    #[test]
    fn from_art_works_as_expected() {
        let art =
            "rnbqkbnr\n\
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
