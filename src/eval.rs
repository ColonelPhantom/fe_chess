use crate::board::*;

type ValCp = isize;

const KING_VAL: ValCp = 200000;
const QUEEN_VAL: ValCp = 900;
const ROOK_VAL: ValCp = 500;
const BISHOP_VAL: ValCp = 320;
const KNIGHT_VAL: ValCp = 330;
const PAWN_VAL: ValCp = 100;

const BISHOP_PAIR_BONUS: ValCp = 25;

pub fn eval(b: &Board) -> isize {
    let mut score = 0;
    for rank in 0..8 {
        for file in 0..8 {
            let c: Coord0x88 = c0x88(file, rank);
            let p: Piece = b[c];
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
            } * match p.color {
                WHITE => 1,
                BLACK => -1,
            };
            score += p_score;
        }
    }
    return score;
}
