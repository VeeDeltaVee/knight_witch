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
        let occupied = self.get_occupied();

        let single_move_offset: i8;
        if *to_move == Color::White {
            single_move_offset = 8;
        } else {
            single_move_offset = -8;
        }

        let pawn_bitboard = self.get_piece_bitboard_of_color(Piece::Pawn, *to_move);
        let pawn_squares = squares_from_bitboard(pawn_bitboard);
        for pawn in pawn_squares {
            // If the place in front of the pawn isn't occupied
            if get_bitboard_square(occupied, (pawn as i16 + single_move_offset as i16) as u8) == false {
                // ... then we can try move it
                let new_bitboard = move_piece(pawn_bitboard, pawn, pawn + 8);

                // if we got a new bit_board that works
                match new_bitboard {
                    Some(some_new_bitboard) => {
                        // Add it to the list
                        let mut new_board = *self;
                        new_board.set_piece_bitboard_of_color(
                            Piece::Pawn,
                            *to_move,
                            some_new_bitboard);
                        boards.push(new_board);
                    }, None => {}
                }
            }
        }

        boards
    }

    fn pawn_captures(&self, to_move: &Color) -> Vec<BoardState> {
        return vec![];
    }

    fn get_occupied(&self) -> u64 {
        let mut occupied: u64 = 0;
        for i in 0..12 {
            occupied &= self.bitboards[i];
        }

        occupied
    }

    //fn move_piece(&self, 

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
