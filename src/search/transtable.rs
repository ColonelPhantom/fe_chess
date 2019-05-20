use crate::board;
use crate::search;

fn expand_en_passant(ep: u8) -> board::EnPassantState {
    use board::EnPassantState::*;
    use std::num::Wrapping;
    match ep {
        0 => None,
        1..=128 => Possible(Wrapping(ep as usize)),
        129..=255 => Capture(Wrapping(ep as usize - 128)),
    }
}

fn compress_en_passant(ep: &board::EnPassantState) -> u8 {
    match ep {
        board::EnPassantState::None => 0,
        board::EnPassantState::Possible(c) => c.0 as u8,
        board::EnPassantState::Capture(c) => c.0 as u8 + 128,
    }
}

#[derive(Copy, Clone)]
struct MoveCompact {
    from: u8,
    to: u8,
    promote_to: board::PieceType,
    castling: u8,
    en_passant: u8,
}
impl MoveCompact {
    fn from_move(m: &board::Move) -> Self {
        Self {
            from: m.from.0 as u8,
            to: m.to.0 as u8,
            promote_to: m.promote_to,
            castling: match m.castling {
                None => 255,
                Some(c) => c as u8,
            },
            en_passant: compress_en_passant(&m.en_passant),
        }
    }
    pub fn to_move(&self) -> board::Move {
        let castling = match self.castling {
            255 => None,
            0..=3 => Some(self.castling as usize),
            _ => panic!("Invalid castling. Value: {}", self.castling)
        };
        use std::num::Wrapping;
        board::Move {
            from: Wrapping(self.from as usize),
            to: Wrapping(self.to as usize),
            castling,
            en_passant: expand_en_passant(self.en_passant),
            promote_to: self.promote_to,
        }
    }
}

#[derive(Copy, Clone)]
pub struct TtEntry {
    pub full_zobrist: u64,
    first_move: MoveCompact,
    pub depthleft: u16,
    pub eval_score: search::Score,
}
impl Default for TtEntry {
    fn default() -> Self {
        Self {
            full_zobrist: 0,
            first_move: MoveCompact{
                from: 0,
                to: 0,
                promote_to: board::PieceType::None,
                castling: 0,
                en_passant: 0,
            },
            depthleft: 0,
            eval_score: search::Score::Draw
        }
    }
} 
impl TtEntry {
    pub fn get_move(&self) -> board::Move {
        self.first_move.to_move()
    }
}

pub struct TransTable {
    t: Vec<TtEntry>,
    len: usize
}

impl TransTable {
    pub fn new(size: u32) -> Self {
        let len = 2usize.pow(size);
        let mut t = Vec::with_capacity(len);
        t.resize(len, TtEntry::default());
        Self {t, len: len-1}
    }

    pub fn put(&mut self, zob: u64, m: &board::Move, depth: u16, score: search::Score) {
        self.t[zob as usize & self.len] = TtEntry {
            full_zobrist: zob,
            first_move: MoveCompact::from_move(m),
            depthleft: depth,
            eval_score: score,
        };
    }

    pub fn get(&self, zob: u64) -> Option<TtEntry> {
        let e = self.t[zob as usize & self.len];
        if e.full_zobrist == zob {
            Some(e)
        } else {
            None
        }
    }
}
