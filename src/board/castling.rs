mod implementation;

use self::implementation::{
    get_king_starting_square, get_rook_starting_square, CastlingMovementImpl,
    CastlingStateImpl,
};

use super::{chess_move::ChessMove, Board};

/// Define which side castlign is going to happen
#[derive(Debug, Clone, Copy)]
pub enum CastlingDirection {
    Kingside,
    Queenside,
}

/// Trait to access castling state of a chess board
pub trait CastlingState {
    /// Given a `chess_move`, figures out if the castling state should be updated
    ///
    /// Castling should be disallowed if the king moves for a side
    /// Castling should be disallowed in a particular direction if a rook moves
    fn update_castling_state(&mut self, chess_move: &ChessMove);
}

/// Trait to generate and apply castling moves
pub trait CastlingMovement {
    /// Generate a `Vec` of castling boards, with the given move applied
    ///
    /// If `checked` is true, only returns moves where the king wouldn't be
    /// in danger. If false, returns moves where king might be in danger as
    /// well
    fn generate_castling_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;

    /// Changes `self` in place by castling in the given `dir`
    ///
    /// Returns `Err` if castling isn't allowed.
    /// If `checked`, also returns `Err` if the king would be in check
    /// anywhere along the path.
    fn castle(
        &mut self,
        dir: CastlingDirection,
        checked: bool,
    ) -> Result<(), &'static str>;
}

impl CastlingState for Board {
    /// This assumes that the castling state was correct for the board we were in.
    /// Therefore, it wouldn't catch weird conditions like a king being out of positions
    /// but the castling flag being set as true.
    ///
    /// In those cases we rely on other checks to disallow castling
    fn update_castling_state(&mut self, chess_move: &ChessMove) {
        match *chess_move {
            ChessMove::SimpleMove(from, to) => {
                // If something moves away from the starting position of our king,
                // then clearly we can no longer castle. Either the king moved,
                // or something else was in it's position and the king was already not in the
                // starting location
                if from == get_king_starting_square(self.current_move) {
                    self.disable_castling(
                        self.current_move,
                        CastlingDirection::Queenside,
                    );
                    self.disable_castling(
                        self.current_move,
                        CastlingDirection::Kingside,
                    );
                }

                // If we move our rook, we should also lose castling privileges
                if from
                    == get_rook_starting_square(
                        self.current_move,
                        CastlingDirection::Queenside,
                    )
                {
                    self.disable_castling(
                        self.current_move,
                        CastlingDirection::Queenside,
                    );
                } else if from
                    == get_rook_starting_square(
                        self.current_move,
                        CastlingDirection::Kingside,
                    )
                {
                    self.disable_castling(
                        self.current_move,
                        CastlingDirection::Kingside,
                    );
                }

                // If we capture our opponent's rook, they shouldn't be allowed to castle in that
                // direction anymore
                if to
                    == get_rook_starting_square(
                        self.current_move.flip(),
                        CastlingDirection::Queenside,
                    )
                {
                    self.disable_castling(
                        self.current_move.flip(),
                        CastlingDirection::Queenside,
                    );
                } else if to
                    == get_rook_starting_square(
                        self.current_move.flip(),
                        CastlingDirection::Kingside,
                    )
                {
                    self.disable_castling(
                        self.current_move.flip(),
                        CastlingDirection::Kingside,
                    );
                }
            }

            // If we're currently castling, then we can never castle again,
            // so disable castling for this side
            ChessMove::Castling(_) => {
                self.disable_castling(
                    self.current_move,
                    CastlingDirection::Queenside,
                );
                self.disable_castling(
                    self.current_move,
                    CastlingDirection::Kingside,
                );
            }
        }
    }
}

impl CastlingMovement for Board {
    /// Assumes that the game is being played on a standard
    /// board with default configuration, so for now doesn't
    /// support stuff like 960 or odd sized boards
    fn generate_castling_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<Self>, &'static str> {
        let any_castling_state_enabled = self.is_any_castling_state_enabled();
        if !any_castling_state_enabled {
            return Ok(vec![]);
        }

        if self.check_king_threat()? {
            return Ok(vec![]);
        }

        let mut moves = vec![];

        let directions =
            [CastlingDirection::Queenside, CastlingDirection::Kingside];
        for dir in directions {
            if !self.can_castle(dir, checked)? {
                continue;
            }

            let mut castled_board = self.clone();
            castled_board.unchecked_castle(dir);

            moves.push(castled_board);
        }

        Ok(moves)
    }

    fn castle(
        &mut self,
        dir: CastlingDirection,
        checked: bool,
    ) -> Result<(), &'static str> {
        if !self.can_castle(dir, checked)? {
            return Err("Can't castle");
        }

        self.unchecked_castle(dir);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod castling_state {
        use super::*;

        mod update_castling_state {
            use std::convert::TryInto;

            use crate::board::Side;

            use super::*;

            #[test]
            fn doesnt_disallow_castling_if_king_and_rook_dont_move() {
                let mut board = Board::default();

                // Queen's gambit declined
                let moves = ["d2d4", "d7d5", "c2c4", "e7e6"];
                for chess_move in moves {
                    board
                        .make_move(chess_move.try_into().unwrap(), true)
                        .unwrap();
                }

                assert!(board.castling_availability.iter().all(|&a| a));
            }

            #[test]
            fn disallow_castling_if_king_moves() {
                let mut board = Board::default();

                // Bongcloud
                let moves = ["e2e4", "e7e5", "e1e2"];
                for chess_move in moves {
                    board
                        .make_move(chess_move.try_into().unwrap(), true)
                        .unwrap();
                }

                assert!(!board.get_castling_state(
                    Side::White,
                    CastlingDirection::Queenside
                ));
                assert!(!board.get_castling_state(
                    Side::White,
                    CastlingDirection::Kingside
                ));
            }

            #[test]
            fn disallow_castling_if_rook_moves() {
                let mut board = Board::default();

                // Bongcloud
                let moves = ["h2h4", "h7h5", "h1h3"];
                for chess_move in moves {
                    board
                        .make_move(chess_move.try_into().unwrap(), true)
                        .unwrap();
                }

                assert!(board.get_castling_state(
                    Side::White,
                    CastlingDirection::Queenside
                ));
                assert!(!board.get_castling_state(
                    Side::White,
                    CastlingDirection::Kingside
                ));
            }
        }
    }
}
