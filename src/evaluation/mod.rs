pub mod material;
pub mod result;

mod test_utils;

use std::cmp::Ordering;

use crate::board::{game::ChessResult, piece::Side, Board};

/// An evaluator takes a board as an input, and returns it's value in centipawns
/// with a positive value favouring white.
///
/// Implementations of evaluator could be complex, and take time to initialize,
/// so the same evaluator should be persisted as long as possible.
pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> Result<Evaluation, &'static str>;
}

pub type Centipawns = i32;
pub type Depth = u8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Evaluation {
    /// An evaluation where the outcome isn't certain. Centipawns is an estimate
    /// of how much better/worse a position is in terms of 1/100ths of a pawn.
    Estimate(Centipawns),

    /// A certain game result, in a number of moves
    Certain(ChessResult, Depth),
}

/// Evaluation has a custom implementation for Ord, because comparing vague
/// centipawn evaluations with a concrete Draw and Checkmate is not
/// straightforward
///
/// `self` > `other` when it's a better evaluation for white
/// `self` < `other` when it's a better evaluation for black
///
/// This fits the standard evaluation that UCI uses, and other engines use.
///
/// Partial Ord is the same as Ord, except wrapped in an Option. It will always
/// be Some and can be safely unwrapped
impl Ord for Evaluation {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        use ChessResult::*;
        use Evaluation::*;
        use Ordering::*;
        use Side::*;

        match self {
            Evaluation::Estimate(self_centipawns) => {
                match other {
                    // We can just directly compare centipawns with integer
                    // comparision
                    Estimate(other_centipawns) => {
                        self_centipawns.cmp(other_centipawns)
                    }

                    // Any evaluation better than 0 in centipawns is better than
                    // a certain draw. We also consider an evaluation of 0
                    // centipawns to be better than a draw (fight for the win!)
                    Certain(Draw, _) => {
                        if *self_centipawns >= 0 {
                            Greater
                        } else {
                            Less
                        }
                    }

                    // Black in checkmate is better than any centipawn
                    // evaluation for white. White in checkmate is worse than
                    // any centipawn evaluation for white.
                    Certain(Checkmate(side_in_checkmate), _) => {
                        match *side_in_checkmate {
                            Side::Black => Less,
                            Side::White => Greater,
                        }
                    }
                }
            }

            Certain(Draw, self_plies) => {
                match other {
                    // For comparing draws, the best we can do is figure out
                    // which draw happens faster. If you're trying to force a
                    // draw, then you want it to happen faster. If you're trying
                    // to not force a draw, you want it to happen slower.
                    //
                    // In this case, we don't know what White wants to do. We
                    // assume that if the choice is just between draws, White is
                    // just trying to force a draw. In that case, we try to end
                    // the game as fast as possible.
                    Certain(Draw, other_plies) => {
                        self_plies.cmp(other_plies).reverse()
                    }

                    // It doesn't matter how long it will take, white being in
                    // checkmate is worse for white than a draw.
                    Certain(Checkmate(White), _) => Greater,

                    // Similarly, black being in checkmate is always better
                    Certain(Checkmate(Black), _) => Less,

                    // We can just implement this as an inline, reversed call to
                    // self. There shouldn't be a performance penalty because
                    // of everything being inlined.
                    Estimate(_) => other.cmp(self).reverse(),
                }
            }
            Certain(Checkmate(self_side_in_mate), self_plies) => {
                match other {
                    Certain(Checkmate(other_side_in_mate), other_plies) => {
                        match (*self_side_in_mate, *other_side_in_mate) {
                            // Note, we have to be explicit about sides enums
                            // here, because rustc thinks they're just variables
                            // otherwise

                            // If white will get checkmated anyway, it'll prefer
                            // the position where it takes longer
                            (Side::White, Side::White) => {
                                self_plies.cmp(other_plies)
                            }

                            // Black being in checkmate is always better for
                            // White
                            (Side::White, Side::Black) => Less,

                            // Black being in checkmate is always better for
                            // White
                            (Side::Black, Side::White) => Greater,

                            // If Black will get checkmated anyway, White will
                            // prefer the position where it takes fewer moves
                            (Side::Black, Side::Black) => {
                                other_plies.cmp(self_plies)
                            }
                        }
                    }

                    // We can just implement this as an inline, reversed call to
                    // self. There shouldn't be a performance penalty because
                    // of everything being inlined.
                    Certain(Draw, _) | Estimate(_) => other.cmp(self).reverse(),
                }
            }
        }
    }
}

impl PartialOrd for Evaluation {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
