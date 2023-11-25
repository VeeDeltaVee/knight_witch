use crate::board::Board;

use super::{Direction, PieceType};

pub trait StraightMovingPieceMovement {
    fn generate_straight_moves(&self, directions: &[Direction], piece_type: PieceType, checked: bool) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl StraightMovingPieceMovement for Board {
    fn generate_straight_moves(&self, directions: &[Direction], piece_type: PieceType, checked: bool) -> Result<Vec<Board>, &'static str> {
        let positions = self
            .get_positions_of_pieces_with_given_side_and_type(piece_type, self.current_move)?;

        let moves = positions
            .into_iter()
            .map(|pos| {
                directions
                    .iter()
                    .map(move |dir| (dir, self.check_ray_for_pieces(pos, *dir, true)))
                    .filter_map(move |(dir, extent)| self.get_all_squares_between(pos, extent, *dir).ok())
                    .flatten()
                    .map(move |new| (pos, new))
            })
            .flatten()
            .filter_map(|(old, new)| {
                self.new_board_with_moved_piece(old, new, checked).ok()
            })
            .collect();

        Ok(moves)
    }
}
