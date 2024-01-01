use crate::board::Board;

use super::evaluation_result::Evaluation;
use super::Evaluator;

pub struct CompositeEvaluator {
    children: Vec<Box<dyn Evaluator>>,
}

impl CompositeEvaluator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        CompositeEvaluator { children: vec![] }
    }

    #[allow(dead_code)]
    pub fn push(&mut self, e: Box<dyn Evaluator>) -> &mut Self {
        self.children.push(e);
        self
    }
}

impl Evaluator for CompositeEvaluator {
    fn evaluate(&self, board: &Board) -> Result<Evaluation, &'static str> {
        self.children.iter().map(|e| e.evaluate(board)).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::game::ChessResult,
        evaluation::{
            evaluation_result::Evaluation, test_utils::DummyEvaluator,
        },
    };

    use super::*;

    #[test]
    fn sum_of_estimates_when_not_certain() {
        let mut evaluator = CompositeEvaluator::new();
        evaluator
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Estimate(30),
            }))
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Estimate(-20),
            }))
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Estimate(13),
            }));

        assert_eq!(
            evaluator.evaluate(&Board::default()).unwrap(),
            Evaluation::Estimate(23)
        );
    }

    #[test]
    fn certain_when_even_one_is_certain() {
        let mut evaluator = CompositeEvaluator::new();
        evaluator
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Estimate(30),
            }))
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Estimate(-20),
            }))
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Certain(ChessResult::Draw, 3),
            }))
            .push(Box::new(DummyEvaluator {
                result: Evaluation::Estimate(13),
            }));

        assert_eq!(
            evaluator.evaluate(&Board::default()).unwrap(),
            Evaluation::Certain(ChessResult::Draw, 3)
        );
    }

    #[test]
    fn empty_estimate() {
        let evaluator = CompositeEvaluator::new();

        assert_eq!(
            evaluator.evaluate(&Board::default()).unwrap(),
            Evaluation::Estimate(0)
        );
    }
}
