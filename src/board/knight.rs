use crate::board::Board;

use super::{Direction, PieceType, Side, Square};

pub trait KnightMovement {
    fn generate_knight_moves(&self) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl KnightMovement for Board {
    fn generate_knight_moves(&self) -> Result<Vec<Board>, &'static str> {
        let jumps = vec![
            (-1, 2),
            (1, 2),
            (-2, 1),
            (2, 1),
            (-2, -1),
            (2, -1),
            (-1, -2),
            (1, -2),
        ];

        let knight_positions =
            self.get_positions_of_pieces_with_given_side_and_type(PieceType::Knight, Side::White)?;

        let mut possible_boards = vec![];
        for old_pos in knight_positions {
            let new_boards = jumps
                .iter()
                .map(|(file, rank)| Direction {
                    file: *file,
                    rank: *rank,
                })
                // Get target square and check for out-of-bounds moves
                .filter_map(|dir| self.add_direction_to_position(old_pos, dir).ok())
                // Check target square: can't take own pieces
                .filter(
                    |new_pos| match self.get_piece_at_position(*new_pos).unwrap() {
                        None => true,
                        Some((_, Side::Black)) => true,
                        Some((_, Side::White)) => false,
                    },
                )
                // Should be able to move there without error
                .filter_map(|new_pos| self.new_board_with_moved_piece(old_pos, new_pos).ok());

            possible_boards.extend(new_boards);
        }

        Ok(possible_boards)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::test_utils::check_for_moves;

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
                Some((PieceType::Knight, Side::White)),
                Square { rank: 1, file: 3 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Some((PieceType::Knight, Side::White)),
                Square { rank: 3, file: 1 },
            )
            .unwrap();

        board
            .set_piece_at_position(
                Some((PieceType::Pawn, Side::White)),
                Square { rank: 2, file: 3 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Some((PieceType::Pawn, Side::White)),
                Square { rank: 0, file: 5 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Some((PieceType::Pawn, Side::White)),
                Square { rank: 5, file: 0 },
            )
            .unwrap();

        board
            .set_piece_at_position(
                Some((PieceType::Pawn, Side::Black)),
                Square { rank: 2, file: 5 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Some((PieceType::Pawn, Side::Black)),
                Square { rank: 3, file: 4 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Some((PieceType::Pawn, Side::Black)),
                Square { rank: 4, file: 3 },
            )
            .unwrap();

        board
    }

    #[test]
    fn moves_like_a_knight() {
        let board = get_board_for_simple_knight_moves();

        let moved_boards = board.generate_moves().unwrap();

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
            Some((PieceType::Knight, Side::White)),
        );
    }
}
