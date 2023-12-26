use crate::board::Board;

use super::{
    chess_move::ChessMove,
    Offset, Piece,
    PieceType::{self, *},
    Side::*,
    Square, UncheckedSquare,
};

pub trait PawnState {
    /// Figure out how `chess_move` affects en_passant_target
    /// and update accordingly
    fn update_en_passant_target(
        &mut self,
        chess_move: &ChessMove,
    ) -> Result<(), &'static str>;
}

pub trait PawnMovement {
    fn generate_pawn_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<Self>, &'static str>
    where
        Self: Sized;
}

impl PawnState for Board {
    fn update_en_passant_target(
        &mut self,
        chess_move: &ChessMove,
    ) -> Result<(), &'static str> {
        match *chess_move {
            ChessMove::Castling(_) => {
                self.en_passant_target = None;
                Ok(())
            }
            ChessMove::SimpleMove(from, to) => {
                // Note: we're getting the piece at `to` because at this point
                // the piece has already been moved and is at the new position
                let old_piece = self.get_piece_at_position(to)?;
                let offset = self.get_offset_of_move(from, to);

                if old_piece.is_some_and(|p| p.piece_type == PieceType::Pawn)
                    && offset.rank.abs() == 2
                    && from.file == to.file
                {
                    let en_passent_target_dir = Offset {
                        rank: offset.rank / 2,
                        file: 0,
                    };

                    self.en_passant_target =
                        Some(self.add_offset_to_position(
                            from,
                            en_passent_target_dir,
                        )?);
                } else {
                    self.en_passant_target = None;
                }

                Ok(())
            }
        }
    }
}

impl PawnMovement for Board {
    fn generate_pawn_moves(
        &self,
        checked: bool,
    ) -> Result<Vec<Board>, &'static str> {
        let mut possible_moves = vec![];
        let pawn_positions = self.get_positions_of_matching_pieces(
            Piece::new(self.current_move, Pawn),
        )?;

        let single_move_offset: isize = match self.current_move {
            White => 1,
            Black => -1,
        };

        let starting_rank_for_current_side = match self.current_move {
            White => 1,
            Black => self.width - 2,
        };

        let current_side = self.current_move;
        let opposite_side = current_side.flip();

