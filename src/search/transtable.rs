use crate::board;
use crate::search;

const HASH_SHIFTS: [u64; 4] = [0, 26, 52, 14];

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

#[derive(Copy, Clone, Debug)]
struct MoveCompact {
    from: u8,
    to: u8,
    promote_to: board::PieceType,
    castling: u8,
    en_passant: u8,
}
impl MoveCompact {
    fn from_move(m: board::Move) -> Self {
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

#[derive(Copy, Clone, Debug)]
pub struct TtEntry {
    pub full_zobrist: u64,
    first_move: Option<MoveCompact>,
    pub depthleft: i16,
    pub eval_score: search::Score,
    pub node_type: search::NodeType,
}
impl Default for TtEntry {
    fn default() -> Self {
        Self {
            full_zobrist: 0,
            first_move: None,
            depthleft: std::i16::MIN,
            eval_score: search::Score::Draw,
            node_type: search::NodeType::None,
        }
    }
} 
impl TtEntry {
    pub fn get_move(&self) -> Option<board::Move> {
        match self.first_move {
            None => None,
            Some(m) => Some(m.to_move())
        }
    }
}

pub struct TransTable {
    t: Vec<TtEntry>,
    len: u64,
}

impl TransTable {
    pub fn new(size: u32) -> Self {
        let len = 2usize.pow(size);
        let mut t = Vec::with_capacity(len);
        t.resize(len, TtEntry::default());
        Self {t, len: len as u64 - 1}
    }

    fn put_actual(&mut self, zob: u64, key: u64, m: Option<board::Move>, depth: i16, score: search::Score, node_type: search::NodeType) -> PutState {
        let e = &self.t[key as usize];
        if e.depthleft >= depth {
            // Occupied by better entry: abort/skip
            if zob == e.full_zobrist {
                // Better version of self, do not retry elsewhere in the table.
                return PutState::Abort;
            } else {
                return PutState::Occupied;
            }
        }

        // No objections, so put the move in.
        self.t[key as usize] = TtEntry {
            full_zobrist: zob,
            first_move: match m {
                None => None,
                Some(m) => Some(MoveCompact::from_move(m)),
            },
            depthleft: depth,
            eval_score: score,
            node_type,
        };
        return PutState::Ok;
    }
    pub fn put(&mut self, zob: u64, m: Option<board::Move>, depth: i16, score: search::Score, node_type: search::NodeType) {
        for i in &HASH_SHIFTS {
            let key = zob >> i;
            match self.put_actual(zob, key & self.len, m, depth, score, node_type) {
                PutState::Ok => { return },
                PutState::Occupied => { continue },
                PutState::Abort => { return },
            }
        }
    }

    fn get_actual(&self, zob: u64, key: u64) -> GetState {
        let e = self.t[key as usize];
        if e.full_zobrist == zob {
            GetState::Ok(e)
        } else if e.full_zobrist == 0 {
            GetState::Abort
        } else {
            GetState::Occupied
        }
    }
    pub fn get(&self, zob: u64) -> Option<TtEntry> {
        for i in &HASH_SHIFTS {
            let key = zob >> i;
            match self.get_actual(zob, key & self.len) {
                GetState::Ok(t) => { return Some(t); },
                GetState::Occupied => { continue; },
                GetState::Abort => { return None; },
            };
        };
        // No entry found.
        return None;
    }

    #[allow(dead_code)]
    pub fn filled(&self) -> usize {
        let mut c = 0;
        for i in 0..self.len {
            if self.t[i as usize].full_zobrist != 0 {
                c += 1;
            }
        }
        return c;
    }
}

enum PutState {
    Ok,
    Occupied,
    Abort,
}

enum GetState {
    Ok(TtEntry),
    Occupied,
    Abort,
}