use crate::board;
use crate::search;

pub struct TtEntry {
    full_zobrist: u64,
    first_move: board::Move,
    depthleft: u16,
    eval_score: search::Score,

}