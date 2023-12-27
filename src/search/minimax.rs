use crate::{
    board::{chess_move::ChessMove, Board},
    evaluation::{Centipawns, Evaluator},
};

use super::Searcher;

#[derive(Clone)]
pub struct MinimaxSearch<E> {
    depth: usize,
    evaluator: E,
}

impl<E> MinimaxSearch<E>
where
    E: Evaluator,
{
    pub fn new(evaluator: E) -> Self {
        MinimaxSearch {
            depth: 4,
            evaluator,
        }
    }
}

impl<E> Searcher for MinimaxSearch<E>
where
    E: Evaluator,
{
    fn search_order(
        &self,
        _board: &Board,
    ) -> Result<Vec<(ChessMove, Centipawns)>, &'static str> {
        todo!()
    }
}
