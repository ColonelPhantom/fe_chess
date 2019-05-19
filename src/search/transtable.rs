use crate::board;
use crate::search;

fn expandEnPassant(ep: u8) -> board::EnPassantState {
    use board::EnPassantState::*;
    use std::num::Wrapping;
    match ep {
        0 => None,
        1..=128 => Possible(Wrapping(ep as usize)),
        129..=255 => Capture(Wrapping(ep as usize - 128)),
    }
}

fn compressEnPassant(ep: board::EnPassantState) -> u8 {
    match ep {
        board::EnPassantState::None => 0,
        board::EnPassantState::Possible(c) => c.0 as u8,
        board::EnPassantState::Capture(c) => c.0 as u8 + 128,
    }
}

struct MoveCompact {
    from: u8,
    to: u8,
    promote_to: u8,
    castling: u8,
    en_passant: u8,
}

pub struct TtEntry {
    full_zobrist: u64,
    first_move: MoveCompact,
    depthleft: u16,
    eval_score: search::Score,
}
impl Default for TtEntry {
    fn default() -> Self {
        Self {
            full_zobrist: 0,
            first_move: MoveCompact{
                from: 0,
                to: 0,
                promote_to: 0,
                castling: 0,
                en_passant: 0,
            },
            depthleft: 0,
            eval_score: search::Score::Draw
        }
    }
} 

}