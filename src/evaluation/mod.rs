pub mod material;

use crate::board::Board;

/// An evaluator takes a board as an input, and returns it's value in centipawns
/// with a positive value favouring white.
///
/// Implementations of evaluator could be complex, and take time to initialize,
/// so the same evaluator should be persisted as long as possible.
pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> Centipawns;
}

pub type Centipawns = i32;
