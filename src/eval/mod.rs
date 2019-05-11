use crate::board::*;
use crate::movegen;

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
    let sign = match b.side_to_move {
        WHITE => 1,
        BLACK => -1,
    };
    for rank in 0..8 {
        for file in 0..8 {
            let c: Coord0x88 = c0x88(file, rank);
            let p: Piece = b[c];
            let sign = match p.color {
                WHITE => 1,
                BLACK => -1,
            };
            let p_score = match p.piece_type {
                PieceType::None => 0,
                PieceType::Pawn => PAWN_VAL + match p.color {
                    WHITE => 5* rank,
                    BLACK => 5*(7-rank)
                },
                PieceType::Knight => KNIGHT_VAL,
                PieceType::Bishop => BISHOP_VAL,
                PieceType::Rook => ROOK_VAL,
                PieceType::Queen => QUEEN_VAL,
                PieceType::King => KING_VAL,
                PieceType::Any => 0,
            } * sign;
            score += p_score;
        }
    }

    score += MOBIL_VAL * sign * movegen::movegen(b).len() as isize;
    score -= MOBIL_VAL * sign * movegen::enemygen::enemygen(b).len() as isize;

    return score;
}
