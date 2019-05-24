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
    match tt.get(b.zobrist) {
        Some(tt_entry) if tt_entry.node_type == NodeType::AllNode && tt_entry.depthleft < 0 && tt_entry.eval_score >= beta => {
            // stand_pat = tt_entry.eval_score;
            // sp = match stand_pat {
            //     Score::Value(v) => v,
            //     Score::Draw => 0,
            //     Score::Win(_d) => std::i32::MAX - 10,
            //     Score::Loss(_d) => std::i32::MIN + 10,
            // };
            return tt_entry.eval_score;
        }
        Some(tt_entry) if tt_entry.node_type == NodeType::CutNode && tt_entry.eval_score >= beta => {
            return tt_entry.eval_score;
        },
        Some(tt_entry) if tt_entry.node_type == NodeType::PvNode && tt_entry.depthleft < 0 && tt_entry.eval_score >= beta => {
            //println!("PvNode hit in quiesce; alpha {}; beta; {}; tt_entry.eval {}", alpha, beta, tt_entry.eval_score);
            return tt_entry.eval_score;
        },
        _ => {
            let sign = match b.side_to_move {
                board::WHITE => 1,
                board::BLACK => -1,
            };
            
            sp = sign * eval(b);
            stand_pat = Score::Value(sp);
        },

    };

    
    let mut local_alpha = stand_pat;
    let mut local_alpha_move = None;

    if stand_pat >= beta  {
        tt.put(b.zobrist, None, -qdepth, stand_pat, NodeType::CutNode);
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

    

    let cap_moves = movegen::capturegen::cap_gen(b);
    for m in cap_moves {
        let see = super::see::see_capt(b, &m, b.side_to_move);
        if see < 0 {
            //println!("SEE cut");
            continue;
        }
        if let Score::Value(a) = alpha {
            if sp + see < a - SEE_DELTA {
                //println!("SEE delta prune");
                continue;
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
            tt.put(b.zobrist, Some(m), -qdepth, score, NodeType::CutNode);
            return beta;
        }
        
        if score > alpha  {
            alpha = score;
            local_alpha = score;
            local_alpha_move = Some(m);
        } else if score > local_alpha {
            local_alpha = score;
            local_alpha_move = Some(m);

        }
    }
    //println!("End of quiescence. Alpha: {}; local_alpha: {}, stand_pat {}", alpha, local_alpha, stand_pat);
    
    match !(local_alpha < alpha) {  // match alpha_raised
        false => tt.put(b.zobrist, local_alpha_move, -qdepth, local_alpha, NodeType::AllNode),
        true => {
            //println!("TtPut PvNode; alpha {}, local_alpha {}", alpha, local_alpha);
            tt.put(b.zobrist, local_alpha_move, -qdepth, local_alpha, NodeType::PvNode);
        },
    };

    return alpha;
}