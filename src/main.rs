#[derive(PartialEq)]
enum Color {
    White,
    Black,
}

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

        return self.bitboards[piece_offset + color_offset];
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

        let pawn_indicies = indicies_from_bitboard(self.pawns[0]);
        for pawn in pawn_indicies {
            // If the place in front of the pawn isn't occupied
            if get_bitboard_index(occupied, (pawn as i16 + single_move_offset as i16) as u8) == false {
                // ... then we can try move it
                let new_bitboard = move_piece(self.pawns[0], pawn, pawn + 8);

                // if we got a new bit_board that works
                match new_bitboard {
                    Some(some_new_bitboard) => {
                        // Add it to the list
                        let mut new_board = *self;
                        new_board.pawns[0] = some_new_bitboard;
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
        for i in 1..2 {
            occupied &=
                self.pawns[i] &
                self.knights[i] &
                self.bishops[i] &
                self.rooks[i] &
                self.queens[i] &
                self.kings[i]
        }

        occupied
    }

    fn move_piece(&self, 

}

fn indicies_from_bitboard(bitboard: u64) -> Vec<u8> {
    let mut indicies = vec![];
    let mut index = 0;
    while bitboard > 0 {
        if bitboard & 1 == 1 {
            indicies.push(index);
        }

        bitboard >>= 1;
        index += 1;
    }

    indicies
}

fn get_bitboard_index(board: u64, index: u8) -> bool {
    if is_valid_square(index) {
        let mask : u64 = 1 << index;
        mask & board != 0
    } else {
        false
    }
}

fn set_bitboard_index(board: u64, index: u8) -> Option<u64> {
    if is_valid_square(index) {
        let mask : u64 = 1 << index;
        Some(board | mask)
    } else {
        None
    }
}

fn unset_bitboard_index(board: u64, index: u8) -> Option<u64> {
    if is_valid_square(index) {
        let mask : u64 = 1 << index;
        Some(board & !mask)
    } else {
        None
    }
}

fn move_piece(bitboard: u64, from: u8, to: u8) -> Option<u64> {
    let unset_bitboard = unset_bitboard_index(bitboard, from);
    match unset_bitboard {
        Some(some_unset_bitboard) => set_bitboard_index(some_unset_bitboard, to),
        None => None
    }
}

fn is_valid_square(square: u8) -> bool {
    square < 64
}

fn main() {
    println!("Hello, world!");
}
