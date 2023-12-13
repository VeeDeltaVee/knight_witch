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
        Side, square::Square,
    };

    use super::*;

    #[test]
    fn moves_both_orthogonally_and_diagonally() {
        let board = get_board_for_simple_straight_moves(PieceType::Queen);

        let moved_boards = board.generate_moves(true).unwrap();

        let expected_moves = vec![
            Square::new(2,3),
            Square::new(2,2),
            Square::new(2,1),
            Square::new(2,0),
            Square::new(1,4),
            Square::new(0,4),
            Square::new(3,4),
            Square::new(4,4),
            Square::new(5,4),
            Square::new(6,4),
            Square::new(4,1),
            Square::new(4,0),
            Square::new(4,3),
            Square::new(4,4),
            Square::new(4,5),
            Square::new(4,6),
            Square::new(4,7),
            Square::new(3,2),
            Square::new(2,2),
            Square::new(1,2),
            Square::new(5,2),
            Square::new(6,2),
            Square::new(1,3),
            Square::new(0,2),
            Square::new(1,5),
            Square::new(0,6),
            Square::new(3,3),
            Square::new(3,5),
            Square::new(4,6),
            Square::new(5,3),
            Square::new(6,4),
            Square::new(5,1),
            Square::new(6,0),
        ];

        check_for_moves(
            moved_boards,
            expected_moves,
            vec![],
            Some((PieceType::Queen, Side::White)),
        );
    }
}
