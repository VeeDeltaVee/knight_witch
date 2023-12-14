#![cfg(test)]
use crate::board::Board;

use super::{Piece, PieceType, PieceType::*, Side::*, Square};

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
                .any(|x| x.get_piece_at_position(square).unwrap() == Some(piece)),
            "Didn't find {:?} move at rank {}, file {}",
            piece,
            square.rank,
            square.file
        );
    }

    for square in unexpected_moves {
        assert!(
            !boards
                .clone()
                .into_iter()
                .any(|x| x.get_piece_at_position(square).unwrap() == Some(piece)),
            "Found unexpected {:?} move at rank {}, file {}",
            piece,
            square.rank,
            square.file
        );
    }
}

// Returns a board with the setup
// ........
// ..p.p...
// .......P
// ..X.....
// .P......
// ....XP..
// ..p.....
// ........
// with X as a straight-moving piece (bishop, rook, or queen)
pub fn get_board_for_simple_straight_moves(piece_type: PieceType) -> Board {
    let mut board = Board::with_pieces(vec![None.into(); 8 * 8], 8);

    board
        .set_piece_at_position(Piece::new(White, piece_type).into(), Square { rank: 2, file: 4 })
        .unwrap();
    board
        .set_piece_at_position(Piece::new(White, piece_type).into(), Square { rank: 4, file: 2 })
        .unwrap();

    board
        .set_piece_at_position(
            Some(Piece::new(White, Pawn)),
            Square { rank: 2, file: 5 },
        )
        .unwrap();
    board
        .set_piece_at_position(
            Some(Piece::new(White, Pawn)),
            Square { rank: 3, file: 1 },
        )
        .unwrap();
    board
        .set_piece_at_position(
            Some(Piece::new(White, Pawn)),
            Square { rank: 5, file: 7 },
        )
        .unwrap();

    board
        .set_piece_at_position(
            Some(Piece::new(Black, Pawn)),
            Square { rank: 1, file: 2 },
        )
        .unwrap();
    board
        .set_piece_at_position(
            Some(Piece::new(Black, Pawn)),
            Square { rank: 6, file: 2 },
        )
        .unwrap();
    board
        .set_piece_at_position(
            Some(Piece::new(Black, Pawn)),
            Square { rank: 6, file: 4 },
        )
        .unwrap();

    board
}
