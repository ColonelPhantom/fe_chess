use crate::board;
use crate::eval::eval;
use crate::movegen;
use board::Board;

use super::NodeType;
use super::Score;

const MAX_DELTA: crate::eval::ValCp = 1000;
const SEE_DELTA: crate::eval::ValCp = 100;

pub fn quiesce(
    b: &mut Board,
    mut alpha: Score,
    beta: Score,
    qdepth: i16,
    tt: &mut super::transtable::TransTable,
) -> Score {
    let sp;
    let stand_pat;
    let sign = match b.side_to_move {
        board::WHITE => 1,
        board::BLACK => -1,
    };
    
    match tt.get(b.zobrist) {
        None => {
            sp = sign * eval(b);
            stand_pat = Score::Value(sp);
            tt.put(b.zobrist, None, -qdepth, stand_pat, super::NodeType::QuiesceEval, beta, Some(sp), false);
        }
        Some(tt_entry) => match tt_entry.node_type {
            NodeType::QuiesceEval => {
                sp = tt_entry.eval.unwrap();
                stand_pat = Score::Value(sp);
            }
            NodeType::QuiesceFull | NodeType::AllNode | NodeType::PvNode => {
                if beta <= tt_entry.beta {
                    return tt_entry.eval_score;
                } else {
                    sp = tt_entry.eval.unwrap();
                    stand_pat = Score::Value(sp); 
                }
            }
            NodeType::QuiesceCut | NodeType::CutNode => {
                if tt_entry.eval_score >= beta {
                    return tt_entry.eval_score;
                } else {
                    sp = tt_entry.eval.unwrap_or_else(|| sign * eval(b));
                    stand_pat = Score::Value(sp); 
                }
            }
            NodeType::None => panic!("Tt.get returned some but type is None"),
        },
        }

    if stand_pat >= beta {
        return beta;
    }
    if alpha < stand_pat {
        alpha = stand_pat;
    }

    // Delta pruning
    match alpha {
        Score::Value(a) => if sp < a - MAX_DELTA {
            return alpha;
        },
        Score::Draw => if sp < 0 - MAX_DELTA {
            return alpha;
        }
        Score::Win(_d) => {
            return stand_pat;
        }
        Score::Loss(_d) => {}
    };

    let mut local_alpha = stand_pat;

    let mut cap_moves_see: Vec<_> = movegen::capturegen::cap_gen(b).into_iter().map(|m| (m, super::see::see_capt(b, &m, b.side_to_move))).collect();
    // if cap_moves.len() == 0 {
    //     tt.put(b.zobrist, None, -qdepth, stand_pat, NodeType::QuiesceFull, beta, Some(sp));
    // }
    cap_moves_see.sort_unstable_by_key(|t| -t.1);
    for (m, see) in cap_moves_see {
        if see < 0 {
            //println!("SEE cut");
            break;
        }
        if let Score::Value(a) = alpha {
            if sp + see < a - SEE_DELTA {
                //println!("SEE delta prune");
                break;
            }
        }
        b.make(&m);
        if !b.is_check(!b.side_to_move).is_safe() {
            b.unmake();
            continue;
        }
        let score = -quiesce(b, -beta, -alpha, qdepth + 1, tt);
        b.unmake();

        if score >= beta  {
            tt.put(b.zobrist, Some(m), -qdepth, score, super::NodeType::QuiesceCut, beta, Some(sp), false);
            return beta;
        }
        
        if score > alpha {
            alpha = score;
        }
        if score > local_alpha {
            local_alpha = score;
        } else {
        }
    }

    tt.put(b.zobrist, None, -qdepth, local_alpha, NodeType::QuiesceFull, beta, Some(sp), false);

    return local_alpha;
}
