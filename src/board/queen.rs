use crate::board::Board;

use super::{Offset, PieceType, straight_moving_piece::StraightMovingPieceMovement};

pub trait QueenMovement: StraightMovingPieceMovement {
    fn generate_queen_moves(&self, checked: bool) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl QueenMovement for Board {
    fn generate_queen_moves(&self, checked: bool) -> Result<Vec<Board>, &'static str> {
        let offsets: Vec<Offset> = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]
            .iter()
            .map(|(x, y)| Offset { rank: *y, file: *x })
            .collect();

        self.generate_straight_moves(&offsets, PieceType::Queen, checked)
    }
}

#[cfg(test)]
mod test {
    use crate::board::{
        test_utils::{check_for_moves, get_board_for_simple_straight_moves},
        Side, square::UncheckedSquare,
    };

    use super::*;

    #[test]
    fn moves_both_orthogonally_and_diagonally() {
        let board = get_board_for_simple_straight_moves(PieceType::Queen);

        let moved_boards = board.generate_moves(true).unwrap();

        let expected_moves = vec![
            UncheckedSquare { rank: 2, file: 3 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 2, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 2, file: 1 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 2, file: 0 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 1, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 0, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 3, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 5, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 6, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 1 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 0 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 3 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 5 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 6 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 7 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 3, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 2, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 1, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 5, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 6, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 1, file: 3 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 0, file: 2 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 1, file: 5 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 0, file: 6 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 3, file: 3 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 3, file: 5 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 4, file: 6 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 5, file: 3 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 6, file: 4 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 5, file: 1 }.validate(&board).unwrap(),
            UncheckedSquare { rank: 6, file: 0 }.validate(&board).unwrap(),
        ];

        check_for_moves(
            moved_boards,
            expected_moves,
            vec![],
            Some((PieceType::Queen, Side::White)),
        );
    }
}
