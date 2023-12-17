pub mod bishop;
pub mod castling;
pub mod chess_move;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;


mod errors;
mod piece;
mod straight_moving_piece;
mod test_utils;

use crate::board::pawn::PawnMovement;
use std::convert::TryFrom;
use std::fmt;

pub use piece::*;

use self::bishop::BishopMovement;
use self::castling::{CastlingMovement, CastlingState};
use self::chess_move::ChessMove;
use self::errors::*;
use self::king::KingMovement;
use self::knight::KnightMovement;
use self::queen::QueenMovement;
use self::rook::RookMovement;

// Represents a square on the board
//
// File counts from the left, starts at 0
// Rank counts from the bottom, starts at 0
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Square {
    file: usize,
    rank: usize,
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
        write!(f, "Board:\n")?;
        // We want to print rank 0 at the bottom
        for rank in (0..self.squares.len() / self.width).rev() {
            write!(f, "\t")?;
            for file in 0..self.width {
                let square = Square {
                    rank: rank,
                    file: file,
                };
                if self.en_passant_target == Some(square) {
                    write!(f, "*")?;
                    continue;
                }

                let piece = self.get_piece_at_position(square).unwrap();
                let character = piece.as_ref().map_or('.', char::from);
                write!(f, "{}", character)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Board {
    // Construct a default board
    pub fn default() -> Board {
        use { PieceType::*, Side::* };
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
        let mut empty_ranks = vec![None.into(); 8 * 4];
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
        let pieces  = art.lines()
            .map(|line| line.chars().map(Piece::try_from).map(Result::ok))
            .rev();

        let mut widths = pieces.clone().map(|rank| rank.count());
        let first_width = widths.next().ok_or("Can't create board with no height")?;
        widths.all(|w| w == first_width);

        Ok(Board::with_pieces(pieces.flatten().collect(), first_width))
    }

    pub fn with_pieces(pieces: Vec<Option<Piece>>, width: usize) -> Self {
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
    ) -> Result<Option<Piece>, InvalidSquareError> {
        self.validate_square(square).map(|square| {
            self.squares[square.rank * self.width + square.file]
        }) 
    }

    // Sets the piece at the given position to be the given piece
    //
    // Returns error if position is out of bounds
    pub fn set_piece_at_position(
        &mut self,
        piece: Option<Piece>,
        square: Square,
    ) -> Result<(), &'static str> {
        Ok(self.validate_square(square).map(|square| {
            self.squares[square.rank * self.width + square.file] = piece;
            ()
        })?)
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
        new_board.make_move(ChessMove::SimpleMove(old_pos, new_pos), checked)?;
        Ok(new_board)
    }

    pub fn check_king_threat(&self) -> Result<bool, &'static str> {
        use PieceType::King;
        let kings = self.get_positions_of_matching_pieces(Piece::new(self.current_move, King))?;
        let num_kings = kings.len();

        let mut skipped_move_board = self.clone();
        skipped_move_board.current_move = skipped_move_board.current_move.flip();

        let other_sides_potential_moves = skipped_move_board.generate_moves(false)?;

        let is_king_in_threat = other_sides_potential_moves
            .iter()
            .map(|b| b
                .get_positions_of_matching_pieces(Piece::new(self.current_move, King))
                .map(|ks| ks.len()).unwrap_or(0))
            .any(|n| {
                num_kings != n
            });

        Ok(is_king_in_threat)
    }

    // Executes the given `chess_move` in place on self
    //
    // Returns error if the move can't be performed
    pub fn make_move(&mut self, chess_move: ChessMove, checked: bool) -> Result<(), &'static str> {
        match chess_move {
            ChessMove::SimpleMove(from, to) => self.make_simple_move(from, to, checked)?,
            ChessMove::Castling(dir) => self.castle(dir, checked)?,
        };

        self.update_en_passant_target(&chess_move)?;
        self.update_castling_state(&chess_move);

        self.current_move = self.current_move.flip();

        Ok(())
    }

    fn make_simple_move(
        &mut self,
        from: Square,
        to: Square,
        checked: bool,
    ) -> Result<(), &'static str> {
        let old_piece = self.get_piece_at_position(from)?;
        let new_piece = self.get_piece_at_position(to)?;

        if old_piece
            .ok_or("Can't make move, old_pos doesn't have piece")?
            .side != self.current_move
        {
            Err("Can't make move, piece at old_pos isn't currently moving")
        } else if new_piece.is_some_and(|s| s.side == self.current_move) {
            Err("Can't make move, friendly piece exists at new_pos")
        } else {
            self.set_piece_at_position(None, from)?;
            self.set_piece_at_position(old_piece, to)?;

            if checked && self.check_king_threat()? {
                Err("Can't make move, there's King in check")
            } else {
                Ok(())
            }
        }
    }

    /// Figure out how `chess_move` affects en_passant_target
    /// and update accordingly
    fn update_en_passant_target(&mut self, chess_move: &ChessMove) -> Result<(), &'static str> {
        match *chess_move {
            ChessMove::Castling(_) => {
                self.en_passant_target = None;
                Ok(())
            },
            ChessMove::SimpleMove(from, to) => {
                // Note: we're getting the piece at `to` because at this point
                // the piece has already been moved and is at the new position
                let old_piece = self.get_piece_at_position(to)?;
                let offset = self.get_offset_of_move(from, to);

                if old_piece.is_some_and(|p| p.piece_type == PieceType::Pawn)
                    && offset.rank.abs() == 2
                    && from.file == to.file
                {
                    let en_passent_target_dir = Offset {
                        rank: offset.rank / 2,
                        file: 0
                    };

                    self.en_passant_target = Some(self.add_offset_to_position(from, en_passent_target_dir)?);
                } else {
                    self.en_passant_target = None;
                }

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
                file: file,
            })
        }
    }

    // Checks that the square is a valid square on the board
    //
    // If the square is out of bounds in either or both directions an error is returned
    fn validate_square(&self, square: Square) -> Result<Square, InvalidSquareError> {
        if square.file >= self.width {
            if square.rank * self.width + square.file >= self.squares.len(){
                Err(InvalidSquareError::OutOfBounds(Orientation::Both, square))
            } else {
                Err(InvalidSquareError::OutOfBounds(Orientation::File, square))
            }
        } else if square.rank * self.width + square.file >= self.squares.len() {
            Err(InvalidSquareError::OutOfBounds(Orientation::Rank, square))
        } else {
            Ok(square)
        }
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
    fn check_ray_for_pieces(&self, pos: Square, offset: Offset, can_take: bool) -> Square {
        let mut final_pos = pos;
        loop {
            match self.add_offset_to_position(final_pos, offset) {
                Err(_) => break,
                Ok(new_pos) => match self.get_piece_at_position(new_pos).unwrap() {
                    Some(Piece { side, .. }) if side == self.current_move => break,
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
            Ok(self.validate_square(Square {
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
                assert!(matches!(board.squares[i], Some(Piece {piece_type: PieceType::Pawn, .. })));
            }
            for i in 48..56 {
                assert!(matches!(board.squares[i], Some(Piece {piece_type: PieceType::Pawn, .. })));
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
                assert!(matches!(board.squares[i], Some(Piece { piece_type: PieceType::Rook, ..})));
            }
        }

        #[test]
        fn has_knights_where_it_should() {
            let board = Board::default();
            for i in vec![1, 6, 57, 62] {
                assert!(matches!(board.squares[i], Some(Piece { piece_type: PieceType::Knight, ..})));
            }
        }

        #[test]
        fn has_bishop_where_it_should() {
            let board = Board::default();
            for i in vec![2, 5, 58, 61] {
                assert!(matches!(board.squares[i], Some(Piece { piece_type: PieceType::Bishop, ..})));
            }
        }

        #[test]
        fn has_queens_where_it_should() {
            let board = Board::default();
            for i in vec![3, 59] {
                assert!(matches!(board.squares[i], Some(Piece { piece_type: PieceType::Queen, ..})));
            }
        }

        #[test]
        fn has_kings_where_it_should() {
            let board = Board::default();
            for i in vec![4, 60] {
                assert!(matches!(board.squares[i], Some(Piece { piece_type: PieceType::King, ..})));
            }
        }

        #[test]
        fn has_piece_sides_correct() {
            let board = Board::default();

            for i in 0..16 {
                assert!(matches!(board.squares[i], Some(Piece { side: Side::White, .. })));
            }

            for i in 48..64 {
                assert!(matches!(board.squares[i], Some(Piece { side: Side::Black, .. })));
            }
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
