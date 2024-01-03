use crate::board::Board;

use super::{
    chess_move::ChessMove, straight_moving_piece::StraightMovingPieceMovement,
    Offset, PieceType,
};

pub trait RookMovement: StraightMovingPieceMovement {
    fn generate_rook_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str>;
}

pub const ROOK_OFFSETS: [Offset; 4] = [
    Offset { rank: 0, file: 1 },
    Offset { rank: 1, file: 0 },
    Offset { rank: 0, file: -1 },
    Offset { rank: -1, file: 0 },
];

impl RookMovement for Board {
    fn generate_rook_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str> {
        self.generate_straight_moves(&ROOK_OFFSETS, PieceType::Rook, checked)
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
    fn moves_orthogonally() {
        let board = get_board_for_simple_straight_moves(PieceType::Rook);

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
        ];
        let unexpected_moves = vec![
            Square { rank: 2, file: 5 },
            Square { rank: 2, file: 6 },
            Square { rank: 0, file: 2 },
            Square { rank: 7, file: 2 },
            Square { rank: 1, file: 3 },
            Square { rank: 1, file: 5 },
            Square { rank: 3, file: 3 },
            Square { rank: 3, file: 5 },
            Square { rank: 5, file: 3 },
            Square { rank: 5, file: 1 },
            Square { rank: 3, file: 1 },
        ];

        check_for_moves(
            moved_boards,
            expected_moves,
            unexpected_moves,
            Piece::new(Side::White, PieceType::Rook),
        );
    }
}
