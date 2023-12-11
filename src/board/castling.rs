use super::{Side, Board};

#[derive(Clone, Copy)]
pub enum CastlingDirection {
    Kingside,
    Queenside
}

pub trait CastlingState {
    /// Returns whether or not the given `side` can castle in 
    /// the given `direction`
    fn can_castle(&self, side: Side, direction: CastlingDirection) -> bool;

    /// Updates the castling state to disallow castling for a
    /// given `side` in the given `direction`
    fn disallow_castling(&mut self, side: Side, direction: CastlingDirection);
}

impl CastlingState for Board {
    fn can_castle(&self, side: Side, direction: CastlingDirection) -> bool {
        let index = calculate_index(side, direction);
        self.castling_availability[index]
    }

    fn disallow_castling(&mut self, side: Side, direction: CastlingDirection) {
        let index = calculate_index(side, direction);
        self.castling_availability[index] = false;
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

pub trait CastlingMovement {
    fn generate_castling_moves(&self, checked: bool) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl CastlingMovement for Board {
    /// Implementation of castling movement for Board.
    /// Assumes that the game is being played on a standard
    /// board with default configuration, so for now doesn't
    /// support stuff like 960 or odd sized boards
    fn generate_castling_moves(&self, checked: bool) -> Result<Vec<Self>, &'static str> {
        let can_castle_anywhere = self.can_castle_any()?;
        if !can_castle_anywhere {
            return Ok(vec![]);
        }

        self.ensure_king_position();

        if self.check_king_threat()? {
            return OK(vec![])
        }

        let moves = vec![];

        let directions = [CastlingDirection::Queenside, CastlingDirection::Kingside];
        for dir in directions {
            if !self.can_castle(self.current_move, dir) {
                continue;
            }

            if self.are_pieces_blocking(dir)? {
                continue;
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
                .try_fold(
                    false, |any_in_threat, threat_result| threat_result.map(
                        |threat| any_in_threat | threat
                    ))?;

            if is_king_threatened_on_the_way {
                continue;
            }

            moves.push(self.castle(dir)?);
        }

        Ok(moves)
    }
}

trait CastlingMovementImpl {
    fn ensure_king_position(&self) -> Result<(), &'static str>;
    fn ensure_rook_position(&self, dir: CastlingDirection) -> Result<(), &'static str>;
    fn can_castle_any(&self) -> Result<bool, &'static str>;
    fn are_pieces_blocking(&self, dir: CastlingDirection) -> Result<bool, &'static str>;
    fn moving_king_boards(&self, dir: CastlingDirection) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
    fn castle(&self, dir: CastlingDirection) -> Result<Self, &'static str>
    where
        Self: Sized;
}

impl CastlingMovementImpl for Board {
    fn ensure_king_position(&self) -> Result<(), &'static str> {
        todo!()
    }

    fn ensure_rook_position(&self, dir: CastlingDirection) -> Result<(), &'static str> {
        todo!()
    }

    fn can_castle_any(&self) -> Result<bool, &'static str> {
        todo!()
    }

    fn are_pieces_blocking(&self, dir: CastlingDirection) -> Result<bool, &'static str> {
        todo!()
    }

    fn moving_king_boards(&self, dir: CastlingDirection) -> Result<Vec<Self>, &'static str> {
        todo!()
    }

    fn castle(&self, dir: CastlingDirection) -> Result<Self, &'static str> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod calculate_index {
        use super::*;

        #[test]
        fn works() {
            assert_eq!(calculate_index(Side::Black, CastlingDirection::Queenside), 0);
            assert_eq!(calculate_index(Side::Black, CastlingDirection::Kingside), 1);
            assert_eq!(calculate_index(Side::White, CastlingDirection::Queenside), 2);
            assert_eq!(calculate_index(Side::White, CastlingDirection::Kingside), 3);
        }
    }
}
