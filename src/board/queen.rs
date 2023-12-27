use crate::board::Board;

use super::{
    chess_move::ChessMove, straight_moving_piece::StraightMovingPieceMovement,
    Offset, PieceType,
};

pub trait QueenMovement: StraightMovingPieceMovement {
    fn generate_queen_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str>;
}

impl QueenMovement for Board {
    fn generate_queen_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str> {
        let offsets: Vec<Offset> = [
            (0, 1),
            (1, 0),
            (0, -1),
            (-1, 0),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ]
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
        Piece, Side, Square,
    };

    use super::*;

    #[test]
    fn moves_both_orthogonally_and_diagonally() {
        let board = get_board_for_simple_straight_moves(PieceType::Queen);

        let moved_boards = board.generate_moved_boards(true).unwrap();

        let expected_moves = vec![
            Square { rank: 2, file: 3 },
            Square { rank: 2, file: 2 },
            Square { rank: 2, file: 1 },
            Square { rank: 2, file: 0 },
            Square { rank: 1, file: 4 },
            Square { rank: 0, file: 4 },
            Square { rank: 3, file: 4 },
            Square { rank: 4, file: 4 },
            Square { rank: 5, file: 4 },
            Square { rank: 6, file: 4 },
            Square { rank: 4, file: 1 },
            Square { rank: 4, file: 0 },
            Square { rank: 4, file: 3 },
            Square { rank: 4, file: 4 },
            Square { rank: 4, file: 5 },
            Square { rank: 4, file: 6 },
            Square { rank: 4, file: 7 },
            Square { rank: 3, file: 2 },
            Square { rank: 2, file: 2 },
            Square { rank: 1, file: 2 },
            Square { rank: 5, file: 2 },
            Square { rank: 6, file: 2 },
            Square { rank: 1, file: 3 },
            Square { rank: 0, file: 2 },
            Square { rank: 1, file: 5 },
            Square { rank: 0, file: 6 },
            Square { rank: 3, file: 3 },
            Square { rank: 3, file: 5 },
            Square { rank: 4, file: 6 },
            Square { rank: 5, file: 3 },
            Square { rank: 6, file: 4 },
            Square { rank: 5, file: 1 },
            Square { rank: 6, file: 0 },
        ];

        check_for_moves(
            moved_boards,
            expected_moves,
            vec![],
            Piece::new(Side::White, PieceType::Queen),
        );
    }
}
