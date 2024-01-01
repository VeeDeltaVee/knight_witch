pub mod composite;
pub mod evaluation_result;
pub mod material;
pub mod result;

mod test_utils;

use crate::board::Board;

use self::evaluation_result::Evaluation;

/// An evaluator takes a board as an input, and returns it's value in centipawns
/// with a positive value favouring white.
///
/// Implementations of evaluator could be complex, and take time to initialize,
/// so the same evaluator should be persisted as long as possible.
pub trait Evaluator {
    fn evaluate(&self, board: &Board) -> Result<Evaluation, &'static str>;
}
