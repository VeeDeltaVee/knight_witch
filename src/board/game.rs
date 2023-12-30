use std::cmp::Ordering;

use super::piece::Side;

/// Enum that contains the result of a chess game.
///
/// A game can end in a draw (stalemate or insufficient material), or it can end
/// in checkmate for one side or the other.
#[derive(Debug, PartialEq, Eq)]
pub enum ChessResult {
    Draw,
    Checkmate(Side),
}

/// An ordering for `ChessResult`, which ranks results based on how good they
/// are for `White`.
///
/// Can be fed into a larger comparision for evaluations etc, because generally
/// `White `being better is evaluated as positive, and `Black` being better is
/// evaluated as negative
impl Ord for ChessResult {
    fn cmp(&self, other: &Self) -> Ordering {
        use Ordering::{Equal, Greater, Less};

        match self {
            ChessResult::Draw => match other {
                ChessResult::Draw => Equal,
                ChessResult::Checkmate(side_in_mate) => match *side_in_mate {
                    Side::White => Greater,
                    Side::Black => Less,
                },
            },
            ChessResult::Checkmate(self_side_in_mate) => match other {
                ChessResult::Draw => match self_side_in_mate {
                    Side::White => Less,
                    Side::Black => Greater,
                },
                ChessResult::Checkmate(other_side_in_mate) => {
                    if self_side_in_mate == other_side_in_mate {
                        Equal
                    } else if *self_side_in_mate == Side::White {
                        Less
                    } else {
                        Greater
                    }
                }
            },
        }
    }
}

/// ChessResult is also `PartialOrd` because it is `Ord`.
///
/// This can never return `None`, and can be safely unwrapped
impl PartialOrd for ChessResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ChessResult::*;
    use Side::*;

    #[test]
    fn draws_are_equal() {
        assert_eq!(Draw, Draw);
    }

    #[test]
    fn checkmates_same_side_are_equal() {
        assert_eq!(Checkmate(White), Checkmate(White));
        assert_eq!(Checkmate(Black), Checkmate(Black));
    }

    #[test]
    fn black_in_mate_is_better_for_white_than_white_in_mate() {
        assert!(Checkmate(Black) > Checkmate(White));
        assert!(Checkmate(White) < Checkmate(Black));
    }

    #[test]
    fn draws_are_better_for_white_than_white_in_mate() {
        assert!(Draw > Checkmate(White));
        assert!(Checkmate(White) < Draw);
    }
    #[test]
    fn draws_are_worse_for_white_than_black_in_mate() {
        assert!(Draw < Checkmate(Black));
        assert!(Checkmate(Black) > Draw);
    }
}
