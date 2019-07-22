mod defs;

pub fn get_zob_piece_square(p: super::Piece, coord: super::Coord8x8) -> u64 {
    use super::{PieceType, WHITE, BLACK};
    return match p.piece_type {
        PieceType::Pawn => match p.color {
            WHITE => &defs::WPAWN,
            BLACK => &defs::BPAWN,
        }
        PieceType::Knight => match p.color {
            WHITE => &defs::WKNIGHT,
            BLACK => &defs::BKNIGHT,
        }
        PieceType::Bishop => match p.color {
            WHITE => &defs::WBISHOP,
            BLACK => &defs::BBISHOP,
        }
        PieceType::Rook => match p.color {
            WHITE => &defs::WROOK,
            BLACK => &defs::BROOK,
        }
        PieceType::Queen => match p.color {
            WHITE => &defs::WQUEEN,
            BLACK => &defs::BQUEEN,
        }
        PieceType::King => match p.color {
            WHITE => &defs::WKING,
            BLACK => &defs::BKING,
        }
        _ => panic!("Capture of unexpected piece type"),
    }[coord];
}

impl super::Board {
    pub fn zobrist_init(&self) -> u64 {
        let mut z: u64 = 0;
        for file in 0..8 {
            for rank in 0..8 {
                use super::{c0x88,c8x8, pieces::*};
                let c = c0x88(file, rank);
                z ^= match self[c] {
                    NONE => 0,
                    WPAWN => defs::WPAWN[c8x8(file, rank)],
                    WKNIGHT => defs::WKNIGHT[c8x8(file, rank)],
                    WBISHOP => defs::WBISHOP[c8x8(file, rank)],
                    WROOK => defs::WROOK[c8x8(file, rank)],
                    WQUEEN => defs::WQUEEN[c8x8(file, rank)],
                    WKING => defs::WKING[c8x8(file, rank)],
                    BPAWN => defs::BPAWN[c8x8(file, rank)],
                    BKNIGHT => defs::BKNIGHT[c8x8(file, rank)],
                    BBISHOP => defs::BBISHOP[c8x8(file, rank)],
                    BROOK => defs::BROOK[c8x8(file, rank)],
                    BQUEEN => defs::BQUEEN[c8x8(file, rank)],
                    BKING => defs::BKING[c8x8(file, rank)],
                    _ => 0,
                }
            }
        }
        use super::{WHITE, BLACK};
        z ^= match self.side_to_move {
            WHITE => 0,
            BLACK => defs::SIDE_TO_MOVE,
        };
        if self.en_passant.is_some() {
            z ^= defs::ENPASSANT[self.en_passant.unwrap().0 & 7];
        }
        for cr in 0..4 {
            if self.castling[cr] {
                z ^= defs::CASTLING[cr];
            }
        }

        return z;
    }
    pub fn zobrist_toggle(&mut self, p: super::Piece, coord: super::Coord8x8) {
        self.zobrist ^= get_zob_piece_square(p, coord);
    }
    pub fn zobrist_toggle_ep(&mut self, ep_val: super::Coord0x88) {
        self.zobrist ^= defs::ENPASSANT[ep_val.0 & 0x7];
    }
    pub fn zobrist_toggle_castle(&mut self, right: usize, side: bool) {
        self.zobrist ^= defs::CASTLING[right + side as usize];
    }
    pub fn zobrist_toggle_side(&mut self) {
        self.zobrist ^= defs::SIDE_TO_MOVE;
    }
}