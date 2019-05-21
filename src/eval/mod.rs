use crate::board::*;
use crate::movegen;

mod pst;

type ValCp = i32;

const KING_VAL: ValCp = 200000;
const QUEEN_VAL: ValCp = 900;
const ROOK_VAL: ValCp = 500;
const BISHOP_VAL: ValCp = 320;
const KNIGHT_VAL: ValCp = 330;
const PAWN_VAL: ValCp = 100;

const BISHOP_PAIR_BONUS: ValCp = 25;

const MOBIL_VAL: ValCp = 5;

fn mobil_eval(b: &Board, p: &Piece, from: Coord0x88, offset: Coord0x88) -> ValCp {
    let mut count = 0;
    let mut c = from + offset;
    while c.0 & 0x88 == 0 {
        count += 1;
        if b[c].piece_type != PieceType::None {
            break;
        }
        c += offset;
    };
    return count;
}

pub fn piece_val(p: PieceType) -> ValCp {
    match p {
        PieceType::Pawn => PAWN_VAL,
        PieceType::Knight => KNIGHT_VAL,
        PieceType::Bishop => BISHOP_VAL,
        PieceType::Rook => ROOK_VAL,
        PieceType::Queen => QUEEN_VAL,
        PieceType::King => KING_VAL,
        _ => 0,
    }
}
pub fn eval(b: &mut Board) -> i32 {
    let mut score: i32 = 0;
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
                    lookup_c = c8x8(file, 7 - rank);
                }
                BLACK => {
                    p_sign = -1;
                    lookup_c = c8x8(file, rank);
                }
            };
            let mobility_score = match p.piece_type {
                PieceType::Knight => 8,
                PieceType::Bishop => MOBIL_VAL * (
                    mobil_eval(b, &p, c, o0x88( 1,  1)) +
                    mobil_eval(b, &p, c, o0x88(-1,  1)) +
                    mobil_eval(b, &p, c, o0x88( 1, -1)) +
                    mobil_eval(b, &p, c, o0x88(-1, -1))
                ),
                PieceType::Rook => MOBIL_VAL * (
                    mobil_eval(b, &p, c, o0x88( 1,  0)) +
                    mobil_eval(b, &p, c, o0x88(-1,  0)) +
                    mobil_eval(b, &p, c, o0x88( 0,  1)) +
                    mobil_eval(b, &p, c, o0x88( 0, -1))
                ),
                PieceType::Queen => MOBIL_VAL * (
                    mobil_eval(b, &p, c, o0x88( 1,  1)) +
                    mobil_eval(b, &p, c, o0x88(-1,  1)) +
                    mobil_eval(b, &p, c, o0x88( 1, -1)) +
                    mobil_eval(b, &p, c, o0x88(-1, -1)) +
                    mobil_eval(b, &p, c, o0x88( 1,  0)) +
                    mobil_eval(b, &p, c, o0x88(-1,  0)) +
                    mobil_eval(b, &p, c, o0x88( 0,  1)) +
                    mobil_eval(b, &p, c, o0x88( 0, -1))
                ),
                _ => 0,
            };
            // let mobility_score = 0;
            let p_score = match p.piece_type {
                PieceType::None => 0,
                PieceType::Pawn => PAWN_VAL + pst::PAWN[lookup_c],
                PieceType::Knight => KNIGHT_VAL + pst::KNIGHT[lookup_c],
                PieceType::Bishop => {
                    let pairbonus = bishop_present[p.color as usize] as ValCp * BISHOP_PAIR_BONUS;
                    bishop_present[p.color as usize] = true;
                    BISHOP_VAL + pairbonus + pst::BISHOP[lookup_c]
                },
                PieceType::Rook => ROOK_VAL + pst::ROOK[lookup_c],
                PieceType::Queen => QUEEN_VAL + pst::QUEEN[lookup_c],
                PieceType::King => KING_VAL + pst::KING_MID[lookup_c],  // TODO: taper the eval or something.
                PieceType::Any => 0,
            } * p_sign;
            score += p_score + mobility_score;
        }
    }

    return score;
}
