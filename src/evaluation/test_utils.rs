use crate::board::Board;

use super::{evaluation_result::Evaluation, Evaluator};

#[macro_export]
macro_rules! assert_board_evaluation {
    ($board:expr, $evaluator:expr, $expected_evaluation:expr) => {
        use $crate::evaluation::Evaluator;

        assert_eq!($expected_evaluation, $evaluator.evaluate(&$board).unwrap());
    };
}

#[macro_export]
macro_rules! test_board_evaluation {
    ($test_name:ident, $evaluator:expr, $board:expr, $expected_evaluation:literal) => {
        test_board_evaluation!(
            impl $test_name,
            $evaluator,
            $board,
            $crate::evaluation::Evaluation::Estimate($expected_evaluation)
        );
    };
    ($test_name:ident, $evaluator:expr, $board_art:literal, $expected_evaluation:literal) => {
        test_board_evaluation!(
            impl $test_name,
            $evaluator,
            $board_art,
            $crate::evaluation::Evaluator::Estimate($expected_evaluation)
        );
    };
    ($test_name:ident, $evaluator:expr, $board:expr, $expected_evaluation:expr) => {
        test_board_evaluation!(
            impl $test_name,
            $evaluator,
            $board,
            $expected_evaluation
        );
    };
    ($test_name:ident, $evaluator:expr, $board_art:literal, $expected_evaluation:expr) => {
        test_board_evaluation!(
            impl $test_name,
            $evaluator,
            $board_art,
            $expected_evaluation
        );
    };

    (impl $test_name:ident, $evaluator:expr, $board_art:literal, $expected_evaluation:expr) => {
        #[test]
        fn $test_name() {
            let board = $crate::board::Board::from_art($board_art).unwrap();

            $crate::assert_board_evaluation!(
                board,
                $evaluator,
                $expected_evaluation
            );
        }
    };

    (impl $test_name:ident, $evaluator:expr, $board:expr, $expected_evaluation:expr) => {
        #[test]
        fn $test_name() {
            $crate::assert_board_evaluation!(
                $board,
                $evaluator,
                $expected_evaluation
            );
        }
    };
}

pub struct DummyEvaluator {
    pub result: Evaluation,
}

impl Evaluator for DummyEvaluator {
    fn evaluate(&self, _: &Board) -> Result<Evaluation, &'static str> {
        Ok(self.result)
    }
}
