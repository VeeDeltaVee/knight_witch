/// # Implementation module
///
/// This module is internal implementation of castling and isn't exposed outside
/// of the `castling` module
use crate::board::{Board, Offset, Piece, PieceType, Side, Square};

use super::CastlingDirection;

/// Private implementation for CastlingState only exposed inside the `castling`
/// module
pub trait CastlingStateImpl {
    /// Returns whether or not the pieces of the given `side` that
    /// would castle in the given `dir` are in their starting positions
    fn get_castling_state(&self, side: Side, direction: CastlingDirection) -> bool;

    /// Updates the castling state to disallow castling for a
    /// given `side` in the given `direction`
    fn disable_castling(&mut self, side: Side, direction: CastlingDirection);
}

/// Private implementation for CastlingMovement, only exposed inside the
/// `castling` module
pub trait CastlingMovementImpl {
    /// Checks if the king of the currently moving side is in the starting
    /// position or not
    fn is_king_in_starting_position(&self) -> Result<bool, &'static str>;

    /// Checks if the rook of the currently moving side that would move if
    /// castling in the `dir` is in the starting
    /// position or not
    fn is_rook_in_starting_position(&self, dir: CastlingDirection) -> Result<bool, &'static str>;

    /// Checks if castling is allowed in any direction for the current side Only
    /// checks the castling state, not whether or not the move can be made now
    /// so is a much lighter operation. Still need to call `are_pieces_blocking`
    /// and check that no `moved_king_boards` are in check
    fn is_any_castling_state_enabled(&self) -> bool;

    /// For the `current_move`, checks if there are any pieces blocking if
    /// castling was to be attempted in the `dir`
    fn are_pieces_blocking(&self, dir: CastlingDirection) -> bool;

    /// Returns a list of boards of the current king moving to castle in the
    /// given direction
    fn moving_king_boards(&self, dir: CastlingDirection) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;

    /// Performs castling without checking whether or not it's allowed. Only
    /// call after having called `can_castle`
    fn unchecked_castle(&mut self, dir: CastlingDirection);

    /// Performs a full check about whether or not castling is allowed,
    /// including checking if pieces have moved before, if there's anything
    /// blocking, if the king would be in threat, etc.
    fn can_castle(&self, dir: CastlingDirection, checked: bool) -> Result<bool, &'static str>;
}

impl CastlingStateImpl for Board {
    fn get_castling_state(&self, side: Side, direction: CastlingDirection) -> bool {
        let index = calculate_index(side, direction);
        self.castling_availability[index]
    }
    fn disable_castling(&mut self, side: Side, direction: CastlingDirection) {
        let index = calculate_index(side, direction);
        self.castling_availability[index] = false;
    }
}

impl CastlingMovementImpl for Board {
    fn is_king_in_starting_position(&self) -> Result<bool, &'static str> {
        let king_piece = self.get_piece_at_position(get_king_starting_square(self.current_move))?;

