use crate::board::Board;

use super::{Piece, PieceType, Side, square::{Square, UncheckedSquare}};

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
            square.get_rank(),
            square.get_file()
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
            square.get_rank(),
            square.get_file()
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
    let mut board = Board::with_pieces(vec![None; 8 * 8], 8);

    board
        .set_piece_at_position(Some((piece_type, Side::White)), UncheckedSquare { rank: 2, file: 4 }.validate(&board).unwrap());
    board
        .set_piece_at_position(Some((piece_type, Side::White)), UncheckedSquare { rank: 4, file: 2 }.validate(&board).unwrap());

    board
        .set_piece_at_position(
            Some((PieceType::Pawn, Side::White)),
            UncheckedSquare { rank: 2, file: 5 }.validate(&board).unwrap(),
        );
    board
        .set_piece_at_position(
            Some((PieceType::Pawn, Side::White)),
            UncheckedSquare { rank: 3, file: 1 }.validate(&board).unwrap(),
        );
    board
        .set_piece_at_position(
            Some((PieceType::Pawn, Side::White)),
            UncheckedSquare { rank: 5, file: 7 }.validate(&board).unwrap(),
        );

    board
        .set_piece_at_position(
            Some((PieceType::Pawn, Side::Black)),
            UncheckedSquare { rank: 1, file: 2 }.validate(&board).unwrap(),
        );
    board
        .set_piece_at_position(
            Some((PieceType::Pawn, Side::Black)),
            UncheckedSquare { rank: 6, file: 2 }.validate(&board).unwrap(),
        );
    board
        .set_piece_at_position(
            Some((PieceType::Pawn, Side::Black)),
            UncheckedSquare { rank: 6, file: 4 }.validate(&board).unwrap(),
        );

    board
}
