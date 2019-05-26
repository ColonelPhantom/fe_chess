use crate::eval::eval;
use crate::movegen;
use crate::board;
use board::Board;

use super::Score;
use super::NodeType;

const MAX_DELTA: i32 = 1000;
const SEE_DELTA: i32 = 100;

pub fn quiesce(b: &mut Board, mut alpha: Score, beta:Score, qdepth: i16, tt: &mut super::transtable::TransTable ) -> Score {
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
            tt.put(b.zobrist, None, -qdepth, stand_pat, super::NodeType::QuiesceEval);
        }
        Some(tt_entry) => match tt_entry.node_type {
            NodeType::QuiesceEval | NodeType::PvNode | NodeType::AllNode => {
                stand_pat = tt_entry.eval_score;
                sp = match stand_pat {
                    Score::Value(v) => v,
                    Score::Draw => 0,
                    Score::Win(d) => std::i32::MAX - 1 - d as i32,
                    Score::Loss(d) => std::i32::MIN + 1 + d as i32,
                }
            }
            NodeType::QuiesceCut | NodeType::CutNode => {
                if tt_entry.eval_score >= beta {
                    return beta;
                }
                sp = sign * eval(b);
                stand_pat = Score::Value(sp);
            }
            NodeType::QuiesceFull => {
               return tt_entry.eval_score;

            }
            NodeType::None => panic!("Tt.get returned some but type is None"),
        }
    }

    if stand_pat >= beta  {
        return beta;
    }
    if alpha < stand_pat  {
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
        Score::Win(_d) => {return stand_pat;},
        Score::Loss(_d) => {}
    };

    let mut local_alpha = stand_pat;

    let mut cap_moves = movegen::capturegen::cap_gen(b);
    if cap_moves.len() == 0 {
        tt.put(b.zobrist, None, -qdepth, stand_pat, NodeType::QuiesceFull);
    }
    cap_moves.sort_by_cached_key(|m| {
        -super::see::see_capt(b, &m, b.side_to_move)
    });
    for m in cap_moves {
        // TODO: reuse the value used for sorting
        let see = super::see::see_capt(b, &m, b.side_to_move);
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
            tt.put(b.zobrist, Some(m), -qdepth, score, super::NodeType::QuiesceCut);
            return beta;
        }
        
        if score > alpha  {
            alpha = score;
        }
        if score > local_alpha {
            local_alpha = score;
        } else {
        }
    }

    return local_alpha;
}