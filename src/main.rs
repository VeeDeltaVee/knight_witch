#[derive(PartialEq, Copy, Clone)]
enum Color {
    White,
    Black,
}

#[derive(PartialEq, Copy, Clone)]
enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, Copy, Clone)]
struct BoardState {
    // Representation of the board in Little Endian Rank File Mapping
    // See chessprogramming wiki for details
    //
    // First element is the White pieces, second is Black

    bitboards: [u64; 12],
}

impl BoardState {

    pub fn get_piece_bitboard_of_color(&self, piece: Piece, color: Color) -> u64 {
        let offset = get_bitboard_offset(piece, color);
        return self.bitboards[offset];
    }

    pub fn set_piece_bitboard_of_color(&mut self, piece: Piece, color: Color, new_bitboard: u64) {
        let offset = get_bitboard_offset(piece, color);
        self.bitboards[offset] = new_bitboard;
    }

    pub fn pawn_moves(&self, to_move: &Color) -> Vec<BoardState> {
        let mut pawn_moves = self.pawn_pushes(to_move);
        pawn_moves.append(&mut self.pawn_captures(to_move));

        pawn_moves
    }

    fn pawn_pushes(&self, to_move: &Color) -> Vec<BoardState> {
        let mut boards = vec![];
        let occupied = self.get_occupied(None);

        let single_move_offset: i16;
        if *to_move == Color::White {
            single_move_offset = 8;
        } else {
            single_move_offset = -8;
        }

        let pawn_bitboard = self.get_piece_bitboard_of_color(Piece::Pawn, *to_move);
        let pawn_squares = squares_from_bitboard(pawn_bitboard);
        for pawn in pawn_squares {
            let single_move = self.get_move_if_free_square(Piece::Pawn, *to_move, false,
                                    pawn, (pawn as i16 + single_move_offset) as u8);
            match single_move {
                Some(new_board) => boards.push(new_board),
                None => {},
            }

            let double_move = self.get_move_if_free_square(Piece::Pawn, *to_move, false,
                                    pawn, (pawn as i16 + 2*single_move_offset) as u8);
            match double_move {
                Some(new_board) => boards.push(new_board),
                None => {},
            }
        }

        boards
    }

    fn pawn_captures(&self, to_move: &Color) -> Vec<BoardState> {
        return vec![];
    }

    fn get_occupied(&self, one_color: Option<Color>) -> u64 {
        let mut occupied: u64 = 0;
        let range = match one_color {
            Some(Color::White) => 0..6,
            Some(Color::Black) => 7..12,
            None               => 0..12
        };
        for i in range {
            occupied &= self.bitboards[i];
        }

        occupied
    }

    fn get_move_if_free_square(&self, piece: Piece, to_move: Color, can_take: bool,
                                      from: u8, to: u8) -> Option<BoardState> {
        let occupied = if can_take {
            self.get_occupied(Some(to_move))
        } else {
            self.get_occupied(None)
        };

        let piece_bitboard = self.get_piece_bitboard_of_color(piece, to_move);
        if !get_bitboard_square(occupied, to) {
            let new_bitboard = move_piece(piece_bitboard, from, to);
            if can_take {
                unimplemented!("Logic to take piece on move");
            }

            match new_bitboard {
                Some(some_new_bitboard) => {
                    // Build a new full board from new bitboards
                    let mut new_board = *self;
                    new_board.set_piece_bitboard_of_color(
                        piece,
                        to_move,
                        some_new_bitboard);
                    return Some(new_board);
                }, None => {
                    return None;
                }
            }
        } else {
            None
        }
    }

}

fn get_bitboard_offset(piece: Piece, color: Color) -> usize {
    let color_offset = match color {
        Color::White => 0,
        Color::Black => 6
    };

    let piece_offset = match piece {
        Piece::Pawn => 0,
        Piece::Knight => 1,
        Piece::Bishop => 2,
        Piece::Rook => 3,
        Piece::Queen => 4,
        Piece::King => 5
    };

    color_offset + piece_offset
}

fn squares_from_bitboard(bitboard: u64) -> Vec<u8> {
    let mut squares = vec![];
    let mut square = 0;
    let mut bitboard_copy = bitboard;
    while bitboard > 0 {
        if bitboard & 1 == 1 {
            squares.push(square);
        }

        bitboard_copy >>= 1;
        square += 1;
    }

    squares
}

fn get_bitboard_square(board: u64, square: u8) -> bool {
    if is_valid_square(square) {
        let mask : u64 = 1 << square;
        mask & board != 0
    } else {
        false
    }
}

fn set_bitboard_square(board: u64, square: u8) -> Option<u64> {
    if is_valid_square(square) {
        let mask : u64 = 1 << square;
        Some(board | mask)
    } else {
        None
    }
}

fn unset_bitboard_square(board: u64, square: u8) -> Option<u64> {
    if is_valid_square(square) {
        let mask : u64 = 1 << square;
        Some(board & !mask)
    } else {
        None
    }
}

fn move_piece(bitboard: u64, from: u8, to: u8) -> Option<u64> {
    let unset_bitboard = unset_bitboard_square(bitboard, from);
    match unset_bitboard {
        Some(some_unset_bitboard) => set_bitboard_square(some_unset_bitboard, to),
        None => None
    }
}

fn is_valid_square(square: u8) -> bool {
    square < 64
}

fn main() {
    println!("Hello, world!");
}
