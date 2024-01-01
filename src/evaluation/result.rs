use crate::board::Board;

use super::{
    Evaluation::{self, *},
    Evaluator,
};

/// ResultEvaluator is another really simple evaluator. It just figures out if
/// the game is over, and if it is, it returns that result as a `Certain`
/// result. Otherwise, it returns a zero `Centipawn` result
pub struct ResultEvaluator {}

impl ResultEvaluator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        ResultEvaluator {}
    }
}

impl Evaluator for ResultEvaluator {
    fn evaluate(&self, board: &Board) -> Result<Evaluation, &'static str> {
        let evaluation = match board.get_game_result()? {
            Some(result) => Certain(result, 0),
            None => Estimate(0),
        };

        Ok(evaluation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::game::ChessResult::Checkmate;
    use crate::board::piece::Side::*;
    use crate::evaluation::Evaluation::Certain;
    use crate::test_board_evaluation;

    test_board_evaluation!(
        default_is_equal,
        ResultEvaluator::new(),
        Board::default(),
        0
    );

    test_board_evaluation!(
        white_checkmate_is_certain,
        ResultEvaluator::new(),
        "...K...r\n\
         ........\n\
         ...k....\n\
         ........\n\
         ........\n\
         ........\n\
         ........\n",
        Certain(Checkmate(White), 0)
    );

    test_board_evaluation!(
        black_checkmate_is_certain,
        ResultEvaluator::new(),
        Board::from_art(
            "...k...R\n\
             ........\n\
             ...K....\n\
             ........\n\
             ........\n\
             ........\n\
             ........\n"
        )
        .unwrap()
        .flip_current_side(),
        Certain(Checkmate(Black), 0)
    );
}
