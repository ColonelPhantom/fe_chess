use crate::movegen;
use crate::board;
use board::Board;

use super::quiesce::quiesce;
use super::SearchInfo;
use super::Score;
use super::transtable::TransTable;

pub fn alpha_beta(b: &mut Board, mut alpha: Score, beta: Score, depthleft: usize, prev_pv: &mut Vec<board::Move>, tt: &mut TransTable)
 -> SearchInfo
{
    let mut pv: Vec<board::Move> = vec![];

    if depthleft == 0 {
        return SearchInfo{
            score: quiesce(b, alpha, beta),
            pv: vec![],
        };
    }

    match tt.get(b.zobrist) {
        None => (),
        Some(tt_entry) => {
            if tt_entry.depthleft >= depthleft as u16 {
                println!("Full table hit!");
                return SearchInfo{
                    score: tt_entry.eval_score,
                    pv: vec![]
                }
            } else if tt_entry.get_move().is_some() {
                // Ttable entry too shallow, use it only for move ordering
                println!("Partial table hit!");
                let m = tt_entry.get_move().unwrap();
                b.make(&m);
                let si = alpha_beta(b, -beta, -alpha, depthleft - 1, prev_pv, tt);
                let score = match -si.score {
                    Score::Win(d) => Score::Win(d+1),
                    Score::Loss(d) => Score::Loss(d+1),
                    Score::Value(p) => Score::Value(p),
                    Score::Draw => Score::Draw,
                };
                b.unmake();
                if score >= beta  {
                    return SearchInfo {
                        score: beta,
                        pv
                    };
                }
                if score > alpha  {
                    alpha = score;
                    pv = si.pv;
                    pv.push(m);
                }
            }
        }
    }
    
    // if let Some(m) = prev_pv.pop() {
    //     b.make(&m);
    //     let si = alpha_beta(b, -beta, -alpha, depthleft - 1, prev_pv, tt);
    //     let score = match -si.score {
    //         Score::Win(d) => Score::Win(d+1),
    //         Score::Loss(d) => Score::Loss(d+1),
    //         Score::Value(p) => Score::Value(p),
    //         Score::Draw => Score::Draw,
    //     };
    //     b.unmake();
    //     if score >= beta  {
    //         return SearchInfo {
    //             score: beta,
    //             pv
    //         };
    //     }
    //     if score > alpha  {
    //         alpha = score;
    //         pv = si.pv;
    //         pv.push(m);
    //     }
    // }

    let moves = movegen::movegen(b);

    if moves.len() == 0 {
        if !b.is_check(b.side_to_move).is_safe() {
            return SearchInfo {
                score: Score::Loss(0),
                pv: vec![]
            }
        } else {
            return SearchInfo {
                score: Score::Draw,
                pv: vec![]
            }
        }
    }

    let mut best_move: Option<board::Move> = None;
    for m in moves {
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let si = alpha_beta(b, -beta, -alpha, depthleft - 1, prev_pv, tt);
        let score = match -si.score {
            Score::Win(d) => Score::Win(d+1),
            Score::Loss(d) => Score::Loss(d+1),
            Score::Value(p) => Score::Value(p),
            Score::Draw => Score::Draw,
        };
        b.unmake();
        if score >= beta  {
            // Store self move in TT, move field is refutation move
            tt.put(b.zobrist, Some(m), depthleft as u16, score);
            //println!("Beta cutoff");
            return SearchInfo {
                score: beta,
                pv
            };
        }
        if score > alpha  {
            // Store self move in TT, next move is the best move.
            //println!("Alpha raised");
            best_move = Some(m.clone());
            alpha = score;
            pv = si.pv;
            pv.push(m.clone());
        }
    }
    tt.put(b.zobrist, best_move, depthleft as u16, alpha);
    //if best_move.is_none() { println!("No alpha raise") }
    return SearchInfo {
        score: alpha,
        pv
    }
}
