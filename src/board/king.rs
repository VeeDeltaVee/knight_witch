use crate::board::Board;

use super::{chess_move::ChessMove, Offset, Piece, PieceType::King};

pub trait KingMovement {
    fn generate_king_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str>;
}

pub const KING_OFFSETS: [Offset; 8] = [
    Offset { rank: 0, file: 1 },
    Offset { rank: 1, file: 0 },
    Offset { rank: 0, file: -1 },
    Offset { rank: -1, file: 0 },
    Offset { rank: 1, file: 1 },
    Offset { rank: 1, file: -1 },
    Offset { rank: -1, file: 1 },
    Offset { rank: -1, file: -1 },
];

impl KingMovement for Board {
    fn generate_king_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str> {
        let positions = self.get_positions_of_matching_pieces(Piece::new(
            self.current_move,
            King,
        ))?;

        let moves = positions
            .into_iter()
            .flat_map(|pos| {
                KING_OFFSETS
                    .iter()
                    .filter_map(move |off| {
                        self.add_offset_to_position(pos, *off).ok()
                    })
                    .map(move |new| (pos, new))
            })
            .filter_map(|(old, new)| {
                // TODO: for now, we keep attempting to create a board
                // temporarily, to use make_move as validation. This really
                // should be removed, and we should just have a "validate_move"
                // method
                self.new_board_with_moved_piece(old, new, checked).ok()?;

                // If the last statement didn't short circuit and exit with an
                // error, we know that the move is valid
                ChessMove::SimpleMove(old, new).into()
            })
            .collect();

        Ok(moves)
    }
}

#[cfg(test)]
mod test {
    use crate::board::{Side, Square};

    use super::*;

    fn get_board_for_simple_king_moves() -> Board {
        let mut pieces = vec![None; 9];
        pieces[4] = Piece::new(Side::White, King).into();

        Board::with_pieces(pieces, 3)
    }

    #[test]
    fn moves_one_step_nearby() {
        let board = get_board_for_simple_king_moves();

        let moved_boards = board.generate_moved_boards(true).unwrap();

        // every place other than the centre should have a king move
        for rank in 0..2 {
            for file in 0..2 {
                if (rank, file) != (1, 1) {
                    assert!(moved_boards.iter().any(|x| matches!(
                        x.get_piece_at_position(Square { rank, file }).unwrap(),
                        Some(Piece {
                            piece_type: King,
                            ..
                        })
                    ) && x
                        .get_piece_at_position(Square { rank: 1, file: 1 })
                        .unwrap()
                        .is_none()));
                }
            }
        }
    }
}