        // Append single square pawn moves
        let single_square_pawn_move_boards = pawn_positions
            .iter()
            .filter_map(|pos| {
                Some((
                    pos,
                    UncheckedSquare {
                        file: pos.file,
                        rank: pos
                            .rank
                            .checked_add_signed(single_move_offset)?,
                    },
                ))
            })
            .filter_map(|(pos, new_pos)| {
                self.check_square(new_pos)
                    .ok()
                    .map(|checked| (pos, checked))
            })
            // The final destination should be free
            .filter(|(_, new_pos)| {
                matches!(self.get_piece_at_position(*new_pos), Ok(None))
            })
            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)| {
                self.new_board_with_moved_piece(*old_pos, new_pos, checked)
                    .ok()
            });
        possible_moves.extend(single_square_pawn_move_boards);

        // Append double square pawn moves
        let double_square_pawn_move_boards = pawn_positions
            .iter()
            .filter_map(|pos| {
                Some((
                    pos,
                    Square {
                        file: pos.file,
                        rank: pos
                            .rank
                            .checked_add_signed(2 * single_move_offset)?,
                    },
                ))
            })
            // Should start from second rank
            .filter(|(old_pos, _)| {
                old_pos.rank == starting_rank_for_current_side
            })
            // Should have the intervening space be free
            .filter(|(old_pos, new_pos)| {
                let ray_rank = self
                    .check_ray_for_pieces(
                        **old_pos,
                        Offset {
                            rank: single_move_offset,
                            file: 0,
                        },
                        false,
                    )
                    .rank;

                if single_move_offset > 0 {
                    ray_rank >= new_pos.rank
                } else {
                    ray_rank <= new_pos.rank
                }
            })
            // The final destination should be free
            .filter(|(_, new_pos)| {
                matches!(self.get_piece_at_position(*new_pos), Ok(None))
            })
            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)| {
                self.new_board_with_moved_piece(*old_pos, new_pos, checked)
                    .ok()
                    // Should set the en_passant_target
                    .and_then(|mut board| {
                        board.en_passant_target = Some(Square {
                            rank: new_pos
                                .rank
                                .checked_add_signed(-single_move_offset)?,
                            file: new_pos.file,
                        });
                        Some(board)
                    })
            });

        possible_moves.extend(double_square_pawn_move_boards);

        // Append pawn captures
        let pawn_capture_left_moves = pawn_positions
            .iter()
            .filter(|pos| pos.file > 0)
            .filter_map(|pos| {
                Some((
                    pos,
                    UncheckedSquare {
                        file: pos.file - 1,
                        rank: pos
                            .rank
                            .checked_add_signed(single_move_offset)?,
                    },
                ))
            });
        let pawn_capture_right_moves = pawn_positions
            .iter()
            .filter(|pos| pos.file < self.width - 1)
            .filter_map(|pos| {
                Some((
                    pos,
                    UncheckedSquare {
                        file: pos.file + 1,
                        rank: pos
                            .rank
                            .checked_add_signed(single_move_offset)?,
                    },
                ))
            });

        let pawn_capture_boards = pawn_capture_left_moves
            .clone()
            .chain(pawn_capture_right_moves.clone())
            // The final destination should have an opponent's piece
            .filter_map(|(pos, new_pos)| {
                Some((pos, self.check_square(new_pos).ok()?))
            })
            .filter(|(_, new_pos)| {
                self.get_piece_at_position(*new_pos).is_ok_and(|piece| {
                    piece.is_some_and(|p| p.side == opposite_side)
                })
            })
            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)| {
                self.new_board_with_moved_piece(*old_pos, new_pos, checked)
                    .ok()
            });
        possible_moves.extend(pawn_capture_boards);

        // Append en passant captures
        let en_passant_boards = pawn_capture_left_moves
            .chain(pawn_capture_right_moves)
            // The final destination should be the en passant target, set by the opponent's last move
            .filter_map(|(old_pos, new_pos)| {
                Some((old_pos, self.check_square(new_pos).ok()?))
            })
            .filter(|(_, new_pos)| Some(*new_pos) == self.en_passant_target)
            // Should be able to move there without error
            .filter_map(|(old_pos, new_pos)| {
                self.new_board_with_moved_piece(*old_pos, new_pos, checked)
                    .ok()
                    .and_then(|mut board| {
                        board
                            .set_piece_at_position(
                                None,
                                Square {
                                    rank: new_pos.rank.checked_add_signed(
                                        -single_move_offset,
                                    )?,
                                    file: new_pos.file,
                                },
                            )
                            .unwrap();

                        Some(board)
                    })
            });
        possible_moves.extend(en_passant_boards);

        Ok(possible_moves)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::board::{
        chess_move::ChessMove::SimpleMove, test_utils::check_for_moves,
    };

    // Returns a board with the setup
    // .....
    // .....
    // B.ko.
    // .P.pP
    // .....
    // Where o is the en passant target
    fn get_test_board_for_pawn_captures() -> Board {
        let mut board = Board {
            squares: vec![None; 5 * 5],
            width: 5,
            en_passant_target: Some(Square { rank: 2, file: 3 }),
            current_move: White,
            castling_availability: [false, false, false, false],
        };

        board
            .set_piece_at_position(
                Piece::new(White, Pawn).into(),
                Square { rank: 1, file: 1 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(White, Bishop).into(),
                Square { rank: 2, file: 0 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(Black, Knight).into(),
                Square { rank: 2, file: 2 },
            )
            .unwrap();

        board
            .set_piece_at_position(
                Piece::new(White, Pawn).into(),
                Square { rank: 1, file: 4 },
            )
            .unwrap();
        board
            .set_piece_at_position(
                Piece::new(Black, Pawn).into(),
                Square { rank: 1, file: 3 },
            )
            .unwrap();

        board
    }

    // Returns a board with the setup (FEN piece notation)
    // .......
    // .....p.
    // ....p.p
    // .Pp...P
    // P.P.PP.
    // .......
    fn get_test_board_for_simple_pawn_moves() -> Board {
        let mut squares = vec![None; 7 * 6];
        squares[7] = Some(Piece::new(White, Pawn));
        squares[9] = Some(Piece::new(White, Pawn));
        squares[11] = Some(Piece::new(White, Pawn));
        squares[12] = Some(Piece::new(White, Pawn));

        squares[15] = Some(Piece::new(White, Pawn));
        squares[16] = Some(Piece::new(Black, Pawn));
        squares[20] = Some(Piece::new(White, Pawn));

        squares[25] = Some(Piece::new(Black, Pawn));
        squares[27] = Some(Piece::new(Black, Pawn));

        squares[33] = Some(Piece::new(Black, Pawn));

        Board {
            squares,
            width: 7,
            en_passant_target: None,
            current_move: White,
            castling_availability: [false, false, false, false],
        }
    }

    #[test]
    fn one_square_forward() {
        let board = get_test_board_for_simple_pawn_moves();
        let moved_boards = board.generate_moves(true).unwrap();

        let expected_single_square_pushes = vec![
            Square { rank: 2, file: 0 },
            Square { rank: 2, file: 4 },
            Square { rank: 2, file: 5 },
            Square { rank: 3, file: 1 },
        ];
        let unexpected_single_square_pushes =
            vec![Square { rank: 2, file: 2 }, Square { rank: 3, file: 6 }];

        check_for_moves(
            moved_boards,
            expected_single_square_pushes,
            unexpected_single_square_pushes,
            Piece::new(White, Pawn),
        );
    }

    #[test]
    fn two_squares_forward() {
        let board = get_test_board_for_simple_pawn_moves();
        let moved_boards = board.generate_moves(true).unwrap();

        let expected_double_square_pushes =
            vec![Square { rank: 3, file: 0 }, Square { rank: 3, file: 5 }];
        let unexpected_double_square_pushes = vec![
            Square { rank: 4, file: 1 },
            Square { rank: 3, file: 2 },
            Square { rank: 3, file: 4 },
            Square { rank: 4, file: 6 },
        ];

        check_for_moves(
            moved_boards,
            expected_double_square_pushes,
            unexpected_double_square_pushes,
            Piece::new(White, Pawn),
        );
    }

    #[test]
    fn captures_opponents_pieces() {
        let board = get_test_board_for_pawn_captures();

        let moved_boards = board.generate_moves(true).unwrap();

        // At least one of the moves suggested should have the pawn
        // take a piece
        assert!(moved_boards.into_iter().any(|x| matches!(
            x.get_piece_at_position(Square { rank: 2, file: 2 })
                .unwrap(),
            Some(Piece {
                piece_type: Pawn,
                ..
            })
        )));
    }

    #[test]
    fn doesnt_capture_friendly_pieces() {
        let board = get_test_board_for_pawn_captures();

        let moved_boards = board.generate_moves(true).unwrap();

        // None of the moves should have a pawn taking the friendly piece
        assert!(moved_boards.into_iter().all(|x| !matches!(
            x.get_piece_at_position(Square { rank: 2, file: 0 })
                .unwrap(),
            Some(Piece {
                piece_type: Pawn,
                ..
            })
        )));
    }

    #[test]
    fn captures_en_passant() {
        let board = get_test_board_for_pawn_captures();

        let moved_boards = board.generate_moves(true).unwrap();

        // At least one of the moves suggested should have the pawn
        // take the pawn en passant
        assert!(moved_boards.iter().any(|x| matches!(
            x.get_piece_at_position(Square { rank: 2, file: 3 })
                .unwrap(),
            Some(Piece {
                piece_type: Pawn,
                ..
            })
        ) && x
            .get_piece_at_position(Square { rank: 1, file: 3 })
            .unwrap()
            .is_none()));
    }

    #[test]
    fn generates_en_passant_target() {
        let mut board = Board::from_art(
            "....\n\
             pppp\n\
             ....\n\
             ..P.\n\
             ....\n\
             ....\n",
        )
        .unwrap();

        board.current_move = Black;

        // Push pawn to create en_passant_target
        board
            .make_move(
                SimpleMove(
                    Square { rank: 4, file: 1 },
                    Square { rank: 2, file: 1 },
                ),
                true,
            )
            .unwrap();

        assert_eq!(board.en_passant_target, Some(Square { file: 1, rank: 3 }));
    }
}
