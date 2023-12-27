use crate::board::Board;

use super::{chess_move::ChessMove, Offset, Piece, PieceType::*};

pub trait KnightMovement {
    fn generate_knight_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str>;
}

impl KnightMovement for Board {
    fn generate_knight_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str> {
        let jumps = [
            (-1, 2),
            (1, 2),
            (-2, 1),
            (2, 1),
            (-2, -1),
            (2, -1),
            (-1, -2),
            (1, -2),
        ];

        let knight_positions = self.get_positions_of_matching_pieces(
            Piece::new(self.current_move, Knight),
        )?;

        let mut possible_moves = vec![];
        for old_pos in knight_positions {
            let new_moves = jumps
                .iter()
                .map(|(file, rank)| Offset {
                    file: *file,
                    rank: *rank,
                })
                // Get target square and check for out-of-bounds moves
                .filter_map(|dir| {
                    self.add_offset_to_position(old_pos, dir).ok()
                })
                // Check target square: can't take own pieces
                .filter(|new_pos| {
                    match self.get_piece_at_position(*new_pos).unwrap() {
                        None => true,
                        Some(Piece { side: os, .. })
                            if os != self.current_move =>
                        {
                            true
                        }
                        _ => false,
                    }
                })
                // Should be able to move there without error
                .filter_map(|new_pos| {
                    // TODO: for now, we keep attempting to create a board
                    // temporarily, to use make_move as validation. This really
                    // should be removed, and we should just have a
                    // "validate_move" method
                    self.new_board_with_moved_piece(old_pos, new_pos, checked)
                        .ok()?;

                    // If the last statement didn't short circuit and exit with
                    // an error, we know that the move is valid
                    ChessMove::SimpleMove(old_pos, new_pos).into()
                });

            possible_moves.extend(new_moves);
        }

        Ok(possible_moves)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::test_utils::check_for_moves;
    use crate::board::Side::*;
    use crate::board::Square;

    // Returns a board with the setup
    // P.....
    // ...p..
    // .K..p.
    // ...P.p
    // ...K..
    // .....P
    fn get_board_for_simple_knight_moves() -> Board {
        let mut board = Board::with_pieces(vec![None; 6 * 6], 6);

        board
            .set_piece_at_position(
                Piece::new(White, Knight).into(),
                Square { rank: 1, file: 3 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(White, Knight).into(),
                Square { rank: 3, file: 1 },
            )
            .unwrap();

        board
            .set_piece_at_position(
                Piece::new(White, Pawn).into(),
                Square { rank: 2, file: 3 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(White, Pawn).into(),
                Square { rank: 0, file: 5 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(White, Pawn).into(),
                Square { rank: 5, file: 0 },
            )
            .unwrap();

        board
            .set_piece_at_position(
                Piece::new(Black, Pawn).into(),
                Square { rank: 2, file: 5 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(Black, Pawn).into(),
                Square { rank: 3, file: 4 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(Black, Pawn).into(),
                Square { rank: 4, file: 3 },
            )
            .unwrap();

        board
    }

    #[test]
    fn moves_like_a_knight() {
        let board = get_board_for_simple_knight_moves();

        let moved_boards = board.generate_moved_boards(true).unwrap();

        let expected_moves = vec![
            Square { rank: 5, file: 2 },
            Square { rank: 4, file: 3 },
            Square { rank: 1, file: 2 },
            Square { rank: 1, file: 0 },
            Square { rank: 3, file: 2 },
            Square { rank: 3, file: 4 },
            Square { rank: 2, file: 1 },
            Square { rank: 2, file: 5 },
            Square { rank: 0, file: 1 },
        ];
        let unexpected_moves = vec![
            Square { rank: 5, file: 0 },
            Square { rank: 0, file: 5 },
            Square { rank: 2, file: 3 },
        ];

        check_for_moves(
            moved_boards,
            expected_moves,
            unexpected_moves,
            Piece::new(White, Knight),
        );
    }
}
