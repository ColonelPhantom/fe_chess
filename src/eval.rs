use crate::board::*;

type ValCp = usize;

const king_val: ValCp = 200000;
const queen_val: ValCp = 900;
const rook_val: ValCp = 500;
const bishop_val: ValCp = 320;
const knight_val: ValCp = 330;

const BISHOP_PAIR_BONUS: ValCp = 25;