use crate::board::{
    piece::{Piece, PieceType, Side},
    Board,
};

use super::{Evaluation, Evaluator};

/// MaterialEvaluator is the simplest evaluator that's still somewhat useful:
/// it just evaluates chess positions by material. It would make for a very
/// stereotypically materialistic engine.
pub struct MaterialEvaluator {}

impl MaterialEvaluator {
    #[allow(dead_code)]
    pub fn new() -> Self {
        MaterialEvaluator {}
    }
}

impl Evaluator for MaterialEvaluator {
    fn evaluate(&self, board: &Board) -> Result<Evaluation, &'static str> {
        let centipawns = board
            .get_squares()
            .iter()
            .filter_map(|&op| get_material_value(op?).into())
            .sum::<i32>();

        Ok(Evaluation::Estimate(centipawns))
    }
}

fn get_material_value(piece: Piece) -> i32 {
    let mut value = match piece.piece_type {
        // The king has an arbitrarily high value, since losing it would lose
        // the game
        PieceType::King => 10000,

        // Other piece values are conventional wisdom chess values
        PieceType::Pawn => 100,
        PieceType::Bishop | PieceType::Knight => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
    };

    if piece.side == Side::Black {
        value = -value;
    }

    value
}

#[cfg(test)]
mod test {
    use crate::test_board_evaluation;

    use super::MaterialEvaluator;
    use crate::board::Board;

    test_board_evaluation!(
        default_board_equal,
        MaterialEvaluator::new(),
        Board::default(),
        0
    );

    test_board_evaluation!(
        one_sided_black,
        MaterialEvaluator::new(),
        "........\n\
         ........\n\
         ........\n\
         ........\n\
         ........\n\
         ........\n\
         pppppppp\n\
         rnbqkbnr",
        -13900
    );

    test_board_evaluation!(
        one_sided_white,
        MaterialEvaluator::new(),
        "RNBQKBNR\n\
         PPPPPPPP\n\
         ........\n\
         ........\n\
         ........\n\
         ........\n\
         ........\n\
         ........",
        13900
    );
}
