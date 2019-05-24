use crate::eval::eval;
use crate::movegen;
use crate::board;
use board::Board;

use super::Score;
use super::NodeType;

const MAX_DELTA: i32 = 1000;
const SEE_DELTA: i32 = 100;

pub fn quiesce(b: &mut Board, mut alpha: Score, beta:Score, qdepth: i16, tt: &mut super::transtable::TransTable ) -> Score {
    match tt.get(b.zobrist) {
        None => {},
        Some(tt_entry) => {
            // TODO: maybe TT move ordering?
            // Always return the stored score: depth in quiesce does not matter
            //return tt_entry.eval_score;
        }
    };

    let sign = match b.side_to_move {
        board::WHITE => 1,
        board::BLACK => -1,
    };
    
    let sp = sign * eval(b);
    let stand_pat = Score::Value(sp);
    
    let mut local_alpha = stand_pat;
    let mut local_alpha_move = None;

    if stand_pat >= beta  {
        //tt.put(b.zobrist, None, -qdepth, beta);
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
    tt.put(b.zobrist, local_alpha_move, -qdepth, local_alpha, NodeType::AllNode);
    return alpha;
}