        Ok(king_piece.is_some_and(|p| p.piece_type == PieceType::King))
    }

    fn is_rook_in_starting_position(&self, dir: CastlingDirection) -> Result<bool, &'static str> {
        let rook_piece =
            self.get_piece_at_position(get_rook_starting_square(self.current_move, dir))?;

        Ok(rook_piece.is_some_and(|p| p.piece_type == PieceType::Rook))
    }

    fn is_any_castling_state_enabled(&self) -> bool {
        self.get_castling_state(self.current_move, CastlingDirection::Queenside)
            || self.get_castling_state(self.current_move, CastlingDirection::Kingside)
    }

    fn are_pieces_blocking(&self, dir: CastlingDirection) -> bool {
        let king_starting_position = get_king_starting_square(self.current_move);

        let offset = match dir {
            CastlingDirection::Kingside => Offset { file: 1, rank: 0 },
            CastlingDirection::Queenside => Offset { file: -1, rank: 0 },
        };

        let reachable_position = self.check_ray_for_pieces(king_starting_position, offset, false);
        match dir {
            CastlingDirection::Queenside => reachable_position.file > 1,
            CastlingDirection::Kingside => reachable_position.file < 6,
        }
    }

    fn moving_king_boards(&self, dir: CastlingDirection) -> Result<Vec<Self>, &'static str> {
        let king_starting_position = get_king_starting_square(self.current_move);

        match dir {
            CastlingDirection::Queenside => (1..4).rev().collect::<Vec<_>>(),
            CastlingDirection::Kingside => (5..7).collect(),
        }.into_iter().map(|file| {
                let new_pos = Square {
                    rank: get_starting_rank(self.current_move),
                    file,
                };

                let mut new_board = self.clone();

                new_board.set_piece_at_position(
                    Some(Piece::new(self.current_move, PieceType::King)),
                    new_pos,
                )?;
                new_board.set_piece_at_position(None, king_starting_position)?;

                Ok(new_board)
            })
            .collect()
    }

    fn unchecked_castle(&mut self, dir: CastlingDirection) {
        let king_starting_position = get_king_starting_square(self.current_move);
        let rook_starting_position = get_rook_starting_square(self.current_move, dir);

        let rook_new_position = get_rook_end_position(self.current_move, dir);
        let king_new_position = get_king_end_position(self.current_move, dir);

        self.set_piece_at_position(
            Some(Piece::new(self.current_move, PieceType::King)),
            king_new_position,
        )
        .unwrap();
        self.set_piece_at_position(
            Some(Piece::new(self.current_move, PieceType::Rook)),
            rook_new_position,
        )
        .unwrap();
        self.set_piece_at_position(None, king_starting_position)
            .unwrap();
        self.set_piece_at_position(None, rook_starting_position)
            .unwrap();
    }

    fn can_castle(&self, dir: CastlingDirection, checked: bool) -> Result<bool, &'static str> {
        if !self.get_castling_state(self.current_move, dir) {
            return Ok(false);
        }

        if self.are_pieces_blocking(dir) {
            return Ok(false);
        }

        if !self.is_king_in_starting_position()? || !self.is_rook_in_starting_position(dir)? {
            return Ok(false);
        }

        // If the move isn't checked, then we don't check for king threats, including on
        // the way
        if !checked {
            return Ok(true);
        }

        let boards = self.moving_king_boards(dir)?;

        // This looks kind of weird because Rust doesn't really have a good
        // way to turn a Result of Iter into an Iter of Result. It involves
        // `collect()`ing along the way, which means an extra allocation we
        // don't need.
        //
        // This is basically doing the following:
        // boards.map(is_threat).all(is false)
        let is_king_threatened_on_the_way = boards
            .iter()
            .map(|board| board.check_king_threat())
            .try_fold(false, |any_in_threat, threat_result| {
                threat_result.map(|threat| any_in_threat | threat)
            })?;

        if is_king_threatened_on_the_way {
            return Ok(false);
        }

        Ok(true)
    }
}

/// Calculate the index into [`Board`]'s internal representation
/// of castling state for a given `side` and `direction`
fn calculate_index(side: Side, direction: CastlingDirection) -> usize {
    // The array is indexed top left to bottom right
    let mut side_index = match side {
        Side::Black => 0,
        Side::White => 2,
    };

    // ^ is equivalent to + here
    side_index ^= match direction {
        CastlingDirection::Queenside => 0,
        CastlingDirection::Kingside => 1,
    };

    side_index
}

/// Get rank where the pieces of the given `side` start
fn get_starting_rank(side: Side) -> usize {
    match side {
        Side::White => 0,
        Side::Black => 7,
    }
}

/// Get `Square` where the king of the given `side` starts
pub fn get_king_starting_square(side: Side) -> Square {
    let starting_rank = get_starting_rank(side);
    Square {
        file: 4,
        rank: starting_rank,
    }
}

