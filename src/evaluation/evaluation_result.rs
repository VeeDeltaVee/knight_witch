use std::{cmp::Ordering, iter::Sum, ops::Add};

use crate::board::{game::ChessResult, piece::Side};

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

impl Evaluation {
    /// Returns a deepened evaluation, i.e. if it's certain increases the depth
    /// by one
    pub fn deepen(self) -> Self {
        if let Evaluation::Certain(r, d) = self {
            Evaluation::Certain(r, d + 1)
        } else {
            self
        }
    }

    pub const BEST_FOR_BLACK: Self =
        Evaluation::Certain(ChessResult::Checkmate(Side::White), 0);
    pub const BEST_FOR_WHITE: Self =
        Evaluation::Certain(ChessResult::Checkmate(Side::Black), 0);
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
        use crate::board::piece::Side::*;
        use ChessResult::*;
        use Evaluation::*;
        use Ordering::*;

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
                            Black => Less,
                            White => Greater,
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
                            (White, White) => self_plies.cmp(other_plies),

                            // Black being in checkmate is always better for
                            // White
                            (White, Black) => Less,

                            // Black being in checkmate is always better for
                            // White
                            (Black, White) => Greater,

                            // If Black will get checkmated anyway, White will
                            // prefer the position where it takes fewer moves
                            (Black, Black) => other_plies.cmp(self_plies),
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

impl Add for Evaluation {
    type Output = Evaluation;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        use Evaluation::*;
        match self {
            Estimate(self_cp) => match rhs {
                // We can just add estimates directly...
                Estimate(other_cp) => Estimate(self_cp + other_cp),

                // ... but for certain results, we can ignore estimates.
                // If an evaluator returns a certain result, and other evaluators
                // try to add estiamtes to it, those are meaningless. The certain
                // result will be no less certain
                Certain(_, _) => rhs,
            },
            Certain(_, self_depth) => match rhs {
                // This is just an inline call. It should resolve to the certain/
                // estimate comparator above.
                Estimate(_) => rhs.add(self),

                // If two certain evaluations are added, the overall evaluation
                // is just the the result that is closer (smaller depth)
                //
                // If they're both the same depth, then pick the first one
                // arbitrarily
                Certain(_, other_depth) => match self_depth.cmp(&other_depth) {
                    Ordering::Less | Ordering::Equal => self,
                    Ordering::Greater => rhs,
                },
            },
        }
    }
}

impl Sum for Evaluation {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Evaluation::Estimate(0), |e1, e2| e1 + e2)
    }
}
