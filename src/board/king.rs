use crate::board::Board;

use super::{Offset, Piece, PieceType::King};

pub trait KingMovement {
    fn generate_king_moves(&self, checked: bool) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl KingMovement for Board {
    fn generate_king_moves(&self, checked: bool) -> Result<Vec<Self>, &'static str> {
        let offsets = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)]
            .iter()
            .map(|(x, y)| Offset { rank: *y, file: *x });

        let positions = self.get_positions_of_matching_pieces(Piece::new(self.current_move, King))?;

        let moves = positions
            .into_iter()
            .map(|pos| {
                offsets
                    .clone()
                    .filter_map(move |off| self.add_offset_to_position(pos, off).ok())
                    .map(move |new| (pos, new))
            })
            .flatten()
            .filter_map(|(old, new)| {
                let mut new_board = self.clone();
                new_board.make_move(old, new, checked).ok()?;
                Some(new_board)
            })
            .collect();

        Ok(moves)
    }
}


#[cfg(test)]
mod test {
    use crate::board::{
        Side, Square,
    };

    use super::*;

    fn get_board_for_simple_king_moves() -> Board {
        let mut pieces = vec![None.into(); 9];
        pieces[4] = Piece::new(Side::White, King).into();

        Board::with_pieces(pieces, 3)
    }

    #[test]
    fn moves_one_step_nearby() {
        let board = get_board_for_simple_king_moves();

        let moved_boards = board.generate_moves(true).unwrap();

        // every place other than the centre should have a king move
        for rank in 0..2 {
            for file in 0..2 {
                if (rank, file) != (1, 1) {
                    assert!(moved_boards.iter().any(|x| matches!(
                        x.get_piece_at_position(Square {
                            rank: rank,
                            file: file
                        })
                        .unwrap(),
                        Some(Piece { piece_type: King, .. })
                    ) && matches!(
                        x.get_piece_at_position(Square { rank: 1, file: 1 })
                            .unwrap(),
                        None
                    )));
                }
            }
        }
    }
}
