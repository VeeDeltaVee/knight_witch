pub mod material;

use std::cmp::Ordering;

use crate::board::{Board, piece::Side};

/// An evaluator takes a board as an input, and returns it's value in centipawns
/// with a positive value favouring white.
///
/// Implementations of evaluator could be complex, and take time to initialize,
/// so the same evaluator should be persisted as long as possible.
pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> Evaluation;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Evaluation {
    /// An evaluation where the outcome isn't certain. Centipawns is an estimate
    /// of how much better/worse a position is in terms of 1/100ths of a pawn.
    Centipawns(i32),

    /// A certain game result, be it
    Certain(u32),
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
impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Evaluation::Centipawns(self_centipawns) => {
                match other {
                    // We can just directly compare centipawns with integer
                    // comparision
                    Evaluation::Centipawns(other_centipawns) =>
                        self_centipawns.partial_cmp(other_centipawns),

                    // Any evaluation better than 0 in centipawns is better than
                    // a certain draw. We also consider an evaluation of 0
                    // centipawns to be better than a draw (fight for the win!)
                    Evaluation::Draw(_) =>
                        if *self_centipawns >= 0 {
                            Some(Ordering::Greater)
                        } else {
                            Some(Ordering::Less)
                        },

                    // Black in checkmate is better than any centipawn
                    // evaluation for white. White in checkmate is worse than
                    // any centipawn evaluation for white.
                    Evaluation::Checkmate(_, side_in_checkmate) =>
                        if *side_in_checkmate == Side::Black {
                            Some(Ordering::Less)
                        } else {
                            Some(Ordering::Greater)
                        }
                }
            },

            Evaluation::Draw(self_plies) => {
                match other {
                    Evaluation::Centipawns(ocp) => todo!(),
                    Evaluation::Draw(other_plies) => todo!(),
                    Evaluation::Checkmate(_, side_in_checkmate) => todo!(),
                }
            },
            Evaluation::Checkmate(_, _) => todo!(),
        }
    }
}
