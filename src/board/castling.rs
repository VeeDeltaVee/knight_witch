use super::{Side, Board};


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
