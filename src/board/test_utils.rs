use crate::board::Board;

use super::{Piece, Square};

pub fn check_for_moves(
    boards: Vec<Board>,
    expected_moves: Vec<Square>,
    unexpected_moves: Vec<Square>,
    piece: Piece,
) {
    for square in expected_moves {
        assert!(
            boards
                .clone()
                .into_iter()
                .any(|x| x.get_piece_at_position(square).unwrap() == piece),
            "Didn't find {:?} move at rank {}, file {}",
            piece.unwrap().0,
            square.rank,
            square.file
        );
    }

    for square in unexpected_moves {
        assert!(
            !boards
                .clone()
                .into_iter()
                .any(|x| x.get_piece_at_position(square).unwrap() == piece),
            "Found unexpected {:?} move at rank {}, file {}",
            piece.unwrap().0,
            square.rank,
            square.file
        );
    }
}
