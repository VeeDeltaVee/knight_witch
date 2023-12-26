use crate::board::{
    piece::{Piece, PieceType, Side},
    Board,
};

use super::{Centipawns, Evaluator};

/// MaterialEvaluator is the simplest evaluator that's still somewhat useful:
/// it just evaluates chess positions by material. It would make for a very
/// stereotypically materialistic engine.
struct MaterialEvaluator {}

impl MaterialEvaluator {
    #[allow(dead_code)]
    fn new() -> Self {
        MaterialEvaluator {}
    }
}

impl Evaluator for MaterialEvaluator {
    fn evaluate(&self, board: &Board) -> Centipawns {
        board
            .get_squares()
            .iter()
            .filter_map(|&op| get_material_value(op?).into())
            .sum::<Centipawns>()
    }
}

fn get_material_value(piece: Piece) -> Centipawns {
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
    use super::*;

    #[test]
    fn default_board_equal() {
        let board = Board::default();
        let evaluator = MaterialEvaluator::new();

        assert_eq!(evaluator.evaluate(&board), 0);
    }

    #[test]
    fn one_sided_black() {
        let board = Board::from_art(
            "........\n\
             ........\n\
             ........\n.\
             ........\n\
             ........\n\
             ........\n\
             pppppppp\n\
             rnbqkbnr",
        )
        .unwrap();
        let evaluator = MaterialEvaluator::new();

        assert_eq!(-13900, evaluator.evaluate(&board))
    }

    #[test]
    fn one_sided_white() {
        let board = Board::from_art(
            "
             RNBQKBNR\n\"
             PPPPPPPP\n\
             ........\n\
             ........\n.\
             ........\n\
             ........\n\
             ........\n\
             ........",
        )
        .unwrap();
        let evaluator = MaterialEvaluator::new();

        assert_eq!(13900, evaluator.evaluate(&board))
    }
}
