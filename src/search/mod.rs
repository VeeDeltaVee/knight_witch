pub mod minimax;
use crate::{
    board::{chess_move::ChessMove, Board},
    evaluation::Evaluation,
};

/// A searcher is a type that can look through the move tree and figure out a
/// "good" move for the current position. It may maintain a cache internally of
/// move evaluations, and so shouldn't be discarded cheaply.
pub trait Searcher {
    /// Get the best move in the position
    /// Has a default implementation that returns the top result of `search_order`
    fn search(
        &self,
        board: &Board,
    ) -> Result<(ChessMove, Evaluation), &'static str> {
        let possible_moves = self.search_order(board)?;

        possible_moves.first().ok_or("No moves possible").cloned()
    }

    /// Returns a list of moves ranked from best to worst
    /// along with a centipawn evaluation of each move
    fn search_order(
        &self,
        board: &Board,
    ) -> Result<Vec<(ChessMove, Evaluation)>, &'static str>;
}
