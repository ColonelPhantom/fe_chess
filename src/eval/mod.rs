use crate::board::*;
use crate::movegen;

mod pst;

type ValCp = isize;

const KING_VAL: ValCp = 200000;
const QUEEN_VAL: ValCp = 900;
const ROOK_VAL: ValCp = 500;
const BISHOP_VAL: ValCp = 320;
const KNIGHT_VAL: ValCp = 330;
const PAWN_VAL: ValCp = 100;

const BISHOP_PAIR_BONUS: ValCp = 25;

const MOBIL_VAL: ValCp = 5;

pub fn eval(b: &mut Board) -> isize {
    let mut score = 0;
    let b_sign = match b.side_to_move {
        WHITE => 1,
        BLACK => -1,
    };
    let mut bishop_present = [false, false];
    for rank in 0..8 {
        for file in 0..8 {
            let c: Coord0x88 = c0x88(file, rank);
            let p: Piece = b[c];
            let p_sign;
            let lookup_c;
            match p.color {
                WHITE => {
                    p_sign = 1;
                    lookup_c = c8x8(file, rank);
                }
                BLACK => {
                    p_sign = -1;
                    lookup_c = c8x8(file, 7 - rank);
                }
            };
            let p_score = match p.piece_type {
                PieceType::None => 0,
                PieceType::Pawn => PAWN_VAL + pst::PAWN[lookup_c],
                PieceType::Knight => KNIGHT_VAL + pst::KNIGHT[lookup_c],
                PieceType::Bishop => {
                    let pairbonus = bishop_present[p.color as usize] as isize * BISHOP_PAIR_BONUS;
                    bishop_present[p.color as usize] = true;
                    BISHOP_VAL + pairbonus + pst::BISHOP[lookup_c]
                },
                PieceType::Rook => ROOK_VAL + pst::ROOK[lookup_c],
                PieceType::Queen => QUEEN_VAL + pst::QUEEN[lookup_c],
                PieceType::King => KING_VAL + pst::KING_MID[lookup_c],  // TODO: taper the eval or something.
                PieceType::Any => 0,
            } * p_sign;
            score += p_score;
        }
    }

    // score += MOBIL_VAL * b_sign * movegen::movegen(b).len() as isize;
    // score -= MOBIL_VAL * b_sign * movegen::enemygen::enemygen(b).len() as isize;

    return score;
}
