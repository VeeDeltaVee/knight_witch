use crate::board::Board;

use super::{Direction, PieceType};

pub trait RookMovement {
    fn generate_rook_moves(&self) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl RookMovement for Board {
    fn generate_rook_moves(&self) -> Result<Vec<Board>, &'static str> {
        let rook_positions = self
            .get_positions_of_pieces_with_given_side_and_type(PieceType::Rook, self.current_move)?;

        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .map(|(x, y)| Direction { rank: *y, file: *x });

        let moves = rook_positions
            .into_iter()
            .map(|pos| {
                directions
                    .clone()
                    .map(move |dir| (dir, self.check_ray_for_pieces(pos, dir, true)))
                    .map(move |(dir, extent)| self.get_all_squares_between(pos, extent, dir))
                    .flatten()
                    .flatten()
                    .map(move |new| (pos, new))
            })
            .flatten()
            .filter_map(|(old, new)| {
                let mut new_board = self.clone();
                new_board.make_move(old, new).ok()?;
                Some(new_board)
            })
            .collect();

        Ok(moves)
    }
}

#[cfg(test)]
mod test {
    use crate::board::{
        test_utils::{check_for_moves, get_board_for_simple_straight_moves},
        Side, Square,
    };

    use super::*;

    #[test]
    fn moves_orthogonally() {
        let board = get_board_for_simple_straight_moves(PieceType::Rook);

        let moved_boards = board.generate_moves().unwrap();

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
            Square { rank: 6, file: 4 },
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
            Some((PieceType::Rook, Side::White)),
        );
    }
}
