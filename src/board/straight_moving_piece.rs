use crate::board::Board;

use super::{chess_move::ChessMove, Offset, Piece, PieceType};

pub trait StraightMovingPieceMovement {
    fn generate_straight_moves(
        &self,
        offsets: &[Offset],
        piece_type: PieceType,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str>;
}

impl StraightMovingPieceMovement for Board {
    fn generate_straight_moves(
        &self,
        offsets: &[Offset],
        piece_type: PieceType,
        checked: bool,
    ) -> Result<Vec<ChessMove>, &'static str> {
        let positions = self.get_positions_of_matching_pieces(Piece::new(
            self.current_move,
            piece_type,
        ))?;

        let moves = positions
            .into_iter()
            .flat_map(|pos| {
                offsets
                    .iter()
                    .map(move |dir| {
                        (dir, self.check_ray_for_pieces(pos, *dir, true))
                    })
                    .filter_map(move |(dir, extent)| {
                        self.get_all_squares_between(pos, extent, *dir).ok()
                    })
                    .flatten()
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
