use crate::board::*;

type ValCp = usize;

const KING_VAL: ValCp = 200000;
const QUEEN_VAL: ValCp = 900;
const ROOK_VAL: ValCp = 500;
const BISHOP_VAL: ValCp = 320;
const KNIGHT_VAL: ValCp = 330;

const BISHOP_PAIR_BONUS: ValCp = 25;

pub fn eval(b: &Board) -> isize {
    return 0;
}