/// Get `Square` where the rook of the given `side`, that would move if castling
/// in the given `dir`, starts
pub fn get_rook_starting_square(side: Side, dir: CastlingDirection) -> Square {
    let starting_rank = get_starting_rank(side);

    let rook_starting_file = match dir {
        CastlingDirection::Queenside => 0,
        CastlingDirection::Kingside => 7,
    };

    Square {
        file: rook_starting_file,
        rank: starting_rank,
    }
}

/// Gets the final position of a king of `side` that's moving to castle in the
/// given `dir`
fn get_king_end_position(side: Side, dir: CastlingDirection) -> Square {
    match dir {
        CastlingDirection::Kingside => Square {
            rank: get_starting_rank(side),
            file: 6,
        },
        CastlingDirection::Queenside => Square {
            rank: get_starting_rank(side),
            file: 2,
        },
    }
}

/// Gets the final position of a rook of `side` that's moving to castle in the
/// given `dir`
fn get_rook_end_position(side: Side, dir: CastlingDirection) -> Square {
    match dir {
        CastlingDirection::Kingside => Square {
            rank: get_starting_rank(side),
            file: 5,
        },
        CastlingDirection::Queenside => Square {
            rank: get_starting_rank(side),
            file: 3,
        },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod castling_state {
        use super::*;

        mod get_castling_state {
            use super::*;

            #[test]
            fn works_with_default_board() {
                let board = Board::default();

                let sides = [Side::White, Side::Black];
                let directions = [CastlingDirection::Queenside, CastlingDirection::Kingside];

                for side in sides {
                    for dir in directions {
                        assert!(board.get_castling_state(side, dir));
                    }
                }
            }

            #[test]
            fn returns_false_when_castling_disallowed() {
                let mut board = Board::default();
                board.castling_availability = [true, false, true, false];

                assert!(
                    !board.get_castling_state(Side::Black, CastlingDirection::Kingside)
                );
                assert!(
                    !board.get_castling_state(Side::White, CastlingDirection::Kingside)
                );
                assert!(
                    board.get_castling_state(Side::Black, CastlingDirection::Queenside)
                );
                assert!(
                    board.get_castling_state(Side::White, CastlingDirection::Queenside)
                );
            }
        }

        mod disable_castling {
            use super::*;

            #[test]
            fn works() {
                let mut board = Board::default();

                board.disable_castling(Side::White, CastlingDirection::Kingside);

                assert!(
                    !board.get_castling_state(Side::White, CastlingDirection::Kingside)
                );
                assert!(
                    board.get_castling_state(Side::Black, CastlingDirection::Kingside)
                );
                assert!(
                    board.get_castling_state(Side::White, CastlingDirection::Queenside)
                );
                assert!(
                    board.get_castling_state(Side::Black, CastlingDirection::Queenside)
                );
            }
        }
    }

    mod castling_movement {
        use super::*;

        mod is_king_in_starting_position {

            use super::*;

            #[test]
            fn works_with_default_board() {
                let board = Board::default();

                assert!(board.is_king_in_starting_position().unwrap());
            }

            #[test]
            fn works_with_empty_board() {
                let pieces = vec![None; 64];
                let board = Board::with_pieces(pieces, 8);

                assert!(!board.is_king_in_starting_position().unwrap());
            }

            #[test]
            fn works_with_no_king_of_current_side() {
                let mut board = Board::default();

                // Remove the White king from the board
                board
                    .set_piece_at_position(None, Square { file: 4, rank: 0 })
                    .unwrap();

                // This would fail if this method was somehow finding the other
                // king
                assert!(!board.is_king_in_starting_position().unwrap());
            }
        }

        mod is_rook_in_starting_position {
            use super::*;

            #[test]
            fn works_with_default_board() {
                let board = Board::default();

                assert!(board.is_rook_in_starting_position(CastlingDirection::Kingside).unwrap());
                assert!(board.is_rook_in_starting_position(CastlingDirection::Queenside).unwrap());
            }

            #[test]
            fn works_with_empty_board() {
                let pieces = vec![None; 64];
                let board = Board::with_pieces(pieces, 8);

                assert!(
                    !board.is_rook_in_starting_position(
                        CastlingDirection::Queenside)
                    .unwrap()
                );

                assert!(
                    !board.is_rook_in_starting_position(
                        CastlingDirection::Kingside)
                    .unwrap()
                );
            }

            #[test]
            fn works_with_one_rook_removed_current_side() {
                let mut board = Board::default();

                board.set_piece_at_position(None, Square { file: 0, rank: 0 }).unwrap();

                assert!(
                    !board.is_rook_in_starting_position(
                        CastlingDirection::Queenside)
                    .unwrap()
                );

                assert!(
                    board.is_rook_in_starting_position(
                        CastlingDirection::Kingside)
                    .unwrap()
                );
            }

            #[test]
            fn works_with_both_rooks_removed_current_side() {
                let mut board = Board::default();

                board.set_piece_at_position(None, Square { file: 0, rank: 0 }).unwrap();
                board.set_piece_at_position(None, Square { file: 7, rank: 0 }).unwrap();

                assert!(
                    !board.is_rook_in_starting_position(
                        CastlingDirection::Queenside)
                    .unwrap()
                );

                assert!(
                    !board.is_rook_in_starting_position(
                        CastlingDirection::Kingside)
                    .unwrap()
                );
            }

            #[test]
            fn works_with_one_rook_removed_other_side() {
                let mut board = Board::default();

                board.set_piece_at_position(None, Square { file: 0, rank: 7 }).unwrap();

                assert!(
                    board.is_rook_in_starting_position(
                        CastlingDirection::Queenside)
                    .unwrap()
                );

                assert!(
                    board.is_rook_in_starting_position(
                        CastlingDirection::Kingside)
                    .unwrap()
                );
            }

            #[test]
            fn works_with_both_rooks_removed_other_side() {
                let mut board = Board::default();

                board.set_piece_at_position(None, Square { file: 0, rank: 7 }).unwrap();
                board.set_piece_at_position(None, Square { file: 7, rank: 7 }).unwrap();

                assert!(
                    board.is_rook_in_starting_position(
                        CastlingDirection::Queenside)
                    .unwrap()
                );

                assert!(
                    board.is_rook_in_starting_position(
                        CastlingDirection::Kingside)
                    .unwrap()
                );
            }
        }

        mod is_any_castling_state_enabled {
            use super::*;

            #[test]
            fn works_with_all_true() {
                let board = Board::default();

                assert!(board.is_any_castling_state_enabled());
            }

            #[test]
            fn works_with_one_true() {
                let mut board = Board::default();

                // Set white castling queenside to be allowed
                board.castling_availability = [false, false, true, false];

                assert!(board.is_any_castling_state_enabled());
            }

            #[test]
            fn works_with_none_true() {
                let mut board = Board::default();

                // Black is allowed to castle, white isn't
                board.castling_availability = [true, true, false, false];

                assert!(!board.is_any_castling_state_enabled());
            }
        }

        fn get_test_board() -> Board {
            let mut board = Board::from_art(
                "........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 ........\n\
                 r...k..r\
            ").unwrap();

            board.castling_availability = [false, false, true, true];

            board
        }

        mod are_pieces_blocking {
            use super::*;

            #[test]
            fn doesnt_block_when_free_queenside() {
                let mut board = Board::default();

                // Remove the queen, queenside bishop and knight
                board.set_piece_at_position(None, Square { file: 1, rank: 0 }).unwrap();
                board.set_piece_at_position(None, Square { file: 2, rank: 0 }).unwrap();
                board.set_piece_at_position(None, Square { file: 3, rank: 0 }).unwrap();

                assert!(!board.are_pieces_blocking(CastlingDirection::Queenside));
            }

            #[test]
            fn doesnt_block_when_free_kingside() {
                let mut board = Board::default();

                // Remove the kingside bishop and the knight
                board.set_piece_at_position(None, Square { file: 5, rank: 0 }).unwrap();
                board.set_piece_at_position(None, Square { file: 6, rank: 0 }).unwrap();

                assert!(!board.are_pieces_blocking(CastlingDirection::Kingside));
            }

            #[test]
            fn blocks_when_any_piece_present() {
                let mut board = Board::default();

                // Remove the kingside bishop, but not the knight
                board.set_piece_at_position(None, Square { file: 5, rank: 0 }).unwrap();

                assert!(board.are_pieces_blocking(CastlingDirection::Kingside));
            }
        }

        mod moving_king_boards {
            use super::*;

            fn assert_indices_and_files_for_board(indices_and_files: &[(usize, usize)], boards: &[Board]) {
                for &(index, file) in indices_and_files {
                    let board = boards.get(index).unwrap();
                    assert!(
                        board.get_piece_at_position(
                            Square { file, rank: 0 }
                        ).is_ok_and(|o| o.is_some_and(
                            |k| k.piece_type == PieceType::King
                        ))
                    );
                }
            }

            #[test]
            fn works_kingside() {
                let board = get_test_board();

                let moving_boards = board.moving_king_boards(CastlingDirection::Kingside).unwrap();
                assert_eq!(moving_boards.len(), 2);

                let indices_and_files = [(0, 5), (1, 6)];
                assert_indices_and_files_for_board(
                    &indices_and_files,
                    &moving_boards
                );
            }
            #[test]
            fn works_queenside() {
                let board = get_test_board();

                let moving_boards = board.moving_king_boards(CastlingDirection::Queenside).unwrap();
                assert_eq!(moving_boards.len(), 3);

                let indices_and_files = [(0, 3), (1, 2), (2, 1)];
                assert_indices_and_files_for_board(
                    &indices_and_files,
                    &moving_boards
                );
            }
        }

        mod unchecked_castle {
            use super::*;

            #[test]
            fn can_castle_queenside() {
                let mut board = get_test_board();

                board.unchecked_castle(CastlingDirection::Queenside);
                let king = board.get_piece_at_position(get_king_end_position(Side::White, CastlingDirection::Queenside)).unwrap().unwrap();
                let rook = board.get_piece_at_position(get_rook_end_position(Side::White, CastlingDirection::Queenside)).unwrap().unwrap();

                assert_eq!(king.piece_type, PieceType::King);
                assert_eq!(rook.piece_type, PieceType::Rook);
            }

            #[test]
            fn can_castle_kingside() {
                let mut board = get_test_board();

                board.unchecked_castle(CastlingDirection::Kingside);
                let king = board.get_piece_at_position(get_king_end_position(Side::White, CastlingDirection::Kingside)).unwrap().unwrap();
                let rook = board.get_piece_at_position(get_rook_end_position(Side::White, CastlingDirection::Kingside)).unwrap().unwrap();

                assert_eq!(king.piece_type, PieceType::King);
                assert_eq!(rook.piece_type, PieceType::Rook);
            }
        }
    }

    mod utils {
        use super::*;

        mod calculate_index {
            use super::*;

            use crate::board::{castling::CastlingDirection, Side};

            #[test]
            fn works() {
                assert_eq!(
                    calculate_index(Side::Black, CastlingDirection::Queenside),
                    0
                );
                assert_eq!(calculate_index(Side::Black, CastlingDirection::Kingside), 1);
                assert_eq!(
                    calculate_index(Side::White, CastlingDirection::Queenside),
                    2
                );
                assert_eq!(calculate_index(Side::White, CastlingDirection::Kingside), 3);
            }
        }
    }
}
