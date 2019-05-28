use crate::movegen;
use crate::board;
use board::Board;

use super::quiesce::quiesce;
use super::SearchInfoIntm;
use super::Score;
use super::transtable::TransTable;
use super::NodeType;

pub fn alpha_beta(b: &mut Board, mut alpha: Score, beta: Score, depthleft: usize, tt: &mut TransTable)
 -> SearchInfoIntm
{
    let mut local_alpha = Score::Loss(0);
    let mut best_move: Option<board::Move> = None;
    let mut nodes = 1;

    if depthleft == 0 {
        return SearchInfoIntm{
            score: quiesce(b, alpha, beta, 1, tt),
            nodes: 1
        };
    }

    match tt.get(b.zobrist) {
        None => {},
        Some(tt_entry) => {
            if tt_entry.depthleft >= depthleft as i16 {
                //println!("Full table hit!");
                return SearchInfoIntm{
                    score: tt_entry.eval_score,
                    nodes,
                }
            } else if tt_entry.eval_score.is_decided() {
                //println!("Hit on decided position");
                return SearchInfoIntm {
                    score: tt_entry.eval_score,
                    nodes,
                }
            } else if tt_entry.get_move().is_some() {
                // Ttable entry too shallow, use it only for move ordering
                //println!("Partial table hit!");
                let m = tt_entry.get_move().unwrap();
                let eval = tt_entry.eval.clone();
                b.make(&m);
                let si = alpha_beta(b, -beta, -alpha, depthleft - 1, tt);
                let score = match -si.score {
                    Score::Win(d) => Score::Win(d+1),
                    Score::Loss(d) => Score::Loss(d+1),
                    Score::Value(p) => Score::Value(p),
                    Score::Draw => Score::Draw,
                };
                nodes += si.nodes;
                b.unmake();
                if score >= beta  {
                    // Store self move in TT, move field is refutation move
                    tt.put(b.zobrist, Some(m), depthleft as i16, score, NodeType::CutNode, beta, eval);
                    return SearchInfoIntm {
                        score: beta,
                        nodes,
                    };
                }
                if score > alpha  {
                    // Remember this move to be stored in TT
                    best_move = Some(m.clone());
                    alpha = score;
                    local_alpha = score;
                } else if score > local_alpha { // Local_alpha <= alpha so if first is true second is true as well.
                    best_move = Some(m.clone());
                    local_alpha = score;
                }
            } else {
                // No use for TT entry
                //println!("Useless TT entry with depth difference {}, alpha {} and beta {}: {:?}", depthleft as u16 - tt_entry.depthleft, alpha, beta, tt_entry);
            }
        }
    }
    
    let mut moves = movegen::movegen(b);

    // Sort decending by priority given
    let baseline_eval = crate::eval::eval(b);
    let eval = baseline_eval * match b.side_to_move {
        board::WHITE => 1,
        board::BLACK => -1,
    };
    if depthleft > 1 {
        moves.sort_by_cached_key(|m| {
            -super::moveord::move_priority(m, b, tt, baseline_eval)
        });
    }

    if moves.len() == 0 {
        if !b.is_check(b.side_to_move).is_safe() {
            return SearchInfoIntm {
                score: Score::Loss(0),
                nodes,
            }
        } else {
            return SearchInfoIntm {
                score: Score::Draw,
                nodes,
            }
        }
    }

    for m in moves {
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let si = alpha_beta(b, -beta, -alpha, depthleft - 1, tt);
        let score = match -si.score {
            Score::Win(d) => Score::Win(d+1),
            Score::Loss(d) => Score::Loss(d+1),
            Score::Value(p) => Score::Value(p),
            Score::Draw => Score::Draw,
        };
        nodes += si.nodes;
        b.unmake();
        if score >= beta  {
            // Store self move in TT, move field is refutation move
            tt.put(b.zobrist, Some(m), depthleft as i16, score, NodeType::CutNode, beta, Some(eval));
            return SearchInfoIntm {
                score: beta,
                nodes,
            };
        }
        if score > alpha  {
            // Remember this move to be stored in TT
            best_move = Some(m.clone());
            alpha = score;
            local_alpha = score;
        } else if score > local_alpha {
            best_move = Some(m.clone());
            local_alpha = score;
        }
    }

    match !(local_alpha < alpha) {  // match alpha_raised
        false => tt.put(b.zobrist, best_move, depthleft as i16, local_alpha, NodeType::AllNode, beta, Some(eval)),
        true => tt.put(b.zobrist, best_move, depthleft as i16, local_alpha, NodeType::PvNode, beta, Some(eval)),
    };

    return SearchInfoIntm {
        score: local_alpha,
        nodes
    }
